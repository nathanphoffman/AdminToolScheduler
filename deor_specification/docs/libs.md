<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Standard Library

The `lib/` directory contains importable Deor files that ship with the transpiler. Each file is a normal `.deor` file — import it like any other file and its functions become available. No package manager, no crates required (except where noted).

```deor
import "lib/string.deor"
import "lib/math.deor"
```

Six of the library files are **parameterized** — they use `where Placeholder = Type` to produce a concrete, type-safe version of the module. `T` is the conventional placeholder name but any valid identifier works. See [Parameterized Imports](#parameterized-imports) below.

## Do Not Edit `lib/` Files

Every file in `lib/` is stamped with a `DO NOT EDIT` header for a reason: `deor update` overwrites the entire `lib/` directory with the latest standard library on every run. Any changes made directly to a file in `lib/` are silently discarded the next time you update.

To add functions to a standard library module, create a sibling file named `lib/<module>_extension.deor` and import it alongside the module it extends:

```deor
import "lib/string.deor"
import "lib/string_extension.deor"
```

`lib/string_extension.deor`
```deor
fn string xs_snake_case(string str)
    rust
        str.to_lowercase().replace(' ', "_")
```

The updater only ever writes the standard library's own filenames, so `lib/string_extension.deor` (or `lib/math_extension.deor`, `lib/list_extension.deor`, etc.) is never touched — your additions survive updates indefinitely. Prefix functions with `x` + the module's standard library letter (`xs_` for `string_extension.deor`, `xm_` for `math_extension.deor`) so they're unmistakably non-standard while still reading as part of the same module. See [Naming Convention](#naming-convention) below for the full scheme, including brand-new (non-extension) libs.

---

## `lib/string.deor`

String utilities. `+` cannot be used with strings — use `s_join`/`s_join_with` for concatenation.

| Function | Signature | Description |
|---|---|---|
| `s_trim` | `string → string` | Strip leading and trailing whitespace |
| `s_trim_start` | `string → string` | Strip leading whitespace only |
| `s_trim_end` | `string → string` | Strip trailing whitespace only |
| `s_to_upper` | `string → string` | Uppercase |
| `s_to_lower` | `string → string` | Lowercase |
| `s_contains` | `string, string → bool` | True if the string contains the needle |
| `s_starts_with` | `string, string → bool` | True if the string starts with the prefix |
| `s_ends_with` | `string, string → bool` | True if the string ends with the suffix |
| `s_index_of` | `string, string → int` | Position of needle, or `-1` if not found |
| `s_replace` | `string, string, string → string` | Replace all occurrences of `from` with `dest` |
| `s_substring` | `string, int, int → string` | Characters from `start` (inclusive) to `end` (exclusive) |
| `s_char_at` | `string, int → string` | Single character at index as a string |
| `s_repeat` | `string, int → string` | Repeat the string `count` times |
| `s_split` | `string, string → stringList` | Split on delimiter, returns a `stringList` |
| `s_split_whitespace` | `string → stringList` | Split on runs of whitespace, returns a `stringList` |
| `s_chars` | `string → stringList` | Every character as its own single-character string, returns a `stringList` |
| `s_pad_left` | `string, int → string` | Right-align to `width`, padding with leading spaces |
| `s_pad_right` | `string, int → string` | Left-align to `width`, padding with trailing spaces |
| `s_join` | `stringList → string` | Join a list of strings with no separator |
| `s_join_with` | `stringList, string → string` | Join a list of strings with a separator |

```deor
import "lib/string.deor"

string sentence = "  hello world  "
string trimmed = s_trim(sentence)
string needle = "world"
bool found = s_contains(trimmed, needle)
string space = " "
stringList words = s_split(trimmed, space)
string upper = s_to_upper(trimmed)
string dest = "Deor"
string replaced = s_replace(trimmed, needle, dest)
string target = "hello"
int pos = s_index_of(trimmed, target)
```

---

## `lib/math.deor`

Integer and float math operations.

| Function | Signature | Description |
|---|---|---|
| `m_abs` | `int → int` | Absolute value |
| `m_sign` | `int → int` | −1, 0, or 1 |
| `m_min` | `int, int → int` | Smaller of two |
| `m_max` | `int, int → int` | Larger of two |
| `m_clamp` | `int, int, int → int` | Clamp value between low and high |
| `m_pow` | `int, int → int` | Integer exponentiation |
| `m_absf` | `float → float` | Absolute value |
| `m_minf` | `float, float → float` | Smaller of two |
| `m_maxf` | `float, float → float` | Larger of two |
| `m_clampf` | `float, float, float → float` | Clamp value between low and high |
| `m_powf` | `float, float → float` | Float exponentiation |
| `m_sqrt` | `float → float` | Square root |
| `m_floor` | `float → int` | Round down |
| `m_ceil` | `float → int` | Round up |
| `m_round` | `float → int` | Round to nearest |
| `m_log` | `float → float` | Natural log |
| `m_log2` | `float → float` | Log base 2 |
| `m_log10` | `float → float` | Log base 10 |

```deor
import "lib/math.deor"

int clamped = m_clamp(val, low, high)
float root = m_sqrt(area)
```

---

## `lib/random.deor`

Random number generation with no external crates. Seeded automatically from the system clock.

| Function | Signature | Description |
|---|---|---|
| `m_rand_int` | `int, int → int` | Random integer in the inclusive range `[min, max]` |
| `m_rand_float` | `→ float` | Random float in `[0.0, 1.0)` |
| `m_rand_bool` | `→ bool` | Random boolean |

```deor
import "lib/random.deor"

min as 1
max as 6
int roll = m_rand_int(min, max)
float chance = m_rand_float()
```

---

## `lib/convert.deor`

Type conversions between Deor primitives.

| Function | Signature | Description |
|---|---|---|
| `c_float_to_int` | `float → int` | Truncate float to int |
| `c_int_to_float` | `int → float` | Widen int to float |
| `c_int_to_string` | `int → string` | Decimal string representation |
| `c_float_to_string` | `float → string` | String representation |
| `c_bool_to_string` | `bool → string` | `"true"` or `"false"` |
| `c_string_to_int` | `string → int` | Parse integer, returns `0` on failure |
| `c_string_to_float` | `string → float` | Parse float, returns `0.0` on failure |
| `c_string_to_bool` | `string → bool` | `"true"` → `true`, anything else → `false` |

The `string → int`/`float` conversions never crash — unparseable input silently becomes `0`/`0.0`, the same convention as C's `atoi`. This means a failed parse and a legitimate `"0"` are indistinguishable from the return value alone. That's fine for the overwhelming majority of cases (numeric input is rarely intentionally `0`), but if `0` is a meaningful, distinct input for your use case, check the raw string before converting (`if raw is "0"`) rather than relying on the converted value.

```deor
import "lib/convert.deor"

float precise = c_int_to_float(count)
string label = c_int_to_string(score)
```

---

## Parameterized Imports

Deor has no generics, so a file like `lib/list.deor` can't just be written once for "any type T" the way a generic function could. Instead, `where Placeholder = Value` stands in for that: the transpiler textually substitutes the placeholder identifier throughout the imported file before merging it into the token stream, effectively stamping out a fresh, concrete copy of that file for whatever type you asked for. This is a general language feature — it works on any `.deor` file, not only standard library files.

```deor
import "lib/list.deor" where T = int          # standard library file
import "lib/list.deor" where T = Report
import "workers/pipeline.deor" where Job = Report   # your own file
```

The placeholder can be any valid identifier — `T` is the convention in the standard library but `Item`, `Job`, `Row`, or any other name works equally well. Whatever name you choose, the substitution rules apply using that name as the placeholder.

All Deor primitive types work as the concrete value: `int`, `float`, `string`, and `bool`.

Six standard library files (`lib/list.deor`, `lib/list_order.deor`, `lib/list_numeric.deor`, `lib/optional.deor`, `lib/result.deor`, `lib/tasks.deor`) are parameterized this way. See their sections below for details.

### Substitution rules

The substitution applies to all IDENT tokens in the imported file and to the content of any `rust` blocks. Four patterns are recognised, shown here with placeholder `T` and concrete type `Report`:

| Pattern in source | After substitution |
|---|---|
| `T` — exact match | `Report` |
| `TSender` — PascalCase prefix | `ReportSender` |
| `tSenderFunc` — camelCase prefix | `reportSenderFunc` |
| `t_T_spawn` — snake `_T_` segment | `t_report_spawn` |

Anything that does not match one of these four patterns is left unchanged.

---

## `lib/list.deor`

Parameterized list operations for any element type. Import once per type you need. The placeholder can be any valid identifier — `T` is the standard library convention but `Item`, `Row`, or any other name works.

```deor
import "lib/list.deor" where T = int
import "lib/list.deor" where T = Report     # any type works
import "lib/list.deor" where Item = Report  # any placeholder name works
```

After substitution the shape becomes `intList` (or `reportList`, etc.) and all function names replace `T` with the lowercase concrete type. This file only contains operations that are valid for **any** element type (they need nothing beyond `Clone`/`PartialEq`, which every Deor type has) — safe to import for `int`, `float`, `string`, `bool`, or any struct. Sorting, dedup, sum, min, and max live in separate files (below) because they need trait support (`Ord`, `Hash`, `Copy`) that not every type has — importing them for an unsupported type fails to compile even if you never call the function, since the whole file gets substituted into concrete, non-generic Rust code.

| Template | After `T = int` | Description |
|---|---|---|
| `l_T_first` | `l_int_first` | First element (panics if empty) |
| `l_T_last` | `l_int_last` | Last element (panics if empty) |
| `l_T_is_empty` | `l_int_is_empty` | True if the list has no elements |
| `l_T_reverse` | `l_int_reverse` | Reversed copy |
| `l_T_slice` | `l_int_slice` | Sub-list from `start` (inclusive) to `end` (exclusive) |
| `l_T_concat` | `l_int_concat` | Concatenate two lists |
| `l_T_contains` | `l_int_contains` | True if the list contains the item |
| `l_T_index_of` | `l_int_index_of` | Index of item, or `-1` if not found |
| `l_T_take` | `l_int_take` | First `count` elements |
| `l_T_drop` | `l_int_drop` | All elements after the first `count` |
| `l_T_push` | `l_int_push` | New list with item appended to the end |
| `l_T_pop` | `l_int_pop` | New list with the last element removed |

Signatures use `tList` for the list type and `T` for the element — after substitution these become `intList` and `int` respectively:

| Template | Signature (template) | Signature after `T = int` |
|---|---|---|
| `l_T_first` | `tList → T` | `intList → int` |
| `l_T_last` | `tList → T` | `intList → int` |
| `l_T_is_empty` | `tList → bool` | `intList → bool` |
| `l_T_reverse` | `tList → tList` | `intList → intList` |
| `l_T_slice` | `tList, int, int → tList` | `intList, int, int → intList` |
| `l_T_concat` | `tList, tList → tList` | `intList, intList → intList` |
| `l_T_contains` | `tList, T → bool` | `intList, int → bool` |
| `l_T_index_of` | `tList, T → int` | `intList, int → int` |
| `l_T_take` | `tList, int → tList` | `intList, int → intList` |
| `l_T_drop` | `tList, int → tList` | `intList, int → intList` |
| `l_T_push` | `tList, T → tList` | `intList, int → intList` |
| `l_T_pop` | `tList → tList` | `intList → intList` |

```deor
import "lib/list.deor" where T = int

intList scores = [10, 20, 30]
start as 0
stop as 2
intList top = l_int_slice(scores, start, stop)
target as 10
bool has_ten = l_int_contains(scores, target)
count as 2
intList first_two = l_int_take(scores, count)
new_score as 40
intList grown = l_int_push(scores, new_score)
intList shrunk = l_int_pop(scores)
```

To join a list of strings with a separator, use `s_join_with` from [`lib/string.deor`](#libstringdeor) directly — no generic list import needed, since joining only ever makes sense for `string`.

---

## `lib/list_order.deor`

Sorting and dedup for list types whose element supports ordering and hashing — `int`, `string`, and `bool`. **Not valid for `float`** (no total ordering, so no `Ord`/`Hash`) or struct types (Deor structs only derive `Clone`, `PartialEq`, `Debug` — not `Ord`, `Eq`, or `Hash`). Import alongside `lib/list.deor` for the same `T`.

| Template | After `T = int` | Description |
|---|---|---|
| `l_T_sort` | `l_int_sort` | Sorted copy |
| `l_T_unique` | `l_int_unique` | Copy with duplicates removed, preserving order |

```deor
import "lib/list.deor" where T = int
import "lib/list_order.deor" where T = int

intList scores = [30, 10, 20, 10]
intList sorted = l_int_sort(scores)
intList deduped = l_int_unique(scores)
```

---

## `lib/list_numeric.deor`

Aggregate operations for numeric list types — intended for `int` and `float`. `l_T_sum` needs `Copy` (so not `string`, not structs); `l_T_min`/`l_T_max` need `Ord` + `Default` (compiles for `string`/`bool` too, but grouped here since numeric lists are the practical use case). Import alongside `lib/list.deor` for the same `T`.

| Template | After `T = int` | Description |
|---|---|---|
| `l_T_sum` | `l_int_sum` | Sum of all elements |
| `l_T_min` | `l_int_min` | Minimum element |
| `l_T_max` | `l_int_max` | Maximum element |

```deor
import "lib/list.deor" where T = int
import "lib/list_numeric.deor" where T = int

intList scores = [10, 20, 30]
int total = l_int_sum(scores)
int best = l_int_max(scores)
```

`l_T_sort`, `l_T_sum`, `l_T_min`, `l_T_max`, `l_T_contains`, `l_T_index_of`, and `l_T_unique` require the element type to implement `Ord` / `Hash` / `Clone` in the generated Rust. They work naturally for `int`, `float`, and `string`. For custom structs, use a `rust` block instead.

---

## `lib/optional.deor`

A value that may be missing. It is a [validator type](docs/validator_types.md) over `T` whose rule accepts anything, so it is only not valid when nothing was assigned — the `ValidInt` pattern from [Replacing Null and Undefined](docs/validator_types.md#replacing-null-and-undefined), stamped out for any type, struct included.

```deor
import "lib/optional.deor" where T = string
```

After substitution with `T = string`:

| Name | Kind | Description |
|---|---|---|
| `StringOptional` | validator type | wraps a `string`; not valid until a value is assigned |

| Function | Signature | Description |
|---|---|---|
| `o_string_some` | `(string val) → StringOptional` | An optional holding `val` |
| `o_string_none` | `() → StringOptional` | An optional holding nothing |
| `o_string_or` | `(StringOptional opt, string fallback) → string` | The held value, or `fallback` when nothing is held |

```deor
struct Person
    string first
    StringOptional middle
    string last

StringOptional middle = o_string_some(given)
StringOptional nobody = o_string_none()

if middle is valid
    string name = (avow middle)

string shown = o_string_or(nobody, blank)    # blank — nothing is held
```

An optional is an ordinary validator type, so `is valid`, `is not valid`, `avow`, struct fields and function parameters and returns all work as documented. Import the file once per type and each gets its own names (`IntOptional` / `o_int_some`, `PointOptional` / `o_point_some`, and so on).

---

## `lib/result.deor`

A result that is either a value or an error message, for functions that can fail. Built on a [validator type over a struct](docs/validator_types.md#validator-types-over-structs).

```deor
import "lib/result.deor" where T = Config
```

After substitution with `T = Config`:

| Name | Kind | Description |
|---|---|---|
| `ConfigOk` | validator type | wraps a `Config`; valid whenever a value was assigned |
| `ConfigResult` | struct | fields `value` (a `ConfigOk`) and `message` (a `string`) |

| Function | Signature | Description |
|---|---|---|
| `r_config_ok` | `(Config val) → ConfigResult` | Successful result: `value` is valid, `message` is empty |
| `r_config_error` | `(string message) → ConfigResult` | Failed result: `value` is not valid, `message` says what went wrong |

```deor
fn ConfigResult load(string path)
    if path is ""
        return r_config_error("path is empty")

    Config cfg = read_config(path)

    return r_config_ok(cfg)

fn void main()
    path as "app.conf"
    ConfigResult outcome = load(path)
    (value, message) in outcome

    if value is valid
        Config cfg = (avow value)
    else
        print(message)
```

The `is valid` check runs on the destructured `value`, not on the result itself. Import the file once per type (`T = int`, `T = string`, ...) and each gets its own `IntResult` / `r_int_ok`, `StringResult` / `r_string_ok`, and so on.

---

## `lib/map.deor`

String-to-string hash map backed by `Arc<Mutex<HashMap>>`. The `StringMap` is a `raw` type — pass it around freely, mutations are in-place through the shared reference.

| Function | Signature | Description |
|---|---|---|
| `h_make` | `→ StringMap` | Create an empty map |
| `h_set` | `StringMap, string, string → StringMap` | Insert or update a key |
| `h_get` | `StringMap, string → string` | Value for key, or `""` if absent |
| `h_has` | `StringMap, string → bool` | True if the key exists |
| `h_remove` | `StringMap, string → StringMap` | Remove a key |
| `h_size` | `StringMap → int` | Number of entries |
| `h_keys` | `StringMap → stringList` | All keys |
| `h_values` | `StringMap → stringList` | All values |

```deor
import "lib/map.deor"

StringMap config = h_make()
string host_key = "host"
string host_val = "localhost"
config = h_set(config, host_key, host_val)
string port_key = "port"
string port_val = "8080"
config = h_set(config, port_key, port_val)
bool has_host = h_has(config, host_key)
string host = h_get(config, host_key)
int count = h_size(config)
```

`h_set`/`h_has`/`h_get` are regular user-defined functions taking 2+ arguments, so — like any such call — every argument must be a named variable, not a literal (see [Enforced Practices — Named Arguments](docs/enforced_practices.md#named-arguments-user-defined-functions-only)).

---

## `lib/file.deor`

File system operations. All paths are strings. Functions that can fail return `bool` indicating success.

| Function | Signature | Description |
|---|---|---|
| `f_read` | `string → string` | Read entire file as a string, or `""` on failure |
| `f_write` | `string, string → bool` | Write content to file (creates or overwrites), returns success |
| `f_append` | `string, string → bool` | Append content to file (creates if absent), returns success |
| `f_exists` | `string → bool` | True if the path exists |
| `f_lines` | `string → stringList` | Read file as a list of lines |
| `f_delete` | `string → bool` | Delete a file, returns success |

```deor
import "lib/file.deor"

path as "log.txt"
start_msg as "starting up\n"
bool wrote = f_write(path, start_msg)
next_msg as "step two\n"
bool appended = f_append(path, next_msg)
string contents = f_read(path)
stringList lines = f_lines(path)
bool gone = f_delete(path)
```

---

## `lib/time.deor`

Timestamps and elapsed time. `n_now` returns Unix seconds as `int` (valid until 2038); use `n_now_ms` for millisecond precision as `float`.

| Function | Signature | Description |
|---|---|---|
| `n_now` | `→ int` | Current Unix timestamp in whole seconds |
| `n_now_ms` | `→ float` | Current Unix timestamp in milliseconds |
| `n_elapsed` | `int → int` | Seconds elapsed since the given `n_now` snapshot |
| `n_elapsed_ms` | `float → float` | Milliseconds elapsed since the given `n_now_ms` snapshot |

```deor
import "lib/time.deor"

float start = n_now_ms()
# ... do work ...
float ms = n_elapsed_ms(start)
print(c_float_to_string(ms))
```

---

## `lib/tasks.deor`

Pool-bounded parallel map over a typed list. Imports `lib/taskpool.deor` automatically.

```deor
import "lib/tasks.deor" where T = Score
```

After substitution with `T = Score`:

| Name | Kind | Description |
|---|---|---|
| `scoreList` | shape | `list of Score` |
| `scoreTransformFunc` | shape | `func of Score to Score` |

| Function | Signature | Description |
|---|---|---|
| `t_pool_make` | `() → TaskPool` | Create a thread pool sized to the number of logical CPU cores. Not `T`-substituted — same name regardless of `T`. |

| Function (before substitution) | Signature | Description |
|---|---|---|
| `t_T_run_all` | `TaskPool, tList, tTransformFunc → tList` | Map a list of T through a worker in parallel, return all results |

Pass a list of T and a worker function `T → T`; get back a list of results. All items are dispatched to the pool concurrently; the call blocks until every result is collected. Results are returned in completion order, not input order.

The pool caps concurrency automatically — dispatching 10 000 items still only runs `available_parallelism()` threads at once.

### Example

```deor
import "lib/tasks.deor" where T = Score

struct Score
    string label
    int points

fn Score make_score(string label, int points)
    Score built = move (label, points)
    return built

fn Score apply_bonus(Score score)
    (label, points) in score
    points = points * 2
    Score result = move (label, points)
    return result

fn void main()
    TaskPool pool = t_pool_make()

    string lbl_aaa = "accuracy"
    int pts_aaa = 10
    Score aaa = make_score(lbl_aaa, pts_aaa)

    scoreList jobs = empty
    jobs at end = aaa

    scoreTransformFunc worker = apply_bonus
    scoreList results = t_score_run_all(pool, jobs, worker)

    int count = len(results)
    for idx in range(count)
        Score res = results at idx
        (label, points) in res
        print(label)
        print(points)
```

### Primitive types

All Deor primitive types work as `T`:

```deor
import "lib/tasks.deor" where T = float
import "lib/tasks.deor" where T = int
import "lib/tasks.deor" where T = string
```

Primitives map to their Rust equivalents in the generated code (`float` → `f64`, `int` → `i64`, `string` → `String`).

### Importing multiple types

Each `where T = ...` import is independent:

```deor
import "lib/tasks.deor" where T = Request
import "lib/tasks.deor" where T = Report
```

---

## Writing Custom Wrappers

When you need functionality not in the standard library, wrap a Rust function in a small Deor function using a `rust` block. The Deor function owns the signature and naming; the `rust` block handles the implementation. Keep blocks small — one thing per block.

See [Rust Interop](docs/interop.md) for full `rust` block rules.

### Naming Convention

Follow the same prefix convention as the standard library to keep the global namespace readable:

Standard library prefixes (reserved, one letter):

| Prefix | Module |
|---|---|
| `s_` | `lib/string.deor` |
| `m_` | `lib/math.deor`, `lib/random.deor` |
| `l_` | `lib/list.deor` |
| `c_` | `lib/convert.deor` |
| `h_` | `lib/map.deor` |
| `f_` | `lib/file.deor` |
| `n_` | `lib/time.deor` |
| `t_` | `lib/tasks.deor`, `lib/taskpool.deor` |

Everything that isn't standard library — extensions to a standard module, brand-new custom libs, cargo crate wrappers, third-party files — gets an `x` prefix followed by the module's letter. One rule covers both "eXtension" and "eXternal":

| Prefix | File | Example |
|---|---|---|
| `xs_` | `string_extension.deor` (extends `string.deor`) | `xs_snake_case` |
| `xi_` | `internet.deor` (new custom lib) | `xi_get_ip_address` |
| `xr_` | `rand.deor` (cargo crate wrapper) | `xr_rand_int` |

Pick the module's letter the same way the standard library does — first letter of the concept the file represents (`s` for string, `i` for internet, `r` for rand). If two unrelated custom libs land on the same letter, pick a different one by hand; there's no automatic collision resolution.

### I/O

```deor
fn string read_line()
    rust
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end_matches('\n').to_string()
```

### Parsing

Wrap parse results in a validator type so the caller can check success:

```deor
type ParsedInt(int val)
    true

fn ParsedInt parse_int(string src)
    ParsedInt result
    rust
        if let Ok(num) = src.parse::<i64>() {
            result = Some(ParsedInt(num));
        }
    return result
```

```deor
ParsedInt parsed = parse_int(user_input)
if parsed is valid
    int val = (avow parsed)
    print(val)
```

The same pattern works for `ParsedFloat` — swap `i64` for `f64`.

### Cargo Crates

For anything requiring an external crate, add it to `Cargo.toml` manually and wrap it the same way. The prefix is `x` + the crate's first letter:

```deor
# rand.deor — wraps the rand crate, prefix xr_
fn int xr_rand_int(int min, int max)
    rust
        use rand::Rng;
        rand::thread_rng().gen_range(min..=max)
```
