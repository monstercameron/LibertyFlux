// original: 0x00bbd7c0 NativeImpl_TASK_COMBAT_HATED_TARGETS_AROUND_CHAR
/// Queue a combat-hated-targets task around a ped.
///
/// Returns the state pointer at once when the ped is busy. Otherwise
/// allocates a task node; when the allocator reports empty, assigns the empty
/// task with kind `0x7B`. When a node is available, builds the combat task
/// from the radius, the enabled flag and a zeroed parameter block passed by
/// address, and assigns it with kind `0x7B`. Returns the assign call's answer.
export!(cdecl, rw_00bbd7c0(handle: u32, radius: u32) -> u32 {
    unsafe {
        if handle != 0 {
            let ped: u32 = callee_thiscall!(1, u32,
                *global::<u32>(0x18B6F1C), handle);
            let state = *((ped + 0x6C) as *const u32);
            if state != 0 && *((state + 0xE) as *const u8) != 0 {
                return state;
            }
        }
        let mgr: u32 = callee_thiscall!(2, u32, *global::<u32>(0x167E2A0));
        if mgr == 0 {
            return callee_cdecl!(4, u32, handle, 0, 0x7B);
        }
        let mut params = 0u32;
        let t: u32 = callee_thiscall!(3, u32, mgr,
            &mut params as *mut u32 as u32, radius, 1, 0);
        callee_cdecl!(4, u32, handle, t, 0x7B)
    }
});
