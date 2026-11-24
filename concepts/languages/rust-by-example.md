---
type: Reference
title: Rust by Example
description: Runnable, practical Rust examples covering standard library, traits, error handling, testing, and idiomatic patterns.
resource: https://doc.rust-lang.org/stable/rust-by-example/
tags:
- rust
- examples
- patterns
- std
- traits
- error-handling
- testing
- reference
status: stable
generated:
  by: role:maintainer
  at: 2026-10-14T00:00:00+09:00
verified: []
stale_after: 2027-04-14T00:00:00+09:00
sources:
- id: rust-lang-rbe-local
  resource: file://<home>/references/languages/rust-advanced/rust-by-example/src/
  title: Rust by Example Source (local mirror)
  author: team:rust-lang
  last_modified: 2026-04-19T08:40:28-03:00
  usage_count: 1
  usage_window:
    from: 2026-01-01T00:00:00Z
    to: 2026-12-31T00:00:00Z
- id: rust-lang-rbe-upstream
  resource: https://github.com/rust-lang/rust-by-example
  title: Rust by Example Upstream Repository
  author: team:rust-lang
  last_modified: 2026-04-19T08:40:28-03:00
---
# Overview

**Rust by Example (RBE)** is a collection of runnable examples illustrating Rust concepts and standard library usage. Each example is a complete, compilable program demonstrating idiomatic patterns.

# Major Sections

| Category | Topics | Local Path |
|----------|--------|------------|
| **Hello World** | Basics, comments, formatted print | `rust-by-example/src/hello.rs` |
| **Primitives** | Literals, tuples, arrays, slices | `rust-by-example/src/primitives.rs` |
| **Custom Types** | Structs, enums, `use`, C-like enums | `rust-by-example/src/custom_types/` |
| **Variable Bindings** | Mutability, scope, shadowing, `ref` | `rust-by-example/src/variable_bindings.rs` |
| **Types** | Casting, literals, inference, aliasing | `rust-by-example/src/types.rs` |
| **Conversion** | `From`/`Into`, `TryFrom`/`TryInto`, `ToString` | `rust-by-example/src/conversion/` |
| **Flow Control** | `if`/`else`, `loop`, `while`, `for`, `match` | `rust-by-example/src/flow_control/` |
| **Functions** | Methods, closures, higher-order functions | `rust-by-example/src/fn.rs` |
| **Modules** | Visibility, `use`, `super`/`self`, file hierarchy | `rust-by-example/src/mod.rs` |
| **Crates** | Library vs binary, `extern crate`, dependencies | `rust-by-example/src/crates/` |
| **Attributes** | `#[derive]`, `#[cfg]`, `#[allow]`, custom | `rust-by-example/src/attribute/` |
| **Generics** | Functions, traits, bounds, `where`, associated types | `rust-by-example/src/generics.rs` |
| **Scoping Rules** | RAII, ownership, moves, borrowing, lifetimes | `rust-by-example/src/scope.rs` |
| **Traits** | `Derive`, `From`/`Into`, operator overloading, `Drop` | `rust-by-example/src/trait.rs` |
| **Error Handling** | `Result`, `Option`, `?`, custom errors, `anyhow`/`thiserror` | `rust-by-example/src/error/` |
| **Std Library** | Box, Vec, String, HashMap, Iterator, Thread, Channel, Path, I/O | `rust-by-example/src/std/` |
| **Testing** | Unit tests, integration tests, `#[cfg(test)]`, `doc` tests | `rust-by-example/src/testing.rs` |
| **Unsafe Operations** | Raw pointers, `unsafe` blocks, FFI | `rust-by-example/src/unsafe.rs` |
| **Compatibility** | Edition guides, `std` vs `core` | `rust-by-example/src/compatibility.rs` |

# Source Structure (Local Mirror)

```
~/references/languages/rust-advanced/rust-by-example/
├── src/
│   ├── hello.rs
│   ├── primitives.rs
│   ├── custom_types/
│   ├── variable_bindings.rs
│   ├── types.rs
│   ├── conversion/
│   ├── flow_control/
│   ├── fn.rs
│   ├── mod.rs
│   ├── crates/
│   ├── attribute/
│   ├── generics.rs
│   ├── scope.rs
│   ├── trait.rs
│   ├── error/
│   ├── std/
│   ├── testing.rs
│   ├── unsafe.rs
│   ├── compatibility.rs
│   └── SUMMARY.md
├── .git/                    # Full history, origin=github.com/rust-lang/rust-by-example
└── book.toml                # mdbook configuration
```

# High-Value Example Categories

## Error Handling (`error/`)
- `Result`/`Option` patterns, `?` operator
- Custom error types with `thiserror`
- Error context with `anyhow`
- `Result` combinators (`map`, `and_then`, `ok_or`)

## Std Collections (`std/`)
- `Vec`: creation, iteration, draining, splicing
- `HashMap`: entry API, `or_insert`, `get_mut`
- `String`: formatting, parsing, UTF-8 handling
- `Iterator`: `map`, `filter`, `fold`, `collect`, `partition`

## Concurrency (`std/thread.rs`, `std/sync.rs`)
- `thread::spawn`, `join`, scoped threads
- `Arc`/`Mutex`/`RwLock` patterns
- Channels: `mpsc`, `oneshot`, `crossbeam`
- `Rayon` parallel iterators (external crate)

## Testing (`testing.rs`)
- `#[test]`, `#[should_panic]`, `#[ignore]`
- `assert!`, `assert_eq!`, `assert_matches!`
- Doc tests: `/// ```rust` examples
- Integration tests in `tests/` directory

# Related Concepts

* [The Rustonomicon](/concepts/languages/rust-nomicon.md) — Unsafe operations chapter aligns with RBE `unsafe.rs`
* [Async Programming in Rust](/concepts/languages/rust-async-book.md) — Async patterns in RBE `std/future.rs` (if present)
* [Rust Advanced Skill](/skills/rust-advanced.md) — Agent skill aggregating all three

# Usage Guidance

- **When to consult**: Need a canonical, compilable example for any stdlib type, trait, or pattern.
- **Runnable**: All examples in mirror are `cargo run --example <name>` compatible (mdbook tested).
- **Authority**: Currently `unverified` — spot-check against upstream before `stale_after`.
- **Cross-reference**: Best used alongside Nomicon (theory) and Async Book (async patterns).