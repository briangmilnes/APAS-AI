// Simplified Chap18 with split traits

pub mod ArraySeqStPer {
    pub type N = usize;
    
    #[derive(Clone)]
    pub struct ArraySeqStPerS<T: Clone> {
        data: Box<[T]>,
    }

    // Methods that are NEVER redefined - always inherited
    pub trait ArraySeqStPerBasicTrait<T: Clone> {
        fn from_vec(elts: Vec<T>) -> Self;
        fn length(&self) -> N;
        fn nth(&self, index: N) -> &T;
    }

    // Methods that CAN be redefined in later chapters
    pub trait ArraySeqStPerAdvancedTrait<T: Clone> {
        fn empty() -> Self;
        fn singleton(item: T) -> Self;
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> Self;
    }

    impl<T: Clone> ArraySeqStPerBasicTrait<T> for ArraySeqStPerS<T> {
        fn from_vec(elts: Vec<T>) -> Self {
            Self { data: elts.into_boxed_slice() }
        }
        fn length(&self) -> N { self.data.len() }
        fn nth(&self, index: N) -> &T { &self.data[index] }
    }

    impl<T: Clone> ArraySeqStPerAdvancedTrait<T> for ArraySeqStPerS<T> {
        fn empty() -> Self { Self::from_vec(Vec::new()) }
        fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> Self {
            let mut values = Vec::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            Self::from_vec(values)
        }
    }
}

