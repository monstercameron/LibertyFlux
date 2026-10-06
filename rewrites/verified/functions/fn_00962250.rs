// original: 0x00962250 id_table_reset_all
/// Reset the whole id table to the empty marker.
///
/// Writes `-1` to all `0x2000` dwords of the range `0x12008B0..0x12088B0`.
/// Takes no arguments; returns the end pointer, matching the original's EAX.
lf_checker_rt::export!(cdecl, rw_00962250() -> u32 {
    unsafe {
        const BASE: u32 = 0x12008b0;
        const WORDS: usize = 0x2000;
        const EMPTY: u32 = 0xffff_ffff;
        let dst = lf_checker_rt::global::<u32>(BASE);
        let mut i = 0usize;
        while i < WORDS {
            *dst.add(i) = EMPTY;
            i += 1;
        }
        lf_checker_rt::relocated(BASE).wrapping_add((WORDS * 4) as u32)
    }
});
