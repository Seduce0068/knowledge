# Overview

The **Rustonomicon** is the official guide to writing **unsafe Rust** code. It covers the semantics of `unsafe`, memory layout, FFI, concurrency primitives, and the implementation of core library types like `Vec` and `Arc`.

# Key Topics

| Chapter | Focus | Local Path |
|---------|-------|------------|
| [Memory Layout](file://<home>/references/languages/rust-advanced/nomicon/src/memory-layout.md) | `#[repr(C)]`, alignment, niche optimization | `nomicon/src/memory-layout.md` |
| [FFI](file://<home>/references/languages/rust-advanced/nomicon/src/ffi.md) | `extern "C"`, `c_char`, callbacks, destructors | `nomicon/src/ffi.md` |
| [Concurrency](file://<home>/references/languages/rust-advanced/nomicon/src/concurrency.md) | `Send`/`Sync`, atomics, memory ordering | `nomicon/src/concurrency.md` |
| [Vec Implementation](file://<home>/references/languages/rust-advanced/nomicon/src/vec.md) | Allocation, growth, `RawVec`, `Drain` | `nomicon/src/vec.md` |
| [Arc/Rc](file://<home>/references/languages/rust-advanced/nomicon/src/arc.md) | Reference counting, weak pointers | `nomicon/src/arc.md` |

# Source Structure (Local Mirror)

```
~/references/languages/rust-advanced/nomicon/
├── src/
│   ├── memory-layout.md
│   ├── ffi.md
│   ├── concurrency.md
│   ├── vec.md
│   ├── arc.md
│   └── ... (20+ chapters)
├── .git/                    # Full history, origin=github.com/rust-lang/nomicon
└── Cargo.toml               # mdbook configuration
```

# Related Concepts

* [Rust Async Book](/concepts/languages/rust-async-book.md) — Async/await internals, executors, pinning
* [Rust by Example](/concepts/languages/rust-by-example.md) — Practical patterns, runnable snippets
* [Rust Advanced Skill](/skills/rust-advanced.md) — Agent skill aggregating all three Rust references

# Usage Guidance

- **When to consult**: Implementing `unsafe` code, FFI bindings, custom allocators, lock-free data structures, or debugging UB.
- **Authority**: `human-reviewed` once verified; currently `unverified` (no `verified` entries).
- **Freshness**: `stale_after: 2027-04-14` — re-verify against upstream before then.
- **Cross-reference**: Use `file://` paths for offline access; upstream URLs for canonical source.