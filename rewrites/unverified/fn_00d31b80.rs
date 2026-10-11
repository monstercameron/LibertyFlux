// original: 0x00D31B80 CTaskComplexWaitForSteppingOut::CTaskComplexWaitForSteppingOut
/// Constructs a stepping-out task from the source duration at byte offset 0x14.
/// It returns null when the task-pool allocator returns null; otherwise it calls
/// the class constructor, writes the relocated class vtable at destination offset 0,
/// copies the duration bits to destination offset 0x14, and returns the destination.
/// The receiver is passed in ECX using the thiscall convention.
lf_checker_rt::export!(thiscall, rw_00d31b80(this: u32) -> u32 {
    const TASK_POOL_GLOBAL_FILE_VA: u32 = 0x0167e2a0;
    let pool_manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL_FILE_VA).read_unaligned() };
    let destination = lf_checker_rt::callee_thiscall!(1, u32, pool_manager);
    if destination == 0 { return 0; }
    const SOURCE_DURATION_OFFSET: u32 = 0x14;
    const DESTINATION_VTABLE_OFFSET: u32 = 0;
    const DESTINATION_DURATION_OFFSET: u32 = 0x14;
    let duration_bits = unsafe { (this.wrapping_add(SOURCE_DURATION_OFFSET) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(2, u32, destination, duration_bits);
    const CLASS_VTABLE_FILE_VA: u32 = 0x00ee3244;
    let vtable = lf_checker_rt::relocated(CLASS_VTABLE_FILE_VA);
    unsafe {
        (destination.wrapping_add(DESTINATION_VTABLE_OFFSET) as *mut u32).write_unaligned(vtable);
        (destination.wrapping_add(DESTINATION_DURATION_OFFSET) as *mut u32).write_unaligned(duration_bits);
    }
    destination
});
