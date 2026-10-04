// original: 0x00bbdb00 NativeImpl_TASK_DEAD
/// Queue a dead task for a ped.
///
/// Returns the state pointer when the ped is busy and assigns the empty task
/// with kind `0x3B` when the allocator reports empty. Otherwise builds the
/// dead task from its constant parameters and assigns it with kind `0x3B`.
/// Returns the assign call's answer.
export!(cdecl, rw_00bbdb00(handle: u32) -> u32 {
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
            return callee_cdecl!(4, u32, handle, 0, 0x3B);
        }
        let t: u32 = callee_thiscall!(3, u32, mgr, 0, 0, 0x2C, 0xBE, 0x40800000u32, 0, 1);
        callee_cdecl!(4, u32, handle, t, 0x3B)
    }
});
