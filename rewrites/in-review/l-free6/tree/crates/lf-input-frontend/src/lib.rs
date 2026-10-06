//! `lf-input-frontend` (lane l-free6 tree copy): only the lifted
//! `input_ui` module is present here. The coordinator integrates
//! `src/input_ui/` into the tracked crate, whose `lib.rs` gains one
//! `pub mod input_ui;` line.

pub mod input_ui;
