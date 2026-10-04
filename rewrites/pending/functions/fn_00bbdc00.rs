// original: 0x00bbdc00 NativeImpl_TASK_DIE
/// Queue a die task for a ped and reset the ped's death state.
///
/// Returns the state pointer when the ped is busy. Otherwise allocates a task
/// node and builds the die task unless the allocator reports empty, resets
/// the ped (resolved again through the ped pool) through the reset slot of
/// its own function table, and assigns the task with kind 5. Returns the
/// assign call's answer.
export!(cdecl, rw_00bbdc00(handle: u32) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x18B6F1C);
        if handle != 0 {
            let ped: u32 = callee_thiscall!(1, u32, pool, handle);
            let state = *((ped + 0x6C) as *const u32);
            if state != 0 && *((state + 0xE) as *const u8) != 0 {
                return state;
            }
        }
        let mgr: u32 = callee_thiscall!(2, u32, *global::<u32>(0x167E2A0));
        let task: u32 = if mgr == 0 {
            0
        } else {
            callee_thiscall!(3, u32, mgr, 0, 0, 0x2C, 0xBE, 0x40800000u32, 0, 1)
        };
        let ped: u32 = callee_thiscall!(4, u32, pool, handle);
        if ped != 0 {
            let vtab = *(ped as *const u32);
            let reset: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(*((vtab + 0xF4) as *const u32));
            let _: u32 = reset(ped, 0, 0);
        }
        callee_cdecl!(6, u32, handle, task, 5)
    }
});
