// original: 0x00be0630 CTaskComplexInWater::vf1
/// Build a water task through the manager and, when the source flag at `this + 0x68` is nonzero, copy three float words from offsets 0x20, 0x24 and 0x28 plus the defined uninitialized stack word into the destination at the same offsets and 0x2c. The destination flag is then set to 1. The manager receives the words at offsets 0x44 and 0x48 and a zero third argument. All float values are moved as raw bits without arithmetic. The contract sets the original's otherwise unwritten stack fill to zero. Calling convention: thiscall with no stack arguments; returns the destination pointer or zero.
lf_checker_rt::export!(thiscall, rw_00be0630(this: u32) -> u32 {
    const TASK_MANAGER_POINTER_VA: u32 = 0x0167_E2A0;
    const TASK_MANAGER_LOOKUP: u32 = 1;
    const BUILD_WATER_TASK: u32 = 2;
    const DESTINATION_FLAG_OFFSET: u32 = 0x68;
    const UNWRITTEN_STACK_FLOAT_BITS: u32 = 0;
    let read_word = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let read_byte = |address: u32| unsafe { (address as *const u8).read() };
    let write_word = |address: u32, value: u32| unsafe { (address as *mut u32).write_unaligned(value) };
    let write_byte = |address: u32, value: u8| unsafe { (address as *mut u8).write(value) };
    let task_manager_context = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER_POINTER_VA).read_unaligned() };
    let manager_context = lf_checker_rt::callee_thiscall!(TASK_MANAGER_LOOKUP, u32, task_manager_context);
    let destination = if manager_context == 0 { 0 } else {
        lf_checker_rt::callee_thiscall!(BUILD_WATER_TASK, u32, manager_context,
            read_word(this.wrapping_add(0x44)), read_word(this.wrapping_add(0x48)), 0)
    };
    if read_byte(this.wrapping_add(DESTINATION_FLAG_OFFSET)) != 0 {
        write_word(destination.wrapping_add(0x20), read_word(this.wrapping_add(0x20)));
        write_word(destination.wrapping_add(0x24), read_word(this.wrapping_add(0x24)));
        write_word(destination.wrapping_add(0x28), read_word(this.wrapping_add(0x28)));
        write_word(destination.wrapping_add(0x2C), UNWRITTEN_STACK_FLOAT_BITS);
        write_byte(destination.wrapping_add(DESTINATION_FLAG_OFFSET), 1);
    }
    destination
});
