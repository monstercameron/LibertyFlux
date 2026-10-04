// original: 0x00E66730 fill_table_minus1_stride50 (proposed)

/// Fill the first word of every row of one table with all-bits-set.
///
/// Writes 0xFFFFFFFF to 128 dwords starting at the table base, stepping
/// 0x00000050 bytes per row (a down-counting loop of 128 iterations). No
/// arguments, no calls; returns the end pointer in eax (cdecl).
lf_checker_rt::export!(cdecl, rw_00e66730() -> u32 {
    unsafe {
        const BASE: u32 = 0x0128F620;
        const COUNT: u32 = 128;
        const STRIDE: u32 = 0x00000050;
        const FILL: u32 = 0xFFFFFFFF;
        let mut p = lf_checker_rt::relocated(BASE);
        let mut i = COUNT;
        while i != 0 {
            (p as *mut u32).write_unaligned(FILL);
            p = p.wrapping_add(STRIDE);
            i -= 1;
        }
        p
    }
});
