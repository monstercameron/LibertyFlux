//! The record slot table and its clock: a cluster with globals.
//!
//! Inferred from the rewrites' names and from what they touch: a table of
//! 1,500 eight-byte entries (a value word and a flag byte), a parallel
//! table of "used" bytes with an allocation cursor, two wrapping 15-bit
//! serial counters, a pair of clock words with a tick counter and a stored
//! base, a pending scale slot, and a 4,096-entry key table that resolves
//! keys to values. Every one of these was a global in the original.
//!
//! The lift carries all of them in one [`SlotTableState`], passed
//! explicitly to each function (`&` when it only reads, `&mut` when it
//! writes). Callee slots become [`SlotTableOps`] methods; a callee that may
//! touch the same state receives it as an argument, so no function ever
//! holds two mutable paths to one global. Addresses appear only in
//! [`layout`], which loads and stores the state at the boundary.
//!
//! Deferred from this cluster (address-embedding: the originals add table
//! words that are buffer addresses to offsets, and return or store the
//! result as a pointer): `rw_00952c70` (0x00952C70,
//! `store_adjusted_value`) and `rw_00953950` (0x00953950,
//! `advance_slot_cursor`). They lift with the buffers they point into.

/// Number of entries in the slot table (and of used bytes).
pub const ENTRY_COUNT: usize = 1500;

/// Highest valid entry index.
pub const LAST_ENTRY: u16 = 1499;

/// Number of entries in the key table.
pub const KEY_COUNT: usize = 4096;

/// A serial counter that reaches this value wraps back to zero.
pub const SERIAL_WRAP: u16 = 0x7FFF;

/// Seconds per clock tick: the clock counts milliseconds (Inferred).
pub const SECONDS_PER_TICK: f32 = 0.001;

/// One slot-table entry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Entry {
    /// The entry's value word.
    pub value: u32,
    /// The entry's flag byte; zero means the entry holds nothing. Other
    /// bit meanings are Unknown, so the byte is kept whole.
    pub flags: u8,
}

/// One key-table entry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KeyEntry {
    /// The key, `None` for an empty entry (all ones in the original, so no
    /// lookup can ever match it).
    pub key: Option<u32>,
    /// The value the key resolves to. Kept for empty entries too, so a
    /// load and store round trip is exact.
    pub value: u32,
}

/// Every global the cluster reads or writes.
#[derive(Clone, Debug, PartialEq)]
pub struct SlotTableState {
    /// The slot table.
    pub entries: Box<[Entry; ENTRY_COUNT]>,
    /// Which entries are allocated. Narrowed: a byte in the original, read
    /// only as zero or non-zero and written only as 1 by this cluster.
    pub used: Box<[bool; ENTRY_COUNT]>,
    /// Where the next allocation scan starts. Lifted domain: at most
    /// [`ENTRY_COUNT`] (the cluster never stores more).
    pub cursor: u16,
    /// First serial counter.
    pub serial_a: u16,
    /// Second serial counter.
    pub serial_b: u16,
    /// Clock word the spans are measured from.
    pub clock_base: u32,
    /// Clock word the spans are measured to; also the stamp a stamped
    /// lookup compares against.
    pub clock_mark: u32,
    /// Tick counter.
    pub clock_ticks: u32,
    /// The stored base a stamped lookup answers with.
    pub stamp_base: u32,
    /// A scale value waiting to be taken; zero when none is pending.
    pub pending_scale: f32,
    /// Scale applied to the tick counter. Read-only for this cluster; its
    /// value comes from the original's data when the state is loaded, it is
    /// never written into this crate.
    pub tick_scale: f32,
    /// The key table.
    pub keys: Box<[KeyEntry; KEY_COUNT]>,
}

/// Builds a boxed fixed-size array on the heap (no large stack temporary).
fn boxed_array<T: Clone, const N: usize>(value: T) -> Box<[T; N]> {
    match vec![value; N].into_boxed_slice().try_into() {
        Ok(array) => array,
        Err(_) => unreachable!("the vector has exactly N elements"),
    }
}

