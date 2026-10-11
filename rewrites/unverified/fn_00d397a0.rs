// original: 0x00D397A0 ped_task_clear_member_then_base_destructor

/// Release the optional resource at +0x1c with the caller-cleanup helper, clear its slot, and forward the base-destructor result. ECX is the task pointer; there are no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d397a0(this: u32) -> u32 {
    unsafe {
        const MEMBER_POINTER: u32 = 0x1c;
        const RELEASE_CALLEE: u32 = 1;
        const BASE_DESTRUCTOR: u32 = 2;
        let member_slot = this.wrapping_add(MEMBER_POINTER);
        let resource = (member_slot as *const u32).read_unaligned();
        if resource != 0 {
            let _ = lf_checker_rt::callee_cdecl!(RELEASE_CALLEE, u32, resource);
            (member_slot as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
