//! Chapter 36 (Single-threaded): Quicksort with three pivot strategies over `ArraySeqStEph`.

pub mod Chapter36St {
    use rand::{Rng, rng};

    use crate::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    pub trait Chapter36StTrait<T: StT + Ord> {
        fn pivot_st_first(&self, lo: N, hi: N) -> T;
        fn pivot_st_median3(&self, lo: N, hi: N) -> T;
        fn pivot_st_random(&self, lo: N, hi: N) -> T;

        fn quick_sort_st_first(&mut self);
        fn quick_sort_st_median3(&mut self);
        fn quick_sort_st_random(&mut self);
    }

    impl<T: StT + Ord> Chapter36StTrait<T> for ArraySeqStEphS<T> {
        fn pivot_st_first(&self, lo: N, _hi: N) -> T {
            self.nth(lo).clone()
        }
        fn pivot_st_median3(&self, lo: N, hi: N) -> T {
            let mid = lo + (hi - lo) / 2;
            let x0 = self.nth(lo).clone();
            let xm = self.nth(mid).clone();
            let xl = self.nth(hi - 1).clone();
            if (x0 <= xm && xm <= xl) || (xl <= xm && xm <= x0) {
                xm
            } else if (xm <= x0 && x0 <= xl) || (xl <= x0 && x0 <= xm) {
                x0
            } else {
                xl
            }
        }
        fn pivot_st_random(&self, lo: N, hi: N) -> T {
            let mut r = rng();
            let idx = r.random_range(lo..hi);
            self.nth(idx).clone()
        }

        fn quick_sort_st_first(&mut self) {
            fn swap<T: StT>(a: &mut ArraySeqStEphS<T>, i: N, j: N) {
                if i == j {
                    return;
                }
                let xi = a.nth(i).clone();
                let xj = a.nth(j).clone();
                let _ = a.set(i, xj);
                let _ = a.set(j, xi);
            }
            fn partition<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N, p: &T) -> (N, N) {
                let mut lt = lo;
                let mut i = lo;
                let mut gt = hi;
                while i < gt {
                    let x = a.nth(i).clone();
                    if x < *p {
                        swap(a, lt, i);
                        lt += 1;
                        i += 1;
                    } else if x > *p {
                        gt -= 1;
                        swap(a, i, gt);
                    } else {
                        i += 1;
                    }
                }
                (lt, gt)
            }
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {
                if hi <= lo + 1 {
                    return;
                }
                let p = a.nth(lo).clone();
                let (m1, m2) = partition(a, lo, hi, &p);
                sort(a, lo, m1);
                sort(a, m2, hi);
            }
            let n = self.length();
            sort(self, 0, n);
        }

        fn quick_sort_st_median3(&mut self) {
            fn swap<T: StT>(a: &mut ArraySeqStEphS<T>, i: N, j: N) {
                if i == j {
                    return;
                }
                let xi = a.nth(i).clone();
                let xj = a.nth(j).clone();
                let _ = a.set(i, xj);
                let _ = a.set(j, xi);
            }
            fn median3<T: StT + Ord>(a: &ArraySeqStEphS<T>, lo: N, hi: N) -> T {
                let mid = lo + (hi - lo) / 2;
                let x0 = a.nth(lo).clone();
                let xm = a.nth(mid).clone();
                let xl = a.nth(hi - 1).clone();
                if (x0 <= xm && xm <= xl) || (xl <= xm && xm <= x0) {
                    xm
                } else if (xm <= x0 && x0 <= xl) || (xl <= x0 && x0 <= xm) {
                    x0
                } else {
                    xl
                }
            }
            fn partition<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N, p: &T) -> (N, N) {
                let mut lt = lo;
                let mut i = lo;
                let mut gt = hi;
                while i < gt {
                    let x = a.nth(i).clone();
                    if x < *p {
                        swap(a, lt, i);
                        lt += 1;
                        i += 1;
                    } else if x > *p {
                        gt -= 1;
                        swap(a, i, gt);
                    } else {
                        i += 1;
                    }
                }
                (lt, gt)
            }
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {
                if hi <= lo + 1 {
                    return;
                }
                let p = median3(a, lo, hi);
                let (m1, m2) = partition(a, lo, hi, &p);
                sort(a, lo, m1);
                sort(a, m2, hi);
            }
            let n = self.length();
            sort(self, 0, n);
        }

        fn quick_sort_st_random(&mut self) {
            fn swap<T: StT>(a: &mut ArraySeqStEphS<T>, i: N, j: N) {
                if i == j {
                    return;
                }
                let xi = a.nth(i).clone();
                let xj = a.nth(j).clone();
                let _ = a.set(i, xj);
                let _ = a.set(j, xi);
            }
            fn partition<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N, p: &T) -> (N, N) {
                let mut lt = lo;
                let mut i = lo;
                let mut gt = hi;
                while i < gt {
                    let x = a.nth(i).clone();
                    if x < *p {
                        swap(a, lt, i);
                        lt += 1;
                        i += 1;
                    } else if x > *p {
                        gt -= 1;
                        swap(a, i, gt);
                    } else {
                        i += 1;
                    }
                }
                (lt, gt)
            }
            fn sort<T: StT + Ord>(a: &mut ArraySeqStEphS<T>, lo: N, hi: N) {
                if hi <= lo + 1 {
                    return;
                }
                let mut r = rng();
                let idx = r.random_range(lo..hi);
                let p = a.nth(idx).clone();
                let (m1, m2) = partition(a, lo, hi, &p);
                sort(a, lo, m1);
                sort(a, m2, hi);
            }
            let n = self.length();
            sort(self, 0, n);
        }
    }
}
