// original: 0x00D09050 create_gun_or_throw_task

/// Select and create a ped's gun or projectile task.
///
/// `owner` is the task object and `ped` is its single stack argument. The ped's
/// weapon component is at `+0x2b0`; the method checks the two state bytes at
/// `+0x218` and `+0x219`, then looks up the weapon flags. A usable throwable
/// gets the projectile-task constructor. Other states create a gun task and
/// set its task flags. The fallback constructor receives the stored subtask
/// pointer at `self+0x30`, or `self+0x20` when that pointer is null.
///
/// The constructors and task-pool lookup are intercepted calls. The pool
/// singleton slot is read through the checker relocation helper. The original
/// overwrites its incoming stack argument before its projectile tail call;
/// that value is observed in the recorded tail-call argument.
lf_checker_rt::export!(thiscall, rw_00d09050(owner: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_POOL_GLOBAL: u32 = 0x0167_E2A0;
        const PED_WEAPON: u32 = 1;
        const WEAPON_INFO: u32 = 2;
        const TASK_POOL: u32 = 3;
        const GUN_TASK_CTOR: u32 = 4;
        const PROJECTILE_TASK_CTOR: u32 = 5;

        const PED_STATE_A: u32 = 0x218;
        const PED_STATE_B: u32 = 0x219;
        const PED_WEAPON_COMPONENT: u32 = 0x2B0;
        const WEAPON_INFO_ID: u32 = 0x18;
        const WEAPON_FLAGS: u32 = 0x20;
        const THROWABLE_BIT: u32 = 1;
        const THROWABLE_SHIFT: u32 = 6;
        const TASK_FLAGS: u32 = 0x74;
        const GUN_TASK_FLAGS: u32 = 0x540;
        const FALLBACK_TASK_FLAGS: u32 = 0x0200_0001;
        const FALLBACK_FIELD_A: u32 = 0x8C;
        const FALLBACK_FIELD_B: u32 = 0x90;
        const NEGATIVE_ONE_F32: u32 = 0xBF80_0000;
        const POSITIVE_ONE_F32: u32 = 0x3F80_0000;
        const FALLBACK_A: u32 = 0xBEC9_0FDB;
        const FALLBACK_B: u32 = 0x3EC9_0FDB;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_task_flags(task: u32, flags: u32) {
            if task == 0 {
                // The original writes through null+0x74 in this case.
                unsafe { write_u32(TASK_FLAGS, flags) };
            }
            let old_flags = unsafe { read_u32(task.wrapping_add(TASK_FLAGS)) };
            unsafe { write_u32(task.wrapping_add(TASK_FLAGS), old_flags | flags) };
        }

        #[inline(always)]
        unsafe fn task_pool() -> u32 {
            let slot = lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL);
            let singleton = unsafe { slot.read_unaligned() };
            unsafe { lf_checker_rt::callee_thiscall!(TASK_POOL, u32, singleton) }
        }

        let may_check_weapon = unsafe { read_u8(ped.wrapping_add(PED_STATE_A)) == 0 }
            && unsafe { read_u8(ped.wrapping_add(PED_STATE_B)) != 0 };
        let mut should_throw = false;

        if may_check_weapon {
            let weapon = ped.wrapping_add(PED_WEAPON_COMPONENT);
            let first_weapon_result =
                unsafe { lf_checker_rt::callee_thiscall!(PED_WEAPON, u32, weapon) };
            if first_weapon_result != 0 {
                let weapon =
                    unsafe { lf_checker_rt::callee_thiscall!(PED_WEAPON, u32, weapon) };
                let info_id = unsafe { read_u32(weapon.wrapping_add(WEAPON_INFO_ID)) };
                let info = unsafe { lf_checker_rt::callee_cdecl!(WEAPON_INFO, u32, info_id) };
                should_throw = ((unsafe { read_u32(info.wrapping_add(WEAPON_FLAGS)) }
                    >> THROWABLE_SHIFT)
                    & THROWABLE_BIT)
                    != 0;
            }
        }

        if should_throw {
            let pool = unsafe { task_pool() };
            if pool == 0 {
                return 0;
            }
            return unsafe {
                lf_checker_rt::callee_thiscall!(PROJECTILE_TASK_CTOR, u32, pool, 0)
            };
        }

        if may_check_weapon {
            let pool = unsafe { task_pool() };
            if pool == 0 {
                unsafe { write_task_flags(0, GUN_TASK_FLAGS) };
            }
            let task = unsafe {
                lf_checker_rt::callee_thiscall!(
                    GUN_TASK_CTOR,
                    u32,
                    pool,
                    1,
                    0,
                    0,
                    NEGATIVE_ONE_F32,
                    0,
                    1,
                    1,
                    POSITIVE_ONE_F32,
                )
            };
            unsafe { write_task_flags(task, GUN_TASK_FLAGS) };
            return task;
        }

        let pool = unsafe { task_pool() };
        let stored_subtask = unsafe { read_u32(owner.wrapping_add(0x30)) };
        let alternate_subtask = if stored_subtask != 0 {
            0
        } else {
            owner.wrapping_add(0x20)
        };
        if pool == 0 {
            unsafe { write_task_flags(0, FALLBACK_TASK_FLAGS) };
        }
        let task = unsafe {
            lf_checker_rt::callee_thiscall!(
                GUN_TASK_CTOR,
                u32,
                pool,
                2,
                stored_subtask,
                alternate_subtask,
                NEGATIVE_ONE_F32,
                0,
                1,
                1,
                POSITIVE_ONE_F32,
            )
        };
        unsafe { write_task_flags(task, FALLBACK_TASK_FLAGS) };
        unsafe { write_u32(task.wrapping_add(FALLBACK_FIELD_A), FALLBACK_A) };
        unsafe { write_u32(task.wrapping_add(FALLBACK_FIELD_B), FALLBACK_B) };
        task
    }
});