impl SlotTableState {
    /// An empty table: every entry, counter and clock zero, every key
    /// empty, with the given tick scale.
    #[must_use]
    pub fn new(tick_scale: f32) -> Self {
        Self {
            entries: boxed_array(Entry::default()),
            used: boxed_array(false),
            cursor: 0,
            serial_a: 0,
            serial_b: 0,
            clock_base: 0,
            clock_mark: 0,
            clock_ticks: 0,
            stamp_base: 0,
            pending_scale: 0.0,
            tick_scale,
            keys: boxed_array(KeyEntry::default()),
        }
    }
}

/// The callees the cluster reaches.
///
/// Each method receives the state, because the real callee may read or
/// write the same globals (Inferred for all three: their bodies are not in
/// this cluster).
pub trait SlotTableOps {
    /// A fresh serial number for an allocation; only its low 16 bits are
    /// used. (`rw_00952de0`'s slot 1.)
    fn next_serial(&mut self, st: &mut SlotTableState) -> u16;
    /// The current stamp, compared against [`SlotTableState::clock_mark`].
    /// (Slot 1 of `rw_009526b0` and `rw_00952660`.)
    fn current_stamp(&mut self, st: &mut SlotTableState) -> u32;
    /// Brings entry `index` up to date before it is read; called for
    /// entries that are not marked used. (`rw_00953110`'s slot 1, which
    /// receives the entry's address in the original.)
    fn refresh_entry(&mut self, st: &mut SlotTableState, index: u16);
}

/// An allocated slot: its index and the serial issued with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ticket {
    /// Entry index.
    pub index: u16,
    /// Serial number (low 16 bits of the serial callee's answer).
    pub serial: u16,
}

impl Ticket {
    /// The original's packed form: index in the low half, serial in the
    /// high half. A failed allocation is all ones, which no ticket packs to.
    #[must_use]
    pub fn pack(self) -> u32 {
        u32::from(self.index) | (u32::from(self.serial) << 16)
    }
}

/// Advances a serial counter, wrapping to zero on reaching
/// [`SERIAL_WRAP`], and returns the new value.
fn bump_serial(counter: &mut u16) -> u16 {
    let next = counter.wrapping_add(1);
    *counter = if next == SERIAL_WRAP { 0 } else { next };
    *counter
}

/// Advances the first serial counter and returns its new value.
///
/// Narrowed: the original returns the value in the low half of its result
/// word; only that half is meaningful.
///
/// Original: `rw_00952db0` (0x00952DB0, `bump_counter_a`).
pub fn bump_serial_a(st: &mut SlotTableState) -> u16 {
    bump_serial(&mut st.serial_a)
}

/// Advances the second serial counter and returns its new value.
///
/// Narrowed as [`bump_serial_a`].
///
/// Original: `rw_00952e60` (0x00952E60, `bump_counter_b`).
pub fn bump_serial_b(st: &mut SlotTableState) -> u16 {
    bump_serial(&mut st.serial_b)
}

/// Allocates the first free entry at or after the cursor, wrapping round
/// to the start; `None` when every entry is used.
///
/// The taken entry is marked used and the cursor moves past it before the
/// serial callee runs; the ticket carries the index and the serial.
///
/// # Panics
///
/// When the cursor is above [`ENTRY_COUNT`]: outside the lifted domain
/// (the original would then scan bytes past the table, which belong to
/// other globals).
///
/// Original: `rw_00952de0` (0x00952DE0, `alloc_slot`); returns
/// [`Ticket::pack`] or all ones.
pub fn allocate<O: SlotTableOps + ?Sized>(st: &mut SlotTableState, ops: &mut O) -> Option<Ticket> {
    let cursor = usize::from(st.cursor);
    assert!(
        cursor <= ENTRY_COUNT,
        "slot cursor {cursor} is past the table: outside the lifted domain"
    );
    let free = (cursor..ENTRY_COUNT)
        .chain(0..cursor)
        .find(|&i| !st.used[i])?;
    st.used[free] = true;
    // `free` is below ENTRY_COUNT, so both fit in 16 bits.
    let index = u16::try_from(free).expect("entry index fits 16 bits");
    st.cursor = index + 1;
    let serial = ops.next_serial(st);
    Some(Ticket { index, serial })
}

