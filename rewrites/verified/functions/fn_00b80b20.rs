// original: 0x00B80B20 ped_task_manager_request_first_bomb_task

/// Request bomb task kind one when the object has a nonzero word at `+0x04`.
/// The task interface takes three stack words and returns a value that this
/// wrapper does not expose as a defined result.
lf_checker_rt::export!(thiscall, rw_b80b20(this: u32) -> u32 {
    unsafe {
        const FIRST_TASK: u32 = 0x04;
        const TASK_INTERFACE: u32 = 1;

        if ((this + FIRST_TASK) as *const u32).read_unaligned() != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(TASK_INTERFACE, u32, this, 0u32, 1u32, 0u32);
        }
        0
    }
});
