//! The layout frame: the byte-level state of the front end's interface
//! toolkit, and the methods that write it.
//!
//! The original frame is a 32-bit object reached through a virtual table.
//! Its fields are read and written at fixed byte offsets, little-endian,
//! so a word or float written here has the same bits the original would
//! hold. This module owns the bytes that the verified methods touch and
//! restates each of those methods on [`UILayoutFrame`], named for what it
//! does. Offsets are in [`field`].
//!
//! Only methods that call no other code are lifted here. Methods that call
//! out (token resolution, follow-up steps, shared notifiers, list release)
//! need a trait for their callees and are not in this module yet; see the
//! registry in [`super::registry`].
//!
//! Every method here is a plain store, a copy, or a branch on a field the
//! frame already holds. Nothing reads the operating system, and nothing in
//! this file depends on the pointer width of the target.

/// Bytes of the frame that the lifted methods read or write: everything up
/// to the end of the last field any of them touches. This is not the size
/// of the original object, which the lift does not model.
pub const SPAN: usize = 0xF0;

/// Field offsets within the frame, named for what the verified methods
/// store there.
pub mod field {
    /// Float stored by [`super::UILayoutFrame::set_float_4`].
    pub const FLOAT_4: usize = 0x04;
    /// Float stored by [`super::UILayoutFrame::set_float_8`].
    pub const FLOAT_8: usize = 0x08;
    /// First 24-byte block, the target of
    /// [`super::UILayoutFrame::copy_block_to_10`]; it spans 0x10 to 0x28.
    pub const BLOCK_10: usize = 0x10;
    /// Word stored by [`super::UILayoutFrame::set_slot_and_fraction`].
    pub const SLOT_14: usize = 0x14;
    /// Float bits stored by [`super::UILayoutFrame::set_slot_and_fraction`].
    pub const FRACTION_18: usize = 0x18;
    /// Word stored by [`super::UILayoutFrame::set_word_a0`].
    pub const WORD_A0: usize = 0xA0;
    /// Second 24-byte block, the target of
    /// [`super::UILayoutFrame::copy_block_to_a4`]; it spans 0xA4 to 0xBC.
    pub const BLOCK_A4: usize = 0xA4;
    /// Float stored by the two methods that write the 0xB0 float.
    pub const FLOAT_B0: usize = 0xB0;
    /// Tag word compared by [`super::UILayoutFrame::store_float_b0_with_tag`].
    pub const TAG_B4: usize = 0xB4;
    /// Word cleared when the tag changes or when the 0xB0 float is stored
    /// with its tag words cleared.
    pub const WORD_B8: usize = 0xB8;
    /// Flag byte stored by [`super::UILayoutFrame::set_flag_c4`].
    pub const FLAG_C4: usize = 0xC4;
    /// Flag byte stored by [`super::UILayoutFrame::set_flag_c7`].
    pub const FLAG_C7: usize = 0xC7;
    /// Flag byte stored by [`super::UILayoutFrame::set_flag_c8`].
    pub const FLAG_C8: usize = 0xC8;
    /// Flag byte stored by [`super::UILayoutFrame::set_flag_c9`].
    pub const FLAG_C9: usize = 0xC9;
    /// Flag byte stored by [`super::UILayoutFrame::set_flag_ca`].
    pub const FLAG_CA: usize = 0xCA;
    /// Flag byte stored by [`super::UILayoutFrame::set_flag_d0`].
    pub const FLAG_D0: usize = 0xD0;
    /// Flag byte stored by [`super::UILayoutFrame::set_flag_d1`].
    pub const FLAG_D1: usize = 0xD1;
    /// Word stored by [`super::UILayoutFrame::set_word_d4`].
    pub const WORD_D4: usize = 0xD4;
    /// Word stored by [`super::UILayoutFrame::set_word_d8`].
    pub const WORD_D8: usize = 0xD8;
    /// Word stored by [`super::UILayoutFrame::set_word_dc`].
    pub const WORD_DC: usize = 0xDC;
    /// Word stored by [`super::UILayoutFrame::set_word_e0`].
    pub const WORD_E0: usize = 0xE0;
    /// Word stored by [`super::UILayoutFrame::set_word_e4`].
    pub const WORD_E4: usize = 0xE4;
    /// Flag byte stored by [`super::UILayoutFrame::set_flag_e8`].
    pub const FLAG_E8: usize = 0xE8;
    /// Flag byte stored by [`super::UILayoutFrame::set_flag_e9`].
    pub const FLAG_E9: usize = 0xE9;
    /// Flag byte stored by [`super::UILayoutFrame::set_flag_ea`].
    pub const FLAG_EA: usize = 0xEA;
    /// Flag byte stored by [`super::UILayoutFrame::set_flag_eb`].
    pub const FLAG_EB: usize = 0xEB;
    /// Cache word filled once by [`super::UILayoutFrame::install_cache_if_empty`].
    pub const CACHE_EC: usize = 0xEC;
}

