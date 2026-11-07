<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Move

By default Deor clones every value — function arguments, loop elements, and assignments all get a fresh copy. This is safe and simple but has a performance cost. `move` opts out of cloning 
for a specific operation and transfers ownership instead, matching Rust's default behavior. The original variable is no longer accessible after a move — **for non-`Copy` types.** 

---

## Move and `Copy` Types

`int`, `float`, and `bool` map to Rust's `i64`/`f64`/`bool`, all of which are `Copy` types — duplicating one is always just a bitwise copy, identical in cost and effect to a `.clone()`. 
Because of this, `move` on a primitive doesn't actually transfer anything: the codegen difference (skipping the `.clone()` call) is real, but the *behavior* is identical either way. 
The original variable stays valid and usable after a `move` of a primitive — Rust copies it instead of consuming it, the same way it would for any other use of a `Copy` value.

```deor
int a = 5
int b = move a
print(a)  # still valid — a was never actually consumed
```

This means `move` only has a real, observable effect (the original becomes inaccessible) on non-`Copy` types: `string`, list shapes, structs, and validator-wrapped structs. `move`-ing a `string`, 
list, or struct genuinely consumes the source — using it afterward is a real Rust "use of moved value" compile error. `move`-ing an `int`/`float`/`bool` never produces that error, since there's 
nothing to consume — it's an inert, no-op annotation there. Reserve `move` for the non-`Copy` cases where it's actually doing something, so its presence in code reliably signals "this value is really being consumed here."

---

## Function Arguments

Pass a value into a function without cloning:

Deor:
```deor
do_something(move big_list)
# big_list cannot be used after this point
```

Rust:
```rust
do_something(big_list);
```

Use this when the function takes the last or only use of a large list or struct and cloning would be wasteful.

---

## Loop Iteration

Iterate a collection consuming each element rather than cloning:

Deor:
```deor
for move (item in collection)
    process(item)
```

Rust:
```rust
for item in collection {
    process(item);
}
```

The collection itself is consumed — it cannot be used after the loop.

---

## Destructuring

Extract fields from a struct without cloning them:

Deor:
```deor
move (label, points) in score
# a later (something_else) in score will error
```

Rust:
```rust
let label = score.label;
let points = score.points;
```

`score` cannot be used afterward for any field that was moved out. See [Structs — Destructuring](docs/structs.md#destructuring-in).

---

## Struct Construction

Build a struct from fields without cloning them:

Deor:
```deor
Score built = move (label, points)
```

Rust:
```rust
let built = Score { label, points };
```

Fields are moved into the struct rather than cloned. Each source variable is consumed and cannot be used after the construction. 
See [Structs — Struct Construction](docs/structs.md#struct-construction).

---

## Variable Assignment

Transfer ownership into a new binding:

Deor:
```deor
string new_var = move prev_var
# prev_var is not accessible here and below
```

Rust:
```rust
let new_var: String = prev_var;
```

---

## Use-After-Move Across `if`/`else`

Move tracking is flow-sensitive across an `if`/`else-if`/`else` chain, matching `rustc`'s own reachability logic: a move inside one branch doesn't poison a sibling branch, since only one branch actually runs.

```deor
if cond
    string used = move nates_string
    print(used)
else
    print(nates_string)   # fine — this branch never moved it
```

But a variable moved in *every* reachable branch of the chain is considered moved once the chain closes, because `rustc` can't know at that point which branch actually ran:

```deor
if cond
    string used = move nates_string
    print(used)
else
    string also_used = move nates_string
    print(also_used)
print(nates_string)   # error — moved on every path through the chain above
```

Two related details:

- The self-reassignment idiom `x = f(move x)` is specially exempted from poisoning `x` — the old value is consumed to build the new one, and a fresh value occupies `x` again as soon as the statement finishes, so `x` stays usable immediately after.
- Moving the same variable twice (`move x` ... `move x` again) gets its own distinct error ("already moved earlier — 'move' cannot consume the same variable twice") rather than the ordinary use-after-move message, since the second `move` is what actually double-consumes it rather than a later read.

---

## Move in `return`

`move` has no effect at all in a `return` statement, for *any* type — not because of `Copy`, but because `return` never clones to begin with. Every other site that defaults to 
cloning a bare identifier (function call arguments, typed-binding right-hand sides, list literal items) explicitly opts into that behavior; `return` was never one of them.

```deor
fn string make_label(string prefix)
    string label = prefix
    return label
```

```rust
fn make_label(prefix: String) -> String {
    let label: String = prefix.clone();
    return label;
}
```

`return label` and `return move label` generate byte-for-byte identical Rust here, even though `string` is not `Copy`. Once a function returns, the local being returned is 
never read again regardless, so there was never a clone to opt out of — `return` already behaves like an implicit move every time. Writing `move` before a `return` value doesn't hurt anything, but it never changes the generated code.

---

## Move in String Concatenation

`move` also has no effect inside `s_join`'s bracket-literal item list, for the opposite reason it's inert on a `Copy` type: `s_join([...])` never clones its operands in the 
first place, so there's nothing to opt out of. It compiles to `[...].concat()`, which only ever borrows (`.as_str()`) — every operand, `move`d or not, stays valid afterward.

```deor
string nates_string = "hello"
print(s_join([move nates_string, " hi"]))
print(nates_string)   # still valid
```

Writing `move` here isn't wrong, it just doesn't change anything — same as `move` on an `int`.

---

## When to Use

`move` is a performance tool. Deor's default clone-everything behavior is always *correct* — your program will produce the right answers without it. But cloning a large list or 
struct on every call or loop iteration has a real cost, and `move` eliminates that cost by transferring ownership instead of copying.

The tradeoff: the original variable is gone after a move. If you need the value again, you cannot use `move`. Reach for `move` when:

- A large collection or struct is passed to a function and never used again afterward
- A loop iterates a collection that can be consumed rather than kept
- A struct is built from fields that have no further use after construction

Use `rust` blocks for the most performance-critical paths where even the transpiler layer is too much overhead.

Reminder: `move` is a no-op on `Copy` types (`int`/`float`/`bool`) and in `return` statements — see above.