// original: 0x00C7EA30 CTaskComplexWanderingScenario::create_first_sub_task

/// Build the first scenario task pair from the receiver's type word at
/// `+0x14`. Mode `0x49` selects a speed of 2.0; other modes select 1.0.
/// The function asks the shared task pool for a wander task, then for an
/// attachable task, and optionally constructs an ambient task from a third
/// allocation. It attaches the two constructor results to the second task
/// and returns the attach helper's result. A missing first or third
/// allocation suppresses only that constructor; a missing second allocation
/// returns zero. The incoming stack word is unused but is removed by the
/// thiscall callee.
lf_checker_rt::export!(thiscall, rw_00c7ea30(this: u32, _unused: u32) -> u32 {
    const TASK_TYPE_OFFSET: u32 = 0x14;
    const TASK_POOL_GLOBAL_FILE_VA: u32 = 0x0167_E2A0;
    const MODE_WITH_DOUBLE_SPEED: u32 = 0x49;
    const SPEED_NORMAL_BITS: u32 = 0x3F80_0000; // 1.0f32
    const SPEED_DOUBLE_BITS: u32 = 0x4000_0000; // 2.0f32
    const BLEND_WEIGHT_BITS: u32 = 0x3D4C_CCCD; // 0.05f32
    const NEGATIVE_ONE_BITS: u32 = 0xBF80_0000; // -1.0f32
    const TASK_POOL_ALLOC: u32 = 1;
    const WANDER_CONSTRUCTOR: u32 = 2;
    const AMBIENT_CONSTRUCTOR: u32 = 3;
    const ATTACH_TO_INTELLIGENCE: u32 = 4;

    let task_type = unsafe {
        (this.wrapping_add(TASK_TYPE_OFFSET) as *const u32).read_unaligned()
    };
    let speed_bits = if task_type == MODE_WITH_DOUBLE_SPEED {
        SPEED_DOUBLE_BITS
    } else {
        SPEED_NORMAL_BITS
    };
    let pool_manager = unsafe {
        lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL_FILE_VA).read_unaligned()
    };

    let wander_allocation =
        lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOC, u32, pool_manager);
    let wander_task = if wander_allocation != 0 {
        lf_checker_rt::callee_thiscall!(
            WANDER_CONSTRUCTOR,
            u32,
            wander_allocation,
            speed_bits,
            0,
            1,
            BLEND_WEIGHT_BITS,
            1,
        )
    } else {
        0
    };

    let attachable_allocation =
        lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOC, u32, pool_manager);
    if attachable_allocation == 0 {
        return 0;
    }

    let ambient_allocation =
        lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOC, u32, pool_manager);
    let ambient_task = if ambient_allocation != 0 {
        lf_checker_rt::callee_thiscall!(
            AMBIENT_CONSTRUCTOR,
            u32,
            ambient_allocation,
            0,
            NEGATIVE_ONE_BITS,
            0,
            0,
            0,
            12,
        )
    } else {
        0
    };

    lf_checker_rt::callee_thiscall!(
        ATTACH_TO_INTELLIGENCE,
        u32,
        attachable_allocation,
        wander_task,
        ambient_task,
        0,
        0,
    )
});