/// The layout frame: an owned block of the bytes the verified methods use.
///
/// The bytes are the object's own, not a view of game memory. Every method
/// reads and writes them through named offsets in [`field`]; no method
/// holds a pointer.
#[derive(Clone, PartialEq, Eq)]
pub struct UILayoutFrame {
    bytes: [u8; SPAN],
}

impl Default for UILayoutFrame {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for UILayoutFrame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UILayoutFrame")
            .field("span", &SPAN)
            .finish_non_exhaustive()
    }
}

impl UILayoutFrame {
    /// A frame with every byte of its span zero. This is a constructor for
    /// the owning type, not a restatement of a verified method.
    #[must_use]
    pub const fn new() -> Self {
        Self { bytes: [0; SPAN] }
    }

    /// A frame holding the given bytes, for tests and for callers that
    /// rebuild a frame from its saved state.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; SPAN]) -> Self {
        Self { bytes }
    }

    /// The frame's bytes, in the layout described in [`field`].
    #[must_use]
    pub const fn bytes(&self) -> &[u8; SPAN] {
        &self.bytes
    }

    /// The little-endian word at `at`. Every caller passes an offset from
    /// [`field`] with four bytes below [`SPAN`].
    pub(super) fn word_at(&self, at: usize) -> u32 {
        let mut word = [0u8; 4];
        word.copy_from_slice(&self.bytes[at..at + 4]);
        u32::from_le_bytes(word)
    }

    /// Stores a little-endian word at `at`.
    fn set_word_at(&mut self, at: usize, value: u32) {
        self.bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }

    /// The byte at t.
    pub(super) fn byte_at(&self, at: usize) -> u8 {
        self.bytes[at]
    }

    /// Stores the low byte of `value` at `at` and answers that byte as a
    /// word with its high bits clear.
    fn set_flag_at(&mut self, at: usize, value: u32) -> u32 {
        let byte = value.to_le_bytes()[0];
        self.bytes[at] = byte;
        u32::from(byte)
    }

    /// Stores the bit pattern of a float at `at`. Nothing is rounded or
    /// converted: the bits go in as they are, not-a-number payloads and
    /// signs included.
    fn set_float_at(&mut self, at: usize, value: f32) {
        self.set_word_at(at, value.to_bits());
    }

    /// Stores the low byte of `value` in the flag at 0xC4 and answers it.
    pub fn set_flag_c4(&mut self, value: u32) -> u32 {
        self.set_flag_at(field::FLAG_C4, value)
    }

    /// Stores the low byte of `value` in the flag at 0xC7 and answers it.
    pub fn set_flag_c7(&mut self, value: u32) -> u32 {
        self.set_flag_at(field::FLAG_C7, value)
    }

    /// Stores the low byte of `value` in the flag at 0xC8 and answers it.
    pub fn set_flag_c8(&mut self, value: u32) -> u32 {
        self.set_flag_at(field::FLAG_C8, value)
    }

    /// Stores the low byte of `value` in the flag at 0xCA and answers it.
    pub fn set_flag_ca(&mut self, value: u32) -> u32 {
        self.set_flag_at(field::FLAG_CA, value)
    }

    /// Stores the low byte of `value` in the flag at 0xD0 and answers it.
    pub fn set_flag_d0(&mut self, value: u32) -> u32 {
        self.set_flag_at(field::FLAG_D0, value)
    }

    /// Stores the low byte of `value` in the flag at 0xD1 and answers it.
    pub fn set_flag_d1(&mut self, value: u32) -> u32 {
        self.set_flag_at(field::FLAG_D1, value)
    }

    /// Stores the low byte of `value` in the flag at 0xEB and answers it.
    pub fn set_flag_eb(&mut self, value: u32) -> u32 {
        self.set_flag_at(field::FLAG_EB, value)
    }

    /// Stores the low byte of `value` in the flag at 0xC9.
    pub fn set_flag_c9(&mut self, value: u32) {
        self.set_flag_at(field::FLAG_C9, value);
    }

    /// Stores the low byte of `value` in the flag at 0xE8.
    pub fn set_flag_e8(&mut self, value: u32) {
        self.set_flag_at(field::FLAG_E8, value);
    }

    /// Stores the low byte of `value` in the flag at 0xE9.
    pub fn set_flag_e9(&mut self, value: u32) {
        self.set_flag_at(field::FLAG_E9, value);
    }

    /// Stores the low byte of `value` in the flag at 0xEA.
    pub fn set_flag_ea(&mut self, value: u32) {
        self.set_flag_at(field::FLAG_EA, value);
    }

    /// Stores `value` in the word at 0xA0 and answers it.
    pub fn set_word_a0(&mut self, value: u32) -> u32 {
        self.set_word_at(field::WORD_A0, value);
        value
    }

    /// Stores `value` in the word at 0xD8 and answers it.
    pub fn set_word_d8(&mut self, value: u32) -> u32 {
        self.set_word_at(field::WORD_D8, value);
        value
    }

    /// Stores `value` in the word at 0xE0 and answers it.
    pub fn set_word_e0(&mut self, value: u32) -> u32 {
        self.set_word_at(field::WORD_E0, value);
        value
    }

    /// Stores `value` in the word at 0xE4 and answers it.
    pub fn set_word_e4(&mut self, value: u32) -> u32 {
        self.set_word_at(field::WORD_E4, value);
        value
    }

    /// Stores `value` in the word at 0xD4.
    pub fn set_word_d4(&mut self, value: u32) {
        self.set_word_at(field::WORD_D4, value);
    }

    /// Stores `value` in the word at 0xDC.
    pub fn set_word_dc(&mut self, value: u32) {
        self.set_word_at(field::WORD_DC, value);
    }

    /// Stores the float `value` at 0x4, bit for bit.
    pub fn set_float_4(&mut self, value: f32) {
        self.set_float_at(field::FLOAT_4, value);
    }

    /// Stores the float `value` at 0x8, bit for bit.
    pub fn set_float_8(&mut self, value: f32) {
        self.set_float_at(field::FLOAT_8, value);
    }

    /// Stores the float `value` at 0xB0 and clears both tag words that
    /// follow it, at 0xB4 and 0xB8.
    pub fn store_float_b0_clear_tags(&mut self, value: f32) {
        self.set_float_at(field::FLOAT_B0, value);
        self.set_word_at(field::TAG_B4, 0);
        self.set_word_at(field::WORD_B8, 0);
    }

    /// Stores the float `value` at 0xB0. When the tag word at 0xB4 differs
    /// from `tag`, the tag is replaced and the word at 0xB8 is cleared; an
    /// equal tag leaves 0xB8 alone. Answers `tag`.
    pub fn store_float_b0_with_tag(&mut self, value: f32, tag: u32) -> u32 {
        self.set_float_at(field::FLOAT_B0, value);
        if self.word_at(field::TAG_B4) != tag {
            self.set_word_at(field::TAG_B4, tag);
            self.set_word_at(field::WORD_B8, 0);
        }
        tag
    }

    /// Stores `slot` at 0x14 and the float bits `fraction_bits` at 0x18.
    /// The fraction is moved as bits: it is never rounded or converted.
    pub fn set_slot_and_fraction(&mut self, slot: u32, fraction_bits: u32) {
        self.set_word_at(field::SLOT_14, slot);
        self.set_word_at(field::FRACTION_18, fraction_bits);
    }

    /// Stores `value` at 0xEC when that word is still zero. A non-zero
    /// cache word is left alone, so the first value installed stays.
    pub fn install_cache_if_empty(&mut self, value: u32) {
        if self.word_at(field::CACHE_EC) == 0 {
            self.set_word_at(field::CACHE_EC, value);
        }
    }

    /// Copies the 24 bytes of `block` into the frame at 0x10, over the
    /// words that follow it (0x10 to 0x28).
    pub fn copy_block_to_10(&mut self, block: &[u8; 24]) {
        self.bytes[field::BLOCK_10..field::BLOCK_10 + 24].copy_from_slice(block);
    }

    /// Copies the 24 bytes of `block` into the frame at 0xA4 (0xA4 to
    /// 0xBC), over the tag words that the 0xB0 methods use.
    pub fn copy_block_to_a4(&mut self, block: &[u8; 24]) {
        self.bytes[field::BLOCK_A4..field::BLOCK_A4 + 24].copy_from_slice(block);
    }
}

