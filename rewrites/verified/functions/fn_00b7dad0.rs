// original: 0x00B7DAD0 BuildChildTaskFromEvent

/// Selects a child-task action from an event code for the current task.
///
/// The thiscall receiver is the task object; the two stack arguments are the
/// event code and actor context. The event comparison above `0x2de` is signed.
/// Low event cases consult the task's actor field, while selected events ask a
/// global task manager for a helper result. Event `0x340` builds from the
/// actor's position, and event `0x390` forwards to the actor task manager's
/// virtual type query. Other event codes return zero. The manager pointer is
/// read through the runtime's relocated global reference.
lf_checker_rt::export!(thiscall, rw_00b7dad0(this: u32, event_type: u32, ped: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL_VA: u32 = 0x0167e2a0;
        const TASK_FIELD: u32 = 0x40;
        const TASK_MODE: u32 = 0x48;
        const PED_FLAG: u32 = 0xa60;
        const PED_POSITION: u32 = 0x20;
        const PED_TASK_MANAGER: u32 = 0x22c;
        const EVENT_LIMIT: u32 = 0x2de;
        const NEGATIVE_FIVE: u32 = 0xffff_fffb;
        const NEGATIVE_FOUR: u32 = 0xffff_fffc;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let manager_root = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
        let current_task_field = read_u32(this + TASK_FIELD);
        let get_manager = || -> u32 {
            lf_checker_rt::callee_thiscall!(1, u32, manager_root)
        };

        if (event_type as i32) > (EVENT_LIMIT as i32) {
            return match event_type {
                0x340 => {
                    let manager = get_manager();
                    if manager == 0 {
                        return 0;
                    }
                    let position = read_u32(ped + PED_POSITION);
                    lf_checker_rt::callee_thiscall!(
                        10,
                        u32,
                        manager,
                        position + 0x30,
                        read_u32(this + 0x44),
                        1
                    )
                }
                0x390 => {
                    let ped_manager = read_u32(ped + PED_TASK_MANAGER);
                    lf_checker_rt::callee_thiscall!(9, u32, ped_manager, ped)
                }
                0x516 => 0,
                _ => 0,
            };
        }

        match event_type {
            0x2c5 => {
                let should_update: u8 = lf_checker_rt::callee_thiscall!(
                    4,
                    u8,
                    current_task_field,
                    ped
                );
                if should_update != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, this);
                }
                let manager = get_manager();
                if manager == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(6, u32, manager, current_task_field, 0)
                }
            }
            0x2ca => {
                let manager = get_manager();
                if manager == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(3, u32, manager, 0, 1, 0, 1)
                }
            }
            0x2cd => {
                let manager = get_manager();
                if manager == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(2, u32, manager)
                }
            }
            0x2de => {
                if read_u8(ped + PED_FLAG) == 1 {
                    let should_stop: u8 = lf_checker_rt::callee_thiscall!(
                        5,
                        u8,
                        current_task_field
                    );
                    if should_stop != 0 {
                        return 0;
                    }
                }
                let manager = get_manager();
                if manager == 0 {
                    0
                } else if read_u8(this + TASK_MODE) != 0 {
                    lf_checker_rt::callee_thiscall!(
                        7,
                        u32,
                        manager,
                        current_task_field,
                        NEGATIVE_FIVE,
                        0x1b,
                        0,
                        0
                    )
                } else {
                    lf_checker_rt::callee_thiscall!(
                        7,
                        u32,
                        manager,
                        current_task_field,
                        NEGATIVE_FOUR,
                        0x1b,
                        0,
                        0
                    )
                }
            }
            _ => 0,
        }
    }
});
