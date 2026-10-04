// original: 0x00ab98e0 slot_search_copy_bytes

/// Search two stride-64 records by key, copy two tag bytes on a hit.
///
/// Same search as its sibling above, but copies the single bytes at offsets
/// 4 and 8 into the two output slots. The hit return value carries the
/// second output pointer with its low byte set, as the original does.
export!(thiscall, rs64_ab98e0(this: *const u8, key: u32, out0: *mut u8, out1: *mut u8) -> u32 {
    unsafe {
        const COUNT: u32 = 2;
        const STRIDE: u32 = 64;
        let base = this as u32;
        let mut i = 0u32;
        while i < COUNT {
            let slot = base.wrapping_add(i.wrapping_mul(STRIDE));
            if *(slot as *const u32) == key {
                *out0 = *((slot.wrapping_add(4)) as *const u8);
                *out1 = *((slot.wrapping_add(8)) as *const u8);
                return (out1 as u32 & 0xFFFF_FF00) | 1;
            }
            i += 1;
        }
        0
    }
});
