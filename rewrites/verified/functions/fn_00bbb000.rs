// original: 0x00bbb000 CTaskSimpleLeaveGroup::vf1

/// Clone `CTaskSimpleLeaveGroup::vf1` using x86 thiscall: the source object arrives in ECX and the clone is returned in EAX.
/// The task context is read from the global context slot and passed to the allocator. A null allocation returns zero.
/// Otherwise the base initializer runs on the clone, the listed fields are copied from the source, and the derived
/// vtable is installed. This proof scripts the allocator and base initializer; it compares call order and arguments,
/// while their internal behavior remains outside the function's proof.
lf_checker_rt::export!(thiscall, rw_00bbb000(this: u32) -> u32 {
    unsafe {
        const TASK_CONTEXT_VA: u32 = 0x0167E2A0;
        const ALLOCATOR: u32 = 1;
        const BASE_INITIALIZER: u32 = 2;
        let task_context = lf_checker_rt::global::<u32>(TASK_CONTEXT_VA).read_unaligned();
        let clone = lf_checker_rt::callee_thiscall!(ALLOCATOR, u32, task_context);
        if clone == 0 {
            return 0;
        }
        let _base_result: u32 = lf_checker_rt::callee_thiscall!(BASE_INITIALIZER, u32, clone);
        const DERIVED_VTABLE: u32 = 0x00EB75DC;
        (clone as *mut u32).write_unaligned(lf_checker_rt::relocated(DERIVED_VTABLE));
        clone
    }
});
