//! The streaming device: entry resolution and per-kind table answers.
//!
//! Lifted from the verified rewrites of `fiStreamingDevice`. The 32-bit
//! object carries a base key added to every lookup delta, an element table
//! whose kind bytes select channels, and a flag that decides whether a
//! span post first closes the channel's object. Every lookup resolves an
//! entry through the resolver role, then reads that entry's words (its
//! first word, kind byte, size word and flag word) and the per-kind table
//! rows the globals hold.

use lf_core::Handle32;

/// Tag for the opaque cookie of a channel object.
pub struct ChannelTag;

/// One resolved streaming entry: the words the device's methods read off
/// the entry the resolver hands back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamEntry {
    /// The entry's byte offset from the entry table's base. The slot
    /// number is this divided by the 24-byte entry stride.
    pub offset: i32,
    /// The entry's first word.
    pub first: u32,
    /// The entry's kind byte, selecting a per-kind table row.
    pub kind: u8,
    /// The entry's size word.
    pub size: u32,
    /// The entry's flag word.
    pub flags: u16,
}

/// One channel row of the per-kind tables: the 64-bit cursor, the current
/// object, and the sink argument posted with every span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamChannel {
    /// Low word of the channel cursor.
    pub cursor_lo: u32,
    /// High word of the channel cursor.
    pub cursor_hi: u32,
    /// The channel's current object.
    pub object: Option<Handle32<ChannelTag>>,
    /// The sink argument posted with every span.
    pub sink_arg: u32,
}

/// The per-kind global tables the device reads, one row per kind byte.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StreamTables {
    /// The word pair selected by kind.
    pub spans: Vec<(u32, u32)>,
    /// The slot word selected by kind (single-word query).
    pub slots_v1: Vec<u32>,
    /// The slot word selected by kind (span query).
    pub slots_v2: Vec<u32>,
    /// The channel row selected by kind.
    pub channels: Vec<StreamChannel>,
}

/// What the streaming device needs from the engine around it: the entry
/// resolver, the entry's own shift and data roles, and the channel
/// object's close and span-sink roles.
///
/// Every method is one callee or virtual-slot role from the verified
/// rewrites, with entry addresses narrowed to [`StreamEntry`] values and
/// channel objects narrowed to opaque cookies.
pub trait StreamWorld {
    /// Resolves the entry for a lookup key; `None` when unresolvable.
    fn resolve(&mut self, key: u32) -> Option<StreamEntry>;
    /// The entry's shifted role; answers its answer.
    fn shifted(&mut self, entry: &StreamEntry) -> u32;
    /// The entry's data word.
    fn data_word(&mut self, entry: &StreamEntry) -> u32;
    /// Closes a channel object.
    fn close_channel_object(&mut self, object: Option<Handle32<ChannelTag>>);
    /// Posts a span to a channel object's sink; answers the sink's answer.
    fn post_span(
        &mut self,
        object: Option<Handle32<ChannelTag>>,
        sink_arg: u32,
        lo: u32,
        hi: u32,
        a3: u32,
        a4: u32,
    ) -> u32;
}

/// A data span answer: the entry data's byte offset and its slot number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataSpan {
    /// The data word minus the kind slot, scaled to a byte offset.
    pub offset: u32,
    /// The entry's slot number, truncated to 16 bits.
    pub slot: u32,
}

/// A streaming device, owning its base key, its element kinds and its
/// close flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamDevice {
    /// The base key added to every lookup delta.
    pub base_key: u32,
    /// The kind byte of each element, indexed by element number.
    pub elem_kinds: Vec<u8>,
    /// Whether a span post first closes the channel's object.
    pub close_first: bool,
}

/// Bits of the size word that must be clear for an entry to count empty.
const SIZE_MASK: u32 = 0xFFFF_FFFC;
/// Flag bit set while an entry is present.
const PRESENT_BIT: u32 = 11;
/// Flag bit gating the first-word query.
const GATE_BIT: u32 = 13;
/// Bit shift scaling a data difference to a byte offset.
const OFFSET_SHIFT: u32 = 11;

impl StreamDevice {
    /// The lookup key for a delta: the base key plus the delta, wrapping.
    fn key(&self, delta: u32) -> u32 {
        self.base_key.wrapping_add(delta)
    }

