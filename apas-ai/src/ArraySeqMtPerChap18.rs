//! Chapter 18 algorithms for ArraySeqMtPer multithreaded, just tabulate needed.

pub mod ArraySeqMtPerChap18 {
    use crate::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    pub trait ArraySeqMtPerChap18Trait<T: MtT> {
        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i)))
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T>;
    }

    impl<T: MtT> ArraySeqMtPerChap18Trait<T> for ArrayMtPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {
            let data: Vec<T> = (0..n).map(|i| f(i)).collect();
            ArrayMtPerS::from_vec(data)
        }
    }
}
