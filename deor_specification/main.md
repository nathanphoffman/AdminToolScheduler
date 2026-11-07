<!-- title: Deor Specification -->
<!-- [Deor Specification](main.md) -->
<!-- themes: blackboard -->

# Deor Language Specification

Deor is a small, highly-procedural, tabbed-block language that transpiles to Rust. It enforces near-book readability, explicit typing, predictable control flow — and exposes a `rust` block for access to the rust ecosystem.

Its goal is to provide a comfier entrance point to Rust with simple syntactical sugar and such uniform composition rules that all human and AI code does not look too dissimilar.

1) To create your first project in minutes visit [Getting Started](getting_started.md)

2) To learn about our core principles visit [Core Principles](core_principles.md)

3) To read the documentation visit [Documentation Index](index.md)

Deor Inspirations (generally ordered from heaviest to least):
1) **Python** - tabbed blocks and heavy readability; deor goes even further in some ways
2) **Rust** - transpiles to it, allows raw rust blocks, supports a lot of similar patterns
3) **C** - took macro inspiration, global root-level importing, and adopted {type} {name} syntax
4) **Fortran** - took pragma inspiration, enforced ordering, and heavy imperativism
5) **Mojo** - inspired the idea of mixing python-like syntax with rust (but with actual rust)
6) **Typescript** - inspired validator types (from DD TS design) and heavy con(de)struction
7) **Go** - took inspiration from its lean syntax, and inspired as, a renamed :=
8) **C#** - specifically took the very readable name "list" and {type} {name} preference