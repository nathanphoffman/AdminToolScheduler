<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Variables and Data Types


## Primitive Types

Deor's built-in primitive types and their Rust equivalents:

| Deor | Rust | Notes |
|---|---|---|
| `int` | `i64` | General-purpose integer |
| `float` | `f64` | General-purpose decimal |
| `bool` | `bool` | |
| `string` | `String` | Owned; available as `&str` via `.as_str()` in `rust` blocks |

For raw binary data (HTTP bodies, files, crypto, pixel buffers) use a `raw` variable and handle it entirely inside `rust` blocks. See [`raw` Variables](#raw-variables) below.

Integer literals may contain underscores as visual separators — see [Numeric Literals](#numeric-literals) below.


## Explicit Typing — Runtime Values

Any value from a function call or other runtime computation uses `Type name = expr`. For list types the type is the shape name.

The declared type is checked against the known-type set (primitive, struct, shape, raw type, validator type, or untyped enum). A misspelled or undeclared type name (`Rooom current = fetch_room()`) is a clean transpiler error, not a later `rustc` failure.

Deor:
```deor
int val = m_rand_int(min, max)
string pick = random_room_name(rooms)
roomList result = empty
```

Rust:
```rust
let val: i64 = m_rand_int(min, max);
let pick: String = random_room_name(&rooms);
let mut result: Vec<i64> = Vec::new();
```

---

## `raw` Variables

Some things are awkward to build in Deor — a `HashMap`, a compiled regex, a connection pool, etc. The "type" `raw` is the escape hatch: a `rust` block builds the thing once, hands 
it back as an opaque value, and Deor carries that value around without needing to understand what's inside it.  `raw` has no type annotation, no Deor operators, 
and can't appear in Deor expressions or struct fields. It's only ever produced by a function call and only ever consumed inside a `rust` block.

```deor
fn Index build_index()
    rust
        entries.iter()
            .map(|e| (e.key.clone(), e.value.clone()))
            .collect::<std::collections::HashMap<String, String>>()

raw index = build_index()
```

See [Rust Interop](docs/interop.md) for full documentation, rules, the build-once pattern, and how a top-level `raw TypeName` declaration is used to share a reference-counted value 
across functions (Deor's only global-like pattern).

---

## Truthiness

Implicit truthiness hides a decision — is `if my_int` checking for nonzero, or for "was this ever set"? Deor makes you write the comparison you actually mean. 
**Only `bool` has a presence check.** Plain `int`, `float`, `string`, `list`, and structs are never truthy or falsy on their own — use explicit comparisons:

```deor
if len(my_list) > 0    # correct — explicit non-empty check
if my_list             # transpiler error — list has no truthiness
if my_bool             # correct - booleans are the only type where this is valid

if my_int is not 0     # correct
if my_int              # transpiler error

if my_string is not "" # correct
if my_string           # transpiler error
```

Validator types use `is valid` / `is not valid` — not bare truthiness. See [Validator Types — is valid / is not valid](docs/validator_types.md#is-valid-is-not-valid).

```deor
if sqft is valid        # correct
if sqft is not valid    # correct
if sqft                 # transpiler error — use is valid/is not valid
```

```rust
if area.is_some() {
    let val: i64 = area.unwrap().0;
}
if area.is_none() {
    // not valid
}
```

---

## Structs (`struct`)

Struct declaration, construction, destructuring, and record update all live on their own page — see [Structs](docs/structs.md).

---

## `as` — Type-Inferred Bindings
`as` creates a binding whose type is derived from the right-hand side at compile time. It has four valid forms:

### Scalar literals
The type is inferred from the literal value.

Deor:
```deor
sum as 0
label as "Office"
flag as true
rate as 3.14
```

Rust:
```rust
let sum = 0;
let label = "Office".to_string();
let flag = true;
let rate = 3.14_f64;
```


**What `as` is not for:**

- **Validator type bindings** — `as` can't tell whether you want a plain `int` or a `Squarefeet` validator (predicate run, `Option<T>` result); use explicit `ValidatorType name = value` instead — see [Validator Type Bindings](#validator-type-bindings) below.
- **Type annotation** — `as` never takes an explicit type prefix.
- **Move transfer** — `as` always clones, so there's nothing for `move` to opt out of; combining them is rejected. Use a typed `=` declaration if you need `move`.

```deor
area as 9             # transpiler error — int or Squarefeet? use Squarefeet area = 9
int count as 0        # transpiler error — annotation not allowed with as; use int count = 0
a as move b           # transpiler error — as always clones, move has nothing to do
```

Record update (`with`) uses `as` — the type is known from the source struct. See [Structs — Record Update](docs/structs.md#record-update-with).

### List construction

A list literal `[item1, item2, ...]` constructs a list. All items must be named variables of the same type already in scope.

Deor:
```deor
rooms as [kitchen, office, bedroom]    # type inferred from items (all Room)
```

Rust:
```rust
let rooms = vec![kitchen.clone(), office.clone(), bedroom.clone()];
```

`[]` is never valid for an empty list — `as` can't infer an element type from nothing, so use `empty` with an explicit shape type instead. See [Collections — Empty List](docs/collections.md#empty-list).

```deor
result as []              # transpiler error — element type unknown
result as empty           # correct usage
```

---

## Validator Types (`type`)

A type that carries its own "is this actually valid?" check, for values that can be built fine but still not make sense — a negative area, an out-of-range roll,
or simply nothing assigned yet (Deor's stand-in for `null`/`undefined`). This is a large enough feature to have its own page: see [Validator Types](docs/validator_types.md)
for how it works, declaration rules, `is valid`/`is not valid`, `avow`, struct fields, and function returns.

Validator types are the only types that can be assigned no value, this is the closest to null/undefined as exists in Deor.

Deor:
```deor
Roll best
```

Reassigning a validator type re-runs the predicate. The variable may transition between valid and not valid.

```deor
Squarefeet area = 9   # valid
area = 16             # valid
int raw = get_user_input()
area = raw            # valid or not valid — predicate runs at runtime
```

---

## `const` — Immutable Typed Bindings

`const` declares a typed binding that is explicitly immutable. The transpiler will never emit `let mut` for a `const` 
variable, even if the surrounding code would otherwise infer mutability.

`const` names must be `SCREAMING_SNAKE_CASE` — all caps, words separated by underscores. This distinguishes constants from regular 
variables at a glance and signals that the value is fixed for the lifetime of the scope.

Deor:
```deor
const string PIPE = "|"
const int MAX_RETRIES = 3
```

Rust:
```rust
let PIPE: String = "|".to_string();
let MAX_RETRIES: i64 = 3;
```

**`const` vs plain typed binding:** a plain `string pipe = "|"` is also immutable if never reassigned, but `const` 
makes the intent explicit and guarantees it at the transpiler level. Use `const` for values that should never change.

**`const` vs `as`:** both produce immutable bindings. `const` requires an explicit type; `as` infers the type from the 
literal. Use `const` when the type must be stated, `as` for simple literals where inference is unambiguous.

---

## Numeric Literals

Underscores may appear anywhere in a numeric literal as a visual separator. They are stripped by the transpiler and have no effect on the value.

Deor:
```deor
int population = 1_000_000
float rate = 0.000_001
int port = 8_080
```

Rust:
```rust
let population: i64 = 1_000_000;
let rate: f64 = 0.000_001;
let port: i64 = 8_080;
```

Underscore placement is free-form — `1_000_000`, `10_00_00`, and `1000000` are all the same value.

Hex literals (`0xFF`) and binary literals (`0b1010`) are deferred to v2. Use a `rust` block for code that requires them.

---