//! The ped task block initialiser: the constructor that sets up a task's
//! parameter area, its countdown and its control words, and runs the
//! block's construction and setup calls in order.
//!
//! The original runs on a block of 0x184 bytes, in place. Three embedded
//! sub-objects, at the start of the block, after its middle and at 0x110,
//! are built by the callees, not by this routine; the routine itself writes
//! only the owned words below, which [`TaskBlock`] names as fields. The
//! sub-objects are opaque here: each callee receives the sub-object's
//! address (the block's handle plus the sub-object's offset) and the bytes
//! inside them are the callee's business.
//!
//! The callees and the one shared tick counter the routine reads travel as
//! [`BlockCallees`]. The two float factors that scale the random draw are
//! the values the original reads from two image words; they travel as
//! [`ScaleFactors`], so the caller supplies the image's values.

/// Size of the block in bytes, as the original lays it out.
pub const BLOCK_BYTES: u32 = 0x184;
/// Offset of the first sub-object constructed in the block.
pub const SUB_HEAD_OFFSET: u32 = 0x80;
/// Offset of the second sub-object constructed in the block.
pub const SUB_TAIL_OFFSET: u32 = 0x110;
/// Number of parameter words cleared at the start of the parameter area.
pub const PARAM_WORDS: usize = 6;
/// Value of the selector word after initialisation: none selected.
pub const SELECTOR_NONE: u32 = 0xFFFF_FFFF;
/// The countdown is this value less the scaled draw, so it lies in
/// [500, 1499] for a draw from zero to 65535 with the image's factors.
pub const COUNTDOWN_BASE: u32 = 500;

/// The two float factors the random draw is scaled by. The original reads
/// them from image words: the first is the reciprocal of 2^15, the second
/// the negative of 500.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaleFactors {
    /// The first factor, multiplied in first.
    pub inverse_range: f32,
    /// The second factor, multiplied in second.
    pub factor: f32,
}

/// The collaborators of the initialiser: the callees it runs in order and
/// the shared tick counter it copies. Each callee takes the block handle or
/// a sub-object handle, exactly as the original passes the addresses.
pub trait BlockCallees {
    /// Constructs the block's first sub-object in place (callee 0, the
    /// block itself).
    fn construct_first(&mut self, block: u32);
    /// Constructs the sub-object at the middle offset (callee 1).
    fn construct_middle(&mut self, sub: u32);
    /// Constructs the sub-object at the tail offset (callee 2).
    fn construct_tail(&mut self, sub: u32);
    /// The first setup step, on the block (callee 3).
    fn setup_first(&mut self, block: u32);
    /// The random draw (callee 4). Only the low sixteen bits are used.
    fn draw(&mut self) -> u32;
    /// The second setup step, with a zero argument (callee 5).
    fn setup_second(&mut self, block: u32, arg: u32);
    /// The third setup step, with a zero argument (callee 6).
    fn setup_third(&mut self, block: u32, arg: u32);
    /// The shared tick counter, read as a plain word.
    fn tick(&self) -> u32;
}

/// The words the initialiser owns, by their offsets in the 32-bit layout.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TaskBlock {
    /// Words at offsets 0x12c..0x144: the parameter area, all zero.
    pub params: [u32; PARAM_WORDS],
    /// Word at offset 0x144: the unit scale, 1.0.
    pub unit_scale: f32,
    /// Word at offset 0x148: the unit scale's second word, zero.
    pub unit_scale_tail: u32,
    /// Word at offset 0x14c: the tick counter copied at initialisation.
    pub tick_copy: u32,
    /// Word at offset 0x150: the countdown, 500 less the scaled draw.
    pub countdown: u32,
    /// Halfword at offset 0x154: the ready flag, set to one.
    pub ready: u16,
    /// Word at offset 0x158: the result slot, cleared.
    pub result: u32,
    /// Word at offset 0x170: the first control word, cleared.
    pub ctrl_a: u32,
    /// Word at offset 0x174: the selector, none.
    pub selector: u32,
    /// Word at offset 0x178: the second control word, cleared.
    pub ctrl_b: u32,
    /// Byte at offset 0x17c: the second flag, cleared.
    pub flag_b: u8,
    /// Word at offset 0x180: the tail word, cleared.
    pub tail: u32,
}

impl TaskBlock {
    /// Initialises a task block at `handle`: runs the three construction
    /// calls and then the parameter area, the countdown from the draw and
    /// the setup calls, in the original's order.
    ///
    /// The routine returns no value of its own; the original returns the
    /// block's handle, which the caller already holds.
    #[must_use]
    pub fn initialise(
        handle: u32,
        callees: &mut impl BlockCallees,
        scale: ScaleFactors,
    ) -> Self {
        callees.construct_first(handle);
        callees.construct_middle(handle.wrapping_add(SUB_HEAD_OFFSET));
        callees.construct_tail(handle.wrapping_add(SUB_TAIL_OFFSET));
        let params = [0; PARAM_WORDS];
        callees.setup_first(handle);
        let draw = callees.draw();
        let tick_copy = callees.tick();
        let scaled = ((draw & 0xFFFF) as f32 * scale.inverse_range) * scale.factor;
        let countdown = COUNTDOWN_BASE.wrapping_sub(truncate_to_i32(scaled) as u32);
        callees.setup_second(handle, 0);
        callees.setup_third(handle, 0);
        Self {
            params,
            unit_scale: 1.0,
            unit_scale_tail: 0,
            tick_copy,
            countdown,
            ready: 1,
            result: 0,
            ctrl_a: 0,
            selector: SELECTOR_NONE,
            ctrl_b: 0,
            flag_b: 0,
            tail: 0,
        }
    }
}

/// Truncation toward zero to a 32-bit integer, as the verified rewrite
/// performs it: the Rust conversion, which gives zero for a not-a-number and
/// saturates outside the signed 32-bit range. The processor's own
/// convert-with-truncation instruction gives the most negative integer for
/// both instead; the difference is recorded in the lane report and is not
/// reachable with the image's factors and a sixteen-bit draw.
#[must_use]
pub fn truncate_to_i32(value: f32) -> i32 {
    value as i32
}