// original: 0x00cf77a0 CTaskSimpleClimbLadder::ctor_body (proposed)

/// Constructor body for a simple climb-ladder task: stamps the vtable, runs
/// the ladder initialiser with the constant -50.0f, then tail-calls the
/// shared base constructor with the object.
///
/// Original: 0x00cf77a0 (thiscall: ecx holds the object, no stack words).
lf_checker_rt::export!(thiscall, rw_00cf77a0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00edf3e4;
        const LADDER_ARG: u32 = 0xc100_0000; // -50.0f
        const LADDER_CALLEE: u32 = 1;
        const BASE_CTOR: u32 = 2;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(LADDER_CALLEE, u32, this, LADDER_ARG);
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this)
    }
});
