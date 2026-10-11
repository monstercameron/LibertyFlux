// original: 0x00D319E0 CTaskComplexClimbIntoVehicle::clone
/// Allocates a task-pool object, calls its class constructor with words
/// read from the source object, copies the class-specific trailing
/// fields, and returns the allocated pointer. Object fields are read
/// unaligned as 32-bit words except for the two one-byte copied flags.
lf_checker_rt::export!(thiscall, rw_00d319e0(this: u32) -> u32 {
    const TASK_POOL_GLOBAL_FILE_VA: u32 = 0x0167e2a0;
    let pool_manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL_FILE_VA).read_unaligned() };
    let destination = lf_checker_rt::callee_thiscall!(1, u32, pool_manager);
    if destination == 0 { return 0; }
    let arg_0 = unsafe { (this.wrapping_add(0x18) as *const u32).read_unaligned() };
    let arg_1 = unsafe { (this.wrapping_add(0x24) as *const u32).read_unaligned() };
    let arg_2 = unsafe { (this.wrapping_add(0x20) as *const u32).read_unaligned() };
    let arg_3 = unsafe { (this.wrapping_add(0x14) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(2, u32, destination, arg_0, arg_1, arg_2, arg_3);
    let value_2c = unsafe { (this.wrapping_add(0x2c) as *const u32).read_unaligned() };
    unsafe { (destination.wrapping_add(0x2c) as *mut u32).write_unaligned(value_2c); }
    destination
});
