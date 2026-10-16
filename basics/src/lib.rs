//! Rust basics: small drills for the core ideas. Read basics/README.md first.
//!
//! Each drill is one file. Do them in order:
//!
//!   cargo test -p rust-basics ownership
//!   cargo test -p rust-basics borrowing
//!   ...
#![allow(unused)] // delete this line when you've done all the drills

#[path = "01_ownership.rs"]
pub mod ownership;

#[path = "02_borrowing.rs"]
pub mod borrowing;

#[path = "03_strings.rs"]
pub mod strings;

#[path = "04_structs.rs"]
pub mod structs;

#[path = "05_enums.rs"]
pub mod enums;

#[path = "06_options.rs"]
pub mod options;

#[path = "07_results.rs"]
pub mod results;

#[path = "08_collections.rs"]
pub mod collections;

#[path = "09_iterators.rs"]
pub mod iterators;

#[path = "10_traits.rs"]
pub mod traits;
