// original: 0xe6aec0 task_table_reset_01 (proposed)

/// Reset a table of fixed-size rows to their start-up values,
/// then notify done.
///
/// Writes 3 constant words/bytes into each of 40 rows of
/// STRIDE bytes from BASE, then calls the shared notifier once
/// with this unit's fixed address (cdecl, one stack word). No
/// arguments, no meaningful return value.
lf_checker_rt::export!(cdecl, rw_00e6aec0() -> u32 {
    const BASE: u32 = 0x016d37f4;
    const COUNT: u32 = 40;
    const STRIDE: u32 = 0x00000010;
    const ARG_ADDR: u32 = 0x00e72a90;
    unsafe {
        let base = lf_checker_rt::relocated(BASE);
        let mut i = 0u32;
        while i < COUNT {
            let row = base.wrapping_add(i.wrapping_mul(STRIDE));
            ((row as *mut u8).wrapping_offset(4) as *mut u32).write_unaligned(0x00000000);
            ((row as *mut u8).wrapping_offset(0) as *mut u32).write_unaligned(0x00000000);
            ((row as *mut u8).wrapping_offset(-4) as *mut u32).write_unaligned(0x00000000);
            i += 1;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(ARG_ADDR));
    }
    0
});
