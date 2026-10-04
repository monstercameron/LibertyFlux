// original: 0x00bbdb80 NativeImpl_TASK_DESTROY_CAR
/// Queue a destroy-car task for a ped.
///
/// Returns the state pointer when the ped is busy. Otherwise resolves the
/// vehicle handle through the vehicle pool, allocates a task node and, unless
/// the allocator reports empty, builds the destroy task from the vehicle and
/// assigns it with kind `0x25`. Returns the assign call's answer.
export!(cdecl, rw_00bbdb80(handle: u32, veh: u32) -> u32 {
    unsafe {
        if handle != 0 {
            let ped: u32 = callee_thiscall!(1, u32,
                *global::<u32>(0x18B6F1C), handle);
            let state = *((ped + 0x6C) as *const u32);
            if state != 0 && *((state + 0xE) as *const u8) != 0 {
                return state;
            }
        }
        let target: u32 = callee_thiscall!(2, u32,
            *global::<u32>(0x12E22A4), veh);
        let mgr: u32 = callee_thiscall!(3, u32, *global::<u32>(0x167E2A0));
        if mgr == 0 {
            return callee_cdecl!(5, u32, handle, 0, 0x25);
        }
        let t: u32 = callee_thiscall!(4, u32, mgr, target, 0, 0, 0);
        callee_cdecl!(5, u32, handle, t, 0x25)
    }
});
