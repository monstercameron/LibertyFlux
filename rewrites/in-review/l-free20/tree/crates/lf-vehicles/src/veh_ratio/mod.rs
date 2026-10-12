//! Vehicle tuning ratio coefficients.
//!
//! The original keeps a large family of derived tuning constants, each
//! one the quotient of two stored single-precision floats. Every member
//! of the family does the same thing: read its numerator, read its
//! denominator, divide once in single precision, store the quotient back.
//! The 113 verified instances differ only in which three globals they
//! touch, so they lift once, as [`RatioCell::refresh`], and each instance
//! is proven against it (see the `lf-vehratiodiff` crates).
//!
//! [`RatioBank`] owns the whole verified set in registry order and
//! refreshes it all at once, mirroring how the game walks its callback
//! table. [`registry`] records what is proven and what each proof
//! narrows.

#![forbid(unsafe_code)]

pub mod bank;
pub mod cell;
pub mod registry;

pub use bank::RatioBank;
pub use cell::RatioCell;
