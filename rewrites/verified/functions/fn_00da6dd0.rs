// original: 0x00DA6DD0 CTaskComplexFleeAndDive::vf1

/// Resolve the task manager, then request a flee-and-dive task. A null
/// manager returns zero. The factory receives the stored float words at
/// `+0x34` and `+0x38`, a pointer to the embedded point at `+0x20`, then
/// the words at `+0x30` and `+0x3C`, in that order. Float words are forwarded
/// unchanged, preserving all bit patterns.
lf_checker_rt::export!(thiscall, rw_00DA6DD0(this: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL_VA: u32 = 0x0167E2A0;
        const MANAGER_LOOKUP: u32 = 1;
        const TASK_FACTORY: u32 = 2;
        const EMBEDDED_POINT: u32 = 0x20;
        const ARGUMENT_F34: u32 = 0x34;
        const ARGUMENT_F38: u32 = 0x38;
        const ARGUMENT_30: u32 = 0x30;
        const ARGUMENT_3C: u32 = 0x3C;

        let manager_context = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
        let manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, manager_context);
        if manager == 0 {
            return 0;
        }

        let f34_bits = (this.wrapping_add(ARGUMENT_F34) as *const u32).read_unaligned();
        let point = this.wrapping_add(EMBEDDED_POINT);
        let f38_bits = (this.wrapping_add(ARGUMENT_F38) as *const u32).read_unaligned();
        let word_30 = (this.wrapping_add(ARGUMENT_30) as *const u32).read_unaligned();
        let word_3c = (this.wrapping_add(ARGUMENT_3C) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            TASK_FACTORY, u32, manager, f34_bits, point, f38_bits, word_30, word_3c
        )
    }
});
