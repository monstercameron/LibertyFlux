// original: 0x00bbb2a0 NativeImpl_ADD_FOLLOW_NAVMESH_TO_PHONE_TASK
/// Build a follow-task for a ped and attach it, unless the ped is busy.
///
/// Resolves the ped handle through the ped pool; when the handle is valid and
/// the ped's state block carries the busy flag, returns the state pointer and
/// does nothing else. Otherwise looks up the ped's task manager, fetches the
/// follow-task factory for kind `0x640` through its slot, checks the factory
/// kind through the factory's own function table, allocates a task node and
/// constructs the task from the two constant factors (one half, three) and
/// the three coordinate arguments, marks the new task's flag word and queues
/// it on the saved task slot. Returns the queue call's answer.
/// When the allocator reports empty the original still runs the flag marking
/// through a null pointer and faults; the rewrite does the same so the fault
/// matches. The three coordinate words are stored to the frame by the
/// original but never read back (only the first is passed on by address).
export!(cdecl, rw_00bbb2a0(handle: u32, f0: u32, _f1: u32, _f2: u32) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x18B6F1C);
        if handle != 0 {
            let ped: u32 = callee_thiscall!(1, u32, pool, handle);
            let state = *((ped + 0x6C) as *const u32);
            if state != 0 && *((state + 0xE) as *const u8) != 0 {
                return state;
            }
        }
        let ped: u32 = callee_thiscall!(2, u32, pool, handle);
        let mgr = (*((ped + 0x224) as *const u32)).wrapping_add(0x44);
        let task: u32 = callee_thiscall!(3, u32, mgr, 3, 0x640);
        if task == 0 {
            return 0;
        }
        let vtab = *(task as *const u32);
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vtab + 0xC) as *const u32));
        let kind = kind_of(task);
        if kind != 0x640 {
            return kind;
        }
        let saved = *((task + 4) as *const u32);
        let alloc: u32 = callee_thiscall!(5, u32, *global::<u32>(0x167E2A0));
        let half = *global::<u32>(0xEE1EB0);
        let three = *global::<u32>(0xEE1EB4);
        let mut coord0 = f0;
        let newtask: u32 = if alloc == 0 {
            0
        } else {
            callee_thiscall!(6, u32, alloc, 0x3F800000u32,
                &mut coord0 as *mut u32 as u32, half, three,
                0xFFFFFFFFu32, 1, 0, 0, 0, 1)
        };
        *((newtask + 0xD8) as *mut u32) |= 0x10000000;
        callee_thiscall!(7, u32, saved, newtask)
    }
});
