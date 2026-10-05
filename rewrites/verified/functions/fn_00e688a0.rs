// original: 0x00E688A0 veh_subsys_register (proposed)
/// Initialise one vehicle subsystem object, then register a callback.
///
/// Calls the object callee (`OBJ_CALLEE`, thiscall, object pointer in `ecx`,
/// no stack arguments) with `OBJ`, then passes the static code block `ARG`
/// to the registrar callee (`REG_CALLEE`, cdecl, one argument), in that
/// order. No memory is touched directly.
///
/// Original: 0x00E688A0 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00e688a0() -> u32 {
    unsafe {
        const OBJ: u32 = 0x0154E090;
        const ARG: u32 = 0x00E72470;
        const OBJ_CALLEE: u32 = 1;
        const REG_CALLEE: u32 = 2;
        lf_checker_rt::callee_thiscall!(OBJ_CALLEE, u32,
            lf_checker_rt::relocated(OBJ));
        lf_checker_rt::callee_cdecl!(REG_CALLEE, u32,
            lf_checker_rt::relocated(ARG));
        0
    }
});
