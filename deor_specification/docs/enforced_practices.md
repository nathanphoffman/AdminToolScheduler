<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Enforced Practices
These rules are enforced by the transpiler. Violations produce warnings or compile-time errors.

---
## Naming Conventions
- enums, structs, and custom types (type validators) MUST be PascalCase — bold and structural, distinct from the code around it
- shapes must be camelCase — see [Shapes — Naming Convention](docs/shapes.md#naming-convention) for the rationale
- variable and function names must be snake_case — the most common case, kept plain and readable
- `const` variable names must be SCREAMING_SNAKE_CASE — signals a fixed, never-reassigned value at a glance
---
## Minimum Name Length — 3 Characters
All identifiers must be at least 3 characters long. This applies to every named thing in Deor source: variables, function parameters, function names, struct names, validator type names, struct field names, and list names.

```deor
int val = 5      # correct
int vl = 5       # transpiler error — 2 characters
int v = 5        # transpiler error — 1 character

fn int add(int left, int right)   # correct
fn int add(int a, int b)          # transpiler error — parameters too short

type Roll(int val)    # correct
type Roll(int n)      # transpiler error — parameter too short
```

There are no exceptions. All runtime identifiers — variables, parameters, fields, functions, type names, and shape names — must be at least 3 characters.

---
## Ordering
A file's top-level declarations must appear in this order:

1. Imports
2. Structural declarations — `enum`, `struct`, `type`, `shape`, in any order relative to each other
3. Macros
4. Functions

The transpiler checks group boundaries, not exact placement within a group — any mix of `enum`/`struct`/`type`/`shape` is fine as long as none of them appear after a `macro` or `fn`, and no `macro` appears after a `fn`. See [Best Practices — Order of Declaration](docs/best_practices.md#order-of-declaration) for the recommended order within the structural group.

Local macros declared inside a function body are exempt — this rule only applies to top-level declarations.

**Correct:**
```deor
import "lib/string.deor"

struct Room
    string name

shape roomList = list of Room

macro log_room
    print("logging")

fn void run()
    ...
```

**Incorrect — transpiler error:**
```deor
fn void run()
    ...

struct Room    # transpiler error — struct after a function
    string name
```


---
## Validator Type Base Must Be a Primitive or Struct

A validator type wraps `int`, `float`, `string`, `bool`, or a declared struct. List shapes, undeclared names, and other validator types (including the type itself) are transpiler errors — see [Validator Types — Declaration](docs/validator_types.md#declaration).

**Correct:**
```deor
type Positive(int val)
    val > 0
```

---
## Validator Type Predicate Required

The predicate body is mandatory — see [Validator Types — Declaration](docs/validator_types.md#declaration) for why.

**Correct:**
```deor
type Positive(int val)
    val > 0
```

**Incorrect — transpiler errors:**
```deor
type Positive(int val)
```

---
## `empty` and List Initialization

List shapes are initialized with `empty`, validator types are not — see [Collections — Empty List](docs/collections.md#empty-list) and [Validator Types — Declaring Without a Value](docs/validator_types.md#declaring-without-a-value).

---
## `const` — No Reassignment

`const` variables cannot be reassigned. Attempting to assign a new value to a `const` variable after its declaration is a transpiler error.

```deor
const string PIPE = "|"
PIPE = "/"              # transpiler error — cannot reassign a const variable
```

Use a plain typed binding (`string pipe = "|"`) if you need a variable that may change.

---
## `as` — No Type Annotation

`as` is the type-inferring binding form. Pairing it with an explicit type is always a transpiler error — when you have an explicit type, use `=` instead.

```deor
count as 0          # correct — int inferred
int count as 0      # transpiler error — annotation not allowed with as
int count = 0       # correct
```

---
## Duplicate Top-Level Names

By default, declaring two top-level items with the same name is **not** an error — whichever one the importer merges in first silently wins, and the rest are dropped. This is deliberate: it's what lets the standard library have several `lib/*.deor` files each redeclare the same convenience alias (`shape stringList = list of string`, for example) so any one of them can be imported standalone.

If you want stricter checking, opt in with a pragma as the very first statement(s) of `fn void main()`:

```deor
fn void main()
    ENFORCE_UNIQUE_FILE_DECLARATIONS
    ENFORCE_UNIQUE_IMPORT_DECLARATIONS
    ...
```

- `ENFORCE_UNIQUE_FILE_DECLARATIONS` — a name declared twice **in the same file** is a hard transpiler error.
- `ENFORCE_UNIQUE_IMPORT_DECLARATIONS` — a name declared in two **different** files is a hard transpiler error.

They're independent — set one, both, or neither, in either order. Both apply across all declaration forms (`struct`, `enum`, `shape`, `type`, `fn`, `macro`, `unsafe_macro`, `raw`) and across declaration kinds, so a `struct Foo` and a `fn Foo` sharing a name are checked the same way. The check runs while imports are being merged, before any other validation, and fails fast on the first collision found.

One exception: a top-level `rust { }` block is never covered by either pragma. Two files with a byte-identical top-level `rust` block are always silently deduplicated — even with both pragmas set — and never raise a duplicate-declaration error.

### Why two separate pragmas, and why opt-in

This is deliberately Fortran-`IMPLICIT NONE`-flavored: loose by default, strict only if you ask for it, and the ask has to be a visible declaration at the top of the entry point rather than a buried config flag.

The two checks are kept independent rather than folded into one "strict mode" because project style varies:

- Some projects want strictness from day one and set both — accepting the cost of pulling shared aliases (like the stdlib's `stringList`/`tList` pattern) out into their own dedicated file so nothing collides and accepting responsibility to clean up built-in lib shapes, or implementing your own libs.
- Some projects are one long, run-on file by convention, with the same shape or struct deliberately redeclared in different regions for local readability — `ENFORCE_UNIQUE_IMPORT_DECLARATIONS` alone fits that style without breaking it.
- Some small projects don't care about cross-file collisions at all but still want a typo like a copy-pasted `struct Player` block in the same file to fail loudly — `ENFORCE_UNIQUE_FILE_DECLARATIONS` alone covers that.
- Some projects want to redeclare a generic library shape locally after `T`-substitution, just to see its concrete form inline — the default, with neither pragma, supports that without any opt-in at all.

Because they're separate statements rather than one combined flag, the choice a project made is visible directly at the top of `main()`, not inferred from a build setting elsewhere.

```deor
struct Room
    string name

struct Room    # error only with ENFORCE_UNIQUE_FILE_DECLARATIONS
    float area

fn void process(Room item)
    ...

fn void process(string label)    # error only with ENFORCE_UNIQUE_FILE_DECLARATIONS
    ...
```

Variable shadowing within function bodies is unrelated to either pragma and is always allowed — see below.

---
## Limiting Cross-File Macro Calls

Macros can call other macros with `macro_run`, and those calls can nest arbitrarily — see [Macros](docs/macros.md). Nesting many macros defined in the same file costs nothing to trace, since it's all in one place. Nesting across files is what actually gets hard to follow, since understanding the chain means paging between files. `ENFORCE_MACRO_FILE_DEPTH` caps that specifically, using the same opt-in pragma convention as `ENFORCE_UNIQUE_*` above — set it as one of the first statement(s) of `fn void main()`:

```deor
fn void main()
    ENFORCE_MACRO_FILE_DEPTH = 3
    ...
```

The count only increases when a `macro_run` call's target macro is defined in a **different file** than the call site. Any number of macros calling each other within the same file costs nothing — the limit only tracks how many distinct file boundaries the chain has crossed, counting the entry file itself as the first one:

```deor
# file1.deor
macro macro1
    macro_run macro2          # crosses into file2.deor — depth 2

fn void main()
    ENFORCE_MACRO_FILE_DEPTH = 3
    macro_run macro1          # depth 1 (file1.deor itself)

# file2.deor
macro macro2
    macro_run macro3          # same file — still depth 2

macro macro3
    macro_run macro4          # same file — still depth 2

macro macro4
    macro_run macro5          # crosses into file3.deor — depth 3, allowed (limit is 3)

# file3.deor
macro macro5
    print("done")
```

If `macro5` called a macro defined in a fourth file, that crossing would be depth 4 and fail — `ENFORCE_MACRO_FILE_DEPTH = 3` only allows a chain to touch 3 distinct files. Re-crossing into a file the chain already left still counts as a new crossing each time — the limit tracks total boundary crossings along the call path, not the count of distinct files touched.

Unset (the default), this is unlimited — no different from today. There's no way to set an unlimited value explicitly once opted in; omit the pragma entirely instead.

---
## Unsafe Macro Nesting

A plain `macro`'s body is always contained (see [Macros — Macro Bodies Are Contained](docs/macros.md#macro-bodies-are-contained)), so it can never introduce a variable that escapes into its caller. `unsafe_macro` is the one construct allowed to do that — see [Macros — `unsafe_macro`](docs/macros.md#unsafe_macro-deliberately-leaking-state).

An `unsafe_macro` must always be called with `unsafe_macro_run`, never plain `macro_run` — and a plain `macro` must always be called with `macro_run`, never `unsafe_macro_run`. Using the wrong one is a transpiler error either direction, so a call site's leak potential is always visible without checking the definition:

```deor
unsafe_macro set_greeting
    string greeting = "hi"

fn void main()
    macro_run set_greeting          # transpiler error — set_greeting is unsafe_macro, use unsafe_macro_run
    unsafe_macro_run set_greeting   # correct
```

Because an `unsafe_macro` body has no `block` around it, an `unsafe_macro` calling — or being called from inside — another `unsafe_macro` would let a leak travel an unbounded distance up the call chain, always a transpiler error:

```deor
unsafe_macro outer_leak
    int outer_val = 1
    unsafe_macro_run inner_leak      # transpiler error — unsafe_macro cannot call another unsafe_macro

unsafe_macro inner_leak
    int inner_val = 2
```

An `unsafe_macro` can freely call, or be called by, an ordinary `macro`, though — that macro's own `block` wrap contains anything spliced into it, so the leak is stopped there regardless of which side is unsafe:

```deor
unsafe_macro outer_leak
    int outer_val = 1
    macro_run helper           # fine — helper is an ordinary, contained macro

macro helper
    unsafe_macro_run inner_leak  # also fine — helper's block wrap contains inner_leak's leak
    print("done")

unsafe_macro inner_leak
    int inner_val = 2
```

---
## Variable Shadowing

Variable shadowing is allowed. A new declaration with the same name in the same block or an inner block replaces the binding from that point forward, and does not affect the outer binding once the inner block ends:

```deor
int val = 5
if condition
    int val = 10    # shadows outer val within this block only
    print(val)      # 10
print(val)          # 5
```

The standalone `block` keyword opens a new scope without needing an accompanying `if`, `for`, or `fn` — useful any time you want to contain a few variables to a short span of code without extracting a separate function:

```deor
int val = 5
block
    int val = 10    # shadows outer val within this block only
    print(val)      # 10
print(val)          # 5
```

`block` works anywhere a statement is valid — top-level function bodies, inside `if`/`for` blocks, and inside macro bodies. That last case is also its most common use: see [Macros — `block` Inside Macros](docs/macros.md#block-inside-macros) for how it keeps a macro's internal variables from leaking into its caller's scope.

**Exception:** built-in function names (`print`, `crash`, `len`, `range`, `args`, `input`) can never be shadowed, even though they aren't reserved keywords — see [Syntax — Built-in Function Names](docs/syntax.md#built-in-function-names).

```deor
int print = 5    # transpiler error — print is a built-in, not shadowable
```

---
## Maximum 3 Parameters per Function

Functions may accept at most 3 parameters, enforced by the transpiler — see [Functions — Parameters](docs/functions.md#parameters).

---
## No Self-Containing Structs

A struct field can't have the struct's own type, nor a validator type that wraps it — the struct would have infinite size. This is a transpiler error, not a later `rustc` failure. See [Validator Types — Over Structs](docs/validator_types.md#validator-types-over-structs).

**Incorrect:**
```deor
struct Node
    int depth
    Node next
```

---
## No `func` Shapes as Struct Fields

Struct fields must be data types — primitives, validator types, other structs, or list shapes. A `func` shape field is a transpiler error — see [Shapes — Func Shapes in Structs](docs/shapes.md#func-shapes-in-structs).

---
## Unified `()` Rule — Named Variables

Everything placed inside `()` must be a named variable already in scope, with the exception of [Destructuring](docs/structs.md#destructuring-in). This rule applies uniformly to:

| Context | Example |
|---|---|
| Function call | `add(value1, value2)` |
| Struct construction | `Room room = (area, name, occupied)` |
| Struct return | `return (quotient, remainder)` |

Order does not matter for struct construction or struct return — fields are matched by name. Order does matter for function calls, since those are positional.

There are no anonymous tuple types in Deor. `return (quotient, remainder)` constructs the function's declared return struct — both `quotient` and `remainder` must be variables in scope that match field names on that struct. See [Functions — Multiple return values](docs/functions.md#multiple-return-values) for the full pattern.

**Correct:**
```deor
struct Room
    Squarefeet area
    string name

Squarefeet area = 20
name as "Office"
Room room = (area, name)      # correct
Room room = (name, area)      # also correct — order doesn't matter for structs
```

**Incorrect — transpiler error:**
```deor
Room room = ("Office", area)  # literal not allowed — "Office" must be a field like name
```

For struct destructuring with `in`, field names drive the binding — order does not matter and any subset is valid.

---
## Named Arguments — User-Defined Functions Only
When a user-defined function is called with **2 or 3 arguments**, every argument must be a named variable already in scope. Literals, arithmetic expressions, inline function call results, and inline struct constructions are not valid in that position.

When called with **exactly 1 argument**, the argument does not need to be a named variable — a literal or expression is allowed.

**Correct:**
```deor
fn int add(int left, int right)
    return left + right

num as 5
amt as 3
int result = add(num, amt)    # 2 args — both must be named
```

```deor
fn int double(int val)
    return val * 2

int result = double(5)        # 1 arg — literal allowed
```

**Incorrect — transpiler errors:**
```deor
int result = add(5, 3)               # 2 args — literals not allowed
int result = add(num + 1, amt)       # 2 args — expression not allowed
```

`move var` counts as a named argument — passing ownership of a variable still satisfies the rule:

```deor
do_something(move big_list, other)    # valid — move var is a named argument
```

**Built-in functions** accept a single literal or expression directly. For built-ins called with 2 or more arguments, the named-variable rule still applies:

```deor
print("Hello, world!")    # 1 arg — literal fine
int cnt = len(rooms)      # 1 arg — variable fine

start as 0
stop as 10
for idx in range(start, stop)    # 2 args — named variables required
    ...
```

---
## No Nested Functions
Functions may only be declared at the top level of a file. Defining a `fn` inside another `fn` body is a transpiler error.

**Correct:**
```deor
fn bool is_valid(int val)
    return val > 0

fn string describe(int val)
    if is_valid(val)
        return "positive"
    return "invalid"
```

**Incorrect — transpiler errors:**
```deor
fn string describe(int val)
    fn bool is_valid(int num)    # not allowed
        return num > 0
    if is_valid(val)
        return "positive"
    return "invalid"
```

Move all helper functions to the top level of the file and call them by name.

---
## `raw` — Opaque Values Deor Doesn't Type-Check

`raw name = expr` must be assigned from a function call, can never be retyped, reassigned, used as an operator operand, or declared as a struct field — see [Rust Interop — When to Use a rust Block](docs/interop.md#when-to-use-a-rust-block) for the full rules and examples.