#[cfg(test)]
mod tests {
    use super::{SPAN, UILayoutFrame, field};

    /// A frame whose every byte is `fill`, so no write can hide behind zero.
    fn filled(fill: u8) -> UILayoutFrame {
        UILayoutFrame::from_bytes([fill; SPAN])
    }

    #[test]
    fn flag_setters_keep_the_low_byte_and_answer_it() {
        let mut frame = UILayoutFrame::new();
        assert_eq!(frame.set_flag_c8(0x1234_56AB), 0xAB);
        assert_eq!(frame.bytes()[field::FLAG_C8], 0xAB);
        assert_eq!(frame.bytes()[field::FLAG_C8 + 1], 0);
        assert_eq!(frame.set_flag_c4(0), 0);
        assert_eq!(frame.set_flag_c7(0xFFFF_FF00), 0);
    }

    #[test]
    fn void_flag_setters_write_only_their_byte() {
        let mut frame = filled(0x55);
        frame.set_flag_e8(0x100);
        frame.set_flag_e9(0x1FF);
        frame.set_flag_ea(0xFF_FF_FF_FF);
        frame.set_flag_c9(0x80);
        assert_eq!(frame.bytes()[field::FLAG_E8], 0);
        assert_eq!(frame.bytes()[field::FLAG_E9], 0xFF);
        assert_eq!(frame.bytes()[field::FLAG_EA], 0xFF);
        assert_eq!(frame.bytes()[field::FLAG_C9], 0x80);
        assert_eq!(frame.bytes()[field::FLAG_E8 + 3], 0x55);
    }

