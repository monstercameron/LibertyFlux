// original: 0x00E687A0 veh_link_node_setup (proposed)
/// Set up one vehicle link node, flagged by the caller's high byte.
///
/// Takes a value in `ecx` (thiscall) and keeps only its top byte as a flag.
/// Zeroes the two header words at `BLOCK`, links the next two words to
/// `BLOCK` itself, stores the flag at `BLOCK + FLAG_OFF`, then passes the
/// static code block `ARG` to the registrar callee (`CALLEE`, cdecl, one
/// argument). The original zeroes sixteen bytes with vector stores first;
/// only the final state is observable and only it is reproduced.
///
/// Original: 0x00E687A0 (thiscall, one register argument, no meaningful
/// return).
lf_checker_rt::export!(thiscall, rw_00e687a0(ecx_in: u32) -> u32 {
    unsafe {
        const BLOCK: u32 = 0x0150E278;
        const FLAG_OFF: u32 = 0x14;
        const ARG: u32 = 0x00E72410;
        const CALLEE: u32 = 1;
        let flag: u8 = (ecx_in >> 24) as u8;
        let base = lf_checker_rt::relocated(BLOCK);
        (base as *mut u32).write_unaligned(0);
        (base.wrapping_add(4) as *mut u32).write_unaligned(0);
        let link = lf_checker_rt::relocated(BLOCK);
        (base.wrapping_add(8) as *mut u32).write_unaligned(link);
        (base.wrapping_add(12) as *mut u32).write_unaligned(link);
        (base.wrapping_add(FLAG_OFF) as *mut u8).write(flag);
        lf_checker_rt::callee_cdecl!(CALLEE, u32,
            lf_checker_rt::relocated(ARG));
        0
    }
});
