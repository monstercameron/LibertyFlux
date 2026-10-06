//! One streaming slot's flag word: the state bits at `+0x254`.
//!
//! Lifted from the verified rewrites. Three neighbouring routines share
//! this word: two clearers that each clear one bit and then review two
//! state bits (instances of one routine, proved separately), and a
//! marker that sets the live bit.

/// The live bit: set by the marker, dropped by the clearers.
const LIVE_BIT: u32 = 1;
/// The state bit both clearers review second.
const SECOND_BIT: u32 = 5;

/// One slot's flag word, owned as a bare word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotFlags(pub u32);

impl SlotFlags {
    /// Builds flag bits from the raw word.
    #[must_use]
    pub const fn new(word: u32) -> Self {
        Self(word)
    }

    /// The raw flag word.
    #[must_use]
    pub const fn word(self) -> u32 {
        self.0
    }

    /// Clears bit `cleared`, then reviews two state bits.
    ///
    /// Restates both `stream_slot_clear_bit3` (cleared 3, other 4) and
    /// `stream_slot_clear_bit4` (cleared 4, other 3): the cleared bit is
    /// dropped first; then bit `other` selects the return of
    /// `flags >> other`, else bit 5 selects `flags >> 5`, else the live
    /// bit is dropped as well and the shifted value is returned. The two
    /// early exits leave only the cleared bit dropped.
    ///
    /// # Panics
    ///
    /// When either bit number is past bit 31.
    #[must_use]
    pub fn clear_and_review(&mut self, cleared: u32, other: u32) -> u32 {
        assert!(cleared < 32 && other < 32, "flag bit out of range");
        self.0 &= !(1 << cleared);
        let by_kind = self.0 >> other;
        if by_kind & 1 != 0 {
            return by_kind;
        }
        let by_state = self.0 >> SECOND_BIT;
        if by_state & 1 != 0 {
            return by_state;
        }
        self.0 &= !LIVE_BIT;
        by_state
    }

    /// Sets the live bit and bit 3.
    ///
    /// Restates `stream_slot_mark_live`. The returned slot pointer
    /// narrows away: callers own the record (the proof checks the
    /// rewrite answers the planted object).
    pub fn mark_live(&mut self) {
        self.0 |= (1 << 3) | LIVE_BIT;
    }
}
