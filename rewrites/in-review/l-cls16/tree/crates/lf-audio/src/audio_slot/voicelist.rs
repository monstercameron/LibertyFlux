//! The voice list: 800 slots behind a liveness bitset.
//!
//! Lifted from the three verified rewrites that share this shape. The
//! object holds a pointer to the bitset words at `+0x28a0`, per-slot
//! records at `+0xFA0` (a value word plus a head half-word, 8 bytes per
//! slot), a dword table at `+0x28a8`, and the voice-list lock at
//! `+0x3210`. [`VOICES`] is 800. The sweep walks link chains whose nodes
//! live in the object itself (next-link half-word plus two tag bytes per
//! 4-byte cell) and resolves each link's voice through the banked file
//! ([`VoiceBankFile`](super::banked::VoiceBankFile)): the bytes are the
//! bank and the slot of a [`VoiceNode`](super::banked::VoiceNode).
//!
//! The lock guard's constructor and destructor (game addresses reached
//! through callee slots) become [`SlotLock`]; the concrete lock lives in
//! `lf-platform`, never here.

use super::banked::{VoiceBankFile, VoiceNode};

/// Voice slots in the list.
pub const VOICES: u32 = 0x320;
/// Offset of the bitset-words pointer.
pub const BITSET_OFF: u32 = 0x28A0;
/// Offset of the per-slot records.
pub const SLOTS_OFF: u32 = 0xFA0;
/// Bytes per slot record.
pub const SLOT_STRIDE: u32 = 8;
/// Offset of the voice-list lock.
pub const LOCK_OFF: u32 = 0x3210;
/// Offset of the dword table.
pub const SPILL_OFF: u32 = 0x28A8;
/// Dwords in the dword table.
pub const SPILL_COUNT: usize = 0x258;
/// Head/link value meaning "none".
pub const NONE: u16 = 0xFFFF;
/// Bit-words covering 800 slots.
pub const BIT_WORDS: usize = 25;

/// One voice-list record: the stored value and the chain head.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoiceSlot {
    /// Value word (record `+0`).
    pub value: u32,
    /// Chain head (record `+4`): [`NONE`] means no chain.
    pub head: u16,
}

/// The voice-list lock: the guard constructor/destructor pair.
pub trait SlotLock {
    /// Takes the lock (guard constructor).
    fn lock(&mut self);
    /// Releases the lock (guard destructor).
    fn unlock(&mut self);
}

/// The sweep's voice calls: refresh, and commit when refresh matches.
pub trait SweepVoices {
    /// Refreshes `node`, answering as the 32-bit refresh.
    fn refresh(&mut self, node: VoiceNode) -> u32;
    /// Commits `node` (runs only when refresh answered the sweep argument).
    fn commit(&mut self, node: VoiceNode);
}

/// One chain cell: the next link plus the bank and slot tag bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChainCell {
    /// Next link, or [`NONE`].
    pub next: u16,
    /// Bank tag byte.
    pub bank: u8,
    /// Slot tag byte.
    pub slot: u8,
}

/// The chain cells the sweep walks, indexed by link value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainStore {
    /// Cells by link value.
    cells: Vec<ChainCell>,
}

impl ChainStore {
    /// A store over `cells`.
    #[must_use]
    pub const fn from_cells(cells: Vec<ChainCell>) -> Self {
        Self { cells }
    }

    /// The cell for `link`.
    ///
    /// # Panics
    ///
    /// When `link` is past the stored cells (the original reads the
    /// object bytes at `link * 4` regardless).
    #[must_use]
    pub fn cell(&self, link: u16) -> ChainCell {
        *self.cells.get(usize::from(link)).unwrap_or_else(|| {
            panic!(
                "link {link:#x} past the {} stored cells: the original reads on regardless",
                self.cells.len()
            )
        })
    }
}

/// The voice list: liveness bits, slot records and the dword table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceList {
    /// Liveness bit-words ([`BIT_WORDS`] words for [`VOICES`] slots).
    bits: Vec<u32>,
    /// Per-slot records.
    slots: Vec<VoiceSlot>,
    /// The dword table ([`SPILL_COUNT`] words).
    spill: Vec<u32>,
}

