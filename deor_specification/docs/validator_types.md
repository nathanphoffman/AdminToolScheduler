<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Validator Types

Sometimes a value can be built just fine but still not make sense — a shape with a negative area, a dice roll outside its range. 
Deor's answer is the **validator type**: a type with its own built-in rule for what counts as valid. Write the rule once, and every place 
that type is used, the rule runs automatically — there's no way to hold one of these values without it having been checked.

A validator type is declared with `type`:

```deor
type Positive(int val)
    val > 0
```

This defines `Positive`, built on `int`, with one rule: greater than zero. Assigning an `int` runs the rule — pass, and you have a valid `Positive`; 
fail, and instead of a crash or a garbage value, you get a `Positive` that's explicitly **not valid**, which the language forces you to check before use.

(Rust readers: under the hood this is `Option<T>` — valid is `Some`, not valid is `None`.) See [Variables and Data Types](docs/variables.md) for primitives, `raw` 
variables, and structs.

## Declaration

**The predicate body is mandatory** — a `type` with no rule means nothing more than its base type, so the transpiler rejects an empty body. 
Just use the base type directly if you don't need a check.

The base type must be a primitive (`int`, `float`, `string`, `bool`) or a declared struct — list shapes and other validator types (including the type 
referencing itself) are not valid as a validator base type and are transpiler errors:

```deor
type Foo(int val)          # correct — primitive base type
type Foo(Point val)        # correct — struct base type (see Validator Types Over Structs)
type Foo(intList val)      # transpiler error — list shapes cannot be validator base types
type Foo(Bar val)          # transpiler error — Bar is another validator type (or not declared)
type Foo(Foo val)          # transpiler error — a validator type cannot reference itself
```

The parameter name cannot shadow the type name or its own base type — both are transpiler errors:

```deor
type Roll(int Roll)    # transpiler error — parameter name shadows the type name
type Roll(int int)     # transpiler error — parameter name shadows its base type
type Roll(int val)     # correct
```

The body evaluates to a `bool` — a single expression for simple predicates, or intermediate bindings followed by a final bool expression 
for more complex ones, same rules as a function body.

Plain primitives and structs can never be "missing" — only validator types carry that possibility, and only because the predicate runs on assignment.

