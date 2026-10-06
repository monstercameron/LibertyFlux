//! The fixed streaming tables: key entries, state slots, the alloc table.
//!
//! Lifted from the verified rewrites. Four shapes share this module
//! (one or two routines each): the 256 keyed entries scanned by key
//! and by liveness, the 64 state slots scanned for a state word, the
//! 128-entry alloc table with its counter, and the 256 geometric slots
//! searched by id within a radius. Every entry address narrows to its
//! index; the proofs rebuild the address per case.

/// Byte length of one keyed entry.
pub const KEY_ENTRY_LEN: usize = 0x1c;
/// Byte length of one state slot.
pub const STATE_SLOT_LEN: usize = 0x70;
/// Byte length of one alloc entry.
pub const ALLOC_ENTRY_LEN: usize = 16;
/// Byte length of one geometric slot.
pub const GEO_SLOT_LEN: usize = 0x30;
/// Bias added to the alloc counter for a fresh id.
const ALLOC_ID_BIAS: u32 = 0x1000_0000;
/// Largest direct index into the alloc table.
const ALLOC_MAX_INDEX: u32 = 0x7f;
/// Empty flag byte of an alloc entry.
const ALLOC_FREE: u8 = 0;
/// Taken flag byte of an alloc entry.
const ALLOC_TAKEN: u8 = 1;
/// Fresh zero word of an allocated entry.
const ALLOC_ZERO: u32 = 0;
/// Fresh link word of an allocated entry.
const ALLOC_LINK: u32 = 0xffff_ffff;

/// One keyed entry: the two key words and its link target's state.
///
/// The 32-bit entry is 28 bytes; the lift owns the key word at `+0x08`
/// and the key/link word at `+0x0c`. The link address narrows to the
/// owned state word of the link target: `None` is the null link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEntry {
    /// The key word at `+0x08`, read by the liveness scan.
    pub key_a: u32,
    /// The key/link word at `+0x0c`: the key scan's key, the live
    /// scan's link (zero is null).
    pub key_b: u32,
    /// The link target's state word: `None` exactly when `key_b` is 0.
    pub target: Option<u32>,
}

/// The 256 keyed entries scanned by key and by liveness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEntries {
    /// Entries in scan order.
    pub entries: Vec<KeyEntry>,
}

impl KeyEntries {
    /// Finds the first entry whose `+0x0c` word equals `key`.
    ///
    /// Restates `stream_find_entry_by_key`. The entry address narrows
    /// to its index (the proof rebuilds it per case).
    #[must_use]
    pub fn find_by_key(&self, key: u32) -> Option<usize> {
        self.entries.iter().position(|entry| entry.key_b == key)
    }

    /// Finds the first entry whose `+0x08` word equals `key` and whose
    /// link is live: non-null with a nonzero target state word.
    ///
    /// Restates `stream_find_live_entry`. The entry address narrows to
    /// its index (the proof rebuilds it per case).
    #[must_use]
    pub fn find_live(&self, key: u32) -> Option<usize> {
        self.entries
            .iter()
            .position(|entry| entry.key_a == key && entry.target.is_some_and(|s| s != 0))
    }
}

/// The 64 state slots scanned for a state word.
///
/// The 32-bit slots are 112 bytes; the lift owns each slot's first
/// word, the only word the scan reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateSlots {
    /// State words in scan order.
    pub states: Vec<u32>,
}

impl StateSlots {
    /// Finds the first slot holding `wanted`.
    ///
    /// Restates `stream_find_slot_by_state`, an exact-equality scan
    /// the original pins to 3 (the proof pins it too). The slot
    /// address narrows to its index (the proof rebuilds it per case).
    #[must_use]
    pub fn find_first_holding(&self, wanted: u32) -> Option<usize> {
        self.states.iter().position(|state| *state == wanted)
    }
}

/// One alloc entry: 16 bytes with an id, a flag and two fresh words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocEntry {
    /// The id word at `+0`.
    pub id: u32,
    /// The flag byte at `+4`: 0 is free, 1 is taken.
    pub flag: u8,
    /// The untouched bytes at `+5` through `+7`, preserved as read.
    pub pad: [u8; 3],
    /// The word at `+8`: 0 when freshly allocated.
    pub zero: u32,
    /// The word at `+12`: all-ones when freshly allocated.
    pub link: u32,
}

