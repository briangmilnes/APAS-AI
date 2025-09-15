//! Chapter 19 algorithms for ArraySeqMtPer, just the one multi-threaded set of code that Umut and Guy snuck into this chapter.

pub mod ArraySeqMtPerChap19 {
    use std::sync::Mutex;

    use crate::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::ArraySeqMtPerChap18::ArraySeqMtPerChap18::*;
    use crate::Types::Types::*;

    pub trait ArraySeqMtPerChap19Trait<T: MtT> {
        fn inject(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn atomicWrite(
            values_with_change_number: &mut ArrayMtPerS<Pair<T, N>>,
            changes: &ArrayMtPerS<Pair<N, T>>,
            change_index: N,
        );
        /// APAS: Work Θ(|values|+|changes|), Span Θ(1) PRAM
        fn inject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;
        fn AtomicWriteLowestChangeNumberWins(
            values_with_change_number: &ArrayMtPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayMtPerS<Pair<N, T>>,
            change_index: N,
        );
        /// APAS: Work Θ(|values|+|changes|), Span Θ(1) PRAM
        fn ninject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;
        fn AtomicWriteHighestChangeNumberWins(
            values_with_change_number: &ArrayMtPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayMtPerS<Pair<N, T>>,
            change_index: N,
        );
    }

    impl<T: MtT + Clone> ArraySeqMtPerChap19Trait<T> for ArrayMtPerS<T> {
        fn inject(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {
            let n = values.length();
            let mut with_num: ArrayMtPerS<Pair<T, N>> =
                <ArrayMtPerS<Pair<T, N>> as ArraySeqMtPerChap18Trait<Pair<T, N>>>::tabulate(
                    |i| Pair(values.nth(i).clone(), n),
                    n,
                );
            for ci in 0..changes.length() {
                Self::atomicWrite(&mut with_num, changes, ci);
            }
            <ArrayMtPerS<T> as ArraySeqMtPerChap18Trait<T>>::tabulate(|i| with_num.nth(i).0.clone(), n)
        }

        fn atomicWrite(
            values_with_change_number: &mut ArrayMtPerS<Pair<T, N>>,
            changes: &ArrayMtPerS<Pair<N, T>>,
            change_index: N,
        ) {
            let Pair(loc, val) = changes.nth(change_index).clone();
            if loc < values_with_change_number.length() {
                let Pair(_, num) = values_with_change_number.nth(loc).clone();
                if change_index < num {
                    let _ = *values_with_change_number =
                        values_with_change_number.set(loc, Pair(val, change_index)).unwrap();
                }
            }
        }

        fn inject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {
            let n = values.length();
            let with_num: ArrayMtPerS<Mutex<Pair<T, N>>> =
                <ArrayMtPerS<Mutex<Pair<T, N>>> as ArraySeqMtPerChap18Trait<Mutex<Pair<T, N>>>>::tabulate(
                    |i| Mutex::new(Pair(values.nth(i).clone(), n)),
                    n,
                );
            std::thread::scope(|scope| {
                for ci in 0..changes.length() {
                    let vr = &with_num;
                    let cr = changes;
                    scope.spawn(move || {
                        Self::AtomicWriteLowestChangeNumberWins(vr, cr, ci);
                    });
                }
            });
            <ArrayMtPerS<T> as ArraySeqMtPerChap18Trait<T>>::tabulate(
                |i| {
                    let g = with_num.nth(i).lock().unwrap();
                    g.0.clone()
                },
                n,
            )
        }

        fn AtomicWriteLowestChangeNumberWins(
            values_with_change_number: &ArrayMtPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayMtPerS<Pair<N, T>>,
            change_index: N,
        ) {
            let Pair(loc, val) = changes.nth(change_index).clone();
            if loc >= values_with_change_number.length() {
                return;
            }
            let mut g = values_with_change_number.nth(loc).lock().unwrap();
            let Pair(ref mut cur, ref mut num) = *g;
            if change_index < *num {
                *cur = val;
                *num = change_index;
            }
        }

        fn ninject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {
            let n = values.length();
            let with_num: ArrayMtPerS<Mutex<Pair<T, N>>> =
                <ArrayMtPerS<Mutex<Pair<T, N>>> as ArraySeqMtPerChap18Trait<Mutex<Pair<T, N>>>>::tabulate(
                    |i| Mutex::new(Pair(values.nth(i).clone(), 0)),
                    n,
                );
            std::thread::scope(|scope| {
                for ci in 0..changes.length() {
                    let vr = &with_num;
                    let cr = changes;
                    scope.spawn(move || {
                        Self::AtomicWriteHighestChangeNumberWins(vr, cr, ci);
                    });
                }
            });
            <ArrayMtPerS<T> as ArraySeqMtPerChap18Trait<T>>::tabulate(
                |i| {
                    let g = with_num.nth(i).lock().unwrap();
                    g.0.clone()
                },
                n,
            )
        }

        fn AtomicWriteHighestChangeNumberWins(
            values_with_change_number: &ArrayMtPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayMtPerS<Pair<N, T>>,
            change_index: N,
        ) {
            let Pair(loc, val) = changes.nth(change_index).clone();
            if loc >= values_with_change_number.length() {
                return;
            }
            let mut g = values_with_change_number.nth(loc).lock().unwrap();
            let Pair(ref mut cur, ref mut num) = *g;
            if change_index >= *num {
                *cur = val;
                *num = change_index;
            }
        }
    }
}