/// The value of entry `index`, or zero when the index is past the table or
/// the entry's flag byte is clear. An entry not marked used is refreshed
/// through the callee first, and the value is read after the refresh.
///
/// Narrowed: the original takes a word and uses its low 16 bits; the
/// boundary passes those bits.
///
/// Original: `rw_00953110` (0x00953110, `get_entry_value`).
pub fn entry_value<O: SlotTableOps + ?Sized>(
    st: &mut SlotTableState,
    ops: &mut O,
    index: u16,
) -> u32 {
    if index > LAST_ENTRY {
        return 0;
    }
    let i = usize::from(index);
    if st.entries[i].flags == 0 {
        return 0;
    }
    if !st.used[i] {
        ops.refresh_entry(st, index);
    }
    st.entries[i].value
}

/// The row a 16-bit key addresses, or `None` past the table.
///
/// Narrowed: the original returns the row's address (`table + key × 8`) or
/// null, and takes a word of which it uses the low 16 bits. The boundary
/// form is [`layout::entry_addr`].
///
/// Original: `rw_00953210` (0x00953210, `row_slot_or_null`).
#[must_use]
pub fn entry_row(key: u16) -> Option<u16> {
    (key <= LAST_ENTRY).then_some(key)
}

/// Clears every entry's value word and flag byte.
///
/// Narrowed: the original returns the address one past the table, which
/// has no lifted meaning; the differential test checks it on the 32-bit
/// side. The three bytes after each flag byte are untouched in both.
///
/// Original: `rw_00e63c90` (0x00E63C90, `zero_byte_table`).
pub fn clear_entries(st: &mut SlotTableState) {
    for entry in st.entries.iter_mut() {
        *entry = Entry::default();
    }
}

/// Ticks from the base clock word to the mark, wrapping.
///
/// Original: `rw_00953900` (0x00953900, `clock_span`).
#[must_use]
pub fn clock_span(st: &SlotTableState) -> u32 {
    st.clock_mark.wrapping_sub(st.clock_base)
}

/// [`clock_span`] in seconds: the unsigned span converted exactly, rounded
/// once to `f32`, then scaled by [`SECONDS_PER_TICK`].
///
/// Original: `rw_00953910` (0x00953910, `clock_span_seconds`).
#[must_use]
#[allow(clippy::cast_possible_truncation)] // the original's one rounding to f32
pub fn clock_span_seconds(st: &SlotTableState) -> f32 {
    (f64::from(clock_span(st)) as f32) * SECONDS_PER_TICK
}

/// The tick counter plus the base clock word, wrapping.
///
/// Original: `rw_00952700` (0x00952700, `stamp_sum`).
#[must_use]
pub fn stamp_sum(st: &SlotTableState) -> u32 {
    st.clock_ticks.wrapping_add(st.clock_base)
}

/// The tick counter as a float, scaled: the unsigned counter converts
/// exactly, rounds once to `f32`, is multiplied by
/// [`SlotTableState::tick_scale`], and widens to `f64`.
///
/// Original: `rw_009526d0` (0x009526D0, `scaled_counter_as_float`).
#[must_use]
#[allow(clippy::cast_possible_truncation)] // the original's one rounding to f32
pub fn scaled_ticks(st: &SlotTableState) -> f64 {
    let ticks = f64::from(st.clock_ticks) as f32;
    f64::from(ticks * st.tick_scale)
}

/// Takes the pending scale, leaving zero behind, and returns it widened.
///
/// Original: `rw_00953160` (0x00953160, `take_scale_and_clear`).
pub fn take_pending_scale(st: &mut SlotTableState) -> f64 {
    f64::from(core::mem::take(&mut st.pending_scale))
}

/// The stored base, or the base plus one when the stamp callee's answer
/// equals the mark clock word (read after the callee returns).
///
/// Original: `rw_009526b0` (0x009526B0, `stamped_base_or_next`).
pub fn stamped_base<O: SlotTableOps + ?Sized>(st: &mut SlotTableState, ops: &mut O) -> u32 {
    let stamp = ops.current_stamp(st);
    if stamp == st.clock_mark {
        st.stamp_base.wrapping_add(1)
    } else {
        st.stamp_base
    }
}

/// The value the first matching key-table entry holds; on a miss, the
/// answer of [`stamped_base`].
///
/// Original: `rw_00952660` (0x00952660, `key_table_lookup`).
pub fn key_value<O: SlotTableOps + ?Sized>(st: &mut SlotTableState, ops: &mut O, key: u32) -> u32 {
    match st.keys.iter().find(|entry| entry.key == Some(key)) {
        Some(entry) => entry.value,
        None => stamped_base(st, ops),
    }
}

