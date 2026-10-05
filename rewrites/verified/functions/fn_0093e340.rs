// original: 0x0093e340 stream_density_raise (proposed)

/// Raise the density cell (`a0`, `a1`) to at least the scaled `a2`.
///
/// The cell index is `a0 + 120 * a1` into the byte table at
/// `DENSITY_TABLE`. The candidate is `(a2 + 1) / 2` clamped to 0xFF; it is
/// stored only when not below the current byte. Returns the previous byte.
///
/// Original: 0x0093e340 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_0093e340(a0: u32, a1: u32, a2: u32) -> u32 {
    const DENSITY_TABLE: u32 = 0x11A4FC8;
    const ROW_STRIDE: u32 = 120;
    const CELL_MAX: u32 = 0xFF;
    unsafe {
        let cell = lf_checker_rt::relocated(DENSITY_TABLE)
            .wrapping_add(a0)
            .wrapping_add(a1.wrapping_mul(ROW_STRIDE));
        let mut cand = a2.wrapping_add(1) >> 1;
        if cand >= 0x100 {
            cand = CELL_MAX;
        }
        let old = (cell as *const u8).read() as u32;
        if cand >= old {
            (cell as *mut u8).write(cand as u8);
        }
        old
    }
});
