//! The handle-table slot store: creation, lookup, notify, announce, destroy.
//!
//! One global table holds every slot object; a static default index names
//! the slot readers fall back to when a slot's kind byte is zero. Big
//! records hold every field below; small records hold only the kind byte
//! (zero, so every reader falls back) plus padding the destroy routine
//! clears a word of. The store owns both forms as plain data.

use core::fmt::Debug;

/// Number of handle-table slots the store owns.
pub const TABLE_LEN: u32 = 0x5DC;
/// Big record size, as the allocator is asked for it.
pub const BIG_SIZE: usize = 0xA0;
/// Small record size, as the allocator is asked for it.
pub const SMALL_SIZE: usize = 0x28;

// The allocator takes words: both sizes fit (the casts below rely on it).
const _: () = assert!(BIG_SIZE <= u32::MAX as usize);
const _: () = assert!(SMALL_SIZE <= u32::MAX as usize);
/// Kind byte: zero means unset, so readers fall back to the default slot.
pub const KIND_OFF: usize = 0x08;
/// Word the destroy routine clears.
pub const CLEAR_OFF: usize = 0x0C;
/// Flag byte gating the announce path.
pub const ANNOUNCE_FLAG_OFF: usize = 0x20;
/// Bit selecting the format-and-emit announce path.
pub const ANNOUNCE_BIT: u8 = 0x40;
/// Word handed to the sink on the direct notify path.
pub const NOTIFY_ARG_OFF: usize = 0x24;
/// Device-kind word selecting the notify sink.
pub const DEVICE_OFF: usize = 0x48;
/// Mode word.
pub const MODE_OFF: usize = 0x54;
/// Flag byte.
pub const FLAG_OFF: usize = 0x58;
/// Payload start; the announce path hands this block on.
pub const PAYLOAD_OFF: usize = 0x60;
/// Payload length (record end minus payload start).
pub const PAYLOAD_LEN: usize = BIG_SIZE - PAYLOAD_OFF;
/// Owned-flag offset inside the thread entry the destroy routine reads.
pub const THREAD_OWNED_OFF: usize = 0x08;
/// Mask keeping every result bit except the low byte.
pub const HI_MASK: u32 = 0xFFFF_FF00;

/// A slot object: a big record holding every field, or a small record
/// holding only the kind byte plus padding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotObject {
    /// A fully populated record.
    Big([u8; BIG_SIZE]),
    /// A kind byte of zero plus allocator fill.
    Small([u8; SMALL_SIZE]),
}

impl SlotObject {
    /// The kind byte: zero means unset.
    #[must_use]
    pub fn kind(&self) -> u8 {
        self.as_bytes()[KIND_OFF]
    }

    /// One byte of the record.
    ///
    /// # Panics
    ///
    /// When the offset is past the record's end (a small record read for
    /// a big-only field, where the original over-reads the heap).
    #[must_use]
    pub fn byte_at(&self, off: usize) -> u8 {
        self.as_bytes()[off]
    }

    /// One little-endian word of the record.
    ///
    /// # Panics
    ///
    /// When the word runs past the record's end, as [`byte_at`](Self::byte_at).
    #[must_use]
    pub fn word_at(&self, off: usize) -> u32 {
        u32::from_le_bytes(self.as_bytes()[off..off + 4].try_into().unwrap())
    }

    /// Writes one little-endian word of the record.
    ///
    /// # Panics
    ///
    /// When the word runs past the record's end, as [`byte_at`](Self::byte_at).
    pub fn set_word(&mut self, off: usize, value: u32) {
        self.as_bytes_mut()[off..off + 4].copy_from_slice(&value.to_le_bytes());
    }

    /// The announce payload block.
    ///
    /// # Panics
    ///
    /// For a small record, which has no payload block.
    #[must_use]
    pub fn payload(&self) -> &[u8; PAYLOAD_LEN] {
        self.as_bytes()[PAYLOAD_OFF..PAYLOAD_OFF + PAYLOAD_LEN]
            .try_into()
            .unwrap()
    }

