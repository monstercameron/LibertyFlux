// original: 0x00A86000 CCover::can_start_peeking

/// Report whether either of the two cover-peeking state bits is set.
///
/// The `this` pointer addresses a cover object. Only the byte at offset
/// `0x35` is read; the method returns one in AL when bits `0x10` or `0x20`
/// are present, and zero otherwise. It has no stack arguments and uses the
/// 32-bit thiscall convention.
lf_checker_rt::export!(thiscall, rw_00a86000(this: u32) -> u8 {
    unsafe {
        const PEEK_STATE: u32 = 0x35;
        const PEEK_BITS: u8 = 0x30;
        let state = ((this + PEEK_STATE) as *const u8).read();
        u8::from(state & PEEK_BITS != 0)
    }
});
