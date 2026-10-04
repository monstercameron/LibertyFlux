// original: 0x00b6e510 CTaskSimpleCarShuffle::vf1

/// Clone a task (clone a car-shuffle task from its two seat/door fields).
///
/// `this` points to the source task. The function loads the task pool
/// pointer from its global slot, allocates a fresh task through the pool
/// allocator (callee 1, thiscall/0), and returns 0 when allocation fails
/// Otherwise it forwards this task's fields to the class-specific
/// second callee (thiscall/2) with the allocator result in ECX and
/// returns that callee's answer
/// unchanged.
///
/// Layout read: dword at +0x1c, dword at +0x20.
///
/// Original: 0x00b6e510 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00b6e510(this: u32) -> u32 {
    unsafe {
        const TASK_POOL_SLOT: u32 = 0x167e2a0;
        const ARG0_OFF: u32 = 0x1c;
        const ARG1_OFF: u32 = 0x20;
        let pool = lf_checker_rt::global::<u32>(TASK_POOL_SLOT).read();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(1, u32, pool);
        if fresh == 0 {
            return 0;
        }
        let a0 = ((this + ARG0_OFF) as *const u32).read_unaligned();
        let a1 = ((this + ARG1_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, fresh, a0, a1)
    }
});
