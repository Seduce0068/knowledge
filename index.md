---
okf_version: "0.2"
title: "Local Knowledge Bundle"
description: "Curated reference knowledge for AI-assisted development, sourced from ~/references mirrors."
tags: [knowledge-bundle, okf, references]
generated:
  by: role:maintainer
  at: 2026-10-14T00:00:00+09:00
verified: []
status: stable
---

# Local Knowledge Bundle

This bundle organizes curated reference material from `~/references/` (read-only upstream mirrors) into OKF v0.2 concepts for agent consumption.

## Structure

- [concepts/](concepts/) — Domain knowledge concepts (specifications, references, guides)
- [skills/](skills/) — Agent skill definitions (OKF `type: Skill`)

## Top-Level Concept Index

### Languages & Runtimes

* [Rust: The Rustonomicon](/concepts/languages/rust-nomicon.md) - Unsafe Rust, memory layout, FFI
* [Rust: Async Programming](/concepts/languages/rust-async-book.md) - Async/await internals, executors, pinning
* [Rust: By Example](/concepts/languages/rust-by-example.md) - Practical Rust patterns

### Skills (Agent Definitions)

* [Rust Advanced](/skills/rust-advanced.md) - Unsafe, async, practical Rust

## Maintenance

- **Source mirrors**: `~/references/` (updated via `git pull --ff-only`)
- **Bundle updates**: Edit concepts in `~/knowledge/`, commit to this repo
- **Sync script**: `rust-script sync_sources.rs` updates `sources.last_modified` from mirrors
- **Link check**: CI validates `file://` paths in `sources.resource` exist