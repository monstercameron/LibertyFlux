// original: 0x00D00F50 CTaskComplexCombatFireSubtask::vf1
/// Allocate a task from the shared task pool, forward the source object's constructor fields in the original ABI order, and return the allocated task. The receiver is the source task and the pool manager is loaded from the engine's task-pool slot. Constructor float arguments preserve their source bit patterns.
lf_checker_rt::export!(thiscall, rw_00d00f50(this: u32) -> u32 {
    const TASK_POOL_MANAGER_FILE_VA: u32 = 0x0167E2A0;
    let pool_manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_FILE_VA).read_unaligned() };
    let destination = lf_checker_rt::callee_thiscall!(1, u32, pool_manager);
    if destination == 0 { return 0; }
    const SOURCE_ARG_0_OFFSET: u32 = 0x18;
    let arg_0 = unsafe { (this.wrapping_add(SOURCE_ARG_0_OFFSET) as *const u32).read_unaligned() };
    const SOURCE_ARG_1_OFFSET: u32 = 0x50;
    let arg_1 = this.wrapping_add(SOURCE_ARG_1_OFFSET);
    const SOURCE_ARG_2_OFFSET: u32 = 0x60;
    let arg_2 = (unsafe { (this.wrapping_add(SOURCE_ARG_2_OFFSET) as *const u8).read_unaligned() } as u32 >> 0) & 1;
    const SOURCE_ARG_3_OFFSET: u32 = 0x60;
    let arg_3 = (unsafe { (this.wrapping_add(SOURCE_ARG_3_OFFSET) as *const u8).read_unaligned() } as u32 >> 1) & 1;
    const SOURCE_ARG_4_OFFSET: u32 = 0x40;
    let arg_4 = unsafe { (this.wrapping_add(SOURCE_ARG_4_OFFSET) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(2, u32, destination, arg_0, arg_1, arg_2, arg_3, arg_4);
    destination
});