    #[test]
    fn word_setters_store_the_whole_value_and_answer_it() {
        let mut frame = filled(0);
        assert_eq!(frame.set_word_a0(0x8000_0001), 0x8000_0001);
        assert_eq!(frame.set_word_e4(u32::MAX), u32::MAX);
        assert_eq!(frame.word_at(field::WORD_A0), 0x8000_0001);
        assert_eq!(frame.word_at(field::WORD_E4), u32::MAX);
        frame.set_word_d4(7);
        frame.set_word_dc(9);
        assert_eq!(frame.word_at(field::WORD_D4), 7);
        assert_eq!(frame.word_at(field::WORD_DC), 9);
    }

    #[test]
    fn float_stores_keep_nan_payloads_and_signs() {
        let mut frame = UILayoutFrame::new();
        let quiet_nan = f32::from_bits(0xFFC0_1234);
        let negative_zero = -0.0_f32;
        frame.set_float_4(quiet_nan);
        frame.set_float_8(negative_zero);
        assert_eq!(frame.word_at(field::FLOAT_4), 0xFFC0_1234);
        assert_eq!(frame.word_at(field::FLOAT_8), 0x8000_0000);
    }

    #[test]
    fn clear_tags_store_zeroes_both_tag_words() {
        let mut frame = filled(0xFF);
        frame.store_float_b0_clear_tags(1.5);
        assert_eq!(frame.word_at(field::FLOAT_B0), 1.5_f32.to_bits());
        assert_eq!(frame.word_at(field::TAG_B4), 0);
        assert_eq!(frame.word_at(field::WORD_B8), 0);
    }

