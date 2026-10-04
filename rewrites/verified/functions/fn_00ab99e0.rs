// original: 0x00ab99e0 slot_search_copy_block30

/// Search two stride-64 records by key, copy the 16-byte block at 0x30.
///
/// Same shape as its sibling above with the payload at record offset 0x30,
/// moved as two 8-byte halves. A hit returns the output pointer with its low
/// byte set; a miss returns 0.
export!(thiscall, rs64_ab99e0(this: *const u8, key: u32, out: *mut u32) -> u32 {
    unsafe {
        const COUNT: u32 = 2;
        const STRIDE: u32 = 64;
        const BLOCK_OFF: u32 = 0x30;
        let base = this as u32;
        let mut i = 0u32;
        while i < COUNT {
            let slot = base.wrapping_add(i.wrapping_mul(STRIDE));
            if *(slot as *const u32) == key {
                let w0 = *((slot.wrapping_add(BLOCK_OFF)) as *const u32);
                let w1 = *((slot.wrapping_add(BLOCK_OFF + 4)) as *const u32);
                let w2 = *((slot.wrapping_add(BLOCK_OFF + 8)) as *const u32);
                let w3 = *((slot.wrapping_add(BLOCK_OFF + 12)) as *const u32);
                *out = w0;
                *out.add(1) = w1;
                *out.add(2) = w2;
                *out.add(3) = w3;
                return (out as u32 & 0xFFFF_FF00) | 1;
            }
            i += 1;
        }
        0
    }
});
