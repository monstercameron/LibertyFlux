// original: 0x00E68C60 veh_record_table_init (proposed)
/// Initialise a table of vehicle records: flags plus slot call.
///
/// For each of `COUNT` records of `STRIDE` bytes starting at `BASE`, writes
/// `0xFFFF_FFFF` to the ten flag words at `OFFS` and calls the slot callee
/// (`CALLEE`, thiscall, object pointer in `ecx`, no stack arguments) with
/// the record's slot block at offset `SLOT_OFF`.
///
/// Original: 0x00E68C60 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00e68c60() -> u32 {
    unsafe {
        const BASE: u32 = 0x015E8918;
        const COUNT: u32 = 2;
        const STRIDE: u32 = 0x210;
        const SLOT_OFF: u32 = 0xD8;
        const CALLEE: u32 = 1;
        const OFFS: [i32; 10] = [-0x8, -0x4, 0x0, 0x24, 0x28, 0x50, 0x54, 0x58, 0x90, 0x94];
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
            lf_checker_rt::callee_thiscall!(CALLEE, u32,
                r.wrapping_add(SLOT_OFF));
            i += 1;
        }
        0
    }
});
