<!-- title: Deor Specification -->
<!-- [Deor Specification Index](index.md) -->
<!-- themes: blackboard -->
# Collections
List operations assume a list shape has already been declared — see [Shapes](docs/shapes.md) for how to declare one and use it in function signatures and struct fields.

---
## Empty List
To define an empty list use the `empty` keyword. `[]` is never valid for initializing an empty list — `[` and `]` are only used for list literals with items.

Deor:
```deor
stringList list_names = empty
```

Rust:
```rust
let mut list_names: Vec<String> = Vec::new();
```

Check emptiness with `is empty` / `is not empty` — see [Operators — Comparison](docs/operators.md#comparison).

---
## Index Read
Elements are read by index using `at`. Zero-indexed, matching Rust's behavior.

Deor:
```deor
intList scores = [10, 20, 30, 40]
int first = scores at 0    # 10
int last = scores at 3     # 40
```

Rust:
```rust
let scores: Vec<i64> = vec![10.clone(), 20.clone(), 30.clone(), 40.clone()];
let first: i64 = scores[0 as usize].clone();
let last: i64 = scores[3 as usize].clone();
```

Dynamic computed indices are fine:

Deor:
```deor
int idx = 2
int mid = scores at idx    # 30
```

Rust:
```rust
let idx: i64 = 2;
let mid: i64 = scores[idx as usize].clone();
```

Out-of-bounds access is a runtime panic. The transpiler inserts `as usize` casts and a trailing `.clone()` on all index reads automatically.

---
## Index Write
Elements are replaced by index using `at` on the left side of an assignment. The right-hand side must be a named variable of the list's element type.

Deor:
```deor
rooms at idx = new_room
scores at idx = updated_score
```

Rust:
```rust
rooms[idx as usize] = new_room.clone();
scores[idx as usize] = updated_score.clone();
```

Out-of-bounds assignment is a runtime panic. The right-hand side is always cloned, same as any other named-variable assignment.

---
## Append
`at end` appends a new element to the end of the list. `end` is a reserved keyword meaning "the position after the last element" — it is only valid in this position.

Deor:
```deor
result at end = item
rooms at end = new_room
```

Rust:
```rust
result.push(item.clone());
rooms.push(new_room.clone());
```

Any identifier pushed is always cloned, regardless of type — the transpiler does not special-case this.

---
## Remove
`remove at` removes the element at a given index, shifting subsequent elements left.

Deor:
```deor
result remove at 2
```

Rust:
```rust
result.remove(2);
```

For removing multiple elements, remove from highest index to lowest to avoid index-shifting errors:

Deor:
```deor
result remove at 5
result remove at 2
result remove at 1
```

Rust:
```rust
result.remove(5);
result.remove(2);
result.remove(1);
```

---
## No Membership Test
Deor has no built-in membership operator. For a simple value check against `int`, `float`, or `string` lists, `lib/list.deor` ships `l_T_contains`/`l_T_index_of` — see [Libraries](docs/libs.md#liblistdeor). For anything else (a predicate, a custom struct field match, etc.), write an explicit loop or define a reusable helper function:

Deor:
```deor
shape matchFunc = func of Room to bool

fn bool any_match(roomList items, matchFunc predicate)
    for item in items
        if predicate(item)
            return true
    return false
```

---
## Updating a Struct Inside a List
Struct values inside a list are replaced, not mutated in place — since Deor has no dot syntax, there's no way to reach into a list element's field directly. Extract the struct, build an updated copy with `with`, write it back.

Deor:
```deor
# 1. Read the existing struct
Room old_room = rooms at idx

# 2. Build the updated version
Squarefeet area = 25
int ceiling_height = 10
Room new_room = old_room with (area, ceiling_height)

# 3. Write back
rooms at idx = new_room
```

Rust:
```rust
let old_room: Room = rooms[idx as usize].clone();
let area: Option<Squarefeet> = Squarefeet::new(25);
let ceiling_height: i64 = 10;
let new_room: Room = Room { area, ceiling_height, ..old_room.clone() };
rooms[idx as usize] = new_room.clone();
```