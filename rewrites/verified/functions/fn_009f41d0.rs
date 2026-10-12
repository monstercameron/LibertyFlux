// original: 0x009F41D0 ped_task_time_gate_open
/// Tests a task flag and compares one f32 deadline. The branch is signed by
/// COMISS flags: less-than or unordered returns one; equality returns zero.
lf_checker_rt::export!(thiscall, rw_009F41D0(task: u32, deadline_bits: u32) -> u8 {
    unsafe {
        const TASK_FLAGS: u32 = 0x260;
        const TIME_GATE: u32 = 0xB28;
        const HAS_TIME_GATE: u8 = 0x08;
        let flags = (task.wrapping_add(TASK_FLAGS) as *const u8).read();
        if flags & HAS_TIME_GATE == 0 {
            return 1;
        }
        let stored = f32::from_bits((task.wrapping_add(TIME_GATE) as *const u32).read_unaligned());
        let deadline = f32::from_bits(deadline_bits);
        u8::from(!(deadline >= stored))
    }
});
