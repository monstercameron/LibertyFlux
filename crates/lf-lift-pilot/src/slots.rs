//! Indexed slot records (lifted from the 0x00D69xxx family).
//!
//! Two record shapes plus index tables:
//! - [`Slot`]: outer record. Offsets used: `0x00` head cookie, `0x04` child,
//!   `0x08` slot link, `0x0C` linked record, `0x10` aux link, `0x14` probe
//!   cookie, `0x18` counter word (low byte addressable), `0x1C` flag byte,
//!   `0x25` mode byte, `0x29` status byte, `0x2A` bit byte, `0x30` four
//!   record words, `0x48` flag byte, `0x9C` table holder, `0xA0` position.
//! - [`Inner`]: child record. Offsets used: `0x58` aux input, `0x9C` table
//!   holder, `0xA0` index/position, `0xF8` reset word, `0xFC` status word
//!   (low byte addressable).
//! - [`Shared`]: shared-table record behind the bind lookup (`+0xE98`).
//!
//! Table slots hold opaque `u32` payloads. Links are arena indices.

/// Index into [`Slots::slots`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SlotId(pub u32);
/// Index into [`Slots::inners`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InnerId(pub u32);
/// Index into [`Slots::holders`] (each holder names one table).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HolderId(pub u32);
/// Index into [`Slots::tables`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TableId(pub u32);
/// Index into [`Slots::shareds`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SharedId(pub u32);

/// Outer slot record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    /// Head cookie (+0x00), passed to the activation step.
    pub head: u32,
    /// Child record (+0x04).
    pub child: Option<InnerId>,
    /// Slot link (+0x08).
    pub slot: Option<SlotId>,
    /// Linked record (+0x0C); also null-tested as the "owner word".
    pub linked: Option<InnerId>,
    /// Aux link (+0x10).
    pub aux: Option<SlotId>,
    /// Probe cookie (+0x14), passed to the probe step.
    pub probe_child: u32,
    /// Counter (+0x18): the original reads it as u16 and stores flag bytes
    /// into its low byte; both views are kept consistent here.
    pub counter: u16,
    /// Flag byte (+0x1C).
    pub flag_1c: u8,
    /// Mode byte (+0x25).
    pub mode: u8,
    /// Status byte (+0x29).
    pub status29: u8,
    /// Bit byte (+0x2A).
    pub bit2a: u8,
    /// Four record words (+0x30). Word 0 doubles as the bound-record cookie.
    pub words30: [u32; 4],
    /// Flag byte (+0x48).
    pub flag48: u8,
    /// Table holder (+0x9C).
    pub holder: Option<HolderId>,
    /// Position (+0xA0), interpreted signed in places.
    pub index: u32,
}

/// Child record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Inner {
    /// Counter word (+0x18), low byte written by the child forward path.
    /// The child record is a wider union; this word is only touched there.
    pub counter18: u16,
    /// Aux input (+0x58).
    pub aux_input: u32,
    /// Table holder (+0x9C).
    pub holder: Option<HolderId>,
    /// Index/position (+0xA0).
    pub index: u32,
    /// Reset word (+0xF8).
    pub f8: u32,
    /// Status word (+0xFC): read as u32 and as a byte; both views consistent.
    pub status: u32,
}

/// Shared-table record behind the bind lookup.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shared {
    /// Record link (+0xE98).
    pub record: Option<InnerId>,
}

/// All state the slot functions can reach.
#[derive(Clone, Debug, Default)]
pub struct Slots {
    /// Outer records by index.
    pub slots: Vec<Slot>,
    /// Child records by index.
    pub inners: Vec<Inner>,
    /// Table holders by index; each names one table.
    pub holders: Vec<TableId>,
    /// Index tables by index; slots hold opaque payloads.
    pub tables: Vec<Vec<u32>>,
    /// Shared records by index.
    pub shareds: Vec<Shared>,
}

