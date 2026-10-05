// original: 0x00E68CF0 veh_sparse_flag_reset (proposed)
/// Reset the sparse flag words of a vehicle record table.
///
/// For each of 225 records of `STRIDE` bytes starting at `BASE`, writes
/// `0xFFFF_FFFF` to the five flag words at offsets `-8, 0, 0x14, 0x20,
/// 0x24`. Words between the flags are left untouched. No calls.
///
/// Original: 0x00E68CF0 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00e68cf0() -> u32 {
    unsafe {
        const BASE: u32 = 0x015DE3D4;
        const COUNT: u32 = 225;
        const STRIDE: u32 = 0xBC;
        const OFFS: [i32; 5] = [-8, 0, 0x14, 0x20, 0x24];
        const FILL: u32 = 0xFFFF_FFFF;
        let base = lf_checker_rt::relocated(BASE);
        let mut i = 0u32;
        while i < COUNT {
            let r = base.wrapping_add(i.wrapping_mul(STRIDE));
            let mut k = 0usize;
            while k < OFFS.len() {
                ((r as i32).wrapping_add(OFFS[k]) as u32 as *mut u32)
                    .write_unaligned(FILL);
                k += 1;
            }
            i += 1;
        }
        0
    }
});
