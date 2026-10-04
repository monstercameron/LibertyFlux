// original: 0x009b7440 NativeImpl_GET_CAM_MOTION_BLUR
/// Look up the camera object for `key`, then load the word at +0x74 into `*out`.
/// Returns `out`. (Engine getter behind GET_CAM_MOTION_BLUR.)
export!(stdcall, rw_009b7440(key: u32, out: u32) -> u32 {
    unsafe {
        const FIELD: u32 = 0x74;
        let obj = callee_stdcall!(1, u32, key);
        *(out as *mut u32) = *((obj.wrapping_add(FIELD)) as *const u32);
        out
    }
});
