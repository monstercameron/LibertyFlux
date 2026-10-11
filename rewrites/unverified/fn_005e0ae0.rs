// original: 0x005E0AE0 CTaskComplexUseWaterCannon::control_sub_task

/// Ask the current child for its type. Unless that type equals 0x2c5, return
/// the current child. For that type, ask whether the fire is out; if not,
/// return the current child. If it is out, allocate a timed-drive task and,
/// when allocation succeeds, construct it from the character's current task
/// pointer and a zero flag. Allocation failure returns null. The helper call
/// results and outgoing arguments are preserved through checker stand-ins.
lf_checker_rt::export!(thiscall, rw_005e0ae0(this: u32, character: u32) -> u32 {
    unsafe {
        const CURRENT_TASK: u32 = 0x08;
        const TASK_TYPE_SLOT: u32 = 0x0c;
        const CHARACTER_TASK: u32 = 0x0b30;
        const TASK_POOL_GLOBAL: u32 = 0x0167_E2A0;
        const FIRE_TASK_TYPE: u32 = 0x2c5;
        const CHECK_FIRE_OUT: u32 = 2;
        const TASK_POOL_ALLOC: u32 = 3;
        const CONSTRUCT_TIMED_DRIVE: u32 = 4;
        type GetTaskType = extern "thiscall" fn(u32) -> u32;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let current_task = read_u32(this.wrapping_add(CURRENT_TASK));
        let task_vtable = read_u32(current_task);
        let get_task_type: GetTaskType = core::mem::transmute(
            read_u32(task_vtable.wrapping_add(TASK_TYPE_SLOT)) as usize,
        );
        if get_task_type(current_task) != FIRE_TASK_TYPE {
            return current_task;
        }

        let fire_is_out: u8 = lf_checker_rt::callee_thiscall!(
            CHECK_FIRE_OUT,
            u8,
            this,
            character
        );
        if fire_is_out == 0 {
            return current_task;
        }

        let task_pool = *lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL);
        let allocated = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOC, u32, task_pool);
        if allocated == 0 {
            return 0;
        }

        let character_task = read_u32(character.wrapping_add(CHARACTER_TASK));
        lf_checker_rt::callee_thiscall!(
            CONSTRUCT_TIMED_DRIVE,
            u32,
            allocated,
            character_task,
            0
        )
    }
});
