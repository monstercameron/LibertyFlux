//! What is proven for each lifted method, and what each proof narrows.

/// Proof state of one method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// Lifted, and compared with its verified rewrite by the differential
    /// test crate.
    Proven,
    /// Not lifted in this lane.
    Missing,
}

/// One method of the family.
#[derive(Clone, Copy, Debug)]
pub struct Row {
    /// The class the method belongs to.
    pub class: &'static str,
    /// The method's name in the original's inventory.
    pub method: &'static str,
    /// The Rust method that restates it, or `-` when missing.
    pub rust: &'static str,
    /// Proof state.
    pub state: State,
    /// What the proof does not cover.
    pub narrows: &'static [&'static str],
}

/// The rows of the seek-cover family.
pub const ROWS: &[Row] = &[];
