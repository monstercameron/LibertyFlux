// original: 0x00b6e4c0 CTaskSimpleCarSetPedOut::vf1

/// Clone a task (clone a set-ped-out task and copy its flag bytes into the new task).
///
/// `this` points to the source task. The function loads the task pool
/// pointer from its global slot, allocates a fresh task through the pool
/// allocator (callee 1, thiscall/0), and returns 0 when allocation fails
/// except that the shared copy tail still runs: with a null destination
/// the flag-byte stores fault, exactly like the original.
/// Otherwise it forwards this task's fields to the class-specific
/// second callee (thiscall/4) with the allocator result in ECX and
/// returns that callee's answer
/// after copying the four flag bytes (+0x1d..+0x20) into it.
///
/// Layout read: dword at +0x14, dword at +0x18, byte at +0x1c, byte at +0x21.
///
/// Original: 0x00b6e4c0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00b6e4c0(this: u32) -> u32 {
    unsafe {
        const TASK_POOL_SLOT: u32 = 0x167e2a0;
        const ARG0_OFF: u32 = 0x14;
        const ARG1_OFF: u32 = 0x18;
        const FLAG2_OFF: u32 = 0x1c;
        const FLAG3_OFF: u32 = 0x21;
        let pool = lf_checker_rt::global::<u32>(TASK_POOL_SLOT).read();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(1, u32, pool);
        let a0 = ((this + ARG0_OFF) as *const u32).read_unaligned();
        let a1 = ((this + ARG1_OFF) as *const u32).read_unaligned();
        let a2 = ((this + FLAG2_OFF) as *const u8).read() as u32;
        let a3 = ((this + FLAG3_OFF) as *const u8).read() as u32;
        let dst: u32 = if fresh == 0 { 0 } else { lf_checker_rt::callee_thiscall!(2, u32, fresh, a0, a1, a2, a3) };
        ((dst + 0x1d) as *mut u8).write_volatile(((this + 0x1d) as *const u8).read());
        ((dst + 0x1e) as *mut u8).write_volatile(((this + 0x1e) as *const u8).read());
        ((dst + 0x1f) as *mut u8).write_volatile(((this + 0x1f) as *const u8).read());
        ((dst + 0x20) as *mut u8).write_volatile(((this + 0x20) as *const u8).read());
        dst
    }
});
