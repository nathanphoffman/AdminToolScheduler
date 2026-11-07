<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Functions

## Declaration

Return type is written as a prefix before the function name. Parameters follow `Type name` order.

Deor:
```deor
fn int add(int left, int right)
    left + right
```

Rust:
```rust
fn add(left: i64, right: i64) -> i64 {
    left + right
}
```

**Conversion notes:** the prefix return type becomes Rust's trailing `-> Type`. The transpiler must ensure the final expression has no trailing `;` for it to act as the return value.

### Void Functions

`void` is the explicit return type for functions that return nothing. It is mandatory — omitting the return type is a transpiler error.

Deor:
```deor
fn void greet(string name)
    print(name)
```

Rust:
```rust
fn greet(name: String) {
    println!("{}", name);
}
```

The entry point follows this same rule:

Deor:
```deor
fn void main()
    # program starts here
```

Void functions fall through to the end — `return` is not valid inside a void function. For loop control, use `continue` to skip to the next iteration or `break` to exit the loop early:

Deor:
```deor
shape itemList = list of Item

fn void process(itemList items, bool skip_invalid)
    for item in items
        (active) in item
        if skip_invalid and not active
            continue
        handle(item)
```

Rust:
```rust
fn process(items: Vec<Item>, skip_invalid: bool) {
    for item in &items {
        let active = item.active;
        if skip_invalid && !active {
            continue;
        }
        handle(item.clone());
    }
}
```

### Multiple return values

There are no anonymous tuple return types. A function returning multiple values must declare a named struct for the return type. The struct is then constructed and destructured using the existing `as`/`in` syntax — no new keywords. A bare `return (fields)` finds its struct by field names, so if another struct has the same fields, bind an explicit type first — see [Structs — As an Expression](docs/structs.md#as-an-expression).

Deor:
```deor
struct DivResult
    int quotient
    int remainder

fn DivResult divmod(int left, int right)
    int quotient = left / right
    int remainder = left % right
    return (quotient, remainder)
```

Rust:
```rust
struct DivResult { quotient: i64, remainder: i64 }

fn divmod(left: i64, right: i64) -> DivResult {
    let quotient = left / right;
    let remainder = left % right;
    DivResult { quotient, remainder }
}
```

Capturing the result uses standard struct destructuring:

Deor:
```deor
(quotient, remainder) in divmod(num, div)
print(quotient)
print(remainder)
```

Rust:
```rust
let result = divmod(num, div);
let (quotient, remainder) = (result.quotient, result.remainder);
```

The struct name documents what the paired values represent — `DivResult` communicates more than an anonymous `(int, int)`. If you need a general-purpose pair, declare a `struct Pair` with `value1` and `value2` fields.

---

## Return Rules

A function body that is a **single expression** — one line, nothing else — implicitly returns that expression. `return` is optional there.

Any function body with **more than one statement** requires explicit `return` at every exit point.

Deor:
```deor
fn int square(int val)
    val * val    # single expression — return implicit
```

Rust:
```rust
fn square(val: i64) -> i64 {
    val * val
}
```

Deor:
```deor
fn int abs(int val)
    if val < 0       # multiple statements — return required everywhere
        return -val
    return val
```

Rust:
```rust
fn abs(val: i64) -> i64 {
    if val < 0 {
        return -val;
    }
    return val;
}
```

**Conversion notes:** the transpiler can always emit explicit `return` safely — implicit single-expression return is a source-level convenience, not a Rust requirement.

---

## Validator Type Returns

A function may return a validator type, which the caller checks with `is valid` — see [Validator Types — Functions Returning Validator Types](docs/validator_types.md#functions-returning-validator-types) for the pattern and rules. Primitive return types (`fn int`, `fn bool`, etc.) are never valid/not valid — they always have a value.

---

## No Lambdas / Closures / Nested Functions

All callable values are named top-level `fn`s. There 
is no anonymous-function syntax, and functions may not be defined inside other function bodies. No built-in `filter`, `map`, or `reduce` — 
write explicit loops instead.

To pass behavior as a value, declare a `func` shape and accept it as a typed parameter. The caller passes a named top-level function by name — 
this is Deor's equivalent of a lambda. See [Shapes — Func Shapes](docs/shapes.md#func-shapes) for declaration syntax, single-param constraint, 
and conversion notes.

---

## Recursion

Functions may call themselves. Recursion follows the same rules as any other function call — when called with 2 or more arguments, every argument must already be a named variable in scope (see [Enforced Practices — Named Arguments](docs/enforced_practices.md#named-arguments-user-defined-functions-only); this is a general call-site rule, not specific to recursion), and the 
return type must match.

Deor:
```deor
fn int factorial(int val)
    if val <= 1
        return 1
    int prev = val - 1
    int sub = factorial(prev)
    return val * sub
```

Rust:
```rust
fn factorial(val: i64) -> i64 {
    if val <= 1 {
        return 1;
    }
    let prev: i64 = val - 1;
    let sub: i64 = factorial(prev);
    return val * sub;
}
```

No tail-call optimization is guaranteed — deep recursion can stack-overflow just as in Rust. For large inputs, prefer iterative loops.

---

## Parameters

Functions accept at most **3 parameters**. If more context is needed, bundle values into a struct first. This is enforced by the transpiler. `func` shape parameters count toward the limit the same as data parameters.

Deor:
```deor
fn roomList filter(roomList items, string query, filterFunc predicate)
    # 3 params: list, data, behavior — the natural ceiling
```

`func` shape parameters are regular parameters — no special keyword, no annotation, just a typed name like any other.

A parameter's name cannot be identical to its type name — the transpiler rejects the exact same string appearing in both positions:

```deor
fn void process(Room Room)    # transpiler error — name identical to type
fn void process(Room item)    # correct
```

In practice this is rarely an issue since type names are PascalCase and parameter names are snake_case, but the transpiler enforces it explicitly.

Every parameter type and the return type are checked against the known-type set (primitive, struct, shape, raw type, validator type, or untyped enum). A misspelled or undeclared type name (`fn void process(Rooom item)`) is a clean transpiler error, not a later `rustc` failure.

Deor:
```deor
shape roomList = list of Room
shape filterFunc = func of Room to bool

fn roomList filter(roomList items, filterFunc predicate)
    roomList result = empty
    for item in items
        if predicate(item)
            result at end = item
    return result
```

## Entry Point

The function named `main` is always the program entry point. It must be declared `fn void main()`.

Deor:
```deor
fn void main()
    # program starts here
```

Rust:
```rust
fn main() {
    // program starts here
}
```

Only one `fn void main()` may exist per project.
