// original: 0x00D01190 CTaskComplexNewUseCover::vf1

/// Clone a new-use-cover task, then propagate one control bit.
///
/// Asks the task manager (patched callee 1) for a clone target; a null answer
/// means no clone and the function returns null. Otherwise forwards the task's
/// slot word at `+0x30`, a pointer to its parameter block at `+0x20` and the
/// mode byte at `+0x34` to the clone routine (patched callee 2). When control
/// bit 2 of byte `+0x35` is set, the clone's status byte at `+0x35` is updated
/// from the task's bit 1: the low bit is placed at bit 2 with bit 1 forced.
/// A null clone with the control bit set faults on the original (kept as is).
///
/// Original: 0x00D01190 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d01190(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const ALLOC: u32 = 1;
        const CLONE: u32 = 2;
        const TASK_MANAGER_SLOT: u32 = 0x0167E2A0;
        const SLOT: u32 = 0x30;
        const PARAMS: u32 = 0x20;
        const MODE: u32 = 0x34;
        const CONTROL: u32 = 0x35;
        const CONTROL_BIT: u8 = 4;

        let manager = lf_checker_rt::global::<u32>(TASK_MANAGER_SLOT).read();
        let target = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        let mut clone = 0u32;
        if target != 0 {
            let mode = ((this + MODE) as *const u8).read() as u32;
            clone = lf_checker_rt::callee_thiscall!(CLONE, u32, target,
                rd32(this + SLOT), this + PARAMS, mode);
        }
        if ((this + CONTROL) as *const u8).read() & CONTROL_BIT != 0 {
            let bit = ((((this + CONTROL) as *const u8).read() >> 1) & 1) | 2;
            let merged = bit.wrapping_add(bit);
            let status = ((clone + CONTROL) as *const u8).read() & 0xfd;
            ((clone + CONTROL) as *mut u8).write(status | merged);
        }
        clone
    }
});
