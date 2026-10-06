//! The voice-slot file: 96-byte records behind index bytes.
//!
//! Lifted from the verified rewrites of the four `r-s471` slot routines
//! (no class): the ready-flag check, the slot update with its notify
//! call, the clear-by-owner-id sweep and the ready-to-done flag advance.
//! The 32-bit object is one flat allocation the routines index with
//! wrapping 32-bit arithmetic: index bytes at `+0x368`, the flag-check
//! table at `+0x540`, fixed owner words at `+0x370/+0x3d0/+0x430`, and
//! records of [`RECORD_SIZE`] bytes selected by the index bytes. The lift
//! owns those bytes as a vector and keeps the routines' offset arithmetic
//! exactly; accesses past the store panic (the original touches the
//! wrapped address regardless; see the registry).

/// Base of the three index bytes.
pub const INDEX_BASE: u32 = 0x368;
/// Base of the flag-check index table.
pub const TABLE_BASE: u32 = 0x540;
/// Size of one voice record in bytes.
pub const RECORD_SIZE: u32 = 96;
/// Key bias of the flag check.
pub const KEY_BIAS: u32 = 0x6a;
/// Flag value meaning "ready".
pub const FLAG_READY: u8 = 2;
/// Flag value the sweep and the advance write ("free"/"done").
pub const FLAG_CLEARED: u8 = 3;
/// Indexes the flag check accepts.
pub const MAX_INDEX: u32 = 3;
/// First store offset of the slot update, from the record start.
pub const STORE_A_OFF: u32 = 0x14;
/// Notify offset of the slot update, from the record start.
pub const NOTIFY_OFF: u32 = 0x18;
/// Second store offset of the slot update, from the record start.
pub const STORE_B_OFF: u32 = 0x58;
/// Per-slot owner offsets of the sweep, from each indexed record start.
pub const INDEXED_BASES: [u32; 3] = [0x0c, 0x12c, 0x24c];
/// Owner offsets of the sweep's three fixed slots, from the object start.
pub const FIXED_OWNERS: [u32; 3] = [0x370, 0x3d0, 0x430];
/// Flag offset of the flag advance, from the record start.
pub const FLAG_OFF: u32 = 8;

/// Notified about the middle of an updated record: the slot update's callee.
///
/// The 32-bit callee receives the record field's absolute address; the
/// lift passes its offset from the object start (the proof rebuilds the
/// address per case, exactly even across a wrap).
pub trait Notify {
    /// Records the update of the record field at `at` with cookie `c`.
    fn notify(&mut self, at: u32, c: u32);
}

impl<F: FnMut(u32, u32)> Notify for F {
    fn notify(&mut self, at: u32, c: u32) {
        self(at, c);
    }
}

/// The voice-slot file: the 32-bit object's bytes, owned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceSlots {
    /// The object image; every offset below is into this store.
    mem: Vec<u8>,
}

impl VoiceSlots {
    /// A slot file over `mem`.
    #[must_use]
    pub const fn from_bytes(mem: Vec<u8>) -> Self {
        Self { mem }
    }

