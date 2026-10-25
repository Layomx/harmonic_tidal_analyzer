//! # `tides`
//!
//! Building blocks for harmonic tidal analysis in Rust.
//!
//! This crate is the library part of the Harmonic Tidal Analyzer project. It
//! currently provides the first astronomical calculations needed by a future
//! harmonic model, together with the shared error type used by the public API.
//! The project is focused on studying tidal regimes on the Pacific and
//! Caribbean coasts of Panama.
//!
//! ## Current scope
//!
//! The [`astro`] module calculates UTC-based astronomical arguments from a
//! number of days since the Unix epoch. It also provides Julian-century
//! conversion, Doodson variable rates, angle normalization, and lunar-node
//! values for later nodal corrections.
//!
//! The [`constituents`] module is reserved for tidal constituent definitions
//! and is currently empty. Harmonic fitting, time-series input, nodal
//! corrections, and tide-height predictions are not implemented yet.
//!
//! Errors returned by library operations use [`Result`] and [`Error`].
//!
//! ## Example
//!
//! Astronomical arguments can already be calculated directly:
//!
//! ```
//! use tides::astro::astro_args;
//!
//! let arguments = astro_args(0.0);
//! assert!(arguments.doodson.iter().all(|angle| (0.0..360.0).contains(angle)));
//! ```

pub mod astro;
pub mod constituents;
pub mod error;

pub use error::{Error, Result};
