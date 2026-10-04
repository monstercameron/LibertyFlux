// original: 0x009b7350 NativeImpl_GET_CAM_FAR_CLIP
/// Look up the camera object for `key`, then load the word at +0x68 into `*out`.
/// Returns `out`. (Engine getter behind GET_CAM_FAR_CLIP.)
export!(stdcall, rw_009b7350(key: u32, out: u32) -> u32 {
    unsafe {
        const FIELD: u32 = 0x68;
        let obj = callee_stdcall!(1, u32, key);
        *(out as *mut u32) = *((obj.wrapping_add(FIELD)) as *const u32);
        out
    }
});
