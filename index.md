---
okf_version: "0.2"
title: "Local Knowledge Bundle"
description: "Curated reference knowledge for AI-assisted development, sourced from ~/references mirrors."
tags: [knowledge-bundle, okf, references]
generated:
  by: human:koppe
  at: 2026-10-14T00:00:00+09:00
verified: []
status: stable
---

# Local Knowledge Bundle

This bundle organizes curated reference material from `~/references/` (read-only upstream mirrors) into OKF v0.2 concepts for agent consumption.

## Structure

- [concepts/](concepts/) — Domain knowledge concepts (specifications, references, guides)
- [skills/](skills/) — Agent skill definitions (OKF `type: Skill`)
- [references/](references/) — Mirrored external artifacts (per OKF §6.3 convention)
- [computations/](computations/) — Attested computations (future use)

## Top-Level Concept Index

### Languages & Runtimes

* [Rust: The Rustonomicon](/concepts/languages/rust-nomicon.md) - Unsafe Rust, memory layout, FFI
* [Rust: Async Programming](/concepts/languages/rust-async-book.md) - Async/await internals, executors, pinning
* [Rust: By Example](/concepts/languages/rust-by-example.md) - Practical Rust patterns
* [CPython Internals](/concepts/languages/cpython-internals.md) - Python interpreter implementation
* [ECMAScript 2026 Spec](/concepts/languages/ecma262-spec.md) - JavaScript language specification
* [Node.js Runtime](/concepts/runtimes/node-runtime.md) - Node.js implementation reference

### Frameworks & Platforms

* [Tauri Framework](/concepts/frameworks/tauri-framework.md) - Rust-based desktop app framework
* [React.dev Documentation](/concepts/frameworks/react-dev-docs.md) - React official docs
* [opencode (TypeScript)](/concepts/platforms/opencode-ts.md) - opencode CLI core implementation
* [opencode (Rust)](/concepts/platforms/opencode-rust.md) - opencode Rust port
* [Google Go](/concepts/platforms/google-go.md) - Go language implementation
* [Google AIP Design Guides](/concepts/platforms/google-aip.md) - API Improvement Proposals

### Specifications & Standards

* [CommonMark Spec](/concepts/specs/commonmark-spec.md) - Markdown standard
* [OpenAPI Specification](/concepts/specs/openapi-spec.md) - REST API description format
* [Typst Spec](/concepts/specs/typst-spec.md) - Modern typesetting system
* [Sphinx Doc System](/concepts/specs/sphinx-doc-system.md) - Python documentation generator
* [MCP Protocol](/concepts/specs/mcp-protocol.md) - Model Context Protocol
* [Mermaid Diagrams](/concepts/specs/mermaid-spec.md) - Diagram-as-code syntax

### Tools & Protocols

* [markdownlint](/concepts/tools/markdownlint-tool.md) - Markdown linter
* [Playwright](/concepts/tools/playwright-tool.md) - Browser automation
* [Violentmonkey](/concepts/tools/violentmonkey-tool.md) - Userscript manager
* [Chrome DevTools Protocol](/concepts/tools/chrome-devtools-protocol.md) - Debugging protocol

### Operating Systems & Environments

* [Linux Administration](/concepts/os/linux-admin.md) - Linux system administration
* [Kubuntu 26.04](/concepts/os/kubuntu-2604.md) - Local OS environment
* [Nix Package Manager](/concepts/os/nix-package-manager.md) - Reproducible builds

### Skills (Agent Definitions)

* [Rust Advanced](/skills/rust-advanced.md) - Unsafe, async, practical Rust
* [opencode Core](/skills/opencode-core.md) - opencode TS implementation
* [MCP Specification](/skills/mcp-spec.md) - Model Context Protocol
* [Tauri Development](/skills/tauri-dev.md) - Tauri app development

## Maintenance

- **Source mirrors**: `~/references/` (updated via `git pull --ff-only`)
- **Bundle updates**: Edit concepts in `~/knowledge/`, commit to this repo
- **Sync script**: `rust-script sync_sources.rs` updates `sources.last_modified` from mirrors
- **Link check**: CI validates `file://` paths in `sources.resource` exist