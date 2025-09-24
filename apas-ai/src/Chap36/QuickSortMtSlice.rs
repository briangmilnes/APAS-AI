//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 36 (Multi-threaded Slice): Quicksort over `ArraySeqMtEphSlice` without extra copies.

pub mod Chapter36MtSlice {
    use std::thread;

    use rand::{Rng, rng};

    use crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
    use crate::Types::Types::*;

    const MIN_PAR_SLICE: N = 2;

    pub trait Chapter36MtSliceTrait<T: StT + Ord + Send> {
        fn pivot_mt_first(&self, lo: N, hi: N) -> T;
        fn pivot_mt_median3(&self, lo: N, hi: N) -> T;
        fn pivot_mt_random(&self, lo: N, hi: N) -> T;

        fn quick_sort_mt_first(&self);
        fn quick_sort_mt_median3(&self);
        fn quick_sort_mt_random(&self);
    }

    impl<T: StT + Ord + Send> Chapter36MtSliceTrait<T> for ArraySeqMtEphSliceS<T> {
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

        fn quick_sort_mt_first(&self) {
            if self.length() <= 1usize {
                return;
            }
            self.with_exclusive(|data| {
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {
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
                            scope.spawn(|| sort(left));
                            sort(right);
                        });
                    } else {
                        sort(left);
                        sort(right);
                    }
                }
                sort(data);
            });
        }

        fn quick_sort_mt_median3(&self) {
            if self.length() <= 1usize {
                return;
            }
            self.with_exclusive(|data| {
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {
                    let len = data.len();
                    if len <= 1 {
                        return;
                    }
                    let pivot = {
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
                            scope.spawn(|| sort(left));
                            sort(right);
                        });
                    } else {
                        sort(left);
                        sort(right);
                    }
                }
                sort(data);
            });
        }

        fn quick_sort_mt_random(&self) {
            if self.length() <= 1usize {
                return;
            }
            self.with_exclusive(|data| {
                fn sort<T: StT + Ord + Send>(data: &mut [T]) {
                    let len = data.len();
                    if len <= 1 {
                        return;
                    }
                    let mut r = rng();
                    let idx = r.random_range(0..len);
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
                            scope.spawn(|| sort(left));
                            sort(right);
                        });
                    } else {
                        sort(left);
                        sort(right);
                    }
                }
                sort(data);
            });
        }
    }
}
