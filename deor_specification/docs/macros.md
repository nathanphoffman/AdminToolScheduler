<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Macros

A macro is a named block of code that is inlined at every `macro_run` call site — equivalent to copy-pasting the macro body at that point in the source.

Macros are useful because they avoid the clone overhead of function calls while still letting you organize repetitive logic. 
This matters most inside tight loops where calling helper functions repeatedly introduces unnecessary cloning. Unlike Rust macros, Deor macros need no parameter declarations 
and require no special syntax — they benefit from Rust's compile-time checks because the inlined code goes through the same transpile and compile pipeline.

Macros can be declared at the top level or inside a function body. A top-level macro is available to any file that imports the file it is defined in — no special import syntax needed. 
A macro declared inside a function body is locally scoped: it exists only within that block and is not visible outside it.

You can call other macros with `macro_run` from inside a macro body — nesting calls is fully supported. However, defining a `macro` inside another macro body is a compile error. 
Define all macros at the appropriate scope and call them with `macro_run`.

```deor
macro say_hello
    print(hello)

fn void greet()
    hello as "Hi There"
    macro_run say_hello

    hello as "Hi There Again"
    macro_run say_hello

# output is "Hi There"
# and "Hi There Again"
```

---

## Macro Bodies Are Contained

A `macro` body is always inlined as if it were wrapped in Deor's `block` scoping keyword — automatically, with nothing extra to write. 
That means any variable a macro declares is local to that one inlining and never escapes into whatever called it. Call the same macro twice in a row and there's no collision:

A macro can still read and assign to variables already declared in the caller's scope — containment only stops it from introducing *new* names that outlive the call. 
If you want to hand a computed value back to the caller, have the caller declare the variable first and let the macro assign into it:

```deor
macro double_into_result
    result = value * 2

fn int double(int value)
    int result = 0
    macro_run double_into_result
    return result
```

The one construct that's allowed to declare a variable that escapes into the caller's scope is `unsafe_macro` — see below.

---

## `block` Inside Macros

`block` is a general-purpose scoping keyword — see [Enforced Practices — Variable Shadowing](docs/enforced_practices.md#variable-shadowing). 
Since a macro body is already contained on its own, you rarely need it *inside* a macro purely to stop a leak into the caller — that's automatic. 
It's still useful if you want to scope off a subset of a macro's own variables from the rest of its body, exactly as `block` works anywhere else in Deor:

```deor
macro process_batch
    block
        temp as compute_intermediate()
        print(temp)
    # temp is gone here, even though we're still inside the same macro body
    finalize()
```

---

## `unsafe_macro` — Deliberately Leaking State

Sometimes a macro needs to hand a *new* variable to its caller — not assign into one the caller already declared, but introduce one that didn't exist before. 
The canonical case is a start/end pair that brackets a piece of code, where the start half has to introduce state the end half will read later:

```deor
unsafe_macro timer_start
    int _timer_start = now_ms()

macro timer_end
    int _timer_elapsed = elapsed_ms(_timer_start)
    string _timer_str = n_to_str(_timer_elapsed)
    string _timer_sfx = "ms"
    print(s_join([_timer_label, _timer_str, _timer_sfx]))
```

```deor
string _timer_label = "[timer] load: "
unsafe_macro_run timer_start
tokenList raw_tokens = collect_all_tokens_with_all_imports(input_path)
macro_run timer_end
```

Only `timer_start` needs `unsafe_macro` — it introduces `_timer_start`, which `timer_end` then reads from a completely separate `macro_run` call later in the same function. `timer_end` itself doesn't need to leak anything (it only reads `_timer_start`/`_timer_label` and uses its own locals internally), so it stays a plain, contained `macro`, called with plain `macro_run`.

`unsafe_macro` behaves like `macro` in every way except one: its body is **not** automatically wrapped in `block`, so any variable it declares becomes part of the caller's scope. The name 
is deliberate — reach for it only when a macro genuinely needs to introduce new state for something else to read later, and prefer the caller-predeclares-it/macro-assigns-into-it 
pattern shown above whenever that's enough.

**Call it with `unsafe_macro_run`, not `macro_run`.** Using the wrong call keyword for either kind is a transpiler error either direction, so a call site's leak potential is always 
visible without checking the definition — see [Enforced Practices — Unsafe Macro Nesting](docs/enforced_practices.md#unsafe-macro-nesting) for the compile-time check and a worked example.

**Restriction:** an `unsafe_macro` cannot call, or be called from inside, another `unsafe_macro`. Two unwrapped leaking macros chained together would let a leak travel an unbounded distance 
up the call chain — the exact "unclear what scope a macro can touch" problem containment exists to prevent. This is the only restriction — an `unsafe_macro` can freely call 
(or be called by) an ordinary `macro`, since that macro's own `block` wrap already contains anything spliced into it; the leak dies right there regardless of which side is unsafe. 
See [Enforced Practices — Unsafe Macro Nesting](docs/enforced_practices.md#unsafe-macro-nesting) for the compile-time check and a worked example of the error.

---

## Limiting Cross-File Macro Nesting

`macro_run` calls can nest across files as freely as within one — there's no restriction by default. If a chain of macros calling macros grows hard to trace because 
it keeps hopping between files, opt into `ENFORCE_MACRO_FILE_DEPTH` to cap it. Same-file macro calls are always free regardless of the limit; only crossing into a different file counts against 
it. See [Enforced Practices — Limiting Cross-File Macro Calls](docs/enforced_practices.md#limiting-cross-file-macro-calls) for the pragma and worked example.


