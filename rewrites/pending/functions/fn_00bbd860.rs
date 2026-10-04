// original: 0x00bbd860 NativeImpl_TASK_COMBAT_HATED_TARGETS_AROUND_CHAR_TIMED
/// Queue a timed combat-hated-targets task around a ped.
///
/// Same shape as the untimed variant with kind `0x81`: returns the state
/// pointer when the ped is busy, assigns the empty task when the allocator
/// reports empty, otherwise builds the combat task from the radius, the
/// enabled flag, the time limit and a zeroed parameter block passed by
/// address. Returns the assign call's answer.
export!(cdecl, rw_00bbd860(handle: u32, radius: u32, limit: u32) -> u32 {
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
            return callee_cdecl!(4, u32, handle, 0, 0x81);
        }
        let mut params = 0u32;
        let t: u32 = callee_thiscall!(3, u32, mgr,
            &mut params as *mut u32 as u32, radius, 1, limit);
        callee_cdecl!(4, u32, handle, t, 0x81)
    }
});
