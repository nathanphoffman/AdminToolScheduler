<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Syntax

## Block Structure (No Colons)

Indentation alone opens a block after `fn`, `if`, `for`, `type`, `struct`, `shape`, `enum`, or `rust`. No colon is written.

```deor
fn int abs(int val)
    if val < 0
        -val
    else
        val
```

```rust
fn abs(val: i64) -> i64 {
    if val < 0 {
        -val
    } else {
        val
    }
}
```

**Conversion notes:** each indented block becomes a `{ }` block in the generated Rust — one level of indentation in, one level of `{ }` out, with nothing subtle happening in between.

---

## One Statement Per Line

No line continuations except inside delimiters. Long expressions wrap inside `()` or `[]`:

```deor
Connection conn = Connect(
    host,
    port,
    timeout
)
```

```rust
let conn: Connection = Connect(
    host,
    port,
    timeout
);
```

**Conversion notes:** a comma before the closing `)`/`]` is optional — the transpiler accepts the argument list with or without one. See [Best Practices](docs/best_practices.md#multi-line-calls-no-trailing-comma) for the recommended style.

---

## Comments

`#` starts a line comment. Everything from `#` to end of line is ignored. No block comments.

```deor
# this is a comment
int val = 5    # inline comment
```

```rust
// this is a comment
let val: i64 = 5;    // inline comment
```

Only `#` is valid — `//`, `/*`, `*/`, and `--` are syntax errors in Deor.

**Conversion notes:** `#` → `//` in generated Rust. No multi-line comment form is needed in source since generated Rust is not intended to be hand-read.

---

## Reserved Words

These identifiers have fixed meaning in Deor and cannot be used as variable, function, parameter, struct, or type names.

### Block Headers
Open an indented block when followed by a newline.

| Word | Use |
|---|---|
| `fn` | Function declaration |
| `if`,`else`,`else if` | Conditional block |
| `for` | Loop |
| `type` | Validator type declaration |
| `struct` | Struct declaration (`struct`) |
| `shape` | Shape declaration (`shape name = list of T` / `func of T to O`) |
| `enum` | Enum declaration — named variant type |
| `rust` | Inline Rust block |
| `block` | Scoped block — variables declared inside do not escape into the surrounding scope |
| `macro` | Named block of code inlined at every `macro_run` call site, body always contained (like `block`) — see [Macros](docs/macros.md) |
| `unsafe_macro` | Same as `macro`, but the body is **not** contained — declared variables leak into the caller. Cannot call, or be called from inside, another `unsafe_macro` — see [Macros — `unsafe_macro`](docs/macros.md#unsafe_macro-deliberately-leaking-state) |

### Statement Keywords

| Word | Use |
|---|---|
| `return` | Return a value from a function |
| `avow` | Forced unwrap of a validator type — panics if not valid |
| `break` | Exit the innermost loop |
| `continue` | Skip to the next loop iteration |
| `move` | Transfer ownership instead of cloning — argument, loop, or assignment |
| `const` | Explicitly non-mutable typed binding — emits `let` instead of `let mut`; name must be `SCREAMING_SNAKE_CASE` |
| `macro_run` | Inlines a `macro` definition's body at this call site — transpiler error if the target is an `unsafe_macro` — see [Macros](docs/macros.md) |
| `unsafe_macro_run` | Inlines an `unsafe_macro` definition's body at this call site — transpiler error if the target is a plain `macro` — see [Macros — `unsafe_macro`](docs/macros.md#unsafe_macro-deliberately-leaking-state) |

### Operators and Expression Keywords

| Word | Use |
|---|---|
| `and` | Logical AND (`&&`) |
| `or` | Logical OR (`\|\|`) |
| `not` | Logical NOT (`!`) |
| `is` | Structural equality (`is`) and inequality (`is not`) |
| `in` | Destructuring / import / loop iteration source |
| `as` | Shape-derived binding |
| `with` | Record update (inside `as` binding) |
| `at` | Index access and write (`list at idx`, `list at idx = val`, `list at end = val`) |
| `end` | Reserved keyword — "end of list" in `list at end = val`, cannot be used as a variable name |
| `of` | Element type connector in shape declarations (`list of Room`) |
| `to` | Return type connector in func shapes (`func of Room to bool`) |

### Values

| Word | Use |
|---|---|
| `true` | Boolean true |
| `false` | Boolean false |
| `empty` | Empty initial value for list shapes — `Vec::new()` (declaration only) |

### Built-in Type Keywords

| Word | Use |
|---|---|
| `list` | Parameterized list — always used inside a `shape` declaration |
| `func` | Parameterized function type — always used inside a `shape` declaration |

**Note:** `remove` is a reserved mutation verb for lists and cannot be used as an identifier. `range` is a for-loop-only construct — it is not a callable function and cannot be used outside a `for` header (e.g. assigned to a variable or passed as an argument). `end` is a reserved keyword — only valid as `list at end = val`, and cannot be used as a variable name. `valid` is a reserved keyword — only valid after `is` or `is not`. `none` is a reserved keyword — it exists solely so `return none` produces a clear error (see [Functions](docs/functions.md)); it is not a usable value and cannot be used as an identifier.

### Built-in Function Names

Rust-only keywords (`box`, `match`, `impl`, `loop`, `ref`, `mod`, `trait`, `let`, `mut`, `pub`, `use`, `self`, `Self`, and the rest of Rust's reserved and future-reserved words) are also rejected as names: Deor doesn't reserve them, but they would produce Rust that fails to compile. Names that merely contain one (`boxes`, `looper`) are fine.

`print`, `crash`, `len`, `range`, `args`, and `input` are not lexer keywords — they're plain identifiers wired to specific codegen behavior (see [Builtins](docs/builtins.md)). Despite not being reserved words, they cannot be used as a variable, parameter, function, struct, or type name, and cannot be shadowed — doing so is a transpiler error. This is a deliberate exception to normal [variable shadowing](docs/enforced_practices.md#variable-shadowing): every other identifier can be shadowed, but redefining a built-in's name would silently break the behavior every other use of it relies on.

### Banned Rust Type Names

The following Rust type names are rejected as identifiers — using them as a variable, parameter, struct, or type name is a transpiler error:

`Option`, `Vec`, `Box`, `Rc`, `Arc`, `Result`

These collide with Rust types that the transpiler emits internally. They are banned even where no actual naming conflict would occur.