    /// The object image.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.mem
    }

    /// The byte at wrapped offset `at`.
    ///
    /// # Panics
    ///
    /// When `at` is past the store (the original touches the wrapped
    /// address regardless).
    fn byte(&self, at: u32) -> u8 {
        *self.mem.get(at as usize).unwrap_or_else(|| {
            panic!(
                "slot offset {at:#x} past the {}-byte store: the original touches the wrapped address",
                self.mem.len()
            )
        })
    }

    /// The unaligned little-endian word at wrapped offset `at`.
    ///
    /// # Panics
    ///
    /// When the word would leave the store.
    fn word(&self, at: u32) -> u32 {
        let lo = at as usize;
        let bytes: [u8; 4] = self.mem.get(lo..lo.wrapping_add(4)).unwrap_or_else(|| {
            panic!(
                "slot word {at:#x} past the {}-byte store: the original touches the wrapped address",
                self.mem.len()
            )
        }).try_into().expect("slice of four converts");
        u32::from_le_bytes(bytes)
    }

    /// Writes the byte at wrapped offset `at`.
    ///
    /// # Panics
    ///
    /// When `at` is past the store.
    fn set_byte(&mut self, at: u32, v: u8) {
        let len = self.mem.len();
        *self.mem.get_mut(at as usize).unwrap_or_else(|| {
            panic!(
                "slot offset {at:#x} past the {len}-byte store: the original touches the wrapped address"
            )
        }) = v;
    }

    /// Writes the unaligned little-endian word at wrapped offset `at`.
    ///
    /// # Panics
    ///
    /// When the word would leave the store.
    fn set_word(&mut self, at: u32, v: u32) {
        let lo = at as usize;
        let len = self.mem.len();
        let slot = self.mem.get_mut(lo..lo.wrapping_add(4)).unwrap_or_else(|| {
            panic!(
                "slot word {at:#x} past the {len}-byte store: the original touches the wrapped address"
            )
        });
        slot.copy_from_slice(&v.to_le_bytes());
    }

    /// Whether indexed slot `index` currently holds the ready flag.
    ///
    /// Reads the table byte at `index + TABLE_BASE`, keys the flag byte
    /// at `4 * (3 * (byte + 2 * index + KEY_BIAS))` (all wrapping), and
    /// reports whether it equals [`FLAG_READY`]. Out-of-range indexes
    /// answer false without reading memory.
    ///
    /// # Panics
    ///
    /// When a read would leave the store.
    #[must_use]
    pub fn flag_check(&self, index: u32) -> bool {
        if index >= MAX_INDEX {
            return false;
        }
        let b = self.byte(index.wrapping_add(TABLE_BASE));
        let k = (b as u32)
            .wrapping_add(index.wrapping_mul(2))
            .wrapping_add(KEY_BIAS);
        let at = k.wrapping_mul(3).wrapping_mul(4);
        self.byte(at) == FLAG_READY
    }

    /// Stores two values into one voice record, notifying mid-way.
    ///
    /// Reads the index byte at `a + INDEX_BASE`, selects record
    /// `(byte + 3 * a) * RECORD_SIZE` (all wrapping), stores `b` at
    /// record + [`STORE_A_OFF`], notifies with (record +
    /// [`NOTIFY_OFF`], `c`), then stores `d` at record +
    /// [`STORE_B_OFF`]. The notify answer is ignored.
    ///
    /// # Panics
    ///
    /// When a read or write would leave the store.
    pub fn slot_update(
        &mut self,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        notify: &mut impl Notify,
    ) {
        let t = self.byte(a.wrapping_add(INDEX_BASE)) as u32;
        let rec = t.wrapping_add(a.wrapping_mul(3)).wrapping_mul(RECORD_SIZE);
        self.set_word(rec.wrapping_add(STORE_A_OFF), b);
        notify.notify(rec.wrapping_add(NOTIFY_OFF), c);
        self.set_word(rec.wrapping_add(STORE_B_OFF), d);
    }

    /// Drops every slot whose owner id equals `id`.
    ///
    /// Sweeps the three indexed slots (index byte at `INDEX_BASE + k`
    /// selecting a record, owner at record + [`INDEXED_BASES`]`[k]`)
    /// and the three fixed owners ([`FIXED_OWNERS`]); every owner
    /// equalling `id` is zeroed and its flag byte (4 below the owner)
    /// set to [`FLAG_CLEARED`].
    ///
    /// # Panics
    ///
    /// When a read or write would leave the store.
    pub fn clear_by_id(&mut self, id: u32) {
        for k in 0..3u32 {
            let rec = (self.byte(INDEX_BASE.wrapping_add(k)) as u32).wrapping_mul(RECORD_SIZE);
            let owner = rec.wrapping_add(INDEXED_BASES[k as usize]);
            if self.word(owner) == id {
                self.set_word(owner, 0);
                self.set_byte(owner.wrapping_sub(4), FLAG_CLEARED);
            }
        }
        for base in FIXED_OWNERS {
            let owner = base;
            if self.word(owner) == id {
                self.set_word(owner, 0);
                self.set_byte(owner.wrapping_sub(4), FLAG_CLEARED);
            }
        }
    }

    /// Moves one slot's flag from ready to cleared.
    ///
    /// Selects record `(t % 3) + 3 * a` where `t = b + 1`, all in
    /// wrapping signed arithmetic matching the original's divide, and
    /// sets the flag byte at record * [`RECORD_SIZE`] + [`FLAG_OFF`]
    /// from [`FLAG_READY`] to [`FLAG_CLEARED`], leaving any other value
    /// alone.
    ///
    /// # Panics
    ///
    /// When the flag byte would leave the store.
    pub fn flag_advance(&mut self, a: u32, b: u32) {
        let t = (b as i32).wrapping_add(1);
        let r = t % 3;
        let idx = r.wrapping_add((a as i32).wrapping_mul(3));
        let at = (idx.wrapping_mul(RECORD_SIZE as i32) as u32).wrapping_add(FLAG_OFF);
        if self.byte(at) == FLAG_READY {
            self.set_byte(at, FLAG_CLEARED);
        }
    }
}
