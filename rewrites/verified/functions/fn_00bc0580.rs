// original: 0x00BC0580 build_and_attach_condition_task

/// Derive task flags from five input words, allocate a task, and assign it to
/// the ped's current task slot. The function is cdecl with nine stack words;
/// argument 2 is passed to the secondary lookup, argument 4 is copied as
/// float bits, and argument 9 is tested as signed when argument 5 is zero. A zero argument 8 adds flag
/// 0x8000; argument 7 selects 0x200, otherwise a nonzero argument 6 selects
/// 0x400. The constructor receives the derived flags and the scripted helper
/// results, then the assignment method receives the constructed task.
lf_checker_rt::export!(cdecl, rw_00BC0580(
    ped_handle: u32,
    unused_word: u32,
    resource_handle: u32,
    value_bits: u32,
    gate_a: u32,
    gate_b: u32,
    mode: u32,
    context_word: u32,
    signed_threshold: u32,
) -> u32 {
    const PED_POOL_GLOBAL: u32 = 0x018b_6f1c;
    const TASK_POOL_GLOBAL: u32 = 0x018b_69b4;
    const TASK_TYPE: u32 = 3;
    const TASK_ID: u32 = 1;
    const FLAG_CONTEXT: u32 = 0x20;
    const FLAG_MODE: u32 = 0x200;
    const FLAG_GATE: u32 = 0x400;
    const FLAG_NULL_CONTEXT: u32 = 0x8000;
    const PED_LOOKUP: u32 = 1;
    const RESOURCE_INDEX: u32 = 2;
    const RESOURCE_LOOKUP: u32 = 3;
    const TASK_ALLOC: u32 = 4;
    const TASK_CONSTRUCTOR: u32 = 5;
    const ASSIGN_TASK: u32 = 6;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    let ped_pool = unsafe {
        lf_checker_rt::global::<u32>(PED_POOL_GLOBAL).read_unaligned()
    };
    let ped = lf_checker_rt::callee_thiscall!(PED_LOOKUP, u32, ped_pool, ped_handle);
    let mut task_flags = 0_u32;
    if gate_a != 0 || ((signed_threshold as i32) > 0 && context_word == 0) {
        task_flags |= FLAG_CONTEXT;
    }
    if mode != 0 {
        task_flags |= FLAG_MODE;
    }
    if gate_b != 0 {
        task_flags |= FLAG_GATE;
    }
    if context_word == 0 {
        task_flags |= FLAG_NULL_CONTEXT;
    }

    let resource_index = lf_checker_rt::callee_cdecl!(RESOURCE_INDEX, u32, resource_handle);
    let resource_object = lf_checker_rt::callee_cdecl!(
        RESOURCE_LOOKUP,
        u32,
        unused_word,
        0_u32,
    );
    let task_pool = unsafe {
        lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL).read_unaligned()
    };
    let allocation = lf_checker_rt::callee_thiscall!(TASK_ALLOC, u32, task_pool);
    let task = if allocation != 0 {
        lf_checker_rt::callee_thiscall!(
            TASK_CONSTRUCTOR,
            u32,
            allocation,
            resource_index,
            resource_object,
            value_bits,
            TASK_TYPE,
            task_flags,
        )
    } else {
        0
    };
    let current_task = unsafe { read_word(ped.wrapping_add(0x6c)) };
    lf_checker_rt::callee_thiscall!(ASSIGN_TASK, u32, current_task, task, TASK_ID, TASK_ID)
});
