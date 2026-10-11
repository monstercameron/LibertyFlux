// original: 0x00D39740 ped_task_release_member_then_base_destructor

/// Release the optional member stored at +0x10, clear that slot, install the task vtable, and forward the base-destructor result. ECX is the task pointer; there are no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d39740(this: u32) -> u32 {
    unsafe {
        const MEMBER_POINTER: u32 = 0x10;
        const TASK_VTABLE: u32 = 0x00EE37F0;
        const RELEASE_CALLEE: u32 = 1;
        const BASE_DESTRUCTOR_CALLEE: u32 = 2;
        let member_slot = this.wrapping_add(MEMBER_POINTER);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE));
        let resource = (member_slot as *const u32).read_unaligned();
        if resource != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, resource, member_slot);
            (member_slot as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR_CALLEE, u32, this)
    }
});
