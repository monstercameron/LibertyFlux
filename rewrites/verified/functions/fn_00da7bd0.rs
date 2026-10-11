// original: 0x00DA7BD0 CTaskComplexFleeShooting::vf19

/// Seed the flee-shooting task's tick fields and tail-forward its ped
/// argument to the shared update handler.
///
/// The global dword supplies the value stored at task offset `+0x68`;
/// `+0x6C` is reset to the all-ones sentinel and the byte at `+0x70` is
/// set to one. The handler receives `this` and the original stack word,
/// and its return value becomes this method's result.
lf_checker_rt::export!(thiscall, rw_00DA7BD0(this: u32, ped: u32) -> u32 {
    unsafe {
        const GLOBAL_TICK_FILE_VA: u32 = 0x011735B4;
        const TASK_TICK: u32 = 0x68;
        const TASK_SENTINEL: u32 = 0x6C;
        const TASK_ACTIVE: u32 = 0x70;
        const UNSET_TICK: u32 = 0xFFFF_FFFF;
        const UPDATE_HANDLER: u32 = 1;

        let tick = (lf_checker_rt::relocated(GLOBAL_TICK_FILE_VA) as *const u32).read();
        (this.wrapping_add(TASK_TICK) as *mut u32).write_unaligned(tick);
        (this.wrapping_add(TASK_SENTINEL) as *mut u32).write_unaligned(UNSET_TICK);
        (this.wrapping_add(TASK_ACTIVE) as *mut u8).write(1);
        lf_checker_rt::callee_thiscall!(UPDATE_HANDLER, u32, this, ped)
    }
});
