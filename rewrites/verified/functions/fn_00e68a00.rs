// original: 0x00E68A00 veh_pair_slot_clear (proposed)
/// Clear a vehicle pair-slot table, then register a callback.
///
/// Zeroes `COUNT` consecutive 8-byte slots starting at `BASE` (two words per
/// slot), then passes the static code block `ARG` to the registrar callee
/// (`CALLEE`, cdecl, one argument).
///
/// Original: 0x00E68A00 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00e68a00() -> u32 {
    unsafe {
        const BASE: u32 = 0x01593458;
        const COUNT: u32 = 216;
        const SLOT: u32 = 8;
        const ARG: u32 = 0x00E724B0;
        const CALLEE: u32 = 1;
        let mut p = lf_checker_rt::relocated(BASE);
        let mut i = 0u32;
        while i < COUNT {
            (p as *mut u32).write_unaligned(0);
            (p.wrapping_add(4) as *mut u32).write_unaligned(0);
            p = p.wrapping_add(SLOT);
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(CALLEE, u32,
            lf_checker_rt::relocated(ARG));
        0
    }
});
