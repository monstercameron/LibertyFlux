// original: 0x009b7610 NativeImpl_CAM_SEQUENCE_GET_PROGRESS
/// Look up the sequence element for `key` (passing `this` through to the
/// element helper), then load the word at +0x78 into `*out`. Returns `out`.
/// (Engine getter behind CAM_SEQUENCE_GET_PROGRESS.)
export!(thiscall, rw_009b7610(this_: u32, key: u32, out: u32) -> u32 {
    unsafe {
        const FIELD: u32 = 0x78;
        let obj = callee_thiscall!(1, u32, this_, key);
        *(out as *mut u32) = *((obj.wrapping_add(FIELD)) as *const u32);
        out
    }
});
