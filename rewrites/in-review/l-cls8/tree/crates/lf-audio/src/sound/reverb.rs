//! The reverb effect: three channel rows refreshed from a preset.
//!
//! Lifted from the verified rewrites of `rage::audReverbEffect`. Past
//! the base effect's words the 32-bit object carries a next stage, two
//! index words, three five-word channel rows, a four-word staging row,
//! a hold flag and a sub-object; here those are plain fields, with the
//! info block, next stage and sub-object carried as opaque cookies and
//! reached through [`ReverbWorld`]. (There is no verified constructor
//! in the batch, so there is no `new`: tests build the struct
//! literally.)

use lf_core::Handle32;

use super::{InfoTag, NextTag, ReverbSubTag};

/// Channel rows and words per row.
pub const CHAN_ROWS: u32 = 3;
/// Words per channel row.
pub const CHAN_WIDTH: u32 = 5;
/// Live preset words per row.
pub const PRESET_WORDS: usize = 4;

/// What the reverb effect needs from the engine around it: its own base
/// entries, its next stage, and its sub-object.
///
/// Every method is one callee or virtual-slot role from the verified
/// rewrites, with addresses narrowed to opaque cookies or word offsets
/// (see the registry for the per-method narrowings).
pub trait ReverbWorld {
    /// The base advance entry the tick runs first.
    fn base_advance(&mut self);
    /// Hands control to the next stage; answers its answer.
    fn advance_next(&mut self, next: Option<Handle32<NextTag>>) -> u32;
    /// The shared base initialiser the init entry runs first.
    fn base_init(&mut self, a: u32, b: u32) -> u32;
    /// The virtual refresh hook the init entry runs per channel.
    fn refresh_hook(&mut self);
    /// The direct refresh helper; answers its answer.
    fn refresh_direct(&mut self) -> u32;
    /// The entry the poll runs first.
    fn pre_poll(&mut self);
    /// Polls the sub-object against the selected channel row, passed as
    /// a word offset from the object base.
    fn sub_poll(&mut self, sub: Option<Handle32<ReverbSubTag>>, slot_words: u32);
    /// The entry the poll runs last; answers its answer.
    fn post_poll(&mut self) -> u32;
}

/// A reverb effect, owning its index words and channel rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReverbEffect {
    /// The attached info block, holding the preset words.
    pub info: Option<Handle32<InfoTag>>,
    /// The next stage control passes to.
    pub next: Option<Handle32<NextTag>>,
    /// The row the poll selects.
    pub poll_index: u32,
    /// The row the tick shuffles and refreshes.
    pub step: u32,
    /// The three five-word channel rows.
    pub chans: [[u32; CHAN_WIDTH as usize]; CHAN_ROWS as usize],
    /// The staging row: a fourth copy of the preset.
    pub staging: [u32; PRESET_WORDS],
    /// The hold flag: clear forces every refresh to store.
    pub hold: u8,
    /// The polled sub-object.
    pub sub: Option<Handle32<ReverbSubTag>>,
}

impl ReverbEffect {
    /// Advances one tick: shuffles the step row over the next row,
    /// refreshes the four live words from the preset wherever the
    /// stored value has risen past the floor or the hold flag is
    /// clear, stores the new step, and hands control to the next
    /// stage when one is linked. Answers the next stage's answer, or
    /// the last stored word when unlinked (the step counter when the
    /// last refresh did not store).
    ///
    /// The preset words and the floor arrive decoded: the original
    /// reads them from the attached block and a global.
    ///
    /// # Panics
    ///
    /// When the step selects past the last row (the original reads
    /// and writes whatever memory lies there).
    pub fn advance<W: ReverbWorld>(
        &mut self,
        world: &mut W,
        preset: &[u32; PRESET_WORDS],
        floor: f32,
    ) -> u32 {
        world.base_advance();
        let slot = self.step.wrapping_add(1) % CHAN_ROWS;
        let from = self.step as usize;
        self.chans[slot as usize] = self.chans[from];
        let mut last_stored = false;
        for (k, word) in preset.iter().enumerate() {
            let mem = f32::from_bits(self.chans[from][k]);
            let store = floor > mem || self.hold == 0;
            if store {
                self.chans[from][k] = *word;
            }
            if k == PRESET_WORDS - 1 {
                last_stored = store;
            }
        }
        let tail = if last_stored {
            self.chans[from][PRESET_WORDS - 1]
        } else {
            self.step
        };
        self.step = slot;
        match self.next {
            None => tail,
            Some(_) => world.advance_next(self.next),
        }
    }

    /// The current channel row: the row `step` selects.
    ///
    /// The original answers the row's address; the lift answers the row
    /// itself.
    ///
    /// # Panics
    ///
    /// When the step selects past the last row.
    #[must_use]
    pub fn channel(&self) -> &[u32; CHAN_WIDTH as usize] {
        &self.chans[self.step as usize]
    }

    /// Initialises the effect from the preset: unless the base
    /// initialiser's low byte is zero (which returns its answer
    /// unchanged), clears the hold flag, fans the four preset words
    /// out to the three channel rows and the staging row, refreshes
    /// each channel through the hook and the helper, and answers the
    /// last helper answer with its low byte forced to 1.
    pub fn init<W: ReverbWorld>(
        &mut self,
        world: &mut W,
        preset: &[u32; PRESET_WORDS],
        a: u32,
        b: u32,
    ) -> u32 {
        let answer = world.base_init(a, b);
        if answer & 0xFF == 0 {
            return answer;
        }
        self.hold = 0;
        for row in &mut self.chans {
            row[0] = preset[0];
            row[1] = preset[1];
            row[2] = preset[2];
            row[3] = preset[3];
        }
        self.staging = *preset;
        let mut last = 0;
        for _ in 0..CHAN_ROWS {
            world.refresh_hook();
            last = world.refresh_direct();
        }
        (last & 0xFFFF_FF00) | 1
    }

    /// Polls the effect: runs the first entry, polls the sub-object
    /// against the row `poll_index` selects, and answers the last
    /// entry's answer.
    pub fn poll<W: ReverbWorld>(&mut self, world: &mut W) -> u32 {
        world.pre_poll();
        let slot = self.poll_index.wrapping_mul(CHAN_WIDTH).wrapping_add(0x1D);
        world.sub_poll(self.sub, slot);
        world.post_poll()
    }
}