    /// The record's bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            SlotObject::Big(b) => b,
            SlotObject::Small(b) => b,
        }
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        match self {
            SlotObject::Big(b) => b,
            SlotObject::Small(b) => b,
        }
    }
}

/// The collaborator that allocates and fills new slot objects.
pub trait SlotBuild: Debug {
    /// An allocated-but-unfilled block, as the allocator hands it back.
    type Pending;
    /// Allocates a block of `size` bytes, or answers null.
    fn alloc(&mut self, size: u32) -> Option<Self::Pending>;
    /// Constructs a big record's bytes in the block.
    fn construct_big(&mut self, block: Self::Pending) -> [u8; BIG_SIZE];
    /// The allocator fill a small record starts from (the kind byte is
    /// cleared afterwards).
    fn small_fill(&mut self, block: Self::Pending) -> [u8; SMALL_SIZE];
}

/// The three notify sinks, selected by the device-kind word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifyTarget {
    /// First sink (device kind 1).
    Sink1,
    /// Second sink (device kind 2).
    Sink2,
    /// Third sink (device kind 3).
    Sink3,
}

/// The collaborator receiving kind notifications.
pub trait NotifySinks: Debug {
    /// Notifies one sink, answering whatever it answers.
    fn notify(&mut self, target: NotifyTarget, arg: u32) -> u32;
}

/// The collaborator formatting an announce payload into text.
pub trait FormatPayload: Debug {
    /// Formats the payload, answering the text handle the sink takes.
    fn format(&mut self, payload: &[u8; PAYLOAD_LEN]) -> u32;
}

/// The collaborator receiving formatted announcements.
pub trait AnnounceSink: Debug {
    /// Emits one announcement, answering whatever it answers.
    fn emit(&mut self, text: u32) -> u32;
}

/// What the announce routine hands back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Announce {
    /// The payload block itself (the original answers its address).
    Inline,
    /// The sink's answer on the format-and-emit path.
    Emitted(u32),
}

/// The thread entry the destroy routine consults.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadEntry {
    /// Whether the entry's owned word is set.
    pub owned: bool,
}

/// The collaborator resolving a handle to a slot index.
pub trait SlotLookup: Debug {
    /// Resolves a handle, answering the index (possibly negative).
    fn lookup(&mut self, id: u32) -> u32;
}

/// The collaborator notified of a slot's drop.
///
/// It runs after the routine clears the object's word and before the slot
/// is re-read, so it may rewrite the slot (the routine releases whatever
/// it finds).
pub trait SlotDrop: Debug {
    /// Drops the slot, possibly rewriting it in the store.
    fn drop_slot(&mut self, store: &mut SlotStore, idx: u32);
}

/// The collaborator releasing an owned slot object.
pub trait SlotRelease: Debug {
    /// Releases the re-read slot (`None` when the drop cleared it),
    /// answering whatever it answers (only the high bytes travel on).
    fn release(&mut self, slot: Option<&SlotObject>) -> u32;
}

/// What the destroy routine hands back (low-byte residue stripped; the
/// original answers the index, zero, the release answer or the thread
/// entry, each with its low byte cleared or set).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestroyOutcome {
    /// Negative index: nothing ran.
    Invalid,
    /// Null slot: nothing ran.
    Empty,
    /// Owned path: the release answer with its low byte cleared.
    Released(u32),
    /// Unowned path: the slot was cleared.
    Cleared,
}

/// The handle-table slot store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotStore {
    slots: Vec<Option<SlotObject>>,
    default: u32,
}

impl SlotStore {
    /// An empty store with the given default index.
    #[must_use]
    pub fn new(default: u32) -> Self {
        Self {
            slots: (0..TABLE_LEN).map(|_| None).collect(),
            default,
        }
    }

