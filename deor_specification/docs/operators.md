<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Operators

## Arithmetic

| Operator | Meaning | Notes |
|---|---|---|
| `+` | Addition (numeric only) | Rejected on strings — see the Strings doc for `s_join`/`s_join_with` |
| `-` | Subtraction | |
| `*` | Multiplication | |
| `/` | Division | Integer division truncates: `5 / 2 = 2` |
| `%` | Modulo / remainder | |
| `-x` | Unary negation | |

**Integer division** truncates toward zero — `5 / 2 = 2`, not `2.5`. Mix `int` and `float` to get a float result: `5.0 / 2 = 2.5`. This follows Rust's behavior and may surprise developers coming from Python or JavaScript.

```deor
int quo = 5 / 2       # 2 — truncated
float flt = 5.0 / 2   # 2.5
```

Deor has no `**` operator. For exponentiation, use `m_pow` from `lib/math.deor` (see the Libs doc):
```deor
base as 2
exp as 10
int val = m_pow(base, exp)    # 1024
```

---

## Comparison

| Operator | Meaning | Rust equivalent |
|---|---|---|
| `is` | Structural equality (always deep) | `==` |
| `is not` | Not equal | `!=` |
| `<` | Less than | `<` |
| `>` | Greater than | `>` |
| `<=` | Less than or equal | `<=` |
| `>=` | Greater than or equal | `>=` |

`is` and `is not` are two-word keyword operators — not symbols.

```deor
val is 5        # equality — val == 5
val is not 5    # inequality — val != 5
```

`not` negates a boolean — it is not a modifier for `is`. Writing `not x is y` is a transpiler error; the required order is `x is not y`:

```deor
if val is not 5    # correct
if not val is 5    # transpiler error
```

`is empty` and `is not empty` test whether a list has zero elements:

Deor:
```deor
if names is empty
    ...
if names is not empty
    ...
```

Rust:
```rust
if names.is_empty() { ... }
if !names.is_empty() { ... }
```

Forced unwrap of a validator type uses the separate `avow` keyword — it is not part of the `is` operator:

```deor
(avow val)    # forced unwrap — panics if not valid (validator types only)
```

See [Validator Types — Forced Unwrap](docs/validator_types.md#forced-unwrap-avow) for full details.

---

## Logical

| Keyword | Meaning | Rust equivalent |
|---|---|---|
| `and` | Logical AND | `&&` |
| `or` | Logical OR | `\|\|` |
| `not` | Logical NOT (unary) | `!` |

```deor
if val > 0 and val < 100
    ...

if not is_valid
    ...
```

`not` is general-purpose negation — it works on any boolean value or expression, not just as part of a comparison. Use it whenever you're flipping a single boolean on its own, with nothing to compare it against:

```deor
if not done
    ...

opposite as not original
```

`opposite as not original` compiles to `let opposite = !original;` — the concise way to negate a value, versus writing out an `if`/`else` that sets `true`/`false` by hand.

Don't reach for `not` to negate the result of a comparison, though — see [Comparison](#comparison) above for the `is not` rule.

---

## Banned Symbolic Operators

The following are **transpiler errors** — they must never appear in Deor source:

| Banned | Use instead |
|---|---|
| `==` | `is` |
| `!=` | `is not` |
| `&&` | `and` |
| `\|\|` | `or` |

Using symbols where keywords are required reads as an error in source review, not just at compile time.

---

## Operator Precedence

Deor follows Rust's operator precedence. From highest to lowest, the operators you'll use in practice:

| Level | Operators | Notes |
|---|---|---|
| 1 (highest) | `not` | Unary logical NOT |
| 2 | `*` `/` `%` | Multiplicative |
| 3 | `+` `-` | Additive |
| 4 | `is` `is not` `<` `>` `<=` `>=` | Comparison |
| 5 | `and` | Logical AND |
| 6 (lowest) | `or` | Logical OR |

```deor
not val and flag           # (not val) and flag  — not binds tighter than and
left + right * mul         # left + (right * mul)    — standard math precedence
val is 0 or num > 5      # (val is 0) or (num > 5) — comparisons before logical
val is not 0 and num > 0  # (val is not 0) and (num > 0)
```

When in doubt, use parentheses to make the intended grouping explicit.

---

## No Bitwise Operators

Bitwise operations (`&`, `|`, `^`, `~`, `<<`, `>>`) are not exposed in Deor. Use a `rust` block for any code that requires bitwise manipulation.

```deor
fn int mask_flags(int flags)
    rust
        flags & 0xFF
```