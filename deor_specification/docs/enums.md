<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Enums

An `enum` declares a named set of variants. Each variant is a distinct value of that type. Enums are enforced as PascalCase names.

## Declaration

Deor:
```deor
enum ColorTag
    Red
    Green
    Blue
    Orange
    Purple
    Yellow
    White
    Black
```

Rust:
```rust
#[derive(Clone, Copy, PartialEq, Debug)]
enum ColorTag {
    Red,
    Green,
    Blue,
    Orange,
    Purple,
    Yellow,
    White,
    Black,
}
```

Any number of variants. Each is a plain name — no associated data.

**Variant names must be unique across all untyped enums.** A bare variant name (`Medium`) is resolved through one table shared by every enum in the program, including imported files, so two untyped enums cannot both have a `Medium` — the second is a transpiler error. Rename one (for example `MediumStretch`). Typed enums are exempt: their variants are only read with `(Variant) in EnumName`, which names the enum explicitly.

---

## Assignment

Deor:
```deor
ColorTag background = Blue
ColorTag foreground = White
```

Rust:
```rust
let background: ColorTag = ColorTag::Blue;
let foreground: ColorTag = ColorTag::White;
```

---

## Checking Variants

Use `if` / `else if` with `is`. No pattern matching — the same `is` operator used for all equality in Deor.

Deor:
```deor
if background is Blue
    msg as "blue background"
    print(msg)
else if background is Red
    msg as "red background"
    print(msg)
else
    msg as "other"
    print(msg)
```

Rust:
```rust
if background == ColorTag::Blue {
    println!("{}", "blue background");
} else if background == ColorTag::Red {
    println!("{}", "red background");
} else {
    println!("{}", "other");
}
```

Exhaustiveness is not enforced — write an `else` branch as a catch-all when needed.

---

## In Structs and Function Signatures

Enums work as struct fields, function parameters, and return types — the same as any other type.

Deor:
```deor
struct Theme
    string name
    ColorTag background
    ColorTag foreground

fn string describe(ColorTag color)
    other as "other"
    red as "red"
    green as "green"

    if color is Red
        return red
    else if color is Green
        return green
    else
        return other

fn void main()
    name as "Ocean"
    ColorTag background = Blue
    ColorTag foreground = White
    Theme theme = (name, background, foreground)
    string label = describe(background)
    print(label)
```

Rust:
```rust
struct Theme {
    name: String,
    background: ColorTag,
    foreground: ColorTag,
}

fn describe(color: ColorTag) -> String {
    if color == ColorTag::Red {
        return "red".to_string();
    } else if color == ColorTag::Green {
        return "green".to_string();
    }
    return "other".to_string();
}
```

---

## Typed Enums (Value-Backed)

Sometimes a set of named variants each needs a real backing value — an HTTP status, a priority level with actual ordering. `enum string/int/float/bool Name` gives each variant a literal value of that type, one per line as `Variant = value`:

Deor:
```deor
enum string Color
    Red = "Red"
    Green = "Green"
    Blue = "Blue"

enum int Priority
    Low = 1
    Medium = 2
    High = 3
```

A typed enum has no Rust type behind it at all — it's resolved entirely at compile time. Instead of assigning a variable of that type, you pull a variant's value straight out with `(Variant) in EnumName`, the same destructuring syntax used for structs:

Deor:
```deor
(Red) in Color
print(Red)

(Low, High) in Priority
int range = High - Low
```

Rust:
```rust
let Red: String = "Red".to_string();

let Low: i64 = 1;
let High: i64 = 3;
let range: i64 = High - Low;
```

`Red`, `Low`, and `High` come out as plain `string`/`int` variables, so `High - Low` above is ordinary integer subtraction — not enum comparison.

Variant naming rules match untyped enums (PascalCase, 3+ characters), but the `= value` requirement flips: a typed enum variant **must** have `= value`; an untyped variant **must not**, and can never carry data (`Variant(...)` is a transpiler error either way).

A typed enum's name isn't a usable type anywhere else — no Rust enum is generated for it, so there's no `Color background = Red` and no `fn void thing(Color c)`. The only thing you can do with one is extract variant values via `(Variant) in EnumName`.

This is enforced by the transpiler — `Color background = Red` is a clean Deor-level error ("not a known type"), not just a later `rustc` failure.

**Conversion notes:** the `enum` declaration itself emits nothing in the generated Rust. Extraction becomes a plain `let` binding with the literal value already filled in — `.to_string()` for `string`-backed enums, a raw literal for `int`/`float`/`bool`.