impl VoiceList {
    /// A list with `bits` bit-words, `slots` records and `spill` dwords.
    #[must_use]
    pub const fn from_parts(bits: Vec<u32>, slots: Vec<VoiceSlot>, spill: Vec<u32>) -> Self {
        Self { bits, slots, spill }
    }

    /// Whether slot `i`'s liveness bit is set.
    ///
    /// # Panics
    ///
    /// When the bit-word is past the stored bits.
    fn live(&self, i: u32) -> bool {
        let word = self.bits.get((i >> 5) as usize).unwrap_or_else(|| {
            panic!(
                "slot {i} past the {} stored bit-words: the original reads on regardless",
                self.bits.len()
            )
        });
        word & (1u32 << (i & 31)) != 0
    }

    /// Sets slot `i`'s liveness bit.
    ///
    /// # Panics
    ///
    /// When the bit-word is past the stored bits.
    fn set_live(&mut self, i: u32) {
        let len = self.bits.len();
        let word = self.bits.get_mut((i >> 5) as usize).unwrap_or_else(|| {
            panic!("slot {i} past the {len} stored bit-words: the original writes on regardless")
        });
        *word |= 1u32 << (i & 31);
    }

    /// Allocates the first free voice slot and initialises it from `arg`.
    ///
    /// Under the lock, scans slots 1..800 for the first clear liveness
    /// bit. When every slot is taken answers [`NONE`] as a word.
    /// Otherwise stores `arg` and head [`NONE`] in the record, sets the
    /// bit and answers the slot index.
    ///
    /// # Panics
    ///
    /// When a record or bit-word is past the stored data.
    pub fn alloc(&mut self, arg: u32, lock: &mut impl SlotLock) -> u32 {
        lock.lock();
        let mut i = 1u32;
        while i < VOICES {
            if !self.live(i) {
                let len = self.slots.len();
                let slot = self.slots.get_mut(i as usize).unwrap_or_else(|| {
                    panic!(
                        "slot {i} past the {len} stored records: the original writes on regardless"
                    )
                });
                *slot = VoiceSlot {
                    value: arg,
                    head: NONE,
                };
                self.set_live(i);
                lock.unlock();
                return i;
            }
            i += 1;
        }
        lock.unlock();
        u32::from(NONE)
    }

    /// Allocates the first free dword-table entry for a value.
    ///
    /// Scans the table for the first zero entry and stores `value` there.
    /// A zero value, or a full table, stores nothing.
    pub fn spill_alloc(&mut self, value: u32) {
        if value == 0 {
            return;
        }
        for cell in &mut self.spill {
            if *cell == 0 {
                *cell = value;
                break;
            }
        }
    }

    /// Sweeps all live voice slots, refreshing chains that match `arg`.
    ///
    /// Under the lock, visits slots 0..800 with set bits whose head is
    /// not [`NONE`], and walks each slot's chain from the chain store:
    /// every link resolves to a voice node refreshed through
    /// [`SweepVoices`], and when the refresh answers `arg` the commit
    /// runs too. Answers 0.
    ///
    /// # Panics
    ///
    /// When a record, bit-word or chain cell is past the stored data, or
    /// a bank byte has no recorded row (the original reads on
    /// regardless).
    pub fn sweep(
        &self,
        chains: &ChainStore,
        file: &VoiceBankFile,
        arg: u32,
        lock: &mut impl SlotLock,
        voices: &mut impl SweepVoices,
    ) -> u32 {
        lock.lock();
        let mut i = 0u32;
        while i < VOICES {
            if self.live(i) {
                let head = self
                    .slots
                    .get(i as usize)
                    .unwrap_or_else(|| {
                        panic!(
                            "slot {i} past the {} stored records: the original reads on regardless",
                            self.slots.len()
                        )
                    })
                    .head;
                if head != NONE {
                    let mut link = head;
                    loop {
                        let cell = chains.cell(link);
                        let node = VoiceNode {
                            bank: cell.bank,
                            slot: cell.slot,
                        };
                        // The handle computation is observable (the row
                        // read can fault), so it happens here even though
                        // only the key crosses the trait; the proof
                        // rebuilds the 32-bit handle from the key per case.
                        let _ = file.node_addr(node);
                        if voices.refresh(node) == arg {
                            voices.commit(node);
                        }
                        link = cell.next;
                        if link == NONE {
                            break;
                        }
                    }
                }
            }
            i += 1;
        }
        lock.unlock();
        0
    }
}
