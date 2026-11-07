<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Immutability

## Mutability Rules

A struct handed to a function or dropped into a list shouldn't change out from under the code still holding it, so structs are immutable — the only way to get a "changed" version is `with`, which builds a new one, or recomposing it entirely. Lists and primitives don't carry that risk (growing a list or bumping a counter isn't rewriting data someone else is relying on), so they stay mutable.

See [Structs — Record Update](docs/structs.md#record-update-with) for how `with` works.

| Kind | Mutability | Notes |
|---|---|---|
| Primitives (`int`, `float`, `bool`, ...) | Mutable value types | `val = val + 1` always legal |
| `struct` | **Immutable** | No field-assignment syntax exists. The only way to get a "changed" struct is `with` |
| `list` | Mutable container | `at end =`/`remove at` for growable lists; elements may themselves be immutable structs |

---

## Equality

`is` is **always structural** — Deor derives `PartialEq` on all structs, so equality compares field values, not identity. In the generated Rust, `is` maps to `==`.
