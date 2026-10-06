// original: 0x00976d60 audio_bit_alloc (proposed)

/// Allocate the first clear bit of a 500-bit map.
///
/// Scans the words at [this+8] from bit 0 upward (the index is compared
/// unsigned against 500). On a clear bit it sets the bit and returns
/// this+0x10+bit*128; when every bit is set it returns 0.
/// Original: 0x00976D60 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00976d60(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 500;
        const STRIDE: u32 = 128;
        const HDR: u32 = 0x10;
        const MAP: u32 = 8;
        let bits = ((this.wrapping_add(MAP)) as *const u32).read_unaligned();
        let mut i: u32 = 0;
        while i < COUNT {
            let cell = (bits.wrapping_add((i >> 5) * 4)) as *const u32;
            let w = cell.read_unaligned();
            let mask = 1u32 << (i & 31);
            if w & mask == 0 {
                ((bits.wrapping_add((i >> 5) * 4)) as *mut u32).write_unaligned(w | mask);
                return this.wrapping_add(HDR).wrapping_add(i.wrapping_mul(STRIDE));
            }
            i += 1;
        }
        0
    }
});