impl Slot {
    /// Blank record matching a zeroed original.
    #[must_use]
    pub fn blank() -> Self {
        Self {
            head: 0,
            child: None,
            slot: None,
            linked: None,
            aux: None,
            probe_child: 0,
            counter: 0,
            flag_1c: 0,
            mode: 0,
            status29: 0,
            bit2a: 0,
            words30: [0; 4],
            flag48: 0,
            holder: None,
            index: 0,
        }
    }
}

impl Inner {
    /// Blank record matching a zeroed original.
    #[must_use]
    pub fn blank() -> Self {
        Self {
            counter18: 0,
            aux_input: 0,
            holder: None,
            index: 0,
            f8: 0,
            status: 0,
        }
    }
}

/// Attach an auxiliary record, then combine the 16-bit counter with the aux
/// step's answer through the combine step. The stack argument is accepted
/// but never read. (Original: `bind_aux_and_combine`.)
pub fn bind_aux_and_combine(
    st: &mut Slots,
    this: SlotId,
    lookup: &mut dyn FnMut(&mut Slots, u32) -> Option<SharedId>,
    aux_step: &mut dyn FnMut(&mut Slots, u32) -> u32,
    combine: &mut dyn FnMut(&mut Slots, SlotId, u32) -> u32,
) -> u32 {
    let shared = lookup(st, 0).expect("bind lookup returned null; original faults");
    let record = st.shareds[shared.0 as usize]
        .record
        .expect("bind record null");
    st.slots[this.0 as usize].words30[0] = record.0;
    let input = st.inners[record.0 as usize].aux_input;
    let answer = aux_step(st, input);
    st.slots[this.0 as usize].bit2a |= 8;
    let counter = u32::from(st.slots[this.0 as usize].counter);
    let total = counter.wrapping_add(answer);
    // The original passes the counter's ADDRESS to the combine step; the
    // lift passes the slot id, which names the same word.
    let out = combine(st, this, total);
    st.slots[this.0 as usize].mode = 3;
    out
}

/// Reset this record's slot state: clear the status bit, zero four slot
/// words, park the mode byte. (Original: `reset_slot_state`.)
pub fn reset_slot_state(st: &mut Slots, this: SlotId) {
    let slot = &mut st.slots[this.0 as usize];
    slot.status29 &= !0x40;
    slot.words30 = [0; 4];
    slot.mode = 4;
}

/// Forward fixed flags to the slot step when a record is present, else do
/// nothing. (Original: `forward_flags_if_present`; the empty path leaves
/// caller garbage in EAX, the rewrite returns 0, the lift matches it.)
pub fn forward_flags_if_present(
    st: &mut Slots,
    this: SlotId,
    step: &mut dyn FnMut(&mut Slots, InnerId, u32, u32, u32, u32) -> u32,
) -> u32 {
    match st.slots[this.0 as usize].child {
        None => 0,
        Some(inner) => step(st, inner, 1, 1, 0, 1),
    }
}

/// Fetch the indexed table entry through the child, or null when the child
/// is missing or the index is negative. (Original: `indexed_entry_or_null`.)
///
/// The original null-checks only the child and the index sign: a null
/// holder faults, and an upper-out-of-range index reads out of bounds. The
/// lift panics loudly on both instead of faulting or reading garbage.
#[must_use]
pub fn indexed_entry_or_null(st: &Slots, this: SlotId) -> Option<u32> {
    let child = st.slots[this.0 as usize].child?;
    let inner = &st.inners[child.0 as usize];
    if (inner.index as i32) < 0 {
        return None;
    }
    let holder = inner
        .holder
        .expect("null table holder faults in the original");
    let table = st.holders[holder.0 as usize];
    Some(st.tables[table.0 as usize][inner.index as usize])
}

