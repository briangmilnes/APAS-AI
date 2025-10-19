// Simplified Chap19 extending basic, redefining advanced

mod chap18;
use chap18::ArraySeqStPer as Chap18;

pub mod ArraySeqStPer {
    use super::Chap18;
    
    pub type N = usize;
    pub type ArraySeqStPerS<T> = Chap18::ArraySeqStPerS<T>;
    
    // Re-export BasicTrait so users get it via wildcard import
    pub use Chap18::ArraySeqStPerBasicTrait;

    // Chap19 extends Basic, redefines Advanced methods, adds new ones
    pub trait ArraySeqStPerTrait<T: Clone>: ArraySeqStPerBasicTrait<T> {
        // Redefined from Chap18 Advanced (better algorithms)
        fn empty() -> Self;
        fn singleton(item: T) -> Self;
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> Self;
        
        // New Chap19-specific method
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T>;
    }

    impl<T: Clone> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {
        fn empty() -> Self {
            // Algorithm 19.1: empty = tabulate(lambda i.i, 0)
            Self::tabulate(&|_| unreachable!(), 0)
        }
        
        fn singleton(item: T) -> Self {
            // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1)
            Self::tabulate(&|_| item.clone(), 1)
        }
        
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> Self {
            // Same as Chap18 for now, but could be optimized
            <Self as Chap18::ArraySeqStPerBasicTrait<T>>::from_vec(
                (0..n).map(|i| f(i)).collect()
            )
        }
        
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T> {
            if i < a.length() {
                Some(a.nth(i))
            } else {
                let j = i - a.length();
                if j < b.length() {
                    Some(b.nth(j))
                } else {
                    None
                }
            }
        }
    }
}

