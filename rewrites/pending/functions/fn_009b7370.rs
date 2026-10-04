// original: 0x009b7370 NativeImpl_GET_CAM_FAR_DOF
/// Look up the camera object for `key`, then load the word at +0x70 into `*out`.
/// Returns `out`. (Engine getter behind GET_CAM_FAR_DOF.)
export!(stdcall, rw_009b7370(key: u32, out: u32) -> u32 {
    unsafe {
        const FIELD: u32 = 0x70;
        let obj = callee_stdcall!(1, u32, key);
        *(out as *mut u32) = *((obj.wrapping_add(FIELD)) as *const u32);
        out
    }
});
