// original: 0x00D011E0 CTaskComplexSeperate::vf1

/// Clone a separate task through the task manager.
///
/// Null from the manager (patched callee 1) means no clone. Otherwise forwards
/// the waypoint block at `+0x20`, the leash block at `+0x80`, the words at
/// `+0xac`/`+0xb0` and bit 1 of the flag byte at `+0xb8` to the clone routine
/// (patched callee 2) and returns its result.
///
/// Original: 0x00D011E0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d011e0(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const ALLOC: u32 = 1;
        const CLONE: u32 = 2;
        const TASK_MANAGER_SLOT: u32 = 0x0167E2A0;
        const WAYPOINTS: u32 = 0x20;
        const LEASH: u32 = 0x80;
        const RANGE: u32 = 0xac;
        const FLAGS2: u32 = 0xb0;
        const FLAGS: u32 = 0xb8;

        let manager = lf_checker_rt::global::<u32>(TASK_MANAGER_SLOT).read();
        let target = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        if target == 0 {
            return 0;
        }
        let bit = ((((this + FLAGS) as *const u8).read() >> 1) & 1) as u32;
        lf_checker_rt::callee_thiscall!(CLONE, u32, target,
            this + WAYPOINTS, this + LEASH, rd32(this + RANGE), rd32(this + FLAGS2), bit)
    }
});