/// The cluster's 32-bit layout: where each global lives and how it is
/// encoded. Only boundary code (the differential harness, a future
/// in-process shim) uses this module; the lifted functions above never do.
pub mod layout {
    use lf_core::boundary::{BoundaryError, FixedLayout, FromLayout, Image32, IntoLayout};
    use lf_core::{assert_fixed_layout, assert_offset, assert_size};

    use super::{ENTRY_COUNT, Entry, KEY_COUNT, KeyEntry, SlotTableState};

    /// Used bytes, one per entry.
    pub const USED: u32 = 0x011F_6958;
    /// Clock word the spans are measured from.
    pub const CLOCK_BASE: u32 = 0x011F_7028;
    /// Clock word the spans are measured to.
    pub const CLOCK_MARK: u32 = 0x011F_702C;
    /// Tick counter.
    pub const CLOCK_TICKS: u32 = 0x011F_7030;
    /// Pending scale (an `f32`).
    pub const PENDING_SCALE: u32 = 0x011F_7054;
    /// Stored base.
    pub const STAMP_BASE: u32 = 0x011F_70C4;
    /// First serial counter (16 bits).
    pub const SERIAL_A: u32 = 0x011F_7100;
    /// Allocation cursor (16 bits).
    pub const CURSOR: u32 = 0x011F_7104;
    /// Second serial counter (16 bits).
    pub const SERIAL_B: u32 = 0x011F_7108;
    /// The slot table.
    pub const ENTRIES: u32 = 0x011F_7110;
    /// The key table.
    pub const KEYS: u32 = 0x0120_08B0;
    /// Tick scale (an `f32`, read-only data).
    pub const TICK_SCALE: u32 = 0x00FE_86B4;

    /// Bytes per slot-table entry.
    pub const ENTRY_STRIDE: u32 = 8;
    /// Bytes per key-table entry.
    pub const KEY_STRIDE: u32 = 8;
    /// Entries in the slot table, as an address-space length.
    const ENTRY_COUNT_32: u32 = 1500;
    /// Entries in the key table, as an address-space length.
    const KEY_COUNT_32: u32 = 4096;
    const _: () =
        assert!(ENTRY_COUNT_32 as usize == ENTRY_COUNT && KEY_COUNT_32 as usize == KEY_COUNT);
    const _: () = assert!(
        ENTRY_STRIDE as usize == EntryLayout::SIZE && KEY_STRIDE as usize == KeyEntryLayout::SIZE
    );

    /// Every byte range the state models, as `(address, length)`. A
    /// differential test compares exactly these ranges.
    pub const REGIONS: [(u32, u32); 12] = [
        (TICK_SCALE, 4),
        (USED, ENTRY_COUNT_32),
        (CLOCK_BASE, 4),
        (CLOCK_MARK, 4),
        (CLOCK_TICKS, 4),
        (PENDING_SCALE, 4),
        (STAMP_BASE, 4),
        (SERIAL_A, 2),
        (CURSOR, 2),
        (SERIAL_B, 2),
        (ENTRIES, ENTRY_COUNT_32 * ENTRY_STRIDE),
        (KEYS, KEY_COUNT_32 * KEY_STRIDE),
    ];

    /// Address of entry `index` (no range check: the original's form).
    #[must_use]
    pub const fn entry_addr(index: u16) -> u32 {
        lf_core::boundary::element_addr(ENTRIES, index as u32, ENTRY_STRIDE)
    }

