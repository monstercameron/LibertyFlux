//! The process-wide values the mirror-reflection phase reads and writes.
//!
//! The original keeps these in its global data. The lift holds them in one
//! state value that the caller owns and lends to the phase's methods:
//! methods that only read take a shared reference, methods that write take
//! a mutable one. Values the original takes from its image are fields too,
//! filled when the state is loaded, so no game data is written into source.

/// The phase's process-wide state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MirrorGlobals {
    /// The render gate: the phase does nothing while this byte is zero.
    pub render_enabled: u8,
    /// The skip gate: the phase does nothing while this byte is set.
    pub render_skipped: u8,
    /// A byte flag that makes the render method choose selector one rather
    /// than two.
    pub select_flag_a: u8,
    /// A second byte flag with the same effect on the selector choice.
    pub select_flag_b: u8,
    /// A byte flag that makes the render method run its float block.
    pub float_block_enabled: u8,
    /// Set when the reflection matrix is scaled.
    pub scale_enabled: u8,
    /// The scale factor, as float bits.
    pub scale_bits: u32,
    /// One, as float bits: the staged float of the render method.
    pub one_bits: u32,
    /// Non-zero when the refresh swaps in a parameter table entry.
    pub swap_enabled: u32,
    /// The index of the parameter table entry the refresh swaps in.
    pub swap_index: u32,
    /// The parameter table as words; an entry is three words, of which the
    /// first is a count and the second a selector.
    pub table: Vec<u32>,
    /// The selector last published by the reflection method.
    pub published_selector: u32,
    /// The count last published by the reflection method.
    pub published_count: u32,
    /// The session object the refresh fetches its target through.
    pub session: u32,
    /// The stamp counter the render method advances for each block it stamps.
    pub stamp_counter: u32,
    /// The first tag word a stamped block receives.
    pub stamp_tag_first: u32,
    /// The final tag word a stamped block receives.
    pub stamp_tag_final: u32,
    /// The trailer the first stamped block receives.
    pub trailer_first: u32,
    /// The trailer the second stamped block receives.
    pub trailer_second: u32,
}
