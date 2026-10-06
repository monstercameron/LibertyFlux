//! The banked voice table: records behind the scale/table globals.
//!
//! Lifted from the verified rewrites of the `a-12` banked-record trio
//! (no class): set flag bit 3, store the slot word, set flag bit 1 from
//! an argument bit. The 32-bit routines resolve their record through two
//! audio globals: a bank base read from the table at `sub * STRIDE +
//! TABLE_BIAS`, plus `scale * sel`, where `sel`/`sub` are bytes at the
//! argument object's `+4`/`+0x40`. The record's slot word sits at `+0xE0`
//! and its flag byte at `+0xE8`. The lift owns the banks as vectors of
//! records and takes the two selector bytes directly; the proof rebuilds
//! the record address per case, so the translation is proven, not assumed.

/// Stride of one bank entry in the table, in bytes.
pub const STRIDE: u32 = 0x6F40;
/// Bias of the first bank entry from the table base.
pub const TABLE_BIAS: u32 = 0x6F14;
/// Offset of the slot word from the record start.
pub const SLOT_OFF: u32 = 0xE0;
/// Offset of the flag byte from the record start.
pub const FLAG_OFF: u32 = 0xE8;
/// Flag bit the first routine sets.
pub const FLAG_BIT: u8 = 8;
/// Selector value taking the original's faulting path.
pub const FAULT_SEL: u8 = 0xFF;

/// The two bytes selecting a banked record: `sel` (`this+4`) picks the
/// record inside the bank, `sub` (`this+0x40`) picks the bank.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoiceSel {
    /// The record selector.
    pub sel: u8,
    /// The bank selector.
    pub sub: u8,
}

/// One banked voice record: the slot word and the flag byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BankRecord {
    /// The slot word (record + [`SLOT_OFF`]).
    pub slot: u32,
    /// The flag byte (record + [`FLAG_OFF`]).
    pub flags: u8,
}

/// The banked voice table: per-record stride and the banks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BankedVoices {
    /// The scale word: record stride inside a bank.
    pub scale: u32,
    /// The banks; `banks[sub][sel]` is the selected record.
    pub banks: Vec<Vec<BankRecord>>,
}

impl BankedVoices {
    /// The selected record.
    ///
    /// # Panics
    ///
    /// When `sel` is [`FAULT_SEL`] (the original faults at a near-null
    /// address there), or when the bank or record is past the table.
    fn record(&mut self, at: VoiceSel) -> &mut BankRecord {
        assert!(
            at.sel != FAULT_SEL,
            "selector 0xff: the original faults at a near-null address"
        );
        let sub = at.sub as usize;
        let sel = at.sel as usize;
        assert!(
            sub < self.banks.len(),
            "bank {sub} past {} banks: the original reads past the table",
            self.banks.len()
        );
        assert!(
            sel < self.banks[sub].len(),
            "record {sel} past {} records of bank {sub}: the original reads past the bank",
            self.banks[sub].len()
        );
        &mut self.banks[sub][sel]
    }

    /// Sets [`FLAG_BIT`] in the record's flag byte.
    ///
    /// # Panics
    ///
    /// As [`BankedVoices::record`].
    pub fn set_flag8(&mut self, at: VoiceSel) {
        self.record(at).flags |= FLAG_BIT;
    }

    /// Stores `value` into the record's slot word.
    ///
    /// # Panics
    ///
    /// As [`BankedVoices::record`].
    pub fn store_slot(&mut self, at: VoiceSel, value: u32) {
        self.record(at).slot = value;
    }

    /// Sets bit 1 of the record's flag byte from the argument's low bit.
    ///
    /// Computes the adjust mask exactly as the original's
    /// double-xor-mask sequence and answers whether the flag changed.
    /// (The original answers the table base with its low byte replaced
    /// by the mask; the proof rebuilds that answer per case.)
    ///
    /// # Panics
    ///
    /// As [`BankedVoices::record`].
    pub fn set_bit1(&mut self, at: VoiceSel, arg: u32) -> bool {
        let slot = &mut self.record(at).flags;
        // The original doubles the argument's low byte: truncation intended.
        #[allow(clippy::cast_possible_truncation)]
        let doubled = (arg as u8).wrapping_mul(2);
        let adjust = (doubled ^ *slot) & 2;
        *slot ^= adjust;
        adjust != 0
    }
}
