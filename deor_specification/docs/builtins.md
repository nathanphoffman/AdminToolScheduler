<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->

# Built-in Functions
These functions are wired directly into the transpiler — no import, no wrapper needed.

Everything else lives in a standard library file under `lib/` or must be wrapped in a `rust` block. See [Libs](docs/libs.md) for the standard library and custom wrapper patterns.

No shadowing: `print`, `crash`, `len`, `range`, `args`, and `input` aren't reserved keywords in the lexer — they lex as plain identifiers — but the validator hardcodes them as built-in names and rejects any attempt to reuse one as a variable, parameter, function, struct, or type name. See [Syntax — Built-in Function Names](docs/syntax.md#built-in-function-names).

---

## `print`
Writes a value to stdout followed by a newline. Accepts any primitive type.

Deor:
```deor
print("Hello, world!")
print(count)
```

Rust:
```rust
println!("{}", "Hello, world!");
println!("{}", count);
```

Pass a second argument to replace the trailing newline with a string of your choice — a space, a comma, or even an empty string, useful for building output across multiple `print` calls without a line break. Since this is a 2-argument call, the second argument must be a named variable already in scope (the same rule that applies to `range(start, stop)` — see [Enforced Practices](docs/enforced_practices.md#named-arguments-user-defined-functions-only)).

Deor:
```deor
string separator = ", "
print(first, separator)
print(second)
```

Rust:
```rust
print!("{}{}", first.clone(), separator.clone());
println!("{}", second.clone());
```

---

## `len`
Returns the number of elements in a list or the number of characters in a string.

Deor:
```deor
int size = len(rooms)
int chars = len(name)
```

Rust:
```rust
let size: i64 = rooms.len() as i64;
let chars: i64 = name.len() as i64;
```

---

## `range`
Produces an integer sequence for use in `for` loops. Two forms:

| Form | Produces |
|---|---|
| `range(count)` | `0` through `count - 1` |
| `range(start, end)` | `start` through `end - 1` (exclusive upper bound) |

`range` is only valid as the iteration source in a `for` loop — it is not a value and cannot be assigned. See [Loops — Numeric Iteration](docs/loops.md#numeric-iteration) for the full `for` usage and Rust translation.

---

## Iterating Over a List

To loop over every element in a list, write the list name directly after `in` — see [Loops — Collection Iteration](docs/loops.md#collection-iteration).

---

## `crash`
Terminates the program immediately with a message. A `string` is recommended — it produces the clearest panic output. The transpiler accepts exactly one argument and does not enforce the type.

Deor:
```deor
message as "An unknown server problem occurred"
crash(message)
```

Rust:
```rust
panic!("{}", message);
```

The generated `panic!("{}", x)` uses Rust's `Display` trait to format the argument. How other types behave:

| Deor type | Display output |
|---|---|
| `string` | the string value — recommended |
| `int` | decimal integer, e.g. `42` |
| `float` | decimal with fractional part, e.g. `3.14` |
| `bool` | `true` or `false` |

Structs, list shapes, and validator types do not implement `Display` by default and will cause a Rust compile error if passed to `crash`. If you need to crash with a struct or list value, extract a field or build a descriptive string first.

---

## `args()` and `input()` destructuring

Two built-in forms for reading word-split data into named variables. Both use the same `in` destructuring syntax and the same five keywords — any subset, any order.

| Form | Source |
|---|---|
| `(fields) in args()` | command-line arguments passed at launch |
| `(fields) in input()` | one line read from stdin |

```deor
(first, second, third, input_string, input_list) in args()
(first, second, third, input_string, input_list) in input()
```

| Keyword | Type | Value |
|---|---|---|
| `first` | `string` | first word — empty string `""` if not present |
| `second` | `string` | second word |
| `third` | `string` | third word |
| `input_string` | `string` | all words joined with a single space |
| `input_list` | `stringList` | all words as a list |

Missing words default to `""` — no crash. Use `if first is ""` to detect absence.

**`args()` example** — reading CLI flags:
```deor
(first, second) in args()
print(first)
print(second)
```
Running `./prog hello world` prints `hello` then `world`.

**`input()` example** — prompting the user:
```deor
(first, input_list) in input()
print(first)
int count = len(input_list)
print(count)
```
If the user types `hello world extra`, prints `hello` then `3`.

For more than three words, use `input_list` directly (`input_list at 3`, `for item in input_list`, etc.).