    /// One slot-table entry as the original stores it.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct EntryLayout {
        /// Value word.
        pub value: u32,
        /// Flag byte.
        pub flags: u8,
        /// Three bytes this cluster never touches.
        pub rest: [u8; 3],
    }

    assert_size!(EntryLayout, 8);
    assert_offset!(EntryLayout, value, 0);
    assert_offset!(EntryLayout, flags, 4);
    assert_offset!(EntryLayout, rest, 5);
    assert_fixed_layout!(EntryLayout);

    impl FixedLayout for EntryLayout {
        const SIZE: usize = 8;
        fn decode(bytes: &[u8]) -> Self {
            Self {
                value: u32::decode(bytes),
                flags: u8::decode(&bytes[4..]),
                rest: <[u8; 3]>::decode(&bytes[5..]),
            }
        }
        fn encode(&self, out: &mut [u8]) {
            self.value.encode(out);
            self.flags.encode(&mut out[4..]);
            self.rest.encode(&mut out[5..]);
        }
    }

    impl FromLayout<EntryLayout> for Entry {
        type Context = ();
        fn from_layout(layout: &EntryLayout, (): &()) -> Result<Self, BoundaryError> {
            Ok(Self {
                value: layout.value,
                flags: layout.flags,
            })
        }
    }

    impl IntoLayout<EntryLayout> for Entry {
        type Context = ();
        fn write_layout(&self, out: &mut EntryLayout, (): &()) -> Result<(), BoundaryError> {
            out.value = self.value;
            out.flags = self.flags;
            Ok(())
        }
    }

    /// One key-table entry as the original stores it.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct KeyEntryLayout {
        /// Key word; all ones marks an empty entry.
        pub key: u32,
        /// Value word.
        pub value: u32,
    }

    assert_size!(KeyEntryLayout, 8);
    assert_offset!(KeyEntryLayout, value, 4);
    assert_fixed_layout!(KeyEntryLayout);

    /// The key word of an empty key-table entry.
    pub const EMPTY_KEY: u32 = u32::MAX;

    impl FixedLayout for KeyEntryLayout {
        const SIZE: usize = 8;
        fn decode(bytes: &[u8]) -> Self {
            Self {
                key: u32::decode(bytes),
                value: u32::decode(&bytes[4..]),
            }
        }
        fn encode(&self, out: &mut [u8]) {
            self.key.encode(out);
            self.value.encode(&mut out[4..]);
        }
    }

    impl FromLayout<KeyEntryLayout> for KeyEntry {
        type Context = ();
        fn from_layout(layout: &KeyEntryLayout, (): &()) -> Result<Self, BoundaryError> {
            Ok(Self {
                key: (layout.key != EMPTY_KEY).then_some(layout.key),
                value: layout.value,
            })
        }
    }

    impl IntoLayout<KeyEntryLayout> for KeyEntry {
        type Context = ();
        fn write_layout(&self, out: &mut KeyEntryLayout, (): &()) -> Result<(), BoundaryError> {
            out.key = self.key.unwrap_or(EMPTY_KEY);
            out.value = self.value;
            Ok(())
        }
    }

    /// Reads the whole state from an image.
    ///
    /// # Errors
    ///
    /// When any modelled byte is outside the image.
    pub fn load(image: &impl Image32) -> Result<SlotTableState, BoundaryError> {
        let mut st = SlotTableState::new(image.read::<f32>(TICK_SCALE)?);
        let mut used = [0u8; ENTRY_COUNT];
        image.read_bytes(USED, &mut used)?;
        for (slot, byte) in st.used.iter_mut().zip(used) {
            *slot = byte != 0;
        }
        let entries = image.read_array::<EntryLayout>(ENTRIES, ENTRY_COUNT)?;
        for (entry, raw) in st.entries.iter_mut().zip(&entries) {
            *entry = Entry::from_layout(raw, &())?;
        }
        let keys = image.read_array::<KeyEntryLayout>(KEYS, KEY_COUNT)?;
        for (key, raw) in st.keys.iter_mut().zip(&keys) {
            *key = KeyEntry::from_layout(raw, &())?;
        }
        st.cursor = image.read(CURSOR)?;
        st.serial_a = image.read(SERIAL_A)?;
        st.serial_b = image.read(SERIAL_B)?;
        st.clock_base = image.read(CLOCK_BASE)?;
        st.clock_mark = image.read(CLOCK_MARK)?;
        st.clock_ticks = image.read(CLOCK_TICKS)?;
        st.stamp_base = image.read(STAMP_BASE)?;
        st.pending_scale = image.read(PENDING_SCALE)?;
        Ok(st)
    }

    /// Writes the whole state into an image, touching only modelled bytes.
    ///
    /// A used entry stores as 1, a free one as 0.
    ///
    /// # Errors
    ///
    /// When any modelled byte is outside the image.
    pub fn store(st: &SlotTableState, image: &mut impl Image32) -> Result<(), BoundaryError> {
        image.write(TICK_SCALE, &st.tick_scale)?;
        let used: Vec<u8> = st.used.iter().map(|&u| u8::from(u)).collect();
        image.write_bytes(USED, &used)?;
        // Read the tables back first so unmodelled bytes are written back
        // as they were.
        let mut entries = image.read_array::<EntryLayout>(ENTRIES, ENTRY_COUNT)?;
        for (raw, entry) in entries.iter_mut().zip(st.entries.iter()) {
            entry.write_layout(raw, &())?;
        }
        image.write_array(ENTRIES, &entries)?;
        let mut keys = image.read_array::<KeyEntryLayout>(KEYS, KEY_COUNT)?;
        for (raw, key) in keys.iter_mut().zip(st.keys.iter()) {
            key.write_layout(raw, &())?;
        }
        image.write_array(KEYS, &keys)?;
        image.write(CURSOR, &st.cursor)?;
        image.write(SERIAL_A, &st.serial_a)?;
        image.write(SERIAL_B, &st.serial_b)?;
        image.write(CLOCK_BASE, &st.clock_base)?;
        image.write(CLOCK_MARK, &st.clock_mark)?;
        image.write(CLOCK_TICKS, &st.clock_ticks)?;
        image.write(STAMP_BASE, &st.stamp_base)?;
        image.write(PENDING_SCALE, &st.pending_scale)?;
        Ok(())
    }

    /// First address of the smallest span covering every region.
    #[must_use]
    pub fn span_start() -> u32 {
        REGIONS.iter().map(|r| r.0).min().unwrap_or(0)
    }

    /// One past the last address of that span.
    #[must_use]
    pub fn span_end() -> u32 {
        REGIONS.iter().map(|r| r.0 + r.1).max().unwrap_or(0)
    }
}