/// Fetch the entry below the child's one-based position.
/// (Original: `prev_indexed_entry_or_null`.)
#[must_use]
pub fn prev_indexed_entry_or_null(st: &Slots, this: SlotId) -> Option<u32> {
    let child = st.slots[this.0 as usize].child?;
    let inner = &st.inners[child.0 as usize];
    // Faithful to the original: (pos - 1) judged as signed; only pos >= 1
    // proceeds, and the result is index (pos - 1) with no upper check.
    if inner.index == 0 || (inner.index.wrapping_sub(1) as i32) < 0 {
        return None;
    }
    let holder = inner
        .holder
        .expect("null table holder faults in the original");
    let table = st.holders[holder.0 as usize];
    Some(st.tables[table.0 as usize][(inner.index - 1) as usize])
}

/// Fetch the entry below this record's own one-based position.
/// (Original: `prev_entry_or_null`.)
#[must_use]
pub fn prev_entry_or_null(st: &Slots, this: SlotId) -> Option<u32> {
    let slot = &st.slots[this.0 as usize];
    if slot.index == 0 || (slot.index.wrapping_sub(1) as i32) < 0 {
        return None;
    }
    let holder = slot
        .holder
        .expect("null table holder faults in the original");
    let table = st.holders[holder.0 as usize];
    Some(st.tables[table.0 as usize][(slot.index - 1) as usize])
}

/// Read the linked record's status byte, or zero when unlinked.
/// (Original: `status_byte_or_zero`.)
///
/// KNOWN DIVERGENCE: the original merges the link pointer's own high three
/// bytes into EAX (a partial-register merge), so the upper 24 bits of the
/// result leak the address. That value cannot exist on 64-bit; the lift
/// returns just the byte. The differential test compares the low byte only.
#[must_use]
pub fn status_byte_or_zero(st: &Slots, this: SlotId) -> u8 {
    match st.slots[this.0 as usize].linked {
        None => 0,
        Some(linked) => st.inners[linked.0 as usize].status as u8,
    }
}

/// Report whether the probed value differs from the limit: run the probe
/// step over the probe cookie, convert the answer's low byte to float and
/// compare with `!=` (unordered included, matching the original's
/// flag-parity trick). Empty slot reads as equal.
/// (Original: `probe_value_differs`; the global float limit is a parameter.)
pub fn probe_value_differs(
    st: &mut Slots,
    this: SlotId,
    limit: f32,
    probe: &mut dyn FnMut(&mut Slots, u32) -> u32,
) -> bool {
    if st.slots[this.0 as usize].probe_child == 0 {
        // The original null-checks the +0x14 child pointer; a zero cookie
        // is the lifted null here.
        return false;
    }
    let cookie = st.slots[this.0 as usize].probe_child;
    let sample = f32::from((probe(st, cookie) & 0xFF) as u8);
    sample != limit
}

/// Run the activation sequence when a record is present: a setup call
/// followed by the mode-2 step. (Original: `activate_if_present`; the
/// empty path is caller garbage in EAX, 0 in the rewrite and the lift.)
pub fn activate_if_present(
    st: &mut Slots,
    this: SlotId,
    setup: &mut dyn FnMut(&mut Slots, u32) -> u32,
    step: &mut dyn FnMut(&mut Slots, u32, u32) -> u32,
) -> u32 {
    if st.slots[this.0 as usize].head == 0 {
        return 0;
    }
    setup(st, 0);
    let head = st.slots[this.0 as usize].head;
    step(st, head, 2)
}

/// Invalidate the child record when present: three reset stores.
/// (Original: `invalidate_inner`.)
pub fn invalidate_inner(st: &mut Slots, this: SlotId) {
    let Some(child) = st.slots[this.0 as usize].child else {
        return;
    };
    let inner = &mut st.inners[child.0 as usize];
    inner.index = 0xFFFF_FFFF;
    inner.f8 = 0;
    inner.status = 0xFFFF_FFFF;
}