impl AllocEntry {
    /// Decodes an entry from its 16 bytes, little-endian.
    #[must_use]
    pub fn from_bytes(bytes: [u8; ALLOC_ENTRY_LEN]) -> Self {
        Self {
            id: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            flag: bytes[4],
            pad: [bytes[5], bytes[6], bytes[7]],
            zero: u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            link: u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        }
    }

    /// Encodes this entry as its 16 bytes, little-endian.
    #[must_use]
    pub fn to_bytes(self) -> [u8; ALLOC_ENTRY_LEN] {
        let mut out = [0u8; ALLOC_ENTRY_LEN];
        out[0..4].copy_from_slice(&self.id.to_le_bytes());
        out[4] = self.flag;
        out[5..8].copy_from_slice(&self.pad);
        out[8..12].copy_from_slice(&self.zero.to_le_bytes());
        out[12..16].copy_from_slice(&self.link.to_le_bytes());
        out
    }
}

/// The alloc table: a counter over 128 sixteen-byte entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocTable {
    /// The allocation counter.
    pub counter: u32,
    /// Entries in index order.
    pub entries: Vec<AllocEntry>,
}

impl AllocTable {
    /// Looks up entry `idx`, or allocates the first free one.
    ///
    /// Restates `stream_entry_lookup_or_alloc`: indexes above 127
    /// (compared unsigned) allocate the first entry whose flag byte
    /// is 0 — its flag becomes 1, its id the old counter biased by
    /// `0x10000000`, its tail words 0 and all-ones, and the counter
    /// increments — while indexes at or below 127 answer the entry
    /// only when its flag byte is exactly 1. Entry addresses narrow
    /// to indexes (the proof rebuilds them per case).
    ///
    /// # Panics
    ///
    /// When a direct index names no owned entry; the original reads
    /// past its array.
    pub fn lookup_or_alloc(&mut self, idx: u32) -> Option<usize> {
        if idx > ALLOC_MAX_INDEX {
            let at = self.entries.iter().position(|entry| entry.flag == ALLOC_FREE)?;
            let id = self.counter.wrapping_add(ALLOC_ID_BIAS);
            self.counter = self.counter.wrapping_add(1);
            let entry = &mut self.entries[at];
            entry.flag = ALLOC_TAKEN;
            entry.id = id;
            entry.zero = ALLOC_ZERO;
            entry.link = ALLOC_LINK;
            return Some(at);
        }
        let at = usize::try_from(idx).ok().filter(|at| *at < self.entries.len())?;
        if self.entries[at].flag == ALLOC_TAKEN {
            Some(at)
        } else {
            None
        }
    }
}

/// One geometric slot: flag, id and centre point.
///
/// The 32-bit slots are 48 bytes at stride; the lift owns the flag
/// byte, the id word and the three centre floats, the only fields the
/// search reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoSlot {
    /// The flag byte: nonzero slots are candidates.
    pub flag: u8,
    /// The id word.
    pub id: u32,
    /// The centre point.
    pub center: [f32; 3],
}

/// The 256 geometric slots searched by id within a radius.
#[derive(Debug, Clone, PartialEq)]
pub struct GeoSlots {
    /// Slots in scan order.
    pub slots: Vec<GeoSlot>,
}

impl GeoSlots {
    /// Whether any active slot with `id` lies within `radius` of `point`.
    ///
    /// Restates `stream_slot_in_radius`: a slot is a candidate when its
    /// flag byte is nonzero and its id word equals `id`; the squared
    /// distance forms exactly as the original's scalar chain does
    /// (`dy*dy + dx*dx + dz*dz`, additions in that order) and the
    /// first candidate with `radius * radius >= d2` (ordered
    /// comparison) ends the scan. The 1/`0x100` answer narrows to bool.
    #[must_use]
    pub fn any_in_radius(&self, id: u32, point: [f32; 3], radius: f32) -> bool {
        for slot in &self.slots {
            if slot.flag != 0 && slot.id == id {
                let dx = sub(point[0], slot.center[0]);
                let dy = sub(point[1], slot.center[1]);
                let dz = sub(point[2], slot.center[2]);
                let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                if mul(radius, radius) >= d2 {
                    return true;
                }
            }
        }
        false
    }
}

/// Ordered subtraction, pinned against reassociation.
#[inline(always)]
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

/// Ordered multiplication, pinned against reassociation.
#[inline(always)]
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// Ordered addition, pinned against reassociation.
#[inline(always)]
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}
