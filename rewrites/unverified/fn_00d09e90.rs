// original: 0x00D09E90 CTaskComplexCombatFlankSubtask::vf19

/// Clone a combat-flank subtask through the task manager.
///
/// Null from the manager (patched callee 1) means no clone. Otherwise forwards
/// the kind constant 15, the anchor object at `+0x18`, its aim block at
/// `+0x20`+`0x30`, the fixed speed constant (239.0 as bits) and -1 to the
/// clone routine (patched callee 2) and returns its result. The incoming stack
/// word is not read.
///
/// Original: 0x00D09E90 (thiscall, one unread stack word, callee pops).
lf_checker_rt::export!(thiscall, rw_00d09e90(this: u32, _a: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const ALLOC: u32 = 1;
        const CLONE: u32 = 2;
        const TASK_MANAGER_SLOT: u32 = 0x0167E2A0;
        const ANCHOR: u32 = 0x18;
        const AIM_BASE: u32 = 0x20;
        const AIM_OFF: u32 = 0x30;
        const KIND: u32 = 0x0f;
        const SPEED_BITS: u32 = 0x436F0000; // 239.0f

        let manager = lf_checker_rt::global::<u32>(TASK_MANAGER_SLOT).read();
        let target = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        if target == 0 {
            return 0;
        }
        let anchor = rd32(this + ANCHOR);
        let aim = rd32(anchor + AIM_BASE).wrapping_add(AIM_OFF);
        lf_checker_rt::callee_thiscall!(CLONE, u32, target,
            KIND, anchor, aim, SPEED_BITS, 0xFFFFFFFFu32)
    }
});
