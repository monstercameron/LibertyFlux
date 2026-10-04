// original: 0x00b6e5a0 CTaskSimpleCreateCarAndGetIn::vf1

/// Clone a task (clone a create-car-and-get-in task, forwarding a pointer to its +0x20 block).
///
/// `this` points to the source task. The function loads the task pool
/// pointer from its global slot, allocates a fresh task through the pool
/// allocator (callee 1, thiscall/0), and returns 0 when allocation fails
/// Otherwise it forwards this task's fields to the class-specific
/// second callee (thiscall/3) with the allocator result in ECX and
/// returns that callee's answer
/// unchanged.
///
/// Layout read: interior pointer at +0x20, dword at +0x30, byte at +0x34.
///
/// Original: 0x00b6e5a0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00b6e5a0(this: u32) -> u32 {
    unsafe {
        const TASK_POOL_SLOT: u32 = 0x167e2a0;
        const BLOCK0_OFF: u32 = 0x20;
        const ARG1_OFF: u32 = 0x30;
        const FLAG2_OFF: u32 = 0x34;
        let pool = lf_checker_rt::global::<u32>(TASK_POOL_SLOT).read();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(1, u32, pool);
        if fresh == 0 {
            return 0;
        }
        let a0 = this + BLOCK0_OFF;
        let a1 = ((this + ARG1_OFF) as *const u32).read_unaligned();
        let a2 = ((this + FLAG2_OFF) as *const u8).read() as u32;
        lf_checker_rt::callee_thiscall!(2, u32, fresh, a0, a1, a2)
    }
});
