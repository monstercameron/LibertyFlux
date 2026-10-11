// original: 0x005E8B80 CTaskComplexWanderCop::CTaskComplexWanderCop_2

/// Initialize a wander-cop task from three stack words and two low-lane floats.
///
/// `this` is the derived task object. The constructor forwards the low 32-bit
/// payloads of XMM1 and XMM2, followed by its three stack arguments, to the
/// base wander-task constructor in that order. It then installs the derived
/// vtable, clears the task and subtask fields at offsets `0x80` through
/// `0xA4`, and stores the default 8.0 and 30.0 float bit patterns at `0x24`
/// and `0x28`. The helper call is thiscall with five stack words; the outer
/// constructor returns `this` and pops its three incoming stack words.
lf_checker_rt::export!(thiscall, rw_005e8b80(this: u32, first_arg: u32, second_arg: u32, third_arg: u32) -> u32 {
    const WANDER_COP_VTABLE_VA: u32 = 0x00FE1314;
    const BASE_SUBTASK_OFFSET: u32 = 0x80;
    const TASK_STATE_OFFSET: u32 = 0x84;
    const TASK_STATE_WORD_OFFSET: u32 = 0x88;
    const TASK_FLAGS_OFFSET: u32 = 0x8c;
    const TASK_TARGET_OFFSET: u32 = 0x90;
    const TASK_TIMER_OFFSET: u32 = 0x94;
    const TASK_TIMER_FLAGS_OFFSET: u32 = 0x98;
    const TASK_ROUTE_OFFSET: u32 = 0x9c;
    const TASK_NODE_OFFSET: u32 = 0xa0;
    const TASK_EXTRA_OFFSET: u32 = 0xa4;
    const DEFAULT_RADIUS_BITS: u32 = 0x41000000;
    const DEFAULT_DISTANCE_BITS: u32 = 0x41f00000;
    const DEFAULT_RADIUS_OFFSET: u32 = 0x24;
    const DEFAULT_DISTANCE_OFFSET: u32 = 0x28;

    let direction_x_bits = lf_checker_rt::xmm_word(1, 0);
    let direction_y_bits = lf_checker_rt::xmm_word(2, 0);
    let _: u32 = lf_checker_rt::callee_thiscall!(
        1,
        u32,
        this,
        direction_x_bits,
        direction_y_bits,
        first_arg,
        second_arg,
        third_arg
    );

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(WANDER_COP_VTABLE_VA));
        (this.wrapping_add(BASE_SUBTASK_OFFSET) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TASK_STATE_OFFSET) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TASK_STATE_WORD_OFFSET) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TASK_FLAGS_OFFSET) as *mut u16).write_unaligned(0);
        (this.wrapping_add(TASK_TARGET_OFFSET) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TASK_TIMER_OFFSET) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TASK_TIMER_FLAGS_OFFSET) as *mut u16).write_unaligned(0);
        (this.wrapping_add(TASK_ROUTE_OFFSET) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TASK_NODE_OFFSET) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TASK_EXTRA_OFFSET) as *mut u32).write_unaligned(0);
        (this.wrapping_add(DEFAULT_RADIUS_OFFSET) as *mut u32)
            .write_unaligned(DEFAULT_RADIUS_BITS);
        (this.wrapping_add(DEFAULT_DISTANCE_OFFSET) as *mut u32)
            .write_unaligned(DEFAULT_DISTANCE_BITS);
    }
    this
});
