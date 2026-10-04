// original: 0x00bbd900 NativeImpl_TASK_COMBAT_HATED_TARGETS_IN_AREA
/// Queue a combat-hated-targets task inside an area.
///
/// Returns the state pointer when the ped is busy and assigns the empty task
/// with kind `0x7A` when the allocator reports empty. Otherwise builds the
/// combat task from the area corner (passed by address for the first word),
/// the height and two zero words, and assigns it with kind `0x7A`. Returns
/// the assign call's answer.
export!(cdecl, rw_00bbd900(handle: u32, x: u32, _y: u32, _z: u32, h: u32) -> u32 {
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
            return callee_cdecl!(4, u32, handle, 0, 0x7A);
        }
        let mut corner = x;
        let t: u32 = callee_thiscall!(3, u32, mgr,
            &mut corner as *mut u32 as u32, h, 0, 0);
        callee_cdecl!(4, u32, handle, t, 0x7A)
    }
});