#[cfg(test)]
#[allow(clippy::cast_possible_truncation)] // test patterns truncate on purpose
mod tests {
    use super::layout::{self, EntryLayout};
    use super::*;
    use lf_core::boundary::{ByteImage, Image32};

    /// A fake that answers from fixed values and records what it saw.
    #[derive(Default)]
    struct Fake {
        serial: u16,
        stamp: u32,
        refreshed: Vec<u16>,
        stamps_asked: u32,
    }

    impl SlotTableOps for Fake {
        fn next_serial(&mut self, st: &mut SlotTableState) -> u16 {
            // The callee sees the entry already taken.
            assert!(st.used[usize::from(st.cursor) - 1]);
            self.serial
        }
        fn current_stamp(&mut self, _st: &mut SlotTableState) -> u32 {
            self.stamps_asked += 1;
            self.stamp
        }
        fn refresh_entry(&mut self, st: &mut SlotTableState, index: u16) {
            self.refreshed.push(index);
            // A refresh may change the value; the caller must read after.
            st.entries[usize::from(index)].value = 0xFEED;
        }
    }

    #[test]
    fn serial_counters_wrap_before_0x7fff() {
        let mut st = SlotTableState::new(1.0);
        st.serial_a = 0x7FFD;
        assert_eq!(bump_serial_a(&mut st), 0x7FFE);
        assert_eq!(bump_serial_a(&mut st), 0);
        assert_eq!(st.serial_a, 0);
        st.serial_b = 0xFFFF;
        assert_eq!(bump_serial_b(&mut st), 0, "wraps through zero");
        assert_eq!(bump_serial_b(&mut st), 1);
    }

    #[test]
    fn allocation_scans_from_cursor_then_wraps() {
        let mut st = SlotTableState::new(1.0);
        let mut ops = Fake {
            serial: 0xABCD,
            ..Fake::default()
        };
        st.used.iter_mut().for_each(|u| *u = true);
        st.used[3] = false;
        st.used[1400] = false;
        st.cursor = 1000;
        let t = allocate(&mut st, &mut ops).unwrap();
        assert_eq!(
            t,
            Ticket {
                index: 1400,
                serial: 0xABCD
            }
        );
        assert_eq!(t.pack(), 0xABCD_0578);
        assert_eq!(st.cursor, 1401);
        let t = allocate(&mut st, &mut ops).unwrap();
        assert_eq!(t.index, 3);
        assert_eq!(st.cursor, 4);
        assert_eq!(allocate(&mut st, &mut ops), None);
        assert_eq!(st.cursor, 4, "a failed allocation changes nothing");
        st.cursor = 1500;
        st.used[0] = false;
        assert_eq!(allocate(&mut st, &mut ops).unwrap().index, 0);
    }

