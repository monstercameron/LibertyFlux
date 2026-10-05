// original: 0x00E68AA0 veh_subsys_register (proposed)
/// Initialise one vehicle subsystem object, then register a callback.
///
/// Calls the object callee (`OBJ_CALLEE`, thiscall, object pointer in `ecx`,
/// no stack arguments) with `OBJ`, then passes the static code block `ARG`
/// to the registrar callee (`REG_CALLEE`, cdecl, one argument), in that
/// order. No memory is touched directly.
///
/// Original: 0x00E68AA0 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00e68aa0() -> u32 {
    unsafe {
        const OBJ: u32 = 0x015B2D00;
        const ARG: u32 = 0x00E724E0;
        const OBJ_CALLEE: u32 = 1;
        const REG_CALLEE: u32 = 2;
        lf_checker_rt::callee_thiscall!(OBJ_CALLEE, u32,
            lf_checker_rt::relocated(OBJ));
        lf_checker_rt::callee_cdecl!(REG_CALLEE, u32,
            lf_checker_rt::relocated(ARG));
        0
    }
});
