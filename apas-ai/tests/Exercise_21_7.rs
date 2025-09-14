//! Exercise 21.7 (Comprehension with Conditionals): even elements of a paired with vowels of b.

use apas_ai::Types::Types::*;
use apas_ai::ArraySeqPer::ArraySeqPer::*;
use apas_ai::ArraySeqPerChap18::ArraySeqPerChap18Trait;
use apas_ai::ArraySeqPerChap19::ArraySeqPerChap19Trait;
use apas_ai::ArraySeqPer;

fn is_even(x: &N) -> B { if *x % 2 == 0 { B::True } else { B::False } }
fn is_vowel(c: &char) -> B {
    match *c {
        'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => B::True,
        _ => B::False,
    }
}

/// flatten 〈 〈 (x, y) : y ∈ b | isVowel y 〉 : x ∈ a | isEven x 〉
/// gpt-5-hard: Work: Θ(|a|·|b|), Span: Θ(lg |a|)
fn pair_even_with_vowels(a: &ArrayPerS<N>, b: &ArrayPerS<char>) -> ArrayPerS<(N, char)> {
    let filtered_a: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerChap18Trait>::filter(a, |x| is_even(x));
    let nested: ArrayPerS<ArrayPerS<(N, char)>> =
        <ArrayPerS<ArrayPerS<(N, char)>> as ArraySeqPerChap19Trait>::map(&filtered_a, |x| {
            let vb: ArrayPerS<char> = <ArrayPerS<char> as ArraySeqPerChap18Trait>::filter(b, |y| is_vowel(y));
            <ArrayPerS<(N, char)> as ArraySeqPerChap19Trait>::map(&vb, |y| (*x, *y))
        });
    <ArrayPerS<(N, char)> as ArraySeqPerChap18Trait>::flatten(&nested)
}

#[test]
fn test_pair_even_with_vowels_basic() {
    let a = ArraySeqPer![1, 2, 3, 4];
    let b = ArraySeqPer!['a', 'b', 'e'];
    let s = pair_even_with_vowels(&a, &b);
    let expect = ArraySeqPer![(2, 'a'), (2, 'e'), (4, 'a'), (4, 'e')];
    assert_eq!(s, expect);
}

#[test]
fn test_pair_even_with_vowels_debug_shape() {
    let a = ArraySeqPer![2];
    let b = ArraySeqPer!['a', 'x'];
    let s = pair_even_with_vowels(&a, &b);
    let dbg_str = format!("{:?}", s);
    assert!(!dbg_str.is_empty());
}
