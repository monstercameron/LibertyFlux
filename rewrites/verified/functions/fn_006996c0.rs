// original: 0x006996c0 rage::crAnimChannelDeltaFloat::vf19
/// Storage words for a delta-float channel: ceil(n/32) words plus header.
///
/// Reads the count at offset 0x24 and returns the rounded-up word count
/// times four plus the 0x38-byte header.
export!(thiscall, rw_006996c0(this: u32) -> u32 {
    unsafe {
        let n = *((this as *const u32).add(9));
        let words = (n >> 5).wrapping_add(((n & 31) != 0) as u32);
        words.wrapping_mul(4).wrapping_add(0x38)
    }
});
