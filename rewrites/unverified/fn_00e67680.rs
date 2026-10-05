// original: 0x00E67680 task_register_slot_00E67680
/// Register the named task slot and return the registry handle.
///
/// Calls callee 1 (thiscall, one stack argument) with the slot
/// object `OBJ` in ECX and the name string `NAME` on the stack,
/// returning the callee's result.
///
/// Original: 0x00E67680 (cdecl, no arguments, returns u32).
lf_checker_rt::export!(cdecl, rw_00E67680() -> u32 {
    unsafe {
        const NAME: u32 = 0x00E9E210;
        const OBJ: u32 = 0x012FA154;
        lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(OBJ), lf_checker_rt::relocated(NAME))
    }
});
