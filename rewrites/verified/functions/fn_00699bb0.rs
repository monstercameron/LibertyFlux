// original: 0x00699bb0 rage::crAnimChannelQuantizeFloat::vf19
/// Storage words for a quantized-float channel.
///
/// Multiplies the counts at offsets 0x10 and 0xc, divides by 32 rounding the
/// word count down, times four, plus a 0x20-byte header when any low bits
/// remain, else a 0x1c-byte header.
export!(thiscall, rw_00699bb0(this: u32) -> u32 {
    unsafe {
        let a = *((this as *const u32).add(4));
        let b = *((this as *const u32).add(3));
        let n = a.wrapping_mul(b);
        let words = n >> 5;
        let header = if (n & 31) != 0 { 0x20u32 } else { 0x1cu32 };
        words.wrapping_mul(4).wrapping_add(header)
    }
});
