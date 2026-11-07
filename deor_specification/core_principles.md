<!-- title: Deor Specification -->
<!-- [Deor Specification](main.md) -->
<!-- themes: blackboard -->
# Core Principles

---

### Ease
- **Book Readability** - tabbed blocks, variables are 3+ chars, literals can only be passed if a func is 1 parameter
- **Flat Structure** - no namespacing, nested fns, or closures. All imports are global: C simplicity
- **Human not Symbols** - think mostly words, occasional `()`, and rare `[]`, but not: `{}` or `;` or `<>` etc.
- **Simple Macros** - allows easy code organization without the complexity of one-use functions passing around vars
- **Valid not Null** - validator types with `valid`/`not valid` formalize C-default values and simplify Rust Options

---

### Clarity
- **1st-Class not Only-Class** - first-class functions exist but are highly limited, and no lambdas are allowed
- **Explicit over Generic** - explicit types are clearer (although `shapes` allow some minimal generics)
- **No OOP** - structs cannot have methods: they are data only
- **Procedural** - code style is like that of Fortran, rigidly readable top to bottom
- **Uniform Composition** - destructuring / constructing requires exact variable names to match field names

---

### Leanness
- **Limited Syntax** - only one way to do most things: it makes all code, even AI code read similar
- **Only For** - no while/for distinction: `for if` is the while loop, `for in` is collection/range iteration
- **Slim Built-Ins** - only `len`, `range`, `crash`, `input/args`, and `print` -- all others use rust wrappers


---


### Rust Integration
- **Built-In Libs** - provides wrapped-Rust libs for advanced built-ins, no need to reinvent the wheel or learn Rust
- **Rust Blocks** - the `rust` block exposes raw Rust power, for when performance (or dict / bytes / references) matter
- **Rust Ecosystem** - since everything builds to Rust, cargo can be pulled in and rust functions wrapped. see: [Libs](docs/libs.md)
- **Transpiles to Rust** - can theoretically be up to as fast as raw Rust and has Rust safety as a backstop 


---

### Safety
- **Explicit Typing** - types are required, shapes are explicit generics and functions, and enums are available
- **Fns are Always In/Out** - all data flows through parameters and return values; all arguments are cloned by default
- **Sensible Mutability** - structs are immutable, variables and lists are mutable, everything is safely cloned
- **Validator Types** - `type` exposes a boolean predicate that validates data (e.g. `Positive > 0`)

