//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Exercise 21.7 (Comprehension with Conditionals): even elements of a paired with vowels of b.

pub mod Test33Exercise_21_7 {
    use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::Types::Types::*;

fn is_even(x: &N) -> B {
    if *x % 2 == 0 {
        B::True
    } else {
        B::False
    }
}
fn is_vowel(c: &char) -> B {
    match *c {
        | 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => B::True,
        | _ => B::False,
    }
}

/// flatten 〈 〈 (x, y) : y ∈ b | isVowel y 〉 : x ∈ a | isEven x 〉
/// gpt-5-hard: Work: Θ(|a|·|b|), Span: Θ(lg |a|)
fn pair_even_with_vowels(a: &ArraySeqStPerS<N>, b: &ArraySeqStPerS<char>) -> ArraySeqStPerS<Pair<N, char>> {
    let filtered_a = <ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::filter(a, &|x| is_even(x));
    let filtered_b = <ArraySeqStPerS<char> as ArraySeqStPerTrait<char>>::filter(b, &|y| is_vowel(y));

    let nested =
        <ArraySeqStPerS<ArraySeqStPerS<Pair<N, char>>> as ArraySeqStPerTrait<ArraySeqStPerS<Pair<N, char>>>>::tabulate(
            &|i| {
                let x = filtered_a.nth(i);
                <ArraySeqStPerS<Pair<N, char>> as ArraySeqStPerTrait<Pair<N, char>>>::tabulate(
                    &|j| PairLit!(*x, *filtered_b.nth(j)),
                    filtered_b.length(),
                )
            },
            filtered_a.length(),
        );
    <ArraySeqStPerS<Pair<N, char>> as ArraySeqStPerTrait<Pair<N, char>>>::flatten(&nested)
}

#[test]
fn test_pair_even_with_vowels_basic() {
    let a = ArraySeqStPerS::from_vec(vec![1, 2, 3, 4]);
    let b = ArraySeqStPerS::from_vec(vec!['a', 'b', 'e']);
    let s = pair_even_with_vowels(&a, &b);
    let expect = ArraySeqStPerS::from_vec(vec![PairLit!(2, 'a'), PairLit!(2, 'e'), PairLit!(4, 'a'), PairLit!(4, 'e')]);
    assert_eq!(s, expect);
}

#[test]
fn test_pair_even_with_vowels_debug_shape() {
    let a = ArraySeqStPerS::from_vec(vec![2]);
    let b = ArraySeqStPerS::from_vec(vec!['a', 'x']);
    let s = pair_even_with_vowels(&a, &b);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}

}
