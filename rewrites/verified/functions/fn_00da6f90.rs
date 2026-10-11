// original: 0x00DA6F90 CTaskComplexWanderFlee::vf1

/// Allocate and clone this task's wander-flee behavior. The global task
/// manager is queried first; when allocation succeeds, the clone helper
/// receives the stored floats at `+0x14` and `+0x18`, in that order.
/// A null allocation returns zero. The source float words are forwarded
/// unchanged as stack arguments, preserving their exact bit patterns.
lf_checker_rt::export!(thiscall, rw_00DA6F90(this: u32) -> u32 {
    unsafe {
        const MANAGER_SLOT_VA: u32 = 0x0167E2A0;
        const MANAGER_LOOKUP: u32 = 1;
        const CLONE_TASK: u32 = 2;
        const FIRST_RADIUS: u32 = 0x14;
        const SECOND_RADIUS: u32 = 0x18;

        let manager_slot = (lf_checker_rt::relocated(MANAGER_SLOT_VA) as *const u32).read();
        let manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, manager_slot);
        if manager == 0 { return 0; }
        let first_radius_bits = (this.wrapping_add(FIRST_RADIUS) as *const u32).read_unaligned();
        let second_radius_bits = (this.wrapping_add(SECOND_RADIUS) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(CLONE_TASK, u32, manager, first_radius_bits, second_radius_bits)
    }
});
