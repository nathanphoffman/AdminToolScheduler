<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->

# Best Practices
Style recommendations not enforced by the transpiler, except where noted (see [Order of Declaration](#order-of-declaration)).

---
## Order of Declaration
The transpiler enforces four groups, in this order: imports, structural declarations (`enum`/`struct`/`type`/`shape`), macros, then functions — see [Enforced Practices — Ordering](docs/enforced_practices.md#ordering). Within the structural group, the transpiler accepts any order, but this is the recommended arrangement:

1. Imports - Everything else could use it, relies on nothing else in the file
2. Enums - Relies likely on nothing else in the file
3. Structs - Reliant on most everything above but still structural (so above functions)
4. Types - Type validators being types must be defined early
5. Shapes - Shapes can reference almost anything above
6. Macros - Enforced to come before functions
7. Functions - Reliant on everything above - Enforced to be last

---
## Spacing
Add a blank line before `return` in any function body that contains more than one statement. One-liner functions (single expression, no bindings) are exempt. Try to keep blocks spaced out keeping like concepts with one another.

Functions get extra separation from everything around them: leave 2 blank lines (3 returns) between a function and the next function or block. All other top-level blocks (structs, enums, shapes, macros) only need 1 blank line between them.

**Recommended:**
```deor
shape rollResultList = list of RollResult

fn int sum_rolls(rollResultList rolls)
    # is its own thing, just printing new line after these 2 lines
    roll_notice as "Going to roll"
    print(roll_notice)

    # belongs together as it is all part of the loop
    sum as 0
    for roll in rolls
        (value) in roll
        val as 0
        if value is valid
            val = (avow value)
        sum = sum + val

    return sum
```

**Exempt — one-liner, no blank line needed:**
```deor
fn int square(int val)
    val * val
```
---
## Multi-line Calls — No Trailing Comma

When a call wraps across multiple lines, don't put a comma after the last argument. Deor favors reading like a plain human list — nobody writes "lettuce, tomato, onion," with a trailing comma, so don't write one here either.

```deor
Connection conn = Connect(
    host,
    port,
    timeout
)
```

---
## Construction and Destructuring

Field-ordering and `in`-placement conventions for structs now live on their own page — see [Structs — Field Ordering](docs/structs.md#field-ordering).

---
## avow

Always capture `avow` into its own binding — do not use `(avow val)` inline inside a larger expression.

**Recommended:**
```deor
if result is valid
    int val = (avow result)
    total = total + val
```

**Avoid:**
```deor
if result is valid
    total = total + (avow result)
```

Also always guard `avow` with an `is valid` check immediately above it, or add a comment explaining why the value is guaranteed valid at that point.

---
## Avoid Deep Nesting

Limit nesting to two or three levels. Deeply nested code is hard to read and usually signals that logic should be extracted into a helper function. Prefer early returns and guard clauses over deep `if/else` trees.

**Preferred — early return flattens nesting:**
```deor
fn string classify(int val)
    if val < 0
        return "negative"
    if val is 0
        return "zero"
    return "positive"
```

**Avoid — deep nesting:**
```deor
fn string classify(int val)
    if val < 0
        return "negative"
    else
        if val is 0
            return "zero"
        else
            return "positive"
```

---
## Keep Functions Small

A function should do one thing. If a function body is growing long or handles multiple distinct concerns, extract the inner logic into named helper functions. The 3-parameter limit already encourages this — if you need more context, you should already be reaching for a struct.

---
## Functions vs. Macros — Performance in Loops

Prefer functions for reusable logic that is called outside loops. Inside a loop body, prefer macros when the operation is simple and call overhead matters — macros expand inline, whereas function calls add a frame per iteration.

**Use a function — called once or outside a loop:**
```deor
fn int square(int val)
    val * val

int area = square(side)
```

**Use a macro — called inside a loop:**
```deor
macro sq
    int area = sq(val)

for item in items
    macro_run sq
```

In trivial cases, like these inline is probably better anyhow, but in general:
- When the code is small enough (like these examples since we have limited space): leave it inline
- When organization is needed and logic separation isn't needed: use a ```macro``` — its body is contained on its own, so there's no extra decision to make about whether it declares new variables
- When logic separation is logical: use a ```function```
- When a macro genuinely needs to hand a *new* variable to its caller (not just assign into one the caller already declared) — a start/end pair sharing state being the main case — reach for ```unsafe_macro``` instead, see [Macros — `unsafe_macro`](docs/macros.md#unsafe_macro-deliberately-leaking-state)

---
## Reusable Consts — via Macros

Deor has no global scope, so a `const` can't be shared across functions directly. Put shared constants in an `unsafe_macro` and call it from each function with `unsafe_macro_run`. A plain `macro` won't work here — its body is contained, so the consts it declares would never escape into the caller. See [Macros — Macro Bodies Are Contained](docs/macros.md#macro-bodies-are-contained) and [`unsafe_macro`](docs/macros.md#unsafe_macro-deliberately-leaking-state).

```deor
import "lib/string.deor"

unsafe_macro use_log_consts
    const string INFO_PREFIX = "[INFO] "
    const string ERROR_PREFIX = "[ERROR] "

fn void log_info(string msg)
    unsafe_macro_run use_log_consts
    print(s_join([INFO_PREFIX, msg]))

fn void log_error(string msg)
    unsafe_macro_run use_log_consts
    print(s_join([ERROR_PREFIX, msg]))
```

Calling the same `unsafe_macro` more than once in one function body is fine — each call redeclares the same names, and Rust's own shadowing rules allow that.

---
## File Length

Keep files to a reasonable length. There is no hard limit, but when a file starts to feel long, consider splitting it. A natural split point is when the file contains multiple distinct concerns — for example, separate structs and their associated functions into their own files.

A quick way to separate concerns is with ```macros```, since code can just be copied and pasted with macros, with no concern for scope (since they exist where injected), they are the fastest way to organize files and give each small code snippet a name.

In the Deor creator's opinion: Smaller functions with well placed macros > Smaller with only functions > Smaller with only macros > Large files of any kind.  Hence, don't avoid macros as a short time powerhouse for organization until areas can be broke into more manageable functions.

---
## Prefer `is not` Over `not ... is`

The transpiler enforces `is not` over `not ... is` for bare identifiers — see [Operators — Logical](docs/operators.md#logical). The same preference applies even in cases the transpiler doesn't catch, like `not (a > b) is y` or `not some_func() is y` — reorder these to `(a > b) is not y` and `some_func() is not y` instead.

**Recommended:**
```deor
if (a > b) is not y
    ...
```

**Avoid:**
```deor
if not (a > b) is y
    ...
```

---
## Naming External Libs
Because Deor does not support third-party importing, the standard convention is to copy Deor files into the `lib/` folder, with functions prefixed `x` + the module's letter to mark them as non-standard. See [Libs — Naming Convention](docs/libs.md#naming-convention) for the full prefix scheme and examples.

Never edit a standard library file (`lib/string.deor`, `lib/math.deor`, etc.) directly — `deor update` overwrites all of them, silently discarding any changes. If you want to add functions to an existing module rather than wrap something new, create `lib/<module>_extension.deor` instead. See [Libs — Do Not Edit `lib/` Files](docs/libs.md#do-not-edit-lib-files).