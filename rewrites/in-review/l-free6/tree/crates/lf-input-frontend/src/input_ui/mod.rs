//! Lifted input-ui free functions: element holders, the state record,
//! and the bounds accumulator.
//!
//! The 35 verified `input_ui_*` routines share no class, but the creation
//! routines share one data shape: every element holder opens with a table
//! pointer at `+0` and an id word at `+4` whose low 14 bits are stamped
//! from a global counter and whose bits 14..24 are folded from two polled
//! answers. [`holders::UiHolder`] owns that shape; the state record
//! ([`records::UiStateRecord`]) and the bounds accumulator ([`bounds`])
//! are small shapes proven alongside. [`registry`] says what is proven
//! and what each proof narrows.
//!
//! Proven against the verified 32-bit rewrites by the `lf-inputui-diff`
//! test crate. Nothing here counts as checker-verified.

#![forbid(unsafe_code)]

pub mod bounds;
pub mod holders;
pub mod records;
pub mod registry;
