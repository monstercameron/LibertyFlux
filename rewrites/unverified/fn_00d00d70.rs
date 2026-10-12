// original: 0x00D00D70 CTaskComplexCombat::vf1
/// Allocate a task from the shared task pool, forward the source object's constructor fields in the original ABI order, and return the allocated task. The receiver is the source task and the pool manager is loaded from the engine's task-pool slot. Constructor float arguments preserve their source bit patterns.
lf_checker_rt::export!(thiscall, rw_00d00d70(this: u32) -> u32 {
    const TASK_POOL_MANAGER_FILE_VA: u32 = 0x0167E2A0;
    let pool_manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_FILE_VA).read_unaligned() };
    let destination = lf_checker_rt::callee_thiscall!(1, u32, pool_manager);
    if destination == 0 { return 0; } // The contract keeps this original-faulting allocator path out of trials.
    const SOURCE_ARG_0_OFFSET: u32 = 0x3c;
    let arg_0 = unsafe { (this.wrapping_add(SOURCE_ARG_0_OFFSET) as *const u32).read_unaligned() };
    const SOURCE_ARG_1_OFFSET: u32 = 0x5c;
    let arg_1 = unsafe { (this.wrapping_add(SOURCE_ARG_1_OFFSET) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(2, u32, destination, arg_0, arg_1);
    const SOURCE_FLAGS_OFFSET: u32 = 0x60;
    const DESTINATION_FLAGS_OFFSET: u32 = 0x60;
    const COPIED_FLAGS: u32 = 0x0006_40F8;
    let source_flags = unsafe { (this.wrapping_add(SOURCE_FLAGS_OFFSET) as *const u32).read_unaligned() };
    let destination_flags = unsafe { (destination.wrapping_add(DESTINATION_FLAGS_OFFSET) as *const u32).read_unaligned() };
    let updated_flags = (destination_flags & !COPIED_FLAGS) | (source_flags & COPIED_FLAGS);
    unsafe { (destination.wrapping_add(DESTINATION_FLAGS_OFFSET) as *mut u32).write_unaligned(updated_flags); }
    const SOURCE_BYTE_OFFSET: u32 = 0x64;
    let copied_byte = unsafe { (this.wrapping_add(SOURCE_BYTE_OFFSET) as *const u8).read_unaligned() };
    unsafe { (destination.wrapping_add(SOURCE_BYTE_OFFSET) as *mut u8).write_unaligned(copied_byte); }
    destination
});
