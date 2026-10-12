// original: 0x00cfbb60 create_timed_task_effect
/// Toggles the sign bit of argument four and forwards it to the cleanup helper.
/// It asks the receiver at first_object + 0x78 to create a task from arguments two,
/// three, four and sentinel -1, then stores the result at this + 0x60. For a non-null
/// task it writes argument five at task + 0x54 and initializes the task. The return
/// is a C++ bool in AL.
lf_checker_rt::export!(thiscall, rw_00cfbb60(this: *mut u8, first_object: *const u8, second: u32, third: u32, fourth_bits: u32, fifth_bits: u32) -> u32 {
    unsafe {
        const RECEIVER_OFFSET: usize = 0x78;
        const TASK_OFFSET: usize = 0x60;
        const TASK_AMOUNT: usize = 0x54;
        const SIGN_BIT: u32 = 0x80000000;
        let signed_amount = fourth_bits ^ SIGN_BIT;
        lf_checker_rt::callee_thiscall!(1, u32, this as u32, signed_amount);
        let receiver = *((first_object.add(RECEIVER_OFFSET)) as *const u32);
        let task = lf_checker_rt::callee_thiscall!(2, u32, receiver, second, third, fourth_bits, u32::MAX);
        *((this.add(TASK_OFFSET)) as *mut u32) = task;
        if task == 0 {
            return 0;
        }
        *((task as *mut u8).add(TASK_AMOUNT) as *mut u32) = fifth_bits;
        lf_checker_rt::callee_thiscall!(3, u32, task, 2, 0, this as u32);
        1
    }
});
