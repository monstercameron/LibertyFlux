// original: 0x00ab60e0 stream_zero_runs (proposed)

/// Zero three columns of an 11-lane run table.
///
/// The table base is `arg + 0x2C0`; for each lane `i` in `0..11` clears the
/// words at `base+4*i`, `base+0x2c+4*i` and `base+0x58+4*i`. No return value.
///
/// Original: 0x00ab60e0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00ab60e0(arg: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x2C0;
        const LANES: u32 = 11;
        const ROW_STRIDE: u32 = 0x2C;
        const ROWS: u32 = 3;
        let base = arg.wrapping_add(TABLE_OFF);
        for i in 0..LANES {
            for r in 0..ROWS {
                ((base + r * ROW_STRIDE + 4 * i) as *mut u32).write_unaligned(0);
            }
        }
        0
    }
});
