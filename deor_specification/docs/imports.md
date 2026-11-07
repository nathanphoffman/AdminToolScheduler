<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Imports

Imports use the `import` keyword followed by a path:

```deor
import "models/customer.deor"
```

Imports must appear at the top of each file, before any declarations. Placing an import after a function, struct, or any other declaration is a hard error.

All imports are global — there is no way to scope or privatize specific items. Everything declared at the root level is available everywhere. This encourages good naming practices and descriptive, unambiguous identifiers.

If the same file is imported more than once (directly or transitively), the later imports are silently ignored — each file is loaded exactly once, the first time it is encountered.

There is no way to make a root-level declaration private. Use descriptive naming to avoid collisions and keep the global namespace clean. This also encourages smaller, focused files — ideal Deor style.

## Import Ordering

The importer resolves imports **depth-first**: when it encounters an import statement it fully loads that file and all of its transitive dependencies before continuing. This means a file's dependencies always appear before that file's own declarations in the merged output.

The order you write imports only directly controls the relative ordering of files that have no dependency relationship with each other (siblings). Transitive dependencies are always resolved first regardless of where you list them.

The ordering of declarations in the generated Rust output does not affect correctness — Rust does not require forward declarations within a module, and the type registry of structs, shapes, enums, and types is built from the full merged token stream before code generation begins, so a name can be referenced anywhere in the merged output regardless of where it was declared. The only thing import order actually affects is **collision resolution**: when two files define a declaration with the same name, the first one encountered in the merged stream wins. This is the default, loose behavior — see [Duplicate Top-Level Names](enforced_practices.md#duplicate-top-level-names) for the opt-in pragmas that turn same-name collisions into hard errors instead.

### Ordering does not cause errors

Because of that, a file can use a type or function from another file it never imports directly, as long as something else in the merged output pulled that file in first:

```deor
# imports.deor — order does not matter here
import "services/billing.deor"   # uses Customer, even if it doesn't import customer.deor itself
import "models/customer.deor"    # defines Customer
```

This still works, but relying on an implicit dependency like this is bad practice — `billing.deor` should import `models/customer.deor` itself so its dependencies are self-documenting.

## Two Valid Approaches

There are two ways to manage imports in a Deor project. Both are valid.

### Option A: Centralized imports file (recommended)

Create a single `imports.deor` that lists every file in the project, then import only that file from `main.deor`.

`main.deor`
```deor
import "imports.deor"

fn void main()
    ...
```

`imports.deor`
```deor
import "lib/types.deor"
import "models/customer.deor"
import "utility.deor"
import "services/billing.deor"
```

Depth-first resolution (see [Import Ordering](#import-ordering) above) means you never have to manually order transitive dependencies here — only sibling files that don't depend on each other and happen to define colliding names.

| Upsides | Downsides |
|---|---|
| One place to see every file in the project | Every new file must be added here manually |
| Explicit control over sibling ordering | Individual files do not document their own dependencies |
| Clean `main.deor` with no import noise | Slightly more coordination overhead as the project grows |
| Easy to audit the full file graph at a glance | |

### Option B: Per-file imports

Each file imports only what it directly needs. Depth-first traversal handles ordering automatically, same as Option A.

`main.deor`
```deor
import "services/billing.deor"

fn void main()
    ...
```

`services/billing.deor`
```deor
import "models/customer.deor"
import "utility.deor"

fn void process_billing(Customer c)
    ...
```

`models/customer.deor`
```deor
import "lib/types.deor"

struct Customer
    ...
```

| Upsides | Downsides |
|---|---|
| Each file is self-documenting about its dependencies | The same file may be "imported" in many places (harmless but noisy) |
| Adding a new file only requires importing it where needed | Sibling ordering (for collision resolution) is implicit — determined by traversal order |
| Ordering is handled automatically by the traversal | Harder to get a full picture of the project's file graph |
| Scales naturally as the project grows | |
