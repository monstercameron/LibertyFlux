// original: 0x00D31A60 CTaskComplexNewExitVehicle::clone
/// Allocates a destination task through the shared task-pool slot, calls its
/// constructor with source words at offsets 0x18, 0x14, 0x44, 0x48, then copies source
/// fields to destination fields at these byte offsets: 0x2c->0x2c, 0x50->0x50, 0x54->0x54, 0x58->0x58, 0x44->0x5c.
/// The source receiver is in ECX and the function returns the allocated pointer.
lf_checker_rt::export!(thiscall, rw_00d31a60(this: u32) -> u32 {
    const TASK_POOL_GLOBAL_FILE_VA: u32 = 0x0167e2a0;
    let pool_manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL_FILE_VA).read_unaligned() };
    let destination = lf_checker_rt::callee_thiscall!(1, u32, pool_manager);
    if destination == 0 { return 0; }
    const SOURCE_ARG_0_OFFSET: u32 = 0x18;
    let arg_0 = unsafe { (this.wrapping_add(SOURCE_ARG_0_OFFSET) as *const u32).read_unaligned() };
    const SOURCE_ARG_1_OFFSET: u32 = 0x14;
    let arg_1 = unsafe { (this.wrapping_add(SOURCE_ARG_1_OFFSET) as *const u32).read_unaligned() };
    const SOURCE_ARG_2_OFFSET: u32 = 0x44;
    let arg_2 = unsafe { (this.wrapping_add(SOURCE_ARG_2_OFFSET) as *const u32).read_unaligned() };
    const SOURCE_ARG_3_OFFSET: u32 = 0x48;
    let arg_3 = unsafe { (this.wrapping_add(SOURCE_ARG_3_OFFSET) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(2, u32, destination, arg_0, arg_1, arg_2, arg_3);
    const SOURCE_FIELD_2c_OFFSET: u32 = 0x2c;
    const DESTINATION_FIELD_2c_OFFSET: u32 = 0x2c;
    let value_2c = unsafe { (this.wrapping_add(SOURCE_FIELD_2c_OFFSET) as *const u32).read_unaligned() };
    unsafe { (destination.wrapping_add(DESTINATION_FIELD_2c_OFFSET) as *mut u32).write_unaligned(value_2c); }
    const SOURCE_FIELD_50_OFFSET: u32 = 0x50;
    const DESTINATION_FIELD_50_OFFSET: u32 = 0x50;
    let value_50 = unsafe { (this.wrapping_add(SOURCE_FIELD_50_OFFSET) as *const u32).read_unaligned() };
    unsafe { (destination.wrapping_add(DESTINATION_FIELD_50_OFFSET) as *mut u32).write_unaligned(value_50); }
    const SOURCE_FIELD_54_OFFSET: u32 = 0x54;
    const DESTINATION_FIELD_54_OFFSET: u32 = 0x54;
    let value_54 = unsafe { (this.wrapping_add(SOURCE_FIELD_54_OFFSET) as *const u32).read_unaligned() };
    unsafe { (destination.wrapping_add(DESTINATION_FIELD_54_OFFSET) as *mut u32).write_unaligned(value_54); }
    const SOURCE_FIELD_58_OFFSET: u32 = 0x58;
    const DESTINATION_FIELD_58_OFFSET: u32 = 0x58;
    let value_58 = unsafe { (this.wrapping_add(SOURCE_FIELD_58_OFFSET) as *const u32).read_unaligned() };
    unsafe { (destination.wrapping_add(DESTINATION_FIELD_58_OFFSET) as *mut u32).write_unaligned(value_58); }
    const SOURCE_EXTRA_44_OFFSET: u32 = 0x44;
    const DESTINATION_EXTRA_44_OFFSET: u32 = 0x5c;
    let value_44_extra = unsafe { (this.wrapping_add(SOURCE_EXTRA_44_OFFSET) as *const u32).read_unaligned() };
    unsafe { (destination.wrapping_add(DESTINATION_EXTRA_44_OFFSET) as *mut u32).write_unaligned(value_44_extra); }
    destination
});
