// original: 0x00962650 pool_entry_table_init
/// Initialise the 0x400-entry pool table at `0x120F2B8`.
///
/// Each 20-byte entry is set to pointer `0`, tag `0xFFFF`, two zero dwords
/// and a zero word; bytes `+6..+7` and `+18..+19` are left untouched, as in
/// the original. Takes no arguments; returns the end pointer.
lf_checker_rt::export!(cdecl, rw_00962650() -> u32 {
    unsafe {
        const BASE: u32 = 0x120f2b8;
        const ENTRIES: u32 = 0x400;
        const STRIDE: u32 = 0x14;
        const TAG_INIT: u16 = 0xffff;
        let mut i = 0u32;
        while i < ENTRIES {
            let e = lf_checker_rt::relocated(BASE).wrapping_add(i.wrapping_mul(STRIDE));
            (e as *mut u32).write_unaligned(0);
            (e.wrapping_add(4) as *mut u16).write_unaligned(TAG_INIT);
            (e.wrapping_add(8) as *mut u32).write_unaligned(0);
            (e.wrapping_add(12) as *mut u32).write_unaligned(0);
            (e.wrapping_add(16) as *mut u16).write_unaligned(0);
            i += 1;
        }
        lf_checker_rt::relocated(0x120f2c0).wrapping_add(ENTRIES.wrapping_mul(STRIDE))
    }
});
