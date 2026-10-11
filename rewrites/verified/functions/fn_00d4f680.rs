// original: 0x00D4F680 CTaskComplexPickUpObject::vf18
/// Produce the next object pickup task for the current complex task. It first
/// requires a non-null target at `+0x14`, then queries the related task at
/// `+0x08` through virtual slot `+0x0c`. Only mode `0x11d` continues. A helper
/// receives the address of this task's state word at `+0x3c`; its low return
/// byte chooses between delegating the target and secondary pointers to the
/// pickup-task constructor, or initializing the manager object as the new
/// task. The other recognized mode, `0x138`, and all unrecognized modes return
/// zero. When the manager lookup is null, the result is zero in either path.
lf_checker_rt::export!(thiscall, rw_00d4f680(this: u32, _unused: u32) -> u32 {
    const RELATED_TASK: u32 = 0x08;
    const TARGET: u32 = 0x14;
    const SECONDARY: u32 = 0x18;
    const STATE: u32 = 0x3c;
    const MODE_SLOT: u32 = 0x0c;
    const PICKUP_MODE: u32 = 0x11d;
    const EMPTY_MODE: u32 = 0x138;
    const STATE_CHECK: u32 = 2;
    const MANAGER_LOOKUP: u32 = 3;
    const PICKUP_CTOR: u32 = 4;
    const BASE_TASK_CTOR: u32 = 5;
    const TASK_VTABLE: u32 = 0x00eb_391c;
    const MANAGER_GLOBAL: u32 = 0x0167_e2a0;

    unsafe {
        let target = ((this + TARGET) as *const u32).read_unaligned();
        if target == 0 {
            return 0;
        }

        let related_task = ((this + RELATED_TASK) as *const u32).read_unaligned();
        let vtable = (related_task as *const u32).read_unaligned();
        let mode_address = ((vtable + MODE_SLOT) as *const u32).read_unaligned();
        let mode_query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(mode_address as usize);
        let mode = mode_query(related_task);
        if mode == EMPTY_MODE || mode != PICKUP_MODE {
            return 0;
        }

        let state_pointer = this + STATE;
        let state_result: u32 = lf_checker_rt::callee_cdecl!(STATE_CHECK, u32, state_pointer);
        let is_ready = (state_result as u8) != 0;
        let manager_global = lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned();
        let manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, manager_global);
        if manager == 0 {
            return 0;
        }

        if is_ready {
            let secondary = ((this + SECONDARY) as *const u32).read_unaligned();
            return lf_checker_rt::callee_thiscall!(PICKUP_CTOR, u32, manager, target, secondary);
        }

        lf_checker_rt::callee_thiscall!(BASE_TASK_CTOR, u32, manager);
        ((manager) as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE));
        ((manager + 0x14) as *mut u32).write_unaligned(0);
        ((manager + 0x18) as *mut u8).write(0);
        ((manager + 0x1c) as *mut u32).write_unaligned(0);
        manager
    }
});