    /// A store over caller-built slots.
    ///
    /// # Panics
    ///
    /// When the slot count is not [`TABLE_LEN`].
    #[must_use]
    pub fn with_slots(slots: Vec<Option<SlotObject>>, default: u32) -> Self {
        assert_eq!(slots.len(), TABLE_LEN as usize, "slot count");
        Self { slots, default }
    }

    /// The slots, in table order.
    #[must_use]
    pub fn slots(&self) -> &[Option<SlotObject>] {
        &self.slots
    }

    /// Replaces one slot (what the drop collaborator rewrites).
    ///
    /// # Panics
    ///
    /// When the index is past the table.
    pub fn set_slot(&mut self, idx: u32, slot: Option<SlotObject>) {
        let cell = self
            .slots
            .get_mut(idx as usize)
            .expect("slot index past the table");
        *cell = slot;
    }

    /// The default index readers fall back to.
    #[must_use]
    pub fn default_index(&self) -> u32 {
        self.default
    }

    /// The record an index reads: the slot itself when its kind byte is
    /// set, else the default slot.
    ///
    /// # Panics
    ///
    /// When either index is past the table or names a null slot: the
    /// original reads wild memory or faults through null there.
    fn resolve(&self, idx: u32) -> &SlotObject {
        let slot = self
            .slots
            .get(idx as usize)
            .expect("slot index past the table")
            .as_ref()
            .expect("null slot has no kind byte");
        if slot.kind() != 0 {
            return slot;
        }
        self.slots
            .get(self.default as usize)
            .expect("default index past the table")
            .as_ref()
            .expect("null default slot")
    }

    /// Finds a free slot at or after `start` and installs an object there,
    /// answering the index, or all-ones when the table is full above `start`.
    ///
    /// A big install allocates [`BIG_SIZE`] bytes and constructs the record
    /// through the collaborator; a small install allocates [`SMALL_SIZE`]
    /// bytes and clears the kind byte. A failed allocation leaves the cell
    /// empty (it already was) and still answers the index.
    pub fn find_free(&mut self, big: bool, start: u32, build: &mut impl SlotBuild) -> u32 {
        if start >= TABLE_LEN {
            // The original scans wild words here but always answers -1:
            // fitting the install needs an index below the length.
            return u32::MAX;
        }
        let mut i = start;
        while i < TABLE_LEN && self.slots[i as usize].is_some() {
            i += 1;
        }
        if i >= TABLE_LEN {
            return u32::MAX;
        }
        // Small consts: the fit is asserted at compile time above.
        #[allow(clippy::cast_possible_truncation)]
        let big_size = BIG_SIZE as u32;
        #[allow(clippy::cast_possible_truncation)]
        let small_size = SMALL_SIZE as u32;
        if big {
            if let Some(block) = build.alloc(big_size) {
                let bytes = build.construct_big(block);
                self.slots[i as usize] = Some(SlotObject::Big(bytes));
            }
        } else if let Some(block) = build.alloc(small_size) {
            let mut bytes = build.small_fill(block);
            bytes[KIND_OFF] = 0;
            self.slots[i as usize] = Some(SlotObject::Small(bytes));
        }
        i
    }

    /// Notifies the sink matching a slot's device kind, answering the
    /// sink's answer, or zero for a null slot or an unknown kind.
    ///
    /// A slot whose kind byte is set notifies with its own word at
    /// [`NOTIFY_ARG_OFF`]; an unset slot notifies through the default slot
    /// with all-ones. Kinds 1, 2 and 3 select the three sinks.
    ///
    /// # Panics
    ///
    /// When the index or the default index is past the table, or the
    /// default slot is null on the fallback path: the original reads wild
    /// memory or faults through null there.
    pub fn notify_kind(&self, idx: u32, sinks: &mut impl NotifySinks) -> u32 {
        let obj = self
            .slots
            .get(idx as usize)
            .expect("slot index past the table");
        let Some(obj) = obj else { return 0 };
        let (src, arg) = if obj.kind() != 0 {
            (obj, obj.word_at(NOTIFY_ARG_OFF))
        } else {
            let src = self
                .slots
                .get(self.default as usize)
                .expect("default index past the table")
                .as_ref()
                .expect("null default slot");
            (src, u32::MAX)
        };
        let target = match src.word_at(DEVICE_OFF) {
            1 => NotifyTarget::Sink1,
            2 => NotifyTarget::Sink2,
            3 => NotifyTarget::Sink3,
            _ => return 0,
        };
        sinks.notify(target, arg)
    }

