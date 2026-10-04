// original: 0xe6ae30 task_table_reset_00 (proposed)

/// Reset a table of fixed-size rows to their start-up values,
/// then notify done.
///
/// Writes 11 constant words/bytes into each of 117 rows of
/// STRIDE bytes from BASE, then calls the shared notifier once
/// with this unit's fixed address (cdecl, one stack word). No
/// arguments, no meaningful return value.
lf_checker_rt::export!(cdecl, rw_00e6ae30() -> u32 {
    const BASE: u32 = 0x016d21d8;
    const COUNT: u32 = 117;
    const STRIDE: u32 = 0x00000030;
    const ARG_ADDR: u32 = 0x00e72a70;
    unsafe {
        let base = lf_checker_rt::relocated(BASE);
        let mut i = 0u32;
        while i < COUNT {
            let row = base.wrapping_add(i.wrapping_mul(STRIDE));
            ((row as *mut u8).wrapping_offset(-8) as *mut u32).write_unaligned(0x00000000);
            ((row as *mut u8).wrapping_offset(0) as *mut u32).write_unaligned(0xffffffff);
            ((row as *mut u8).wrapping_offset(8) as *mut u32).write_unaligned(0xffffffff);
            ((row as *mut u8).wrapping_offset(20) as *mut u8).write_unaligned(0x00);
            ((row as *mut u8).wrapping_offset(4) as *mut u32).write_unaligned(0xffffffff);
            ((row as *mut u8).wrapping_offset(-24) as *mut u32).write_unaligned(0x00000000);
            ((row as *mut u8).wrapping_offset(-20) as *mut u32).write_unaligned(0x00000000);
            ((row as *mut u8).wrapping_offset(-16) as *mut u32).write_unaligned(0x3f800000);
            ((row as *mut u8).wrapping_offset(12) as *mut u32).write_unaligned(0x00000001);
            ((row as *mut u8).wrapping_offset(-4) as *mut u32).write_unaligned(0x000003e8);
            ((row as *mut u8).wrapping_offset(-12) as *mut u32).write_unaligned(0x00000000);
            i += 1;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(ARG_ADDR));
    }
    0
});