    /// True when the entry at `delta` is active: resolvable and either its
    /// size word reads above the low two bits or its present bit is set.
    pub fn is_active<W: StreamWorld>(&self, world: &mut W, delta: u32) -> bool {
        let Some(entry) = world.resolve(self.key(delta)) else {
            return false;
        };
        entry.size & SIZE_MASK != 0 || (entry.flags as u32 >> PRESENT_BIT) & 1 != 0
    }

    /// The shifted role's answer for the entry at `delta`, or zero when
    /// the entry does not resolve.
    pub fn quarter_size<W: StreamWorld>(&self, world: &mut W, delta: u32) -> u32 {
        let Some(entry) = world.resolve(self.key(delta)) else {
            return 0;
        };
        world.shifted(&entry)
    }

    /// The word pair the entry at `delta`'s kind byte selects.
    ///
    /// # Panics
    ///
    /// When the entry does not resolve (the original faults there) or the
    /// kind selects past the tables.
    pub fn span_pair<W: StreamWorld>(
        &self,
        world: &mut W,
        tables: &StreamTables,
        delta: u32,
    ) -> (u32, u32) {
        let entry = world
            .resolve(self.key(delta))
            .expect("span pair of an unresolvable entry");
        tables.spans[entry.kind as usize]
    }

    /// The first word of the gated entry at `delta`: `None` when the entry
    /// does not resolve or its gate bit is clear.
    pub fn gated_first_word<W: StreamWorld>(&self, world: &mut W, delta: u32) -> Option<u32> {
        let entry = world.resolve(self.key(delta))?;
        if (entry.flags as u32 >> GATE_BIT) & 1 == 0 {
            return None;
        }
        Some(entry.first)
    }

    /// The slot word the entry at `delta`'s kind byte selects.
    ///
    /// # Panics
    ///
    /// When the entry does not resolve (the original faults there) or the
    /// kind selects past the tables.
    pub fn slot_word<W: StreamWorld>(
        &self,
        world: &mut W,
        tables: &StreamTables,
        delta: u32,
    ) -> u32 {
        let entry = world
            .resolve(self.key(delta))
            .expect("slot word of an unresolvable entry");
        tables.slots_v1[entry.kind as usize]
    }

    /// Signed division by the 24-byte entry stride, as the original's
    /// magic multiply sequence computes it.
    fn div_stride(offset: i32) -> i32 {
        let hi = ((i64::from(offset).wrapping_mul(0x2AAAAAAB)) >> 32) as i32;
        let shifted = hi >> 2;
        shifted.wrapping_add(((shifted as u32) >> 31) as i32)
    }

    /// The data span of the entry at `delta`: `None` when the entry does
    /// not resolve, else the data word minus the kind slot scaled to a
    /// byte offset, with the entry's slot number.
    pub fn data_span<W: StreamWorld>(
        &self,
        world: &mut W,
        tables: &StreamTables,
        delta: u32,
    ) -> Option<DataSpan> {
        let entry = world.resolve(self.key(delta))?;
        let data = world.data_word(&entry);
        let slot = tables.slots_v2[entry.kind as usize];
        let offset = data.wrapping_sub(slot).wrapping_shl(OFFSET_SHIFT);
        let slot_no = Self::div_stride(entry.offset) as u16 as u32;
        Some(DataSpan {
            offset,
            slot: slot_no,
        })
    }

    /// Closes the kind channel selected by element `a0` when the close
    /// flag is set, then posts the channel cursor plus (`a1`, `a2`) with
    /// (`a3`, `a4`) to the channel's span sink; answers the sink's answer.
    ///
    /// # Panics
    ///
    /// When the element or the kind selects past the owned tables.
    #[allow(clippy::too_many_arguments)]
    pub fn post_span_for<W: StreamWorld>(
        &self,
        world: &mut W,
        tables: &StreamTables,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
    ) -> u32 {
        let kind = self.elem_kinds[a0 as usize];
        let channel = &tables.channels[kind as usize];
        if self.close_first {
            world.close_channel_object(channel.object);
        }
        let lo = channel.cursor_lo.wrapping_add(a1);
        let carry = u32::from(lo < a1);
        let hi = channel.cursor_hi.wrapping_add(a2).wrapping_add(carry);
        world.post_span(channel.object, channel.sink_arg, lo, hi, a3, a4)
    }
}
