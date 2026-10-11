// original: 0x00BB8640 CEntitySeekPosCalculatorRadiusAngleOffset::vf0

/// Initialize `CEntitySeekPosCalculatorRadiusAngleOffset::vf0` using the x86 thiscall convention: `this` arrives in ECX,
/// the first stack word is the flag, and the object pointer is returned in EAX.
/// Only bit zero of the flag's low byte selects the optional finalization call.
/// The vtable pointer is written at object offset zero. No base initializer runs. The optional finalizer receives the object pointer through a cdecl stack argument.
/// The checker scripts the callees and compares their call order and arguments; their
/// internal behavior is outside this function's proof.
lf_checker_rt::export!(thiscall, rw_00bb8640(this: u32, destroy_flag: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EB7688;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        const CLEANUP: u32 = 1;
        if destroy_flag & 1 != 0 {
            let _cleanup_result: u32 = lf_checker_rt::callee_cdecl!(CLEANUP, u32, this);
        }
        this
    }
});
