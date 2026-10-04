// original: 0x00b6e570 CTaskSimpleCarSlowDragPedOut::vf1

/// Clone a task (clone a slow-drag-ped-out task from two fields and a flag byte).
///
/// `this` points to the source task. The function loads the task pool
/// pointer from its global slot, allocates a fresh task through the pool
/// allocator (callee 1, thiscall/0), and returns 0 when allocation fails
/// Otherwise it forwards this task's fields to the class-specific
/// second callee (thiscall/3) with the allocator result in ECX and
/// returns that callee's answer
/// unchanged.
///
/// Layout read: dword at +0x24, dword at +0x28, byte at +0x2c.
///
/// Original: 0x00b6e570 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00b6e570(this: u32) -> u32 {
    unsafe {
        const TASK_POOL_SLOT: u32 = 0x167e2a0;
        const ARG0_OFF: u32 = 0x24;
        const ARG1_OFF: u32 = 0x28;
        const FLAG2_OFF: u32 = 0x2c;
        let pool = lf_checker_rt::global::<u32>(TASK_POOL_SLOT).read();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(1, u32, pool);
        if fresh == 0 {
            return 0;
        }
        let a0 = ((this + ARG0_OFF) as *const u32).read_unaligned();
        let a1 = ((this + ARG1_OFF) as *const u32).read_unaligned();
        let a2 = ((this + FLAG2_OFF) as *const u8).read() as u32;
        lf_checker_rt::callee_thiscall!(2, u32, fresh, a0, a1, a2)
    }
});
