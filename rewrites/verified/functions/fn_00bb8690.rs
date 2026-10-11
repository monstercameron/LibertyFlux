// original: 0x00BB8690 CTaskSimpleCombatRoll::vf0

/// Initialize `CTaskSimpleCombatRoll::vf0` using the x86 thiscall convention: `this` arrives in ECX,
/// the first stack word is the flag, and the object pointer is returned in EAX.
/// Only bit zero of the flag's low byte selects the optional finalization call.
/// The vtable pointer is written at object offset zero. The base initializer runs first. The optional finalizer receives the task context in ECX and the object pointer on the stack.
/// The checker scripts the callees and compares their call order and arguments; their
/// internal behavior is outside this function's proof.
lf_checker_rt::export!(thiscall, rw_00bb8690(this: u32, destroy_flag: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EB7634;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        const BASE: u32 = 1;
        let _base_result: u32 = lf_checker_rt::callee_thiscall!(BASE, u32, this);
        const CONTEXT_VA: u32 = 0x0167E2A0;
        const CLEANUP: u32 = 2;
        if destroy_flag & 1 != 0 {
            let context = lf_checker_rt::global::<u32>(CONTEXT_VA).read_unaligned();
            let _cleanup_result: u32 = lf_checker_rt::callee_thiscall!(CLEANUP, u32, context, this);
        }
        this
    }
});
