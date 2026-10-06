// original: 0x00962720 chunk_table_init
/// Initialise the 0x200-entry chunk table at `0x11FF070`.
///
/// Each 12-byte entry gets two zero dwords and a zero byte; bytes `+9..+11`
/// are left untouched, as in the original. Takes no arguments; returns the
/// end pointer.
lf_checker_rt::export!(cdecl, rw_00962720() -> u32 {
    unsafe {
        const BASE: u32 = 0x11ff070;
        const ENTRIES: u32 = 0x200;
        const STRIDE: u32 = 0x0c;
        let mut i = 0u32;
        while i < ENTRIES {
            let e = lf_checker_rt::relocated(BASE).wrapping_add(i.wrapping_mul(STRIDE));
            (e as *mut u32).write_unaligned(0);
            (e.wrapping_add(4) as *mut u32).write_unaligned(0);
            (e.wrapping_add(8) as *mut u8).write(0);
            i += 1;
        }
        lf_checker_rt::relocated(0x11ff078).wrapping_add(ENTRIES.wrapping_mul(STRIDE))
    }
});
