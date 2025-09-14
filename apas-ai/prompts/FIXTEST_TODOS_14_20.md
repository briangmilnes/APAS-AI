FixTest TODOs for 14–20
=======================

General FixTest conditions
- Imports/APIs must match Per vs Eph and Chap18 vs Chap19.
- One test per function; split combined tests.
- Use assert_eq on values; only use formatting asserts for Debug/Display tests.
- Ensure data structures have PartialEq/Eq and iter(); add where missing.
- Ignore any Foo* files in src and tests.
- Show full cargo nextest output per file.

14. /home/milnes/APASVERUS/APAS-AI/apas-ai/tests/14_TestArraySeqEphChap19.rs
- Convert all Per→Eph: ArraySeqEphS, ArraySeqEphTrait, ArraySeqEphChap18Trait, ArraySeqEphChap19Trait.
- Adjust select assertions to Option<T> (not references).
- Ensure update uses Eph Chap18 update(&mut ...).
- Run: cargo nextest run --test 14_TestArraySeqEphChap19 --nocapture

15. /home/milnes/APASVERUS/APAS-AI/apas-ai/tests/15_TestAVLTreeSeqPer.rs
- Ensure Per-only imports: AVLTreeSeqPerS, AVLTreeSeqPerTrait (and Chap18 if used).
- Add iterator test collecting inorder values via iter().
- Run: cargo nextest run --test 15_TestAVLTreeSeqPer --nocapture

16. /home/milnes/APASVERUS/APAS-AI/apas-ai/tests/16_TestAVLTreeSeqPerChap18.rs
- Use AVLTreeSeqPerChap18Trait only; verify tabulate/map/append/filter produce expected ArrayPerS via to_arrayseq().
- Add iterator test over a Chap18-produced tree.
- Run: cargo nextest run --test 16_TestAVLTreeSeqPerChap18 --nocapture

17. /home/milnes/APASVERUS/APAS-AI/apas-ai/tests/17_TestAVLTreeSeqPerChap19.rs
- Use AVLTreeSeqPerChap19Trait; cover select/append/append2/deflate/filter.
- Add iterator test after Chap19 pipeline.
- Run: cargo nextest run --test 17_TestAVLTreeSeqPerChap19 --nocapture

18. /home/milnes/APASVERUS/APAS-AI/apas-ai/tests/18_TestAVLTreeSeqEph.rs
- Use AVLTreeSeqEphS and AVLTreeSeqEphTrait; Eph-only semantics.
- Ensure PartialEq/Eq exists; verify iter(); add iterator test.
- Run: cargo nextest run --test 18_TestAVLTreeSeqEph --nocapture

19. /home/milnes/APASVERUS/APAS-AI/apas-ai/tests/19_TestAVLTreeSeqEphChap18.rs
- Use AVLTreeSeqEphChap18Trait; Eph semantics (update via &mut when needed).
- One test per function; add iterator test over Chap18 result.
- Run: cargo nextest run --test 19_TestAVLTreeSeqEphChap18 --nocapture

20. /home/milnes/APASVERUS/APAS-AI/apas-ai/tests/20_TestAVLTreeSeqEphChap19.rs
- Use AVLTreeSeqEphChap19Trait; adjust select to Option<T>.
- Add iterator test after Chap19 operations.
- Run: cargo nextest run --test 20_TestAVLTreeSeqEphChap19 --nocapture
