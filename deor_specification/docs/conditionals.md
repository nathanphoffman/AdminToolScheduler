<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Conditionals

## `if / else if / else`

Standard conditional blocks. No parentheses around the condition. All branches support multi-line bodies. `else if` is a flat two-word keyword pair — not a nested `if` inside an `else` block.

Deor:
```deor
if val > 10
    do_something()
    do_more()
else if val > 5
    do_medium()
    also_this()
else if val > 0
    do_small()
else
    do_default()
```

Rust:
```rust
if val > 10 {
    do_something();
    do_more();
} else if val > 5 {
    do_medium();
    also_this();
} else if val > 0 {
    do_small();
} else {
    do_default();
}
```

Any number of `else if` chains are allowed. `else` is always last and optional.

---

## No Pattern Matching — By Design

Deor has no `match` keyword and no pattern matching syntax. This is intentional: `match` would be a second, structurally different way to branch, with its own nesting and rules to learn. Keeping dispatch on `if`/`else if` with `is` means there's exactly one way to branch in Deor, using the same comparison operator as everywhere else.

Dispatching on enum variants follows the same `if`/`else if`/`is` pattern — see [Enums — Checking Variants](docs/enums.md#checking-variants).
