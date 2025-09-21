## Checklist Review Process

- When the user requests a new module (code or docs touching `src/`), review must include the full `RustRulesChecklist.md` and `APASRulesChecklist.md`.
- For each checklist item, state `Correct` or `Defect` explicitly; no items may be skipped or marked N/A.
- Present the checklist results before writing to disk unless the user explicitly waives the review.
- If any item is `Defect`, fix it (or explain the pending remediation) before declaring the module ready.
