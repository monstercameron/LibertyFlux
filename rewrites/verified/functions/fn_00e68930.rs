// original: 0x00E68930 veh_id_table_reset (proposed)
/// Reset a vehicle id table to all-ones (`NWORDS` words at `BASE`).
///
/// Writes `0xFFFF_FFFF` to every word, the original's `rep stosd` with
/// `eax = -1`. No inputs, no calls.
///
/// Original: 0x00E68930 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00e68930() -> u32 {
    unsafe {
        const BASE: u32 = 0x0158E660;
        const NWORDS: u32 = 0x80;
        const FILL: u32 = 0xFFFF_FFFF;
        let dst = lf_checker_rt::relocated(BASE);
        let mut i = 0u32;
        while i < NWORDS {
            (dst.wrapping_add(i.wrapping_mul(4)) as *mut u32)
                .write_unaligned(FILL);
            i += 1;
        }
        0
    }
});