    #[test]
    #[should_panic(expected = "outside the lifted domain")]
    fn allocation_past_the_table_panics() {
        let mut st = SlotTableState::new(1.0);
        st.cursor = 1501;
        let _ = allocate(&mut st, &mut Fake::default());
    }

    #[test]
    fn entry_reads_refresh_unused_entries_first() {
        let mut st = SlotTableState::new(1.0);
        let mut ops = Fake::default();
        st.entries[5] = Entry { value: 9, flags: 0 };
        st.entries[6] = Entry { value: 9, flags: 4 };
        st.entries[7] = Entry { value: 9, flags: 1 };
        st.used[7] = true;
        assert_eq!(entry_value(&mut st, &mut ops, 5), 0);
        assert_eq!(entry_value(&mut st, &mut ops, 6), 0xFEED);
        assert_eq!(entry_value(&mut st, &mut ops, 7), 9);
        assert_eq!(entry_value(&mut st, &mut ops, 1500), 0);
        assert_eq!(ops.refreshed, [6]);
        assert_eq!(entry_row(1499), Some(1499));
        assert_eq!(entry_row(1500), None);
        assert_eq!(layout::entry_addr(2), layout::ENTRIES + 16);
    }

    #[test]
    fn clock_and_scale_values() {
        let mut st = SlotTableState::new(0.5);
        st.clock_base = 10;
        st.clock_mark = 4;
        st.clock_ticks = 7;
        assert_eq!(clock_span(&st), u32::MAX - 5);
        assert_eq!(stamp_sum(&st), 17);
        assert_eq!(scaled_ticks(&st).to_bits(), 3.5f64.to_bits());
        st.clock_mark = 2010;
        assert_eq!(
            clock_span_seconds(&st).to_bits(),
            (2000.0f32 * 0.001).to_bits()
        );
        assert_eq!(SECONDS_PER_TICK.to_bits(), 0x3A83_126F);
        st.pending_scale = 1.25;
        assert_eq!(take_pending_scale(&mut st).to_bits(), 1.25f64.to_bits());
        assert_eq!(st.pending_scale.to_bits(), 0);
    }

    #[test]
    fn key_lookup_hits_or_falls_back_to_stamped_base() {
        let mut st = SlotTableState::new(1.0);
        st.keys[10] = KeyEntry {
            key: Some(42),
            value: 7,
        };
        st.keys[11] = KeyEntry {
            key: Some(42),
            value: 8,
        };
        st.stamp_base = 100;
        st.clock_mark = 5;
        let mut ops = Fake {
            stamp: 5,
            ..Fake::default()
        };
        assert_eq!(key_value(&mut st, &mut ops, 42), 7, "first match wins");
        assert_eq!(ops.stamps_asked, 0);
        assert_eq!(key_value(&mut st, &mut ops, 43), 101);
        ops.stamp = 6;
        assert_eq!(
            key_value(&mut st, &mut ops, u32::MAX),
            100,
            "empty keys never match"
        );
    }

    #[test]
    fn layout_round_trip_preserves_unmodelled_bytes() {
        let start = layout::span_start();
        let len = (layout::span_end() - start) as usize;
        let mut image = ByteImage::zeroed(start, len);
        // Fill everything with a pattern, as unknown memory would be.
        for (i, b) in image.bytes.iter_mut().enumerate() {
            *b = (i * 7 + 3) as u8;
        }
        let before = image.clone();
        let mut st = layout::load(&image).unwrap();
        layout::store(&st, &mut image).unwrap();
        // Used bytes normalise to 0/1; everything else is unchanged.
        for (i, (a, b)) in before.bytes.iter().zip(&image.bytes).enumerate() {
            let addr = start + i as u32;
            if (layout::USED..layout::USED + ENTRY_COUNT as u32).contains(&addr) {
                assert_eq!(*b, u8::from(*a != 0));
            } else {
                assert_eq!(a, b, "byte at {addr:#x}");
            }
        }
        // Change entries and store: only value and flag bytes move.
        clear_entries(&mut st);
        layout::store(&st, &mut image).unwrap();
        let e: EntryLayout = image.read(layout::entry_addr(3)).unwrap();
        let old: EntryLayout = before.read(layout::entry_addr(3)).unwrap();
        assert_eq!((e.value, e.flags), (0, 0));
        assert_eq!(e.rest, old.rest);
        assert_eq!(layout::load(&image).unwrap(), st);
    }
}
