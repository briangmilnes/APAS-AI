//! Chapter 36 (Multi-threaded): Quicksort with three pivot strategies over `ArraySeqMtEph`.

pub mod Chapter36Mt {
    use std::thread;

    use rand::{rng, Rng};

    use crate::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    // Spawning threads all the way down (always parallelize).
    const MIN_PAR_SLICE: N = 2;

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

    impl<T: StT + Ord + Send> Chapter36MtTrait<T> for ArraySeqMtEphS<T> {
        fn pivot_mt_first(&self, lo: N, _hi: N) -> T { self.nth_cloned(lo) }
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
            if self.length() <= 1 {
                return;
            }

            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {
                let len = data.len();
                if len <= 1 {
                    return;
                }
                let pivot = data[0].clone();
                let mut lt = 0;
                let mut i = 0;
                let mut gt = len;
                while i < gt {
                    if data[i] < pivot {
                        data.swap(lt, i);
                        lt += 1;
                        i += 1;
                    } else if data[i] > pivot {
                        gt -= 1;
                        data.swap(i, gt);
                    } else {
                        i += 1;
                    }
                }
                let (left, mid_and_right) = data.split_at_mut(lt);
                let (_, right) = mid_and_right.split_at_mut(gt - lt);
                if len >= MIN_PAR_SLICE {
                    thread::scope(|scope| {
                        scope.spawn(|| quick_sort(left));
                        quick_sort(right);
                    });
                } else {
                    quick_sort(left);
                    quick_sort(right);
                }
            }

            let mut data = self.to_vec();
            quick_sort(&mut data);
            *self = ArraySeqMtEphS::from_vec(data);
        }
        fn quick_sort_mt_median3(&mut self) {
            if self.length() <= 1 {
                return;
            }

            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {
                let len = data.len();
                if len <= 1 {
                    return;
                }
                let len = data.len();
                let mid = len / 2;
                let last = len - 1;
                let x0 = data[0].clone();
                let xm = data[mid].clone();
                let xl = data[last].clone();
                let pivot = if (x0 <= xm && xm <= xl) || (xl <= xm && xm <= x0) {
                    xm
                } else if (xm <= x0 && x0 <= xl) || (xl <= x0 && x0 <= xm) {
                    x0
                } else {
                    xl
                };
                let mut lt = 0;
                let mut i = 0;
                let mut gt = len;
                while i < gt {
                    if data[i] < pivot {
                        data.swap(lt, i);
                        lt += 1;
                        i += 1;
                    } else if data[i] > pivot {
                        gt -= 1;
                        data.swap(i, gt);
                    } else {
                        i += 1;
                    }
                }
                let (left, mid_and_right) = data.split_at_mut(lt);
                let (_, right) = mid_and_right.split_at_mut(gt - lt);
                if len >= MIN_PAR_SLICE {
                    thread::scope(|scope| {
                        scope.spawn(|| quick_sort(left));
                        quick_sort(right);
                    });
                } else {
                    quick_sort(left);
                    quick_sort(right);
                }
            }

            let mut data = self.to_vec();
            quick_sort(&mut data);
            *self = ArraySeqMtEphS::from_vec(data);
        }
        fn quick_sort_mt_random(&mut self) {
            if self.length() <= 1 {
                return;
            }

            fn quick_sort<T: StT + Ord + Send>(data: &mut [T]) {
                let len = data.len();
                if len <= 1 {
                    return;
                }
                let mut rng_local = rng();
                let idx = rng_local.random_range(0..len);
                let pivot = data[idx].clone();
                let mut lt = 0;
                let mut i = 0;
                let mut gt = len;
                while i < gt {
                    if data[i] < pivot {
                        data.swap(lt, i);
                        lt += 1;
                        i += 1;
                    } else if data[i] > pivot {
                        gt -= 1;
                        data.swap(i, gt);
                    } else {
                        i += 1;
                    }
                }
                let (left, mid_and_right) = data.split_at_mut(lt);
                let (_, right) = mid_and_right.split_at_mut(gt - lt);
                if len >= MIN_PAR_SLICE {
                    thread::scope(|scope| {
                        scope.spawn(|| quick_sort(left));
                        quick_sort(right);
                    });
                } else {
                    quick_sort(left);
                    quick_sort(right);
                }
            }

            let mut data = self.to_vec();
            quick_sort(&mut data);
            *self = ArraySeqMtEphS::from_vec(data);
        }
    }
}
