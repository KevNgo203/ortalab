//! # OrtaLab
//!
//! OrtaLab is a command-line tool for simulating and scoring poker rounds.
//! It provides utilities for parsing game input, evaluating poker hands,
//! and computing chip multipliers.
//!
//! ## Example
//! ```bash
//! cargo run [round_file.yml]
//! ```

pub mod cli;
pub mod io;
pub mod poker;
