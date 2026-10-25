# Harmonic Tidal Analyzer

A Rust harmonic tidal analyzer for studying tidal regimes in Panama's Pacific
and Caribbean coasts. The project is currently in an early research and
development stage.

## Current status

The project currently includes:

- A library crate named `tides`.
- One executable named `tides`.
- Initial Doodson astronomical argument calculations from UTC days since the
  Unix epoch.
- Julian-century conversion from J2000.0 and angle normalization.
- A shared error type prepared for CSV, JSON, and I/O operations.
- Basic unit tests for the astronomical functions.

Harmonic fitting, time-series input, constituent definitions, nodal
corrections, and tide-height predictions are not implemented yet.

## Requirements

- Rust 1.85 or later.
- Cargo.
- `rust-analyzer` for editor completion and diagnostics.

`Cargo.toml` defines the library and the only executable. When the folder
containing that file is opened, `rust-analyzer` automatically discovers the
crate and its modules; it should not be added as a project dependency.

## Usage

Check, test, and format the project:

```bash
cargo check
cargo test
cargo fmt --all
```

Run the current executable:

```bash
cargo run --bin tides
```

The executable is still a minimal entry point. The astronomical functionality
is currently exposed mainly through the library:

```rust
use tides::astro::astro_args;

let arguments = astro_args(0.0);
println!("{:?}", arguments.doodson);
```

## Project layout

```text
src/
├── astro.rs       # Astronomical arguments and time
├── bin/tides.rs   # Only executable
├── error.rs       # Public crate errors
└── lib.rs         # Library API
```

`src/main.rs` is kept as a reference for the initial prototype, but
`autobins = false` prevents Cargo from registering it as a second executable.

## License

MIT