**The predicate re-runs on every validating assignment.** `TypeName varName = expr` triggers it, and so does a later bare reassignment (`varName = expr`) — the variable can flip between valid and not valid each time. Only reassigning with `as` is rejected. See [Variables — Reassignment](docs/variables.md#reassignment) for the general case.

```deor
for if true
    (first) in input()
    Roll attempt = c_string_to_int(first)   # fresh declaration each iteration
    if attempt is valid
        return (avow attempt)
    else
        print("invalid, try again")
```

This kind of shadowing across loop iterations is the normal pattern — a fresh "construct and check" each time, not a one-time declaration you update.

A slightly more realistic predicate, using values from an earlier calculation:

```deor
# import lib/math.deor and lib/convert.deor for these functions
type Squarefeet(int val)
    float flt = c_int_to_float(val)
    float root = m_sqrt(flt)
    int root_i = m_floor(root)
    return root_i * root_i is val
```

`c_int_to_float`, `m_sqrt`, and `m_floor` come from `lib/convert.deor` and `lib/math.deor` (see [Libs](docs/libs.md)). A negative `val` 
makes `m_sqrt` return NaN, `m_floor` turns that into `0`, and `0 * 0 is val` fails — no separate negative-number guard needed.

```deor
Squarefeet area = 9     # valid — predicate passes
Squarefeet area = -1    # transpiles; the predicate fails at runtime, so area is not valid
```

---

## `is valid` / `is not valid`

A validator type variable is always **valid** (rule passed) or **not valid** (rule failed, or nothing assigned yet) — `Some`/`None` for 
Rust readers. There's no keyword to force an invalid state; it happens in exactly two ways:

- Declared without a value: `Squarefeet sqft` — not valid until assigned
- Assigned a value that fails the predicate: `Squarefeet sqft = -10` — predicate fails, not valid

Check with `is valid` / `is not valid`:

```deor
Squarefeet sqft = 9
if sqft is valid
    int val = (avow sqft)
if sqft is not valid
    print("no value")
```

This is Deor's only concept of null — see [Replacing Null and Undefined](#replacing-null-and-undefined) for the pattern and why.

---

## Declaring Without a Value

A validator type variable declared without an initial value starts as not valid. Assign a value later to make it valid.

```deor
Roll best
Squarefeet area
```

```rust
let mut best: Option<Roll> = None;
let mut area: Option<Squarefeet> = None;
```

`empty` is not valid for validator types and is a transpiler error — not valid is expressed by declaring without a value instead:

```deor
Roll best = empty    # transpiler error — empty is not valid for validator types
Roll best            # correct — starts as not valid
```

List shapes use `empty` instead — see [Variables — List Construction](docs/variables.md#list-construction).

---

## Replacing Null and Undefined

Older, C-style code marks "this might not be set" with a comment and a sentinel — a `-1`, a `NULL`, a `0` that quietly means "unset" — enforced by 
nothing but the reader remembering. Rust replaces that with `Option<T>`, fully enforced but asking you to learn and thread pattern-matching through code 
that, most of the time, never needed to hold "nothing" at all. Deor sits in between: most values can't be missing at all, and the rare ones that can 
require naming the concept as a `type` and writing its invalidity rule as real, checked code — the C comment, formalized, without Rust's full `Option` 
machinery.

There are two ways to use a validator type for this. Pick one interpretation per type — a real project wouldn't declare `ValidInt` both ways.

**Nothing means "not set yet"** — the predicate accepts anything, so the only way to be not valid is to never assign it:

```deor
type ValidInt(int val)
    true

ValidInt score          # not valid — nothing assigned, i.e. "null"
ValidInt score = 42     # valid — any int satisfies the predicate
```

`true` isn't a no-op here — it says "any assigned value is fine, only *absence* matters," leaning entirely on the declare-without-a-value mechanic above.

**A sentinel means "missing"** — the predicate itself rules out the value that stands for "nothing":

```deor
type ValidInt(int val)
    val is not 0

ValidInt count = 0      # not valid — 0 means "no count"
ValidInt count = 5      # valid
```

**Note:** Prefer a real, named predicate (`Temperature`, `Squarefeet`, etc.) over this generic pattern whenever the type actually has rules about what values it can't be. Reach for a bare `true`/sentinel predicate only for the rare case where you genuinely just need a nullable-ish type.

The standard library ships this pattern ready-made for any type, structs included: see [`lib/optional.deor`](docs/libs.md#liboptionaldeor).

---

## Forced Unwrap — `avow`

`avow val` (or `(avow val)`) pulls the raw value out of a validator type — the plain `int` inside a `Roll`. It panics at runtime if the value isn't 
actually valid, so use it only when you're sure — typically right after an `if val is valid` check. (Rust readers: this is `.unwrap()`.) Using it on a 
non-validator-type variable is a transpiler error.

`avow` binds only to the next primary (identifier, literal, or parenthesized group) — same rule as `move` — so `avow val + 2` always parses as `(avow val) + 2`. Parens are a readability choice, not required.

Pass the variable directly (no `avow`) when a function accepts that validator type; only reach for `avow` when you need the raw primitive. It can be used 
as a function argument directly too (`show(avow roll)`), no need to capture it first.

```deor
Roll roll = roll_die(d20)
if roll is valid
    int val = (avow roll)          # need the raw int — use avow
    bool crit = is_critical(roll)  # function takes Roll — pass directly, no avow
```

```rust
if roll.is_some() {
    let val: i64 = roll.unwrap().0;
    let crit: bool = is_critical(roll);
}
```

Outside an `if` check, `avow` is your explicit assertion that the value is valid — `int sum = (avow value) + 2` works the same way, just without the safety net.

---

## Validator Types in Structs

Struct fields typed as a validator type are `Option<T>` under the hood. Extracting them with `in` preserves the Option — the extracted variable must be checked with `is valid` / `is not valid` before use.

```deor
struct Room
    Squarefeet area
    Roll max_capacity
```

```deor
(area, max_capacity) in room
if max_capacity is valid
    int cap = (avow max_capacity)
```

---

## Validator Types Over Structs

A validator type can wrap a struct, so a rule can check several fields together. The predicate pulls the fields out with `in`, the same as any function:

```deor
struct DateRange
    int start_day
    int end_day

type ValidRange(DateRange val)
    (start_day, end_day) in val
    return start_day < end_day
```

```deor
DateRange span = (start_day, end_day)
ValidRange checked = span

if checked is valid
    DateRange good = (avow checked)
```

Everything else works like any other validator type: `is valid` / `is not valid`, declaring without a value, and the predicate re-running on every assignment.

**Structs stay immutable.** There is still no field-assignment syntax, and `avow` hands back a copy, so the validator variable stays usable afterwards. To "change" a wrapped struct, build a new one with `with` and assign it back — the rule re-runs on that assignment, so an invalid struct can't be slipped in by editing it:

```deor
start_day = 1
repaired as backwards with (start_day)
rejected = repaired    # predicate re-runs
```

**A struct can't hold itself.** A struct field whose type is the struct itself, or a validator type wrapping that struct, is a transpiler error — in Rust the struct would have infinite size:

```deor
struct Node
    int depth
    Node next        # transpiler error — a struct cannot hold itself

struct Tree
    int depth
    TreeOk left      # transpiler error — TreeOk wraps Tree

type TreeOk(Tree val)
    true
```

A struct may still hold a different struct, or a validator type over a different struct.

**`avow` after destructuring.** Pulling a validator-typed field out with `in` gives a validator variable under the field's own name, so `avow` works on it directly. The check goes by name: `avow` is accepted on any variable that shares its name with a validator-typed struct field somewhere in the program, and Rust catches a real mismatch.

In Rust the wrapper derives `Clone` but not `Copy` (a struct isn't `Copy`), and `(avow checked)` becomes `checked.clone().unwrap().0`.

---

## Functions Returning Validator Types

A function returning a validator type returns a variable that may or may not be valid. To return a not-valid result, either declare the variable without a value and return it unassigned, or assign a value that fails the predicate. `return empty` and `return none` are both transpiler errors — neither is a Deor keyword in return position.

```deor
fn Roll find_best(int val)
    Roll best
    if val > 0
        best = val
    return best    # not valid if val <= 0
```

The caller checks with `is valid`:

```deor
Roll crit = find_best(val)
if crit is valid
    int bonus = (avow crit)
```

The returned value is bound as-is, and the same holds for a later reassignment (`crit = find_best(val)`). A validator type has no way to say *why* it is not valid; when the caller needs an error message, use [`lib/result.deor`](docs/libs.md#libresultdeor), which pairs a validator-typed value with a message.

---

**Conversion notes:**
- Constructor becomes `fn new(n: T) -> Option<Self>` — never panics, returns `None` on predicate failure.
- `is valid` → `.is_some()`, `is not valid` → `.is_none()`.
- `(avow val)` → `.unwrap().0`.
- Equality (`is` / `is not`) transpiles to `==` / `!=` in Rust and falls through to `Option<T>: PartialEq` — `None == None` is true, `Some(x) == Some(y)` compares inner values structurally.
- `and` / `or` / `not` map to `&&` / `||` / `!`.
- The predicate always runs at runtime inside `new()` — even for literals like `Squarefeet area = 9`. There is no compile-time evaluation of the predicate, but it is runtime validated.