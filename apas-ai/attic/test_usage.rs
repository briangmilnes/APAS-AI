// User code importing Chap19

mod chap18;
mod chap19;

use chap19::ArraySeqStPer::ArraySeqStPer::*;

fn main() {
    // Works - from BasicTrait (inherited via re-export)
    let seq = ArraySeqStPerS::from_vec(vec![1, 2, 3]);
    let len = seq.length();  // ✅ NO ambiguity
    let val = seq.nth(0);    // ✅ NO ambiguity
    
    // Works - from Chap19 (redefined)
    let e = ArraySeqStPerS::<i32>::empty();      // ✅ NO ambiguity - only Chap19Trait in scope
    let s = ArraySeqStPerS::singleton(42);        // ✅ NO ambiguity
    let t = ArraySeqStPerS::tabulate(&|i| i, 5); // ✅ NO ambiguity
    
    // Works - Chap19 new method
    let a = ArraySeqStPerS::from_vec(vec![1, 2]);
    let b = ArraySeqStPerS::from_vec(vec![3, 4]);
    let v = ArraySeqStPerS::select(&a, &b, 2);   // ✅ NO ambiguity
    
    println!("len={}, val={}, selected={:?}", len, val, v);
}

