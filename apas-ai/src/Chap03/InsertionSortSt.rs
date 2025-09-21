//! Chapter 3 insertion sort over mutable slices.

pub mod InsertionSortSt {

    pub trait InsertionSortStTrait {
        // APAS - work O(n²), span O(n²)
        // gpt-5-codex-medium: work O(n²), span O(n²)
        fn insSort<T: Ord + Clone>(&self, slice: &mut [T]);
    }

    #[derive(Default, Clone, Copy)]
    pub struct InsertionSortSt;

    impl InsertionSortStTrait for InsertionSortSt {
        fn insSort<T: Ord + Clone>(&self, slice: &mut [T]) {
            for i in 1..slice.len() {
                let key = slice[i].clone();
                let mut j = i;
                while j > 0 && slice[j - 1] > key {
                    slice[j] = slice[j - 1].clone();
                    j -= 1;
                }
                slice[j] = key;
            }
        }
    }
}
