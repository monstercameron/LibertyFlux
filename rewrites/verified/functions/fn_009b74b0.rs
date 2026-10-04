// original: 0x009b74b0 NativeImpl_GET_CAM_POS
/// Look up the camera object for `key`, then copy the 16 bytes at +0x40
/// (a position vector and a fourth word) into `*out`. Returns the last word
/// copied, matching the value the original leaves in EAX.
/// (Engine getter behind GET_CAM_POS.)
export!(stdcall, rw_009b74b0(key: u32, out: u32) -> u32 {
    unsafe {
        const VEC_OFF: u32 = 0x40;
        const WORDS: u32 = 4;
        let obj = callee_stdcall!(1, u32, key);
        let src = (obj.wrapping_add(VEC_OFF)) as *const u32;
        let dst = out as *mut u32;
        let mut i = 0u32;
        while i < WORDS {
            *dst.add(i as usize) = *src.add(i as usize);
            i += 1;
        }
        *src.add((WORDS - 1) as usize)
    }
});
