// original: 0x00D31B80 CTaskComplexWaitForSteppingOut::CTaskComplexWaitForSteppingOut
/// Allocates a task-pool object, calls its class constructor with words
/// read from the source object, copies the class-specific trailing
/// fields, and returns the allocated pointer. Object fields are read
/// unaligned as 32-bit words except for the two one-byte copied flags.
lf_checker_rt::export!(thiscall, rw_00d31b80(this: u32) -> u32 {
    const TASK_POOL_GLOBAL_FILE_VA: u32 = 0x0167e2a0;
    let pool_manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL_FILE_VA).read_unaligned() };
    let destination = lf_checker_rt::callee_thiscall!(1, u32, pool_manager);
    if destination == 0 { return 0; }
    let duration_bits = unsafe { (this.wrapping_add(0x14) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(2, u32, destination, duration_bits);
    const CLASS_VTABLE_FILE_VA: u32 = 0x00ee3244;
    let vtable = lf_checker_rt::relocated(CLASS_VTABLE_FILE_VA);
    unsafe {
        (destination as *mut u32).write_unaligned(vtable);
        (destination.wrapping_add(0x14) as *mut u32).write_unaligned(duration_bits);
    }
    destination
});
