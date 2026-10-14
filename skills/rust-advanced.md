---
type: Skill
title: "Rust Advanced Implementation Reference"
description: "Agent skill for advanced Rust: unsafe code, async/await patterns, practical examples."
tags: [rust, agent-skill, implementation, unsafe, async, advanced]
status: stable
generated:
  by: human:koppe
  at: 2026-10-14T00:00:00+09:00
verified: []
stale_after: 2027-04-14T00:00:00+09:00
sources:
  - id: rust-nomicon
    resource: /concepts/languages/rust-nomicon.md
    title: "The Rustonomicon Concept"
    author: human:koppe
    last_modified: 2026-10-14T00:00:00+09:00
  - id: rust-async-book
    resource: /concepts/languages/rust-async-book.md
    title: "Async Programming in Rust Concept"
    author: human:koppe
    last_modified: 2026-10-14T00:00:00+09:00
  - id: rust-by-example
    resource: /concepts/languages/rust-by-example.md
    title: "Rust by Example Concept"
    author: human:koppe
    last_modified: 2026-10-14T00:00:00+09:00
  - id: rust-advanced-mirror
    resource: file:///home/koppe/references/languages/rust-advanced/
    title: "Rust Advanced Mirror (all three sources)"
    author: team:rust-lang
---

# Usage

**Invoke this skill when:**
- Implementing `unsafe` Rust code (FFI, raw pointers, manual memory management)
- Working with async/await internals (executors, `Future` pinning, wakers)
- Needing concrete, idiomatic Rust examples for standard patterns
- Reviewing code for UB, soundness, or memory safety issues

# References

This skill aggregates three core Rust advanced references:

1. **[The Rustonomicon](/concepts/languages/rust-nomicon.md)** (`type: Reference`)
   - Unsafe Rust semantics, memory layout, FFI, concurrency primitives
   - Source: `~/references/languages/rust-advanced/nomicon/`
   - Key chapters: Memory Layout, FFI, Concurrency, Vec, Arc

2. **[Async Programming in Rust](/concepts/languages/rust-async-book.md)** (`type: Reference`)
   - Async/await transformation, executors, `Future` trait, pinning
   - Source: `~/references/languages/rust-advanced/async-book/`
   - Key chapters: Executor design, Waker, Pin, Stream

3. **[Rust by Example](/concepts/languages/rust-by-example.md)** (`type: Reference`)
   - Runnable examples for standard library, traits, error handling, testing
   - Source: `~/references/languages/rust-advanced/rust-by-example/`
   - Key sections: Flow control, Traits, Std types, Error handling

# Local Mirror Paths

| Reference | Local Mirror Path | Upstream |
|-----------|-------------------|----------|
| Nomicon | `~/references/languages/rust-advanced/nomicon/src/` | github.com/rust-lang/nomicon |
| Async Book | `~/references/languages/rust-advanced/async-book/src/` | github.com/rust-lang/async-book |
| By Example | `~/references/languages/rust-advanced/rust-by-example/src/` | github.com/rust-lang/rust-by-example |

# Agent Behavior

When this skill is active:
- Prefer `file://` references for offline, fast access
- Cite specific chapters via `file://` paths in responses
- Cross-reference between the three sources when relevant
- Flag any `unsafe` usage with Nomicon chapter references
- Verify async patterns against Async Book executor model