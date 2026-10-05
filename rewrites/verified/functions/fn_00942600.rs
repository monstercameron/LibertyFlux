// original: 0x00942600 streaming_table_invalidate (proposed)

/// Invalidate the global slot entry selected by the object.
///
/// Reads the index at `this + 0x2040` and writes -1 to the global table
/// entry at that index. Returns the index in `eax`.
///
/// Original: 0x00942600 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00942600(this: u32) -> u32 {
    unsafe {
        const INDEX: u32 = 0x2040;
        const TABLE: u32 = 0x011D4E98;
        const INVALID: u32 = 0xFFFF_FFFF;
        let index = ((this + INDEX) as *const u32).read_unaligned();
        let table = lf_checker_rt::relocated(TABLE);
        ((table + index.wrapping_mul(4)) as *mut u32).write_unaligned(INVALID);
        index
    }
});
