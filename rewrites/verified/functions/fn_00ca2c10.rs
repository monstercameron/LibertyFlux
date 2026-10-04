// original: 0x00ca2c10 merge_flags_copy

/// Merge a damage-event flag byte and copy its payload words.
///
/// Copies bits 0-6 of the flag byte at `other + 4` into `this + 4`, keeping
/// bit 7 of the destination, then copies the dwords at `+0x08` and `+0x0c`.
/// Returns `this`.
///
/// Original: 0x00ca2c10 (thiscall, one stack word = other pointer).
lf_checker_rt::export!(thiscall, rw_00ca2c10(this: u32, other: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    #[inline(always)]
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn wr8(a: u32, v: u8) {
        unsafe { (a as *mut u8).write(v) }
    }
    unsafe {
        const FLAGS: u32 = 0x04;
        const KEEP_MASK: u8 = 0x80;
        let dst = rd8(this + FLAGS);
        let src = rd8(other + FLAGS);
        wr8(this + FLAGS, (dst & KEEP_MASK) | (src & !KEEP_MASK));
        wr32(this + 0x08, rd32(other + 0x08));
        wr32(this + 0x0c, rd32(other + 0x0c));
        this
    }
});
