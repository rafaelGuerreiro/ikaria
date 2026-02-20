---
name: ikariadb-schema-constraints
description: Use for server schema/table updates to prevent constraint drift and keep contracts aligned with shared domain requirements.
---

# Ikaria Server Schema Constraints

Use this skill for backend schema/table changes under `bins/server`.

## Schema change checklist

- Owning IDs must be `u64` with `#[auto_inc]`, except user identity fields that intentionally use `Identity`.
- Reuse shared constants/enums from `sdks/shared` (world level baseline, direction, skill) instead of redefining them locally.
- Keep domain-specific types in the domain module `types.rs`.
- When requirements say to defer a feature, do not add out-of-scope tables or related server wiring.
- Run event-system compatibility checks for table/field changes that affect emitted events or event consumers.

## Validation before finalize

- Confirm table definitions match reducer/service expectations.
- Confirm shared imports resolve from `ikaria-shared`.
- Confirm no deferred feature tables were introduced.
- Confirm event payloads/subscriptions remain compatible after schema updates.
