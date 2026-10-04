// original: 0x00bbb6c0 NativeImpl_CLEAR_CHAR_SECONDARY_TASK
/// Clear a ped's secondary task slot.
///
/// Resolves the ped through the ped pool and clears slot (0, 1) on the ped's
/// task manager. Returns the clear call's answer.
export!(cdecl, rw_00bbb6c0(handle: u32) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x18B6F1C);
        let ped: u32 = callee_thiscall!(1, u32, pool, handle);
        let mgr = *((ped + 0x224) as *const u32);
        callee_thiscall!(2, u32, mgr, 0, 1)
    }
});
