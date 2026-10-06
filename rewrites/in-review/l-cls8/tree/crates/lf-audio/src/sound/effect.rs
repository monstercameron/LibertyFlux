//! The base audio effect: a rotating bank of gain slots.
//!
//! Lifted from the verified rewrites of `rage::audEffect`. The 32-bit
//! object carries an info-block pointer, an attached voice, two index
//! words and fifteen gain slots in three rows of five; here those are
//! plain fields, with the info block and voice carried as opaque
//! cookies and reached through [`EffectWorld`].

use lf_core::Handle32;

use super::{InfoTag, VoiceTag};

/// The gain every slot resets to on attach (1.0).
pub const GAIN_ONE: u32 = 0x3F80_0000;
/// Rows and width of the gain table.
pub const GAIN_ROWS: u32 = 3;
/// Words per gain row.
pub const GAIN_WIDTH: u32 = 5;
/// Slots in the gain table.
pub const GAIN_SLOTS: usize = 15;

/// What the base effect needs from the engine around it: its attached
/// voice, the voice-handle lookup, and the entry-refresh helper.
///
/// Every method is one callee or virtual-slot role from the verified
/// rewrites, with addresses narrowed to opaque cookies and unused
/// answers unmodelled (see the registry for the per-method narrowings).
pub trait EffectWorld {
    /// Releases the attached voice through its slot-0 entry.
    fn release_voice(&mut self, voice: Option<Handle32<VoiceTag>>);
    /// Looks a voice handle up on the audio manager by tag.
    fn lookup_voice(&mut self, tag: u32, param_plus_one: u32) -> Option<Handle32<VoiceTag>>;
    /// Refreshes the derived entry selected by `slot`.
    fn refresh_entry(&mut self, slot: u32);
    /// Polls the attached voice through its slot-2 entry.
    fn poll_voice(&mut self, voice: Option<Handle32<VoiceTag>>);
}

/// A base audio effect, owning its index words and gain slots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    /// The attached info block.
    pub info: Option<Handle32<InfoTag>>,
    /// The attached voice.
    pub voice: Option<Handle32<VoiceTag>>,
    /// The readiness word: nonzero lets the poll refresh run.
    pub ready: u32,
    /// The parameter word attach records.
    pub param: u32,
    /// The auxiliary word: zeroed by the constructor, untouched by the
    /// rest of the proof set.
    pub aux_word: u32,
    /// The entry index the poll derives its slot from.
    pub index: u32,
    /// The row the rotation step copies from.
    pub count: u32,
    /// The fifteen gain slots: three rows of five.
    pub slots: [u32; GAIN_SLOTS],
    /// The rotation loop bound, re-read every iteration.
    pub limit: u8,
    /// The enable flag: zero skips the poll's refresh.
    pub enabled: u8,
    /// The two bytes past the enable flag, carried opaquely.
    pub tail: [u8; 2],
}

impl Effect {
    /// Builds a fresh effect: everything zeroed, the loop bound one.
    ///
    /// The constructor leaves the index words and the gain slots
    /// untouched (whatever the allocation held); the lift zeroes them.
    #[must_use]
    pub fn new() -> Self {
        Self {
            info: None,
            voice: None,
            ready: 0,
            param: 0,
            aux_word: 0,
            index: 0,
            count: 0,
            slots: [0; GAIN_SLOTS],
            limit: 1,
            enabled: 0,
            tail: [0; 2],
        }
    }

    /// Resets the effect, releasing the attached voice when one is set.
    ///
    /// The release runs but the voice cookie is kept, exactly like the
    /// original, which calls the release entry without clearing the word.
    pub fn reset<W: EffectWorld>(&mut self, world: &mut W) {
        if self.voice.is_some() {
            world.release_voice(self.voice);
        }
    }

    /// Reads one slot-table word by flat index. Index 15 composes the
    /// loop-bound byte, the enable flag and the two tail bytes into one
    /// word, exactly as the original's memory lays them out.
    fn read_slot(&self, idx: u32) -> u32 {
        if idx == GAIN_SLOTS as u32 {
            u32::from_le_bytes([self.limit, self.enabled, self.tail[0], self.tail[1]])
        } else {
            self.slots[idx as usize]
        }
    }

    /// Writes one slot-table word by flat index, splitting index 15
    /// back into the bound byte, the flag and the tail bytes.
    fn write_slot(&mut self, idx: u32, value: u32) {
        if idx == GAIN_SLOTS as u32 {
            let bytes = value.to_le_bytes();
            self.limit = bytes[0];
            self.enabled = bytes[1];
            self.tail = [bytes[2], bytes[3]];
        } else {
            self.slots[idx as usize] = value;
        }
    }

    /// Rotates one gain row: copies row `count` over row
    /// `(count + 1) % 3`, answering the last word moved, or the
    /// division quotient when no iteration runs.
    ///
    /// The bound byte is re-read every iteration, so a copy landing on
    /// the word just past the table changes the bound mid-loop.
    ///
    /// # Panics
    ///
    /// When a copy reads or writes past the modelled window (the table
    /// plus its trailing word); the original reads or writes whatever
    /// memory lies there.
    pub fn rotate_slots(&mut self) -> u32 {
        let count = self.count;
        let dividend = count.wrapping_add(1);
        let quotient = dividend / GAIN_ROWS;
        let dst_base = (dividend % GAIN_ROWS).wrapping_mul(GAIN_WIDTH);
        let src_base = count.wrapping_mul(GAIN_WIDTH);
        let mut last = quotient;
        let mut bl = 0u8;
        loop {
            let limit = self.limit;
            if bl >= limit {
                break;
            }
            let value = self.read_slot(src_base.wrapping_add(u32::from(bl)));
            self.write_slot(dst_base.wrapping_add(u32::from(bl)), value);
            last = value;
            bl = bl.wrapping_add(1);
        }
        last
    }

    /// Attaches an info block: records it with its tag word and the
    /// parameter, resolves the voice handle through the manager unless
    /// the tag is all-ones, and resets every gain slot to 1.0.
    ///
    /// Answers false at once for a null block, true otherwise. The
    /// original's answer carries the handle or block address in its
    /// high 24 bits with the low byte forced to 1; that residue is
    /// narrowed away (the differential test pins its shape).
    pub fn attach<W: EffectWorld>(
        &mut self,
        world: &mut W,
        info: Option<Handle32<InfoTag>>,
        tag: u32,
        param: u32,
    ) -> bool {
        self.info = info;
        if info.is_none() {
            return false;
        }
        self.count = 1;
        self.index = 0;
        self.param = param;
        if tag != 0xFFFF_FFFF {
            self.voice = world.lookup_voice(tag, param.wrapping_add(1));
        } else {
            self.voice = None;
        }
        self.slots = [GAIN_ONE; GAIN_SLOTS];
        true
    }

    /// Polls the effect: refreshes the derived entry when ready and
    /// enabled, then polls the attached voice when one is set.
    ///
    /// The original answers whatever the last call left behind, which
    /// the checker does not compare; the lift answers nothing.
    pub fn poll<W: EffectWorld>(&mut self, world: &mut W) {
        if self.ready != 0 && self.enabled != 0 {
            let slot = self.index.wrapping_mul(GAIN_WIDTH).wrapping_add(13);
            world.refresh_entry(slot);
        }
        if self.voice.is_some() {
            world.poll_voice(self.voice);
        }
    }
}

impl Default for Effect {
    /// A fresh effect: [`Effect::new`].
    fn default() -> Self {
        Self::new()
    }
}
