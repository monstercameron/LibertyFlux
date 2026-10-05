// original: 0x0069C7D0 rage::crAnimChannelCurveFloat::get_alloc_size

/// Allocation size of a curve-float channel: header plus key segments.
///
/// `this` is the channel object; the key count is the unsigned 16-bit word
/// at `+0xC` (compared signed against zero, but a `u16` is never negative,
/// so only zero skips the loop) and the key array starts at `[this+8]`.
/// Each key contributes `byte * 4 + 0xC` over a base of `0x18`, reading the
/// byte 2 past each 8-byte key record. Call-free; returns the size in `eax`.
///
/// Original: 0x0069C7D0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0069C7D0(this: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x0C;
        const KEYS_OFF: u32 = 8;
        const BASE_SIZE: u32 = 0x18;
        const REC_BYTE_OFF: u32 = 2;
        const REC_STRIDE: u32 = 8;
        const SEGMENT_TAIL: u32 = 0x0C;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        let count = rd16(this + COUNT_OFF);
        let mut size = BASE_SIZE;
        if (count as i32) > 0 {
            let mut rec = rd32(this + KEYS_OFF).wrapping_add(REC_BYTE_OFF);
            let mut left = count;
            loop {
                let k = unsafe { (rec as *const u8).read() as u32 };
                size = size.wrapping_add(k.wrapping_mul(4)).wrapping_add(SEGMENT_TAIL);
                rec = rec.wrapping_add(REC_STRIDE);
                left -= 1;
                if left == 0 {
                    break;
                }
            }
        }
        size
    }
});
