// original: 0x00a806f0 CSitDownIdleThenStandTaskInfo::create_task

/// Allocate and initialize the sit-down-then-stand task.
///
/// The `thiscall` receiver stores two constructor words at `+0x18` and `+0x1C`,
/// an exact single-precision argument at `+0x20`, a byte flag at `+0x24`, and
/// an optional embedded-data flag at `+0x25`. A nonzero `+0x25` selects the
/// receiver address `+0x30`; zero selects a null pointer. If allocation fails
/// the method returns zero without reading those fields. Otherwise it passes
/// the two words, the float's original bits, the zero-extended byte flag and
/// the optional pointer to the constructor in that order and returns its `EAX`.
/// The float travels as a raw word because the original places it in a stack
/// argument slot without performing floating-point arithmetic.
lf_checker_rt::export!(thiscall, rw_00a806f0(task_info: u32) -> u32 {
    const TASK_POOL_MANAGER_VA: u32 = 0x018B69B4;
    const FIRST_TASK_WORD: u32 = 0x18;
    const SECOND_TASK_WORD: u32 = 0x1C;
    const FLOAT_ARGUMENT: u32 = 0x20;
    const TASK_FLAGS: u32 = 0x24;
    const HAS_EMBEDDED_DATA: u32 = 0x25;
    const EMBEDDED_DATA: u32 = 0x30;
    const TASK_POOL_ALLOCATOR: u32 = 1;
    const SIT_STAND_TASK_CONSTRUCTOR: u32 = 2;

    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOCATOR, u32, manager);
    if task == 0 {
        return 0;
    }

    let embedded_data_flag = unsafe {
        ((task_info.wrapping_add(HAS_EMBEDDED_DATA)) as *const u8).read()
    };
    let embedded_data = if embedded_data_flag != 0 {
        task_info.wrapping_add(EMBEDDED_DATA)
    } else {
        0
    };
    let first_word = unsafe {
        ((task_info.wrapping_add(FIRST_TASK_WORD)) as *const u32).read_unaligned()
    };
    let second_word = unsafe {
        ((task_info.wrapping_add(SECOND_TASK_WORD)) as *const u32).read_unaligned()
    };
    let float_bits = unsafe {
        ((task_info.wrapping_add(FLOAT_ARGUMENT)) as *const u32).read_unaligned()
    };
    let task_flags = u32::from(unsafe {
        ((task_info.wrapping_add(TASK_FLAGS)) as *const u8).read()
    });

    lf_checker_rt::callee_thiscall!(
        SIT_STAND_TASK_CONSTRUCTOR,
        u32,
        task,
        first_word,
        second_word,
        float_bits,
        task_flags,
        embedded_data
    )
});
