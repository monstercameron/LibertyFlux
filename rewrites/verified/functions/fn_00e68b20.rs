// original: 0x00E68B20 veh_zero_thunk (proposed)
/// Tail-jump to the vehicle zeroing routine with a fixed object.
///
/// Loads `OBJ` into `ecx` and transfers control to the zeroing callee
/// (`CALLEE`, thiscall, object pointer in `ecx`, no stack arguments),
/// forwarding its result. The original ends in a jump, not a call; the
/// rewrite expresses the same transfer as a call.
///
/// Original: 0x00E68B20 (cdecl shape, no arguments; the target takes only
/// `ecx` and returns it in `eax`).
lf_checker_rt::export!(cdecl, rw_00e68b20() -> u32 {
    unsafe {
        const OBJ: u32 = 0x015DBDB8;
        const CALLEE: u32 = 1;
        lf_checker_rt::callee_thiscall!(CALLEE, u32,
            lf_checker_rt::relocated(OBJ))
    }
});
