# Overview

The **Async Book** is the official guide to asynchronous programming in Rust. It covers the `Future` trait, async/await transformation, executor design, pinning, wakers, streams, and ecosystem integration.

# Key Chapters

| Part | Chapter | Focus | Local Path |
|------|---------|-------|------------|
| **Getting Started** | 01 | Why async, basic concepts | `async-book/src/01_getting_started/` |
| **Execution** | 02 | Executors, reactors, runtimes | `async-book/src/02_execution/` |
| **Async/Await** | 03 | State machines, `async fn` desugaring | `async-book/src/03_async_await/` |
| **Pinning** | 04 | `Pin`, `Unpin`, self-referential structs | `async-book/src/04_pinning/` |
| **Streams** | 05 | `Stream` trait, combinators | `async-book/src/05_streams/` |
| **Multiple Futures** | 06 | `join!`, `select!`, `FuturesUnordered` | `async-book/src/06_multiple_futures/` |
| **Workarounds** | 07 | `async` in traits, GATs, boxing | `async-book/src/07_workarounds/` |
| **Ecosystem** | 08 | Tokio, async-std, smol, embassy | `async-book/src/08_ecosystem/` |
| **Example** | 09 | Complete async application | `async-book/src/09_example/` |

# Source Structure (Local Mirror)

```
~/references/languages/rust-advanced/async-book/
├── src/
│   ├── 01_getting_started/
│   ├── 02_execution/
│   ├── 03_async_await/
│   ├── 04_pinning/
│   ├── 05_streams/
│   ├── 06_multiple_futures/
│   ├── 07_workarounds/
│   ├── 08_ecosystem/
│   ├── 09_example/
│   ├── intro.md
│   └── SUMMARY.md
├── .git/                    # Full history, origin=github.com/rust-lang/async-book
└── book.toml                # mdbook configuration
```

# Critical Concepts for Implementation

## `Future` Trait & State Machines
```rust
// Core trait (from 03_async_await)
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```
- `async fn` → anonymous state machine implementing `Future`
- Each `.await` becomes a state transition point

## Pinning & `Unpin`
```rust
// From 04_pinning
struct Pin<P> { /* ... */ }
trait Unpin { }
```
- `Pin<&mut T>` guarantees `T` won't move in memory
- Required for self-referential futures (async blocks capturing refs)
- Most types are `Unpin` (safe to move); `!Unpin` requires `Pin`

## Executor Contract (02_execution)
- **Waker**: Notifies executor when future ready to poll
- **Context**: Provides `Waker` to `poll()`
- **Reactor**: Integrates with OS async I/O (epoll, IOCP, kqueue)

# Related Concepts

* [The Rustonomicon](/concepts/languages/rust-nomicon.md) — Unsafe foundations (pinning uses `unsafe`)
* [Rust by Example](/concepts/languages/rust-by-example.md) — Practical async patterns
* [Rust Advanced Skill](/skills/rust-advanced.md) — Agent skill aggregating all three

# Usage Guidance

- **When to consult**: Implementing custom executors, `Future` combinators, async traits, stream processing, or debugging executor behavior.
- **Authority**: Currently `unverified` — verify against upstream before `stale_after`.
- **Cross-reference**: Pinning chapter essential for understanding `Pin<&mut Self>` in `Future::poll`.