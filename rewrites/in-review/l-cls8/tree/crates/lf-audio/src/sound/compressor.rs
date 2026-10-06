//! The compressor effect: three wide parameter rows past a listener.
//!
//! Lifted from the verified rewrites of `rage::audCompressorEffect`.
//! Past the base effect's words the 32-bit object carries a listener,
//! two index words, a sub-object and three nine-word parameter rows;
//! here those are plain fields, with the listener and sub-object
//! carried as opaque cookies and reached through [`CompressorWorld`].
//! (There is no verified constructor in the batch, so there is no
//! `new`: tests build the struct literally.)

use lf_core::Handle32;

use super::{ListenerTag, SubTag};

/// Parameter rows and words per row.
pub const PARAM_ROWS: u32 = 3;
/// Words per parameter row.
pub const PARAM_WIDTH: u32 = 9;

/// What the compressor effect needs from the engine around it: its own
/// base entries, its listener, and its sub-object.
///
/// Every method is one callee or virtual-slot role from the verified
/// rewrites, with addresses narrowed to opaque cookies or word offsets
/// (see the registry for the per-method narrowings).
pub trait CompressorWorld {
    /// The base rotation entry the row step runs first.
    fn base_rotate(&mut self);
    /// Notifies the listener through its slot-5 entry; answers its answer.
    fn notify_listener(&mut self, listener: Option<Handle32<ListenerTag>>) -> u32;
    /// The shared setter the attach entry forwards to.
    fn set_base(&mut self, a1: u32, a2: u32) -> u32;
    /// The entry the poll runs first.
    fn pre_poll(&mut self);
    /// Polls the sub-object against the selected parameter row, passed
    /// as a word offset from the object base.
    fn sub_poll(&mut self, sub: Option<Handle32<SubTag>>, slot_words: u32);
    /// The entry the poll runs last; answers its answer.
    fn post_poll(&mut self) -> u32;
}

/// A compressor effect, owning its index words and parameter rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompressorEffect {
    /// The listener notified after each row step.
    pub listener: Option<Handle32<ListenerTag>>,
    /// The row the poll selects.
    pub poll_index: u32,
    /// The row the rotation step copies from.
    pub index: u32,
    /// The polled sub-object.
    pub sub: Option<Handle32<SubTag>>,
    /// The three nine-word parameter rows.
    pub params: [[u32; PARAM_WIDTH as usize]; PARAM_ROWS as usize],
}

impl CompressorEffect {
    /// Rotates one parameter row: copies row `index` over row
    /// `(index + 1) % 3`, stores the new index, and notifies the
    /// listener when one is set. Answers the listener's answer, or the
    /// row's last word when unlinked.
    ///
    /// # Panics
    ///
    /// When the index selects past the last row (the original reads
    /// and writes whatever memory lies there).
    pub fn rotate_params<W: CompressorWorld>(&mut self, world: &mut W) -> u32 {
        world.base_rotate();
        let slot = self.index.wrapping_add(1) % PARAM_ROWS;
        let from = self.index as usize;
        let last = self.params[from][PARAM_WIDTH as usize - 1];
        self.params[slot as usize] = self.params[from];
        self.index = slot;
        match self.listener {
            None => last,
            Some(_) => world.notify_listener(self.listener),
        }
    }

    /// The current parameter row: the row `index` selects.
    ///
    /// The original answers the row's address; the lift answers the row
    /// itself.
    ///
    /// # Panics
    ///
    /// When the index selects past the last row.
    #[must_use]
    pub fn slot(&self) -> &[u32; PARAM_WIDTH as usize] {
        &self.params[self.index as usize]
    }

    /// Forwards the two arguments to the shared setter, answering its
    /// answer with the low byte normalised to 0/1 and the upper 24 bits
    /// preserved.
    pub fn set_param<W: CompressorWorld>(&mut self, world: &mut W, a1: u32, a2: u32) -> u32 {
        let answer = world.set_base(a1, a2);
        (answer & 0xFFFF_FF00) | u32::from(answer & 0xFF != 0)
    }

    /// Polls the effect: runs the first entry, polls the sub-object
    /// against the row `poll_index` selects, and answers the last
    /// entry's answer.
    pub fn poll<W: CompressorWorld>(&mut self, world: &mut W) -> u32 {
        world.pre_poll();
        let slot = self
            .poll_index
            .wrapping_mul(PARAM_WIDTH)
            .wrapping_add(0x1E);
        world.sub_poll(self.sub, slot);
        world.post_poll()
    }
}
