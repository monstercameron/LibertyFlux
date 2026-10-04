// original: 0x00DA6110 CTaskComplexShockingEventWatch::vf18

/// Forward to the shared dispatch helper with the caller's argument, then
/// report zero. The helper call is intercepted and answered by script.
/// Original: thiscall, one stack word, callee cleanup (`this` is unused).
lf_checker_rt::export!(thiscall, rw_00da6110(this: u32, arg: u32) -> u32 {
    unsafe {
        const DISPATCH: u32 = 1;
        let _ = this;
        lf_checker_rt::callee_stdcall!(DISPATCH, u32, arg);
        0
    }
});
