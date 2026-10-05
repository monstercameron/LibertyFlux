// original: 0x0093fb70 stream_density_at_cell (proposed)

/// Load a doubled density byte for grid cell (`x`, `y`) onto the x87 stack.
///
/// The grid is `GRID_WIDE` by `GRID_HIGH` cells; out-of-range coordinates
/// yield +0.0. Otherwise reads the byte at `DENSITY_TABLE + x + 120 * y`,
/// doubles it and returns it as a float (the original loads the integer
/// with `fild`, which is exact for every possible byte value).
///
/// Original: 0x0093fb70 (cdecl, two stack words, x87 single-value result).
lf_checker_rt::export!(cdecl, rw_0093fb70(x: u32, y: u32) -> f64 {
    const DENSITY_TABLE: u32 = 0x11A4FC8;
    const GRID_WIDE: u32 = 0x77;
    const GRID_HIGH: u32 = 0x78;
    const ROW_STRIDE: u32 = 120;
    if x > GRID_WIDE || y > GRID_HIGH {
        return 0.0;
    }
    unsafe {
        let addr = lf_checker_rt::relocated(DENSITY_TABLE)
            .wrapping_add(x)
            .wrapping_add(y.wrapping_mul(ROW_STRIDE));
        let byte = (addr as *const u8).read() as u32;
        (byte + byte) as f64
    }
});
