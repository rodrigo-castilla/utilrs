//! # utilrs
//!
//! A modular utility library for Rust organized as a Cargo workspace with
//! crate-level submodules. Each submodule lives in its own crate with its own
//! dependencies, allowing consumers of `utilrs` to only pull and compile
//! the dependencies of the submodules they actually use.
//!
//! ## Submodules & Features
//!
//! - `rost`: Ultra-fast zero-copy `.rost` document parser, indexer, and malleable engine.
//!   - Feature: `rost`
//!   - Standalone crate: `rost` (<https://github.com/rost-rs/rost>)
//!   - Dependencies: `memchr`
//!
//! - `errors`: Lightweight standardized error message and kind structures.
//!   - Feature: `errors`
//!   - Standalone crate: `utilrs-errors`
//!   - Dependencies: none (`std` only)
//!
//! - `all`: Enables all submodules.
//!
//! ## Usage Example
//!
//! In `Cargo.toml`:
//! ```toml
//! [dependencies]
//! # Pull only the rost submodule and its dependencies (e.g., memchr):
//! utilrs = { git = "https://github.com/rodrigo-castilla/utilrs.git", features = ["rost"] }
//! ```
//!
//! In Rust code:
//! ```rust,ignore
//! use utilrs::rost::{Template, RostBuilder};
//! ```

#[cfg(feature = "rost")]
pub use rost;

#[cfg(feature = "errors")]
pub use errors;

/// Backwards-compatibility module preserving the previous `utilrs::models::...` paths.
pub mod models {
    #[cfg(feature = "rost")]
    pub use crate::rost;

    #[cfg(feature = "errors")]
    pub use crate::errors;
}