    /// Reads a slot's flag byte, falling back to the default slot when
    /// the kind byte is unset.
    ///
    /// # Panics
    ///
    /// As [`resolve`](Self::resolve), plus when the chosen record is small:
    /// the original over-reads the heap there.
    #[must_use]
    pub fn flag_byte(&self, idx: u32) -> u8 {
        self.resolve(idx).byte_at(FLAG_OFF)
    }

    /// Reads a slot's mode word, falling back to the default slot when
    /// the kind byte is unset.
    ///
    /// # Panics
    ///
    /// As [`resolve`](Self::resolve), plus when the chosen record is small:
    /// the original over-reads the heap there.
    #[must_use]
    pub fn mode_word(&self, idx: u32) -> u32 {
        self.resolve(idx).word_at(MODE_OFF)
    }

    /// Announces a slot's payload: the block itself when the flag bit is
    /// clear, else formatted and emitted through the collaborators.
    ///
    /// # Panics
    ///
    /// As [`resolve`](Self::resolve), plus when the chosen record is small:
    /// the original over-reads the heap there.
    pub fn announce(
        &self,
        idx: u32,
        fmt: &mut impl FormatPayload,
        sink: &mut impl AnnounceSink,
    ) -> Announce {
        let obj = self.resolve(idx);
        if obj.byte_at(ANNOUNCE_FLAG_OFF) & ANNOUNCE_BIT != 0 {
            let text = fmt.format(obj.payload());
            Announce::Emitted(sink.emit(text))
        } else {
            Announce::Inline
        }
    }

    /// Destroys a slot: clears it, notifies, and releases its object when
    /// the thread entry owns it.
    ///
    /// `by_handle` selects the index through the lookup collaborator,
    /// else `id` is the index. A negative index or a null slot returns
    /// without running anything. Otherwise the object's word is cleared,
    /// the drop collaborator runs (and may rewrite the slot), and when
    /// the thread entry is owned the re-read slot is released. The slot
    /// is cleared on both surviving paths.
    ///
    /// # Panics
    ///
    /// When a non-negative index is past the table: the original reads
    /// and clears wild memory there.
    pub fn destroy(
        &mut self,
        id: u32,
        by_handle: bool,
        thread: &ThreadEntry,
        lookup: &mut impl SlotLookup,
        drop: &mut impl SlotDrop,
        release: &mut impl SlotRelease,
    ) -> DestroyOutcome {
        let idx = if by_handle { lookup.lookup(id) } else { id };
        if idx.cast_signed() < 0 {
            return DestroyOutcome::Invalid;
        }
        let cell = self
            .slots
            .get(idx as usize)
            .expect("slot index past the table");
        if cell.is_none() {
            return DestroyOutcome::Empty;
        }
        self.slots[idx as usize]
            .as_mut()
            .unwrap()
            .set_word(CLEAR_OFF, 0);
        drop.drop_slot(self, idx);
        if thread.owned {
            let answer = release.release(self.slots[idx as usize].as_ref());
            self.slots[idx as usize] = None;
            DestroyOutcome::Released(answer & HI_MASK)
        } else {
            self.slots[idx as usize] = None;
            DestroyOutcome::Cleared
        }
    }
}
