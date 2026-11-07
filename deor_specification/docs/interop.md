<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Rust Interop

Deor transpiles to a single Rust file. When Deor cannot express something — a data structure, an algorithm, a crate call — you drop into a `rust` block. This is intentional and encouraged where appropriate.

---

## `rust` Blocks

A `rust` block is raw Rust inlined directly into the generated output. It is delimited by indentation — everything indented under `rust` is captured verbatim and emitted as-is.

```deor
fn string read_env(string key)
    rust
        std::env::var(key.as_str()).unwrap_or_default()
```

Deor parameters are available inside the block by their Deor names. The last expression is the return value and must match the Deor function's return type.

Deor does not check this — a `rust` block is spliced into the generated file verbatim, with no awareness of what type the block actually produces. If the last expression doesn't match the declared return type, Deor will transpile it without complaint and the mismatch only surfaces as a `rustc` error against the generated `.rs` file, not against your `.deor` source. Anyone writing a `rust` block is expected to know the Rust they're writing well enough to get this right.

---

## When to Use a `rust` Block

Use a rust block when Deor cannot express what you need — crate calls, closures, `HashMap`, async, type casting, anything that requires Rust's full type system. Do not reach for a rust block just to avoid Deor syntax.

The `raw` type exists for passing opaque Rust values through Deor code. It is the right pattern when you need to build something once in Rust and use it repeatedly — a HashMap built from a list, a compiled regex, a connection pool handle. `raw name = expr` must be assigned from a function call — a bare literal or an inline `rust` block on the right of `=` is a transpiler error (and so is `raw name as expr` — it must be `=`, not `as`).

Once declared, a raw variable can be passed to functions (as an argument, or captured from what they return) and used freely inside `rust` blocks. It **can never** be:

- given a different type via a typed binding or an `as` rebind
- reassigned
- used as an operand of a Deor operator (`+`, `is`, `and`, ...)
- passed to `len`, `crash`, or the bracket-literal form of `s_join` — these assume a concrete type
- declared as a struct field

Each of the above is a transpiler error. Passing a raw variable into an ordinary function call is fine — Rust's own compiler is what checks that value is used correctly there.

**Incorrect — transpiler errors:**
```deor
string val = index               # raw cannot be given another type
copy as index                    # as-rebinding is still a type capture
index = build_lookup_table()     # raw cannot be reassigned
int cnt = len(index)             # len assumes a concrete type
int total = index + 1            # operators assume a concrete type

raw bad1 = 5                     # must be a function call, not a literal
raw bad2 = rust
    42                           # must be a function call, not an inline rust block
raw bad3 as build_lookup_table() # must be '=', not 'as'

struct Config
    raw lookup_table              # raw cannot be a struct field
```

This is deliberate, not just a syntax restriction. The whole point of `raw` is "Deor can't see inside this value" — a literal like `5` isn't opaque at all (it's a plain `int` Deor already understands), and `x as some_fn()` is the ordinary binding form used everywhere for normal values, so it wouldn't visibly mark `x` as special either. Only `raw name = expr`, with a function call on the right, keeps every `raw` declaration honestly labeled — anyone reading `raw x = build_index()` knows on sight that `x` can't be used in Deor expressions, passed to `len`/`crash`/`s_join`, or reassigned.

So build it once inside a function, and hold the function's return value as `raw`:

```deor
fn Index build_index()
    rust
        entries.iter()
            .map(|e| (e.key.clone(), e.value.clone()))
            .collect::<std::collections::HashMap<String, String>>()

raw index = build_index()
string result = lookup(index, search_key)
```

This generates the same Rust you would write by hand — no boxing, no overhead.

---

## Global-Style References — Sharing State Across Functions

Deor has no global scope — every ordinary variable exists only inside the block where it's declared. A top-level `raw` type declaration is the one exception, and it's the only way to get something that behaves like a shared global: declare the opaque type once, give it a real Rust definition (usually wrapping the data in `Rc<...>` or `Arc<...>` so it's cheap to clone), build a single instance somewhere central, and thread that instance through function parameters. Because the underlying value is reference-counted, passing it down through many layers of calls is a cheap pointer clone, not a deep copy of the data.

`lib/map.deor`'s `StringMap` (see [Libs](docs/libs.md#libmapdeor)) is a real example already in the standard library:

```deor
raw StringMap

rust
    #[derive(Clone)]
    struct StringMap(std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, String>>>);

fn StringMap h_make()
    rust
        StringMap(std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())))
```

Once `StringMap` is registered this way, callers don't even write `raw` again — they just use it like any other declared type. Note that `h_set` takes 3 arguments, so — like any call with 2 or more arguments — each one must already be a named variable, not a literal (see [Enforced Practices — Named Arguments](docs/enforced_practices.md#named-arguments-user-defined-functions-only)):

```deor
StringMap config = h_make()
string host_key = "host"
string host_val = "localhost"
config = h_set(config, host_key, host_val)
```

Every function that takes a `StringMap` parameter is handed the same underlying `Arc<Mutex<...>>`, not a copy of the map.

**Borrowed Rust references can't live in a struct.** Deor has no lifetime syntax, so a type that is really a borrow (a handle written `&'a Game` in Rust) cannot be a struct field — the generated field would have no lifetime and `rustc` rejects it (E0106). Pass such a handle as its own function parameter instead, or wrap the underlying value in `Rc`/`Arc` and declare it as an owned `raw` type as shown above.

Only reach for this pattern when you actually need reference-style sharing for performance — a large data structure threaded through many layers of calls, or a hot path where repeated cloning would be measurably slow. For everything else, plain function-to-function parameter passing is simpler, keeps ownership explicit, and should stay the default.

---

## The Wrapping Pattern

The recommended pattern is a small rust block inside a Deor function. The Deor function owns the signature and naming; the rust block handles the implementation detail.

```deor
fn string json_get(string src, string key)
    rust
        let v: serde_json::Value = serde_json::from_str(&src).unwrap_or(serde_json::Value::Null);
        v.get(&key).and_then(|x| x.as_str()).unwrap_or("").to_string()
```

Keep rust blocks small. If a rust block is growing large, it is a signal that the logic belongs in a dedicated `.rs` file or should be broken into multiple wrapped functions. A rust block should do one thing.

---

## Using Transpiled Deor from Rust

Every top-level item Deor generates is `pub`, so the generated `.rs` file can be imported as a module from other Rust code:

- functions (`pub fn`)
- structs (`pub struct`), with every field `pub`
- enums (`pub enum`)
- shapes (`pub type` aliases)
- validator types (`pub struct Name(pub T)`, with a `pub fn new`)

Code inside `rust` blocks is copied verbatim, so any types or functions you define there stay private unless you write `pub` yourself.

---

## External `.rs` Files

If you have a large body of Rust code that does not belong inline, you can pull it in with `include!` inside a rust block:

```deor
rust
    include!("helpers.rs");
```

The path is relative to the generated output file. This should be used sparingly. Prefer Deor-wrapped rust blocks in `.deor` lib files over external `.rs` imports — they stay in version control naturally, travel with the import system, and keep the interop surface visible in one place.

Only use `include!` when the Rust code is genuinely too large or complex to live inline and has no Deor-facing wrapper worth writing.
