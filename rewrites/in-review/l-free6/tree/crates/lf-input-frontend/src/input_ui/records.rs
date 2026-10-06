//! The input-ui state record: a 620-byte record copied field by field.
//!
//! [`UiStateRecord::copy_from`] restates `input_ui_state_copy`: plain word
//! copies cover `[0x00..0x60)`, `[0xA0..0x1A0)` and the six tail words at
//! `0x250..0x26C` except the padding hole at `0x25C`, while the
//! `[0x60..0xA0)` and `[0x1A0..0x250)` sub-objects are copied by two
//! intercepted callees, here the [`SubRecord`] trait. The copy order is
//! part of the contract: head words, head sub-object, middle words,
//! tail sub-object, tail words.
//!
//! Evidence: `fn_009284d0.rs` + `r-b232/contracts/fn_009284d0.json`
//! (contract file name per the lane's folder layout).

/// Words in `[0x00..0x60)`.
pub const HEAD_WORDS: usize = 24;
/// Words in `[0xA0..0x1A0)`.
pub const MID_WORDS: usize = 64;
/// Tail words copied (`0x250`, `0x254`, `0x258`, `0x260`, `0x264`,
/// `0x268`); the `0x25C` hole is never touched.
pub const TAIL_WORDS: usize = 6;

/// A delegated sub-object of the state record.
pub trait SubRecord {
    /// Copies `src` into `self`, as one intercepted call.
    fn copy_from_source(&mut self, src: &Self);
}

/// A 620-byte input-ui state record with two delegated sub-objects.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UiStateRecord<H: SubRecord, T: SubRecord> {
    /// Words `[0x00..0x60)`.
    pub head: [u32; HEAD_WORDS],
    /// Sub-object `[0x60..0xA0)`, copied by the first call.
    pub sub_head: H,
    /// Words `[0xA0..0x1A0)`.
    pub mid: [u32; MID_WORDS],
    /// Sub-object `[0x1A0..0x250)`, copied by the second call.
    pub sub_tail: T,
    /// Tail words at `0x250..0x26C` except the `0x25C` hole.
    pub tail: [u32; TAIL_WORDS],
}

impl<H: SubRecord, T: SubRecord> UiStateRecord<H, T> {
    /// Copies `src` into `self`, delegating the two sub-objects in
    /// order. The 32-bit form answers the destination address; the
    /// lift is a method on the destination, so there is nothing to
    /// answer (the differential test pins the address).
    pub fn copy_from(&mut self, src: &Self) {
        self.head = src.head;
        self.sub_head.copy_from_source(&src.sub_head);
        self.mid = src.mid;
        self.sub_tail.copy_from_source(&src.sub_tail);
        self.tail = src.tail;
    }
}