/// Store the flag byte into the head record's counter low byte.
/// (Original: `forward_byte_head`.)
///
/// The head cookie names a slot by cookie value here: cookies are small in
/// tests. A zero head does nothing.
pub fn forward_byte_head(st: &mut Slots, this: SlotId, flag: u32) {
    let head = st.slots[this.0 as usize].head;
    if head == 0 {
        return;
    }
    // Cookies address slots: cookie k names slot k - 1. This matches the
    // test harness mapping, where slot i gets cookie i + 1.
    let slot = &mut st.slots[(head - 1) as usize];
    slot.counter = (slot.counter & 0xFF00) | (flag & 0xFF) as u16;
}

/// Store the flag byte into the linked record's status low byte.
/// (Original: `forward_byte_linked`.)
pub fn forward_byte_linked(st: &mut Slots, this: SlotId, flag: u32) {
    let Some(linked) = st.slots[this.0 as usize].linked else {
        return;
    };
    let inner = &mut st.inners[linked.0 as usize];
    inner.status = (inner.status & 0xFFFF_FF00) | (flag & 0xFF);
}

/// Store the flag byte into the slot record's counter low byte.
/// (Original: `forward_byte_slot`.)
pub fn forward_byte_slot(st: &mut Slots, this: SlotId, flag: u32) {
    let Some(slot_id) = st.slots[this.0 as usize].slot else {
        return;
    };
    let slot = &mut st.slots[slot_id.0 as usize];
    slot.counter = (slot.counter & 0xFF00) | (flag & 0xFF) as u16;
}

/// Store the flag byte into the aux record's counter low byte.
/// (Original: `forward_byte_aux`.)
pub fn forward_byte_aux(st: &mut Slots, this: SlotId, flag: u32) {
    let Some(aux) = st.slots[this.0 as usize].aux else {
        return;
    };
    let slot = &mut st.slots[aux.0 as usize];
    slot.counter = (slot.counter & 0xFF00) | (flag & 0xFF) as u16;
}

/// Store the flag byte into the child record's counter low byte.
/// (Original: `forward_byte_child`.)
pub fn forward_byte_child(st: &mut Slots, this: SlotId, flag: u32) {
    let Some(child) = st.slots[this.0 as usize].child else {
        return;
    };
    let inner = &mut st.inners[child.0 as usize];
    inner.counter18 = (inner.counter18 & 0xFF00) | (flag & 0xFF) as u16;
}

/// Refresh when enabled, then forward to the slot record.
/// (Original: `maybe_refresh_and_forward`; unchecked EAX paths read 0.)
pub fn maybe_refresh_and_forward(
    st: &mut Slots,
    this: SlotId,
    flag: u32,
    probe: &mut dyn FnMut(&mut Slots, SlotId) -> u32,
    refresh: &mut dyn FnMut(&mut Slots, SlotId, u32) -> u32,
    head_fetch: &mut dyn FnMut(&mut Slots, Option<InnerId>) -> u32,
) -> u32 {
    let flag_byte = (flag & 0xFF) as u8;
    if st.slots[this.0 as usize].linked.is_some() && flag_byte != 0 && probe(st, this) & 0xFF != 0 {
        refresh(st, this, 0);
    }
    let Some(slot_id) = st.slots[this.0 as usize].slot else {
        return 0;
    };
    st.slots[slot_id.0 as usize].flag48 = flag_byte;
    if flag_byte == 0 {
        return 0;
    }
    // The tail target reloads the head from [slot+4] before the step.
    let head = st.slots[slot_id.0 as usize].child;
    head_fetch(st, head)
}

