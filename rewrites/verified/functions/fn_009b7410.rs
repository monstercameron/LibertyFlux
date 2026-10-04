// original: 0x009B7410 NativeImpl_GET_FOLLOW_VEHICLE_CAM_SUBMODE
/// Read the follow-vehicle camera's submode byte, sign-extended.
///
/// Queries the camera object and returns the byte at the submode offset as a
/// signed value. A null query result yields -1 without reading.
/// stdcall, no arguments.
lf_checker_rt::export!(stdcall, rw_009B7410() -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x0103E498;
        const SUBMODE: u32 = 0x22C;
        const SEL_A: u32 = 2;
        const SEL_B: u32 = 0;
        const MISS: u32 = 0xFFFFFFFF;
        let cam: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(CAM_MGR));
        let obj: u32 = lf_checker_rt::callee_thiscall!(2, u32, cam, SEL_A, SEL_B);
        if obj == 0 {
            return MISS;
        }
        ((obj + SUBMODE) as *const i8).read_unaligned() as i32 as u32
    }
});
