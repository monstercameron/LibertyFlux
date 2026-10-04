// original: 0x00bbb6e0 NativeImpl_CLEAR_CHAR_TASKS
/// Clear all of a ped's tasks, resetting its task window first.
///
/// Resolves the ped through the ped pool. When the ped has a live task state
/// block, resets the task window (two window calls around a seated check that
/// feeds the ped's signed seat value into the state update and a six-word
/// window report), then clears slot (1, 1) on the task manager and detaches
/// the ped from its current task. Returns the detach call's answer.
export!(cdecl, rw_00bbb6e0(handle: u32) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x18B6F1C);
        let ped: u32 = callee_thiscall!(1, u32, pool, handle);
        if *((ped + 0x6C) as *const u32) != 0 {
            let wnd = relocated(0x1935FA8);
            let _: u32 = callee_thiscall!(2, u32, wnd, 0);
            let _: u32 = callee_cdecl!(3, u32, wnd, 2, relocated(0xEB7200));
            let _: u32 = callee_thiscall!(4, u32, wnd, 1);
            let state = *((ped + 0x6C) as *const u32);
            let seat: u32 = callee_thiscall!(5, u32, relocated(0x18E51E8));
            let signed = ((seat & 0xFF) as u8 as i8) as i32 as u32;
            let updated: u32 = callee_cdecl!(6, u32, signed);
            let _: u32 = callee_cdecl!(7, u32, wnd, 2, relocated(0xEB7270),
                relocated(0xEB71EC), updated, signed);
        }
        let mgr = *((ped + 0x224) as *const u32);
        let _: u32 = callee_thiscall!(8, u32, mgr, 1, 1);
        callee_thiscall!(9, u32, ped, 0, 0xFFFFFFFFu32)
    }
});
