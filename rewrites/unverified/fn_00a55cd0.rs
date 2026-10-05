// original: 0x00a55cd0 vehicle_set_fields (proposed)

/// Store a word, a half-word and a zero byte into the object's fields.
///
/// The first stack word goes to +0x1a74 whole, the low half of the second
/// to +0x1a78, and +0x1afa is cleared. The last two stack words are unread.
/// Thiscall, four stack words, no result.
lf_checker_rt::export!(thiscall, rw_00a55cd0(this: u32, a0: u32, a1: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        const F_WORD: u32 = 0x1a74;
        const F_HALF: u32 = 0x1a78;
        const F_ZERO: u32 = 0x1afa;
        (this.wrapping_add(F_WORD) as *mut u32).write_unaligned(a0);
        (this.wrapping_add(F_HALF) as *mut u16).write_unaligned((a1 & 0xffff) as u16);
        (this.wrapping_add(F_ZERO) as *mut u8).write(0);
        0
    }
});
