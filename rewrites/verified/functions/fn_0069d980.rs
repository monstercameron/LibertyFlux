// original: 0x0069D980 rage::crAnimChannelRleInt::get_alloc_size

/// Allocation size of an RLE integer channel: header plus bit words.
///
/// `this` is the channel object. The bit length at `+0x14` is rounded up to
/// whole 32-bit words (`(n >> 5) + ((n & 0x1F) != 0)`), the sample count
/// (unsigned 16-bit word at `+0xC`) is added, and the total is scaled by 4
/// with a `0x1C`-byte header: `(words + count) * 4 + 0x1C`, all wrapping.
/// Call-free; returns the size in `eax`.
///
/// Original: 0x0069D980 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0069D980(this: u32) -> u32 {
    unsafe {
        const BITLEN_OFF: u32 = 0x14;
        const COUNT_OFF: u32 = 0x0C;
        const WORD_SHIFT: u32 = 5;
        const WORD_MASK: u32 = 0x1F;
        const HEADER_SIZE: u32 = 0x1C;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        let n = rd32(this + BITLEN_OFF);
        let words = (n >> WORD_SHIFT) + ((n & WORD_MASK != 0) as u32);
        let total = words.wrapping_add(rd16(this + COUNT_OFF));
        total.wrapping_mul(4).wrapping_add(HEADER_SIZE)
    }
});
