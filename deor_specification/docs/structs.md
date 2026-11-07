<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Structs

## Declaration

```deor
struct Room
    Squarefeet area
    string name
    bool occupied
```

```rust
#[derive(Clone, PartialEq, Debug)]
struct Room {
    area: Option<Squarefeet>,
    name: String,
    occupied: bool,
}
```

Struct fields may be primitives, validator types, list shapes, or other structs. Func shapes as struct fields are a transpiler error — structs are pure data. See [Shapes — Func Shapes in Structs](docs/shapes.md#func-shapes-in-structs).

A struct can't contain itself, either as a direct field or through a validator type wrapping it — that is a transpiler error, since the struct would have infinite size. A validator type can wrap a struct: see [Validator Types Over Structs](docs/validator_types.md#validator-types-over-structs).

Field names follow the same min-length and snake_case rules as variable names, and can't shadow a built-in function name (`print`, `crash`, `len`, `range`, `args`, `input`) — `struct Foo { int print }` is a transpiler error. Field *types* are validated too — a misspelled or undeclared type name (`Squarefeat area` instead of `Squarefeet area`) is a clean transpiler error, not a later `rustc` failure.

There are no per-field visibility modifiers — all fields are always accessible via destructuring whenever the struct itself is in scope.

Structs are immutable — the only way to get a "changed" version is [Record Update](#record-update-with) below, or recomposing it entirely. See [Immutability](docs/immutability.md#mutability-rules).

---

## Struct Construction

Structs can be constructed with an explicit type (`Type name = (fields)`) or inferred via `as` (`name as (fields)`). The transpiler matches field names to determine the struct type. Every field must already be a variable in scope matching the field name exactly. No `{}`, no `field: value` pairs.

### Inferred with `as`

A parenthesised field list `(f1, f2, ...)` constructs a struct. All items must be named variables already in scope whose names match the target struct's fields. The struct type is inferred by matching field names against all known structs — no type annotation needed.

Deor:
```deor
score as (label, points)    # struct type inferred from field names
```

Rust:
```rust
let score = Score { label: label.clone(), points: points.clone() };
```

**Caveat:** unlike the explicit-type form below, this inferred form has no validation against the struct registry. If no struct's field set matches the given names, it silently emits a bogus `Unknown { ... }` construction that only fails later at `rustc`. If more than one struct shares the exact same field set, it silently picks whichever was declared first in the file — no warning either way. Prefer the explicit-type form when field names alone don't uniquely identify the struct.

### Explicit Type

Deor:
```deor
Squarefeet area = 9
name as "Office"
occupied as true
Room room = (area, name, occupied)
```

Rust:
```rust
let area = Squarefeet::new(9);
let name = "Office".to_string();
let occupied = true;
let room = Room { area, name, occupied };
```

Mirrors destructuring: `in` pulls fields out of a struct, `= (field1, field2,...)` forms a struct, `with` constructs a struct. See [Enforced Practices — Unified `()` Rule](docs/enforced_practices.md#unified-rule-named-variables) for the field-matching/order rule shared across construction, destructuring, and return.

### As an Expression

`(fields)` can also appear as a bare expression in return position — no type annotation needed. See [Functions — Multiple return values](docs/functions.md#multiple-return-values) for the full worked example (`DivResult`/`divmod`).

**Caveat:** like the inferred `as` form, a bare `(fields)` finds its struct by matching field names — it does *not* use the function's declared return type. If two structs share the exact same field names, the one declared first wins, even when the function returns the other one (and `rustc` then reports a type mismatch). When field names alone don't identify the struct, bind it with an explicit type first and return the variable:

```deor
fn StringResult make_result(ValidText value, string message)
    StringResult built = (value, message)    # explicit type — no guessing
    return built
```

If you need a field name that differs from the variable you have, rename it first:

```deor
name in other_room
label = name
Entry entry = (label)
```

---

## Destructuring (`in`)

`in` extracts one or more fields from a struct into the current scope. This is the only way to access struct fields — there is no dot syntax in source, so every field a block of code touches is named up front in one place, instead of scattered across dot-chains wherever they happen to get used.

Parentheses are always required, even for a single field. Deor calls this **bagging**: items come out of (or go into) the struct under their original field name — there is no aliasing.

### Single Field
Deor:
```deor
(area) in room
```
Rust:
```rust
let area = room.area.clone();
```

### Multiple Fields
Deor:
```deor
(area, name) in room
```
Rust:
```rust
let area = room.area.clone();
let name = room.name.clone();
```

Each extracted field becomes its own `let field = src.field.clone();` binding — not a Rust pattern destructure.

**The new names are copies.** Assigning to one of them (`size = new_size`) changes only that variable, not the struct it came from. To get a changed struct, build a new one from named variables or use `with`:

```deor
(text, size) in line
size = new_size
resized as line with (size)
```

### Partial Extraction
You can extract a subset of a struct's fields. Any combination is valid — the struct may have more fields than you extract.

```deor
struct Room
    Squarefeet area
    string name
    bool occupied

(name) in room          # valid — ignores area and occupied
(area, occupied) in room  # valid — any subset, any order
```

### Shadowing
If a name being extracted already exists in scope, the new binding silently shadows it — the same block-scoped shadowing rules apply as anywhere else in Deor. See [Enforced Practices — Variable Shadowing](docs/enforced_practices.md#variable-shadowing) for the full mechanics.

```deor
name as "Alice"
(name) in employee    # name now refers to employee.name — "Alice" is gone
```

Use this deliberately to "update" a name after processing, or avoid it by choosing distinct names.

**Why shadowing is allowed:** macros rely on it — an inlined macro body may reuse a name already in scope, so rejecting that would make macros far more restrictive. It also matches Rust, which permits shadowing, and Deor's own import rule of silently keeping the first definition on a duplicate. If you want a name that can't be silently overwritten, declare it with `const` instead — shadowing a `const` is a transpiler error.

---

## Record Update (`with`)

`with` produces a new struct with one or more fields overridden. The original is unchanged. Each field name must already exist as a variable in scope — the same rule as struct literals.

Works with both binding forms — `as` (type inferred from the source struct) and a typed declaration (`Type name = source with (...)`, as seen in [Updating a Struct Inside a List](docs/collections.md#updating-a-struct-inside-a-list)):

- Single field: `new_room as room with (area)` — parens always required, even for one field
- Multiple fields: `new_room as room with (area, name)`

The transpiler enforces the parens: `with` not immediately followed by `(` is a validation error, for both forms.

Deor:
```deor
Squarefeet area = 2
new_room as room with (area)

area = 20
name as "Bigger Office"
bigger_office as office with (area, name)
```

Rust:
```rust
let mut area: Option<Squarefeet> = Squarefeet::new(2);
let new_room = Room { area, ..room.clone() };

area = Squarefeet::new(20);
let name = "Bigger Office".to_string();
let bigger_office = Room { area, name, ..office.clone() };
```

**Declaration only.** `with` can only appear in a declaration (`Type name = source with (...)` or `name as source with (...)`). `name = source with (...)` is a transpiler error, and so is anything other than field names and commas inside the parens — `with (value as field)` is not supported. Assign the new value to a variable named after the field first, then use `with (field)`.

**Conversion notes:** near 1:1 with Rust's built-in functional record update (`..` spread) syntax, except the spread source is cloned (`..room.clone()`, not `..room`) — every Deor struct derives `Clone`, so `with` never consumes the source; `room`/`office` are still fully usable after the update. Overridden fields that are validator types route through their constructor like any other assignment to that type. The `with` pattern mirrors `in` destructuring: `in` pulls fields out of a struct, `with` pushes variables into one.

---

## Move Variants

`move (f1, f2) in source` destructures without cloning, and `Type name = move (fields)` constructs without cloning — both consume the source variables. See [Move — Destructuring](docs/move.md#destructuring) and [Move — Struct Construction](docs/move.md#struct-construction) for the full examples and rules.

---

## Field Ordering

Even though order is not enforced, write fields in the same order they appear in the struct declaration if possible. It makes construction and destructuring sites easier to scan and keeps things consistent across the codebase.

```deor
struct Employee
    int employee_id
    string first_name
    string last_name

(employee_id, first_name, last_name) in employee
Employee emp = (employee_id, first_name, last_name)
emp as (employee_id, first_name, last_name)
```

Additionally, all `in` extractions should appear before any logic (assignments, expressions, control flow) within their block. Applies to function bodies, loop bodies, and if/else bodies.

**Correct:**
```deor
fn RollResult roll_die(Die die)

    (sides, label) in die

    min as 1
    int raw = m_rand_int(min, sides)

    Roll value = raw
    string source = label
    RollResult result = (value, source)

    return result
```
