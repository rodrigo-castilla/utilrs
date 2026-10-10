# utilrs

A high-performance, modular meta-crate and utility toolkit for modern Rust development.

`utilrs` is designed as a **"library of libraries"**: an extensible collection of decoupled, specialized sub-crates unified under a single cohesive workspace. Consumers can pull the entire suite or cherry-pick individual modules using Cargo features, compiling only the exact dependencies they need with zero overhead.

---

## Architecture & Philosophy

- **Modular Cargo Workspace:** Each submodule lives in its own crate under `crates/`, with isolated dependencies and standalone publication capability.
- **Pay Only for What You Use:** The root `utilrs` crate has zero default dependencies. Enabling a feature like `rost` only pulls the specific dependencies required by that crate (`memchr`), keeping build times minimal.
- **Unified & Ergonomic Facade:** Submodules are cleanly re-exported at the root (`utilrs::rost`, `utilrs::errors`) while allowing downstream users to depend directly on individual standalone crates (`utilrs-rost`, `utilrs-errors`) if preferred.
- **Predictable & Robust:** Built with strict zero-cost abstractions, zero-copy memory patterns where appropriate, and comprehensive automated test suites.

---

## Workspace Crates & Modules

| Module / Crate | Feature | Description | Dependencies |
| :--- | :--- | :--- | :--- |
| **[`rost`](crates/rost)** (`utilrs-rost`) | `rost` | Zero-copy document parser, indexer, and malleable patching engine for `.rost` declarative stylesheets and config files. | `memchr` |
| **[`errors`](crates/errors)** (`utilrs-errors`) | `errors` | Lightweight, standardized error message wrappers and structured kind/code types. | `std` only |
| **`all`** | `all` | Convenience meta-feature that enables all available submodules simultaneously. | Workspace deps |

---

## Quick Start

### 1. Adding `utilrs` to your Project

Add `utilrs` with only the submodules you require in your `Cargo.toml`:

```toml
[dependencies]
# Pull only the rost declarative engine:
utilrs = { version = "0.1.0", features = ["rost"] }

# Or pull multiple specific submodules:
# utilrs = { version = "0.1.0", features = ["rost", "errors"] }

# Or enable the entire toolkit:
# utilrs = { version = "0.1.0", features = ["all"] }
```

Alternatively, if you prefer standalone crate imports without the meta-crate facade:

```toml
[dependencies]
utilrs-rost = "0.1.0"
utilrs-errors = "0.1.0"
```

---

## Modules Overview

### 1. `rost` — High-Throughput Declarative Engine

The `.rost` engine is designed to parse, index, and mutate declarative styling and structured documents at wire-speed:

- **Near-Instant Parsing:** Sustains ~500 MB/s throughput (~70,000 tokens in ~1.3 ms) using SIMD token scanning.
- **Zero-Copy Layout:** Token slices point directly into raw memory arenas without intermediate heap allocations.
- **Malleable Engine:** Supports both zero-allocation querying and in-place document patching for UI live-reloading and game engine state tuning.
- **Tooling Ecosystem:** Natively paired with [`tree-sitter-rost`](https://github.com/rost-rs/tree-sitter-rost) and [`rost-lsp`](https://github.com/rost-rs/rost-lsp) for IDE diagnostics, autocompletion, and syntax highlighting.

#### Example:

```rust
use utilrs::rost::{RostBuilder, Template};

// Programmatic document creation
let document = RostBuilder::new()
    .directive("theme", "cyber-dark")
    .property("width", "140px")
    .enumeration("state", &["idle", "hover", "active"])
    .build();

// Zero-copy query
let template = Template::from_str(&document);
assert_eq!(template.get_directive("theme"), Some("cyber-dark"));
assert_eq!(template.get_value("width"), Some("140px"));
```

---

### 2. `errors` — Structured Error Handling

Provides a clean, uniform abstraction over standard I/O error kinds and numeric status codes:

- Consistent error structures (`ErrMsg`, `ErrKind`) across workspace boundaries.
- Zero external dependencies (`std` only).

#### Example:

```rust
use std::io::ErrorKind;
use utilrs::errors::ErrMsg;

let err = ErrMsg::new("Resource not found".to_string(), ErrorKind::NotFound, 404);
assert_eq!(err.ek.code, 404);
```

---

## Directory Structure

```text
utilrs/
├── Cargo.toml            # Workspace definition and root facade features
├── src/
│   └── lib.rs            # Feature-gated re-exports and API facade
├── crates/
│   ├── rost/             # utilrs-rost crate (core parser & engine)
│   │   ├── assets/       # Official logo vector & graphics
│   │   ├── src/          # SIMD parser, query engine, malleable builder
│   │   └── tests/        # Benchmark & specification tests
│   └── errors/           # utilrs-errors crate (lightweight error types)
│       └── src/
├── docs/                 # Format specifications and design notes
└── tests/                # Workspace integration tests
```

---

## Development & Testing

Run all tests across all workspace crates and feature configurations:

```bash
cargo test --workspace
```

Verify documentation and doc-tests:

```bash
cargo test --doc --all-features
```

---

## License

MIT License. Copyright (c) 2026 The Rost Project Contributors.
