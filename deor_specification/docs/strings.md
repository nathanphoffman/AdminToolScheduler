<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Strings

String utility functions live in `lib/string.deor`. Import the whole file:

```deor
import "lib/string.deor"
```

All functions in `lib/string.deor` follow the `s_` prefix convention. They are regular user-defined functions, so arguments must be named variables — literals are not valid arguments. See [Enforced Practices](docs/enforced_practices.md#named-arguments-user-defined-functions-only).

---

## Escape Sequences

Standard escape sequences are supported inside string literals:

| Sequence | Meaning |
|---|---|
| `\n` | Newline |
| `\t` | Horizontal tab |
| `\\` | Literal backslash |
| `\"` | Literal double quote |

```deor
msg as "Hello\nWorld"
path as "C:\\Users\\name"
quote as "She said \"hello\""
```

```rust
let msg = "Hello\nWorld".to_string();
let path = "C:\\Users\\name".to_string();
let quote = "She said \"hello\"".to_string();
```

No other escape sequences are supported in v1. For Unicode escapes or raw byte strings, use a `rust` block.

An unrecognized escape (`\r`, `\x41`, etc.) isn't a transpiler error — the lexer silently passes it through as a literal backslash followed by the character, rather than rejecting it. `"bad\rescape"` becomes the literal 12-character text `bad\rescape` (backslash included), not a carriage return. Stick to the four sequences above; anything else quietly means something other than what it looks like.

---

## Concatenation

`+` cannot be used with strings — use `s_join`/`s_join_with` instead. Any `+` with a string literal or a `string`-typed variable on either side is a Deor-level error, regardless of what's on the other side:

```deor
string full = "hello " + name    # transpiler error
string other = aaa + bbb         # transpiler error
```

Rust's `+` for `String` only implements `Add<&str>`, never `Add<String>` — a chain of plain string variables (`aaa + bbb`) doesn't compile, and mixing a string with a number (`"count: " + count`) doesn't either. An earlier version of Deor tried to paper over this by detecting a literal somewhere in the chain and compiling to `[...].concat()`, but the detection had real gaps (a literal nested inside parens wasn't seen; a function call's return type wasn't classified), so some chains still silently produced broken Rust. Rather than keep patching that detection, `+` on strings is rejected outright — `s_join` covers every case unconditionally and is the only way to join strings:

```deor
string greeting = s_join(["hello ", name])
string line = s_join([prefix, content, "\n"])
string full = s_join([aaa, bbb])
```

```rust
let greeting: String = ["hello ", name.as_str()].concat();
let line: String = [prefix.as_str(), content.as_str(), "\n"].concat();
let full: String = [aaa.as_str(), bbb.as_str()].concat();
```

`s_join([...])` (the bracket-literal form) compiles to `[...].concat()`. Every operand is borrowed (`.as_str()`), except string literals, which are already `&str`. Nothing is ever cloned or moved, so every variable in the list — first or not — stays usable afterward, and `move` has no effect on any of them (see [Move — Move in String Concatenation](docs/move.md#move-in-string-concatenation)).

Mixing a string with a number is still rejected the same way — convert the number first, e.g. with a `rust` block helper:

```deor
fn string int_to_str(int n)
    rust
        n.to_string()

string msg = s_join(["count: ", int_to_str(count)])
```

---

## Examples

```deor
import "lib/string.deor"

string raw = "  Hello, World!  "
string clean = s_trim(raw)

string lower = s_to_lower(clean)
string query = "world"
bool found = s_contains(lower, query)

string csv = "apple,banana,cherry"
string sep = ","
stringList parts = s_split(csv, sep)
```

```rust
let raw: String = "  Hello, World!  ".to_string();
let clean: String = raw.trim().to_string();
let lower: String = clean.to_lowercase();
let query: String = "world".to_string();
let found: bool = lower.contains(query.as_str());
let csv: String = "apple,banana,cherry".to_string();
let sep: String = ",".to_string();
let parts: Vec<String> = csv.split(sep.as_str()).map(|s| s.to_string()).collect();
```

```deor
string path = "/api/users"
string slash = "/"
bool is_abs = s_starts_with(path, slash)

string filename = "report.pdf"
string ext = ".pdf"
bool is_pdf = s_ends_with(filename, ext)
```

`s_split` always returns at least one element — an input with no delimiter occurrences returns a single-element list containing the original string.

---

## Conversion Notes

| Deor | Rust |
|---|---|
| `s_join(["lit", b])` | `["lit", b.as_str()].concat()` |
| `s_contains(str, needle)` | `str.contains(needle.as_str())` |
| `s_starts_with(str, prefix)` | `str.starts_with(prefix.as_str())` |
| `s_ends_with(str, suffix)` | `str.ends_with(suffix.as_str())` |
| `s_trim(str)` | `str.trim().to_string()` |
| `s_to_upper(str)` | `str.to_uppercase()` |
| `s_to_lower(str)` | `str.to_lowercase()` |
| `s_split(str, delimiter)` | `str.split(delimiter.as_str()).map(\|s\| s.to_string()).collect()` |
| `s_join(parts)` | `parts.join("")` |
| `s_join_with(parts, sep)` | `parts.join(sep.as_str())` |
| `s_trim_start(str)` | `str.trim_start().to_string()` |
| `s_trim_end(str)` | `str.trim_end().to_string()` |
| `s_replace(str, from, dest)` | `str.replace(from.as_str(), dest.as_str())` |
| `s_index_of(str, needle)` | `str.find(needle.as_str()).map(\|i\| i as i64).unwrap_or(-1)` |
| `s_char_at(str, idx)` | `str.chars().nth(idx as usize).map(\|c\| c.to_string()).unwrap_or_default()` |
| `s_substring(str, start, end)` | `str.chars().skip(start).take(end - start).collect()` |
| `s_repeat(str, count)` | `str.repeat(count as usize)` |
| `s_chars(str)` | `str.chars().map(\|c\| c.to_string()).collect()` |
| `s_pad_left(str, width)` | `format!("{:>width$}", str, width = width as usize)` |
| `s_pad_right(str, width)` | `format!("{:<width$}", str, width = width as usize)` |
| `s_split_whitespace(str)` | `str.split_whitespace().map(\|s\| s.to_string()).collect()` |