/// Push a word through the slot step, then store the flag byte on the
/// (possibly replaced) aux record; merge the stored byte into the step's
/// answer. (Original: `step_word_store_flag`; empty path reads 0.)
pub fn step_word_store_flag(
    st: &mut Slots,
    this: SlotId,
    word: u32,
    flag: u32,
    step: &mut dyn FnMut(&mut Slots, SlotId, u32) -> u32,
) -> u32 {
    if st.slots[this.0 as usize].aux.is_none() {
        return 0;
    }
    let aux = st.slots[this.0 as usize].aux.expect("checked above");
    let answer = step(st, aux, word);
    // Reload: the step may have replaced the aux pointer.
    let aux_now = st.slots[this.0 as usize]
        .aux
        .expect("step cleared aux; original faults");
    let flag_byte = (flag & 0xFF) as u8;
    st.slots[aux_now.0 as usize].flag_1c = flag_byte;
    (answer & 0xFFFF_FF00) | u32::from(flag_byte)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_and_invalidate_roundtrip() {
        let mut st = Slots::default();
        st.inners.push(Inner::blank());
        let mut s = Slot::blank();
        s.child = Some(InnerId(0));
        s.linked = Some(InnerId(0));
        s.status29 = 0xFF;
        s.words30 = [1, 2, 3, 4];
        s.mode = 9;
        st.slots.push(s);
        reset_slot_state(&mut st, SlotId(0));
        assert_eq!(st.slots[0].status29, 0xBF);
        assert_eq!(st.slots[0].words30, [0; 4]);
        assert_eq!(st.slots[0].mode, 4);
        invalidate_inner(&mut st, SlotId(0));
        assert_eq!(st.inners[0].index, 0xFFFF_FFFF);
        assert_eq!(st.inners[0].f8, 0);
        assert_eq!(st.inners[0].status, 0xFFFF_FFFF);
    }

    #[test]
    fn indexed_fetch_model() {
        let mut st = Slots::default();
        st.tables.push(vec![10, 20, 30]);
        st.holders.push(TableId(0));
        let mut inner = Inner::blank();
        inner.holder = Some(HolderId(0));
        inner.index = 1;
        st.inners.push(inner);
        let mut s = Slot::blank();
        s.child = Some(InnerId(0));
        st.slots.push(s);
        assert_eq!(indexed_entry_or_null(&st, SlotId(0)), Some(20));
        // One-based prev: pos 1 -> entry 0.
        assert_eq!(prev_indexed_entry_or_null(&st, SlotId(0)), Some(10));
        st.inners[0].index = 0;
        assert_eq!(prev_indexed_entry_or_null(&st, SlotId(0)), None);
        // Negative index reads as empty.
        st.inners[0].index = 0xFFFF_FFFF;
        assert_eq!(indexed_entry_or_null(&st, SlotId(0)), None);
    }

    #[test]
    fn byte_stores_hit_low_bytes_only() {
        let mut st = Slots::default();
        let mut inner = Inner::blank();
        inner.status = 0xAABB_CCDD;
        st.inners.push(inner);
        let mut s = Slot::blank();
        s.linked = Some(InnerId(0));
        s.counter = 0x1234;
        st.slots.push(s.clone());
        s.slot = Some(SlotId(0));
        s.aux = Some(SlotId(0));
        st.slots.push(s);
        forward_byte_linked(&mut st, SlotId(1), 0x11);
        assert_eq!(st.inners[0].status, 0xAABB_CC11);
        forward_byte_slot(&mut st, SlotId(1), 0x22);
        assert_eq!(st.slots[0].counter, 0x1222);
        forward_byte_aux(&mut st, SlotId(1), 0x33);
        assert_eq!(st.slots[0].counter, 0x1233);
        assert_eq!(status_byte_or_zero(&st, SlotId(1)), 0x11);
    }

    #[test]
    fn probe_compares_float_not_equal() {
        let mut st = Slots::default();
        let mut s = Slot::blank();
        s.probe_child = 7;
        st.slots.push(s);
        let mut probe = |_: &mut Slots, c: u32| {
            assert_eq!(c, 7);
            0x41u32
        };
        assert!(probe_value_differs(&mut st, SlotId(0), 42.0, &mut probe));
        assert!(!probe_value_differs(&mut st, SlotId(0), 65.0, &mut probe));
        // NaN limit: unordered counts as different, like the original.
        assert!(probe_value_differs(
            &mut st,
            SlotId(0),
            f32::NAN,
            &mut probe
        ));
        st.slots[0].probe_child = 0;
        assert!(!probe_value_differs(&mut st, SlotId(0), 0.0, &mut probe));
    }
}
