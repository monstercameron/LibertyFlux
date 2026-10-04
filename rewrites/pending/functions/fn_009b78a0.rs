// original: 0x009B78A0 NativeImpl_IS_CAM_HAPPY
/// Report whether the selected camera is in its happy state.
///
/// Resolves the camera for `sel`; a miss reports 1. Otherwise reads the
/// camera's state through its table slot: state 0x17 runs the confirm step
/// and returns its result, any other state reports 1 while keeping the
/// state's high bytes (the original sets only the low byte). stdcall.
lf_checker_rt::export!(stdcall, rw_009B78A0(sel: u32) -> u32 {
    unsafe {
        const CAM_SYS: u32 = 0x0128E400;
        const STATE_SLOT: u32 = 0x28;
        const HAPPY: u32 = 0x17;
        let obj: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(CAM_SYS), sel);
        if obj == 0 {
            return 1;
        }
        let table = (obj as *const u32).read_unaligned();
        let target = ((table + STATE_SLOT) as *const u32).read_unaligned();
        let state: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        let st = state(obj);
        if st != HAPPY {
            (st & 0xFFFFFF00) | 1
        } else {
            lf_checker_rt::callee_thiscall!(3, u32, obj)
        }
    }
});
