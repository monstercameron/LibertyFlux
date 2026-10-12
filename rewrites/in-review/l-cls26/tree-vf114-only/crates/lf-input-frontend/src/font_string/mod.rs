//! Lifted UI font strings: lines of text measured and drawn in a font.
//!
//! The original shows interface text through string objects
//! ([`FontString`]): live layout words, six flag bytes, a 256-byte text
//! buffer, and one row descriptor per indexed slot. This module lifts
//! the string's verified behaviour to ordinary Rust: the string owns
//! its words, flags, text and rows, and everything it calls travels
//! through [`FontWorld`], one trait method per virtual slot and per
//! intercepted callee, so dispatch is static and tests script answers
//! through a fake.
//!
//! Each verified 32-bit method with behaviour in it is restated as a
//! method on [`FontString`]. Proof is differential: every lifted method
//! runs against its verified rewrite on the same generated inputs,
//! comparing results, every written byte and every world call in order,
//! floats bit for bit (see the `lf-fontstr-diff` test crate). Nothing
//! here is verified by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].

#![forbid(unsafe_code)]

mod string;

pub mod registry;

pub use string::{
    DEFAULT_COLOUR, FontString, FontWorld, MeasureInputs, RESET_TAG, ROW_TEXT_LEN, StringRow,
    UI_DRAW_MODE, text_with_nul, up_to_nul,
};
