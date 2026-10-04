// original: 0x00ab9980 slot_search_copy_block20

/// Search two stride-64 records by key, copy the 16-byte block at 0x20.
///
/// On a hit copies four dwords starting at record offset 0x20 into the
/// output buffer (the middle two move through vector registers in the
/// original, as pure bit copies) and returns the last copied word with its
/// low byte set. On a miss returns the table pointer with its low byte
/// cleared, matching the value the original leaves in EAX.
export!(thiscall, rs64_ab9980(this: *const u8, key: u32, out: *mut u32) -> u32 {
    unsafe {
        const COUNT: u32 = 2;
        const STRIDE: u32 = 64;
        const BLOCK_OFF: u32 = 0x20;
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
                return (w3 & 0xFFFF_FF00) | 1;
            }
            i += 1;
        }
        base & 0xFFFF_FF00
    }
});
