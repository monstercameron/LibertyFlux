// original: 0x00cf9de0 CTaskComplexClimbLadder::vf5
/// Queries the incoming subtask, forwards argument one to the base task helper,
/// and examines the child at offset 0x08. A missing child or a child already
/// carrying bit zero at +0x0C succeeds. Otherwise it invokes child vtable slot
/// 0x14 with this task and arguments two and three; a true result sets child flag
/// bit one. The return is a bool in AL.
lf_checker_rt::export!(thiscall, rw_00cf9de0(this: *mut u8, first: u32, second: u32, query_object: *mut u8) -> u32 {
    unsafe {
        const CHILD_TASK: usize = 0x08;
        const FLAGS: usize = 0x0C;
        const QUERY_KIND: u32 = 0x42;
        if !query_object.is_null() && lf_checker_rt::callee_thiscall!(1, u32, query_object as u32) == QUERY_KIND {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(2, u32, this as u32, first);
        let child = *((this.add(CHILD_TASK)) as *const u32);
        if child == 0 {
            return 1;
        }
        let flags_ptr = (child as *mut u8).add(FLAGS) as *mut u32;
        if *flags_ptr & 1 != 0 {
            return 1;
        }
        let accepted = lf_checker_rt::callee_thiscall!(3, u32, child, this as u32, second, query_object as u32) & 0xFF != 0;
        if accepted {
            *flags_ptr |= 2;
            1
        } else {
            0
        }
    }
});
