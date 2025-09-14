FixTest Plan
============

Scope: Process each N_Test file under /home/milnes/APASVERUS/APAS-AI/apas-ai/tests one-by-one.

For each file:
1) Fix imports to match Per/Eph and Chap18/Chap19 naming.
2) Remove duplicated or mismatched tests.
3) Run: cargo nextest run --test <FILE> --nocapture, fix errors until green.
4) Add missing coverage: one test per function; add Debug/Display tests explicitly.
5) Prefer assert_eq on values (not formats) unless testing formatting.
6) Ensure data structures used have PartialEq/Eq and iter() implemented.
7) Keep Per/Eph separation; use only the correct macro and trait set.
8) Show full nextest output after each file.
9) Do not stop for review until the batch is complete.

Status:
- Completed: 01-13 fixed/green.
  * Examples: /home/milnes/APASVERUS/APAS-AI/apas-ai/tests/10_TestArraySeqPerChap18.rs, /home/milnes/APASVERUS/APAS-AI/apas-ai/tests/11_TestArraySeqPerChap19.rs, /home/milnes/APASVERUS/APAS-AI/apas-ai/tests/12_TestArraySeqEph.rs, /home/milnes/APASVERUS/APAS-AI/apas-ai/tests/13_TestArraySeqEphChap18.rs.
- Pending: /home/milnes/APASVERUS/APAS-AI/apas-ai/tests/14_TestArraySeqEphChap19.rs, iterator tests for 01-08.
