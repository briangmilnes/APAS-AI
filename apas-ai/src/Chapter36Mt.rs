//! Chapter 36 (Multi-threaded): Quicksort with three pivot strategies over `ArraySeqMtEph`.

pub mod Chapter36Mt {
    use std::thread;

    use rand::{Rng, rng};

    use crate::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    const MIN_PAR_SLICE: N = 2048;

    // SAFETY: Quicksort can run without explicit locks because each recursive call only
    // receives a disjoint slice of the working buffer. The `partition` step mutates the
    // range `[lo, hi)` exclusively, and the returned indices `lt` and `gt` split that
    // range into three non-overlapping regions: `< pivot`, `== pivot`, and `> pivot`.
    // After partitioning we recurse on `[lo, lt)` and `[gt, hi)`. These slices are
    // disjoint, so at most one thread ever writes to any element. Rust’s borrow checker
    // cannot express this dynamic separation, so we copy into a Vec and use
    // `thread::scope` to spawn threads with unique `&mut [T]` slices. The scope fully
    // joins before returning, ensuring no references escape and eliminating the need for
    // locks or atomics in the sorting logic itself.

    pub trait Chapter36MtTrait<T: StT + Ord + Send> {
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;

        fn quick_sort_mt_first(&mut self);
        fn quick_sort_mt_median3(&mut self);
        fn quick_sort_mt_random(&mut self);
    }

    type PivotFn<T> = fn(&[T]) -> T;

    fn partition<T: StT + Ord>(data: &mut [T], pivot: &T) -> (usize, usize) {
        let mut lt = 0;
        let mut i = 0;
        let mut gt = data.len();
        while i < gt {
            if data[i] < *pivot {
                data.swap(lt, i);
                lt += 1;
                i += 1;
            } else if data[i] > *pivot {
                gt -= 1;
                data.swap(i, gt);
            } else {
                i += 1;
            }
        }
        (lt, gt)
    }

    fn split_regions<T>(data: &mut [T], lt: usize, gt: usize) -> (&mut [T], &mut [T]) {
        let (left, mid_and_right) = data.split_at_mut(lt);
        let (_, right) = mid_and_right.split_at_mut(gt - lt);
        (left, right)
    }

    fn pivot_first<T: StT + Ord>(data: &[T]) -> T {
        data[0].clone()
    }

    fn pivot_median3<T: StT + Ord>(data: &[T]) -> T {
        let len = data.len();
        if len <= 2 {
            return data[len / 2].clone();
        }
        let mid = len / 2;
        let last = len - 1;
        let x0 = data[0].clone();
        let xm = data[mid].clone();
        let xl = data[last].clone();
        if (x0 <= xm && xm <= xl) || (xl <= xm && xm <= x0) {
            xm
        } else if (xm <= x0 && x0 <= xl) || (xl <= x0 && x0 <= xm) {
            x0
        } else {
            xl
        }
    }

    fn pivot_random<T: StT + Ord>(data: &[T]) -> T {
        let mut r = rng();
        let idx = r.random_range(0..data.len());
        data[idx].clone()
    }

    fn quick_sort_parallel_with<T: StT + Ord + Send>(data: &mut [T], pivot_fn: PivotFn<T>) {
        let len = data.len();
        if len <= 1 {
            return;
        }
        let pivot = pivot_fn(data);
        let (lt, gt) = partition(data, &pivot);
        let (left, right) = split_regions(data, lt, gt);
        if len >= MIN_PAR_SLICE {
            thread::scope(|scope| {
                scope.spawn(|| quick_sort_parallel_with(left, pivot_fn));
                quick_sort_parallel_with(right, pivot_fn);
            });
        } else {
            quick_sort_parallel_with(left, pivot_fn);
            quick_sort_parallel_with(right, pivot_fn);
        }
    }

    fn sort_with_pivot<T: StT + Ord + Send>(a: &mut ArraySeqMtEphS<T>, pivot_fn: PivotFn<T>) {
        if a.length() <= 1 {
            return;
        }
        let mut data = a.to_vec();
        quick_sort_parallel_with(&mut data, pivot_fn);
        *a = ArraySeqMtEphS::from_vec(data);
    }

    fn quick_sort_mt_first_impl<T: StT + Ord + Send>(a: &mut ArraySeqMtEphS<T>) {
        sort_with_pivot(a, pivot_first::<T>);
    }

    fn quick_sort_mt_median3_impl<T: StT + Ord + Send>(a: &mut ArraySeqMtEphS<T>) {
        sort_with_pivot(a, pivot_median3::<T>);
    }

    fn quick_sort_mt_random_impl<T: StT + Ord + Send>(a: &mut ArraySeqMtEphS<T>) {
        sort_with_pivot(a, pivot_random::<T>);
    }

    impl<T: StT + Ord + Send> Chapter36MtTrait<T> for ArraySeqMtEphS<T> {
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T {
            self.nth_cloned(lo)
        }
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T {
            let mid = lo + (hi - lo) / 2;
            let x0 = self.nth_cloned(lo);
            let xm = self.nth_cloned(mid);
            let xl = self.nth_cloned(hi - 1);
            if (x0 <= xm && xm <= xl) || (xl <= xm && xm <= x0) {
                xm
            } else if (xm <= x0 && x0 <= xl) || (xl <= x0 && x0 <= xm) {
                x0
            } else {
                xl
            }
        }
        fn pivot_mt_random(&self, lo: N, hi: N) -> T {
            let mut r = rng();
            let idx = r.random_range(lo..hi);
            self.nth_cloned(idx)
        }

        fn quick_sort_mt_first(&mut self) {
            quick_sort_mt_first_impl(self);
        }
        fn quick_sort_mt_median3(&mut self) {
            quick_sort_mt_median3_impl(self);
        }
        fn quick_sort_mt_random(&mut self) {
            quick_sort_mt_random_impl(self);
        }
    }
}
