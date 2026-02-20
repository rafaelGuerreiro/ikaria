---
name: ikaria-requirement-lockstep
description: Use for implementing requests in strict lockstep by capturing constraints, mapping them to files, and verifying each one before finalize.
---

# Ikaria Requirement Lockstep Workflow

Use this skill when translating user requirements into code changes.

## Requirement-capture workflow

1. Extract all explicit requirements as numbered constraints.
2. Map each constraint to a file-level implementation item (target file + intended change).
3. Implement only the mapped items, keeping edits minimal and in scope.
4. Verify each numbered constraint against the resulting file content/diff.
5. Finalize only after every constraint has an explicit satisfied/not-satisfied check.

## Constraint satisfaction checklist

- [ ] Every explicit requirement is captured as a numbered constraint.
- [ ] Every constraint is mapped to at least one file-level implementation item.
- [ ] Each mapped item is implemented in the expected file(s).
- [ ] No requirement detail is missed before final response.
- [ ] No out-of-scope additions were introduced.
- [ ] Final verification marks every constraint as satisfied.