    #[test]
    fn a_new_tag_clears_the_second_word_and_an_equal_tag_keeps_it() {
        let mut frame = filled(0);
        frame.set_word_at(field::TAG_B4, 5);
        frame.set_word_at(field::WORD_B8, 0xAA);

        assert_eq!(frame.store_float_b0_with_tag(2.0, 5), 5);
        assert_eq!(frame.word_at(field::WORD_B8), 0xAA, "equal tag keeps 0xB8");

        assert_eq!(frame.store_float_b0_with_tag(2.0, 6), 6);
        assert_eq!(frame.word_at(field::TAG_B4), 6);
        assert_eq!(frame.word_at(field::WORD_B8), 0, "new tag clears 0xB8");
        assert_eq!(frame.word_at(field::FLOAT_B0), 2.0_f32.to_bits());
    }

    #[test]
    fn slot_and_fraction_move_their_bits() {
        let mut frame = UILayoutFrame::new();
        frame.set_slot_and_fraction(0xDEAD_BEEF, 0x7FC0_0001);
        assert_eq!(frame.word_at(field::SLOT_14), 0xDEAD_BEEF);
        assert_eq!(frame.word_at(field::FRACTION_18), 0x7FC0_0001);
    }

    #[test]
    fn cache_keeps_the_first_value_installed() {
        let mut frame = UILayoutFrame::new();
        frame.install_cache_if_empty(0x1111_1111);
        frame.install_cache_if_empty(0x2222_2222);
        assert_eq!(frame.word_at(field::CACHE_EC), 0x1111_1111);
    }

    #[test]
    fn cache_ignores_a_zero_value_but_takes_the_next_one() {
        let mut frame = UILayoutFrame::new();
        frame.install_cache_if_empty(0);
        assert_eq!(frame.word_at(field::CACHE_EC), 0);
        frame.install_cache_if_empty(3);
        assert_eq!(frame.word_at(field::CACHE_EC), 3);
    }

    #[test]
    fn block_copies_overlap_the_words_around_them() {
        let block: [u8; 24] = core::array::from_fn(|i| i as u8 + 1);
        let mut frame = filled(0xEE);
        frame.copy_block_to_10(&block);
        assert_eq!(&frame.bytes()[0x10..0x28], &block[..]);
        assert_eq!(frame.bytes()[0x0F], 0xEE);
        assert_eq!(frame.bytes()[0x28], 0xEE);

        frame.copy_block_to_a4(&block);
        assert_eq!(&frame.bytes()[0xA4..0xBC], &block[..]);
        assert_eq!(frame.bytes()[0xA3], 0xEE);
        assert_eq!(frame.bytes()[0xBC], 0xEE);
    }

    #[test]
    fn the_frame_span_covers_the_cache_word() {
        assert_eq!(field::CACHE_EC + 4, SPAN);
    }

    #[test]
    fn byte_writes_never_leave_the_span() {
        let mut frame = filled(0);
        frame.set_flag_eb(u32::MAX);
        frame.set_word_e4(u32::MAX);
        frame.copy_block_to_a4(&[0xFF; 24]);
        assert_eq!(frame.bytes().len(), SPAN);
    }
}
