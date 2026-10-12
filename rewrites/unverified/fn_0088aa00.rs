// original: 0x0088AA00 stream_resource_mark_chunk (proposed)

/// Set one bit in a resource group's chunk bitmap.
///
/// `this` points to the resource manager whose table base is at `+0xe8`.
/// `group` selects a group (0x1bd0 words of bitmap each) and `bit` a bit
/// within it; the bit's word lives at `base + (group * 0x1bd0 + bit / 32) * 4
/// + 0x6ee0` and bit `bit % 32` of it is set. Returns the one-hot mask (the
/// original leaves it in eax).
///
/// Original: 0x0088AA00 (thiscall: `this` in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_0088aa00(this: u32, group: u32, bit: u32) -> u32 {
    unsafe {
        const GROUP_WORDS: u32 = 0x1bd0;
        const TABLE_BASE: u32 = 0xe8;
        const BITMAP_OFF: u32 = 0x6ee0;

        let base = ((this + TABLE_BASE) as *const u32).read_unaligned();
        let word = group
            .wrapping_mul(GROUP_WORDS)
            .wrapping_add(bit >> 5);
        let mask = 1u32 << (bit & 31);
        let cell = base.wrapping_add(word.wrapping_mul(4)).wrapping_add(BITMAP_OFF);
        let old = (cell as *const u32).read_unaligned();
        (cell as *mut u32).write_unaligned(old | mask);
        mask
    }
});
