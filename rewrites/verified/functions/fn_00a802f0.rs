// original: 0x00a802f0 CComplexNewGetInVehicleTaskInfo::vf9

/// Task-info virtual slot 9 for CComplexNewGetInVehicleTaskInfo: allocate a task object through the
/// task manager and hand it to the per-task initialiser. (Role inferred from
/// shape: every neighbour in this family does the same with its own offsets.)
///
/// `this` is the task-info object (thiscall, in ECX); there are no stack
/// arguments and the result comes back in EAX.
///
/// Guards, checked first, each returning 0:
/// - returns 0 when the dword at this+0x1C and the word at this+0x20 are both 0.
///
/// The manager pointer is read from the shared global slot, then the
/// allocator callee runs with it in ECX. A null allocation returns 0.
/// Otherwise the initialiser runs with the new object in ECX and these
/// stack arguments, left to right:
/// 0. dword at +0x1C.
/// 1. zero-extended word at +0x20.
/// 2. dword at +0x24.
/// Its return value is the result.
///
/// Original: 0x00a802f0 (thiscall, this in ECX, no stack words).
lf_checker_rt::export!(thiscall, rw_00a802f0(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u32 {
            unsafe { (a as *const u8).read() as u32 }
        }
        const MANAGER_SLOT: u32 = 0x018B_69B4;
        const ALLOC_TASK: u32 = 1;
        const INIT_TASK: u32 = 2;

        if rd32(this.wrapping_add(0x1C)) == 0 && rd16(this.wrapping_add(0x20)) == 0 {
                    return 0;
                }
        let manager = lf_checker_rt::global::<u32>(MANAGER_SLOT).read();
        let task: u32 = lf_checker_rt::callee_thiscall!(ALLOC_TASK, u32, manager);
        if task == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(INIT_TASK, u32, task, rd32(this.wrapping_add(0x1C)), rd16(this.wrapping_add(0x20)), rd32(this.wrapping_add(0x24)))
    }
});
