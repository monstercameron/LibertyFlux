// original: 0x00888020 stream_slot_ptr (proposed)

/// Address of a stream-slot record, or null for the empty index.
///
/// `index` selects one of the fixed-size records (0xa0 bytes each) in the
/// table reached through the table-base global. Index 0xffff_ffff means
/// "no slot" and yields null; otherwise the result is
/// `base + index * 5 * 32` with wrapping arithmetic.
///
/// Original: 0x00888020 (cdecl, one stack word; callee pops nothing).
lf_checker_rt::export!(cdecl, rw_00888020(index: u32) -> u32 {
    unsafe {
        const NONE: u32 = 0xffff_ffff;
        const TABLE_GLOBAL: u32 = 0x0115_a46c;
        const STRIDE: u32 = 0xa0;
        if index == NONE {
            0
        } else {
            let base = (lf_checker_rt::relocated(TABLE_GLOBAL) as *const u32)
                .read_unaligned();
            index.wrapping_mul(5).wrapping_mul(32).wrapping_add(base)
        }
    }
});
