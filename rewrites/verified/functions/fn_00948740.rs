// original: 0x00948740 pool_contains_value
/// Test whether any of 16 scattered dwords spread over four 0x80-byte
/// blocks (offsets 4, 0x24, 0x44, 0x64 within each block) equals the query
/// value. Returns 1 on a match, 0 otherwise.
export!(thiscall, rw_00948740(obj: *const u8, value: u32) -> u32 {
    unsafe {
        for k in 0..4usize {
            let base = obj.add(4 + k * 0x80);
            for off in [0usize, 0x20, 0x40, 0x60] {
                if *(base.add(off) as *const u32) == value {
                    return 1;
                }
            }
        }
        0
    }
});
