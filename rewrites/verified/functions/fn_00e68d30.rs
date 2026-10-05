// original: 0x00E68D30 veh_quad_slot_init (proposed)
/// Initialise four vehicle quad slots, then clear the status word.
///
/// Calls the slot callee (`CALLEE`, thiscall, object pointer in `ecx`, no
/// stack arguments) with each of the four addresses in `OBJS`, in order,
/// then writes zero to `STATUS`.
///
/// Original: 0x00E68D30 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00e68d30() -> u32 {
    unsafe {
        const OBJS: [u32; 4] = [0x015F8B48, 0x015F8B5C, 0x015F8B70,
                                0x015F8B84];
        const STATUS: u32 = 0x015F8B98;
        const CALLEE: u32 = 1;
        let mut k = 0usize;
        while k < OBJS.len() {
            lf_checker_rt::callee_thiscall!(CALLEE, u32,
                lf_checker_rt::relocated(OBJS[k]));
            k += 1;
        }
        lf_checker_rt::global::<u32>(STATUS).write_unaligned(0);
        0
    }
});
