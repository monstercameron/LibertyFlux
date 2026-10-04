// original: 0x0069a6e0 rage::crAnimChannelRawInt::vf10
/// Copy raw integer keyframe `idx` to `out`. Returns `out`.
export!(thiscall, rw_0069a6e0(this: u32, idx: u32, _unused: u32, out: u32) -> u32 {
    unsafe {
        let keys = *((this + 8) as *const u32);
        *(out as *mut u32) = *((keys + idx.wrapping_mul(4)) as *const u32);
        out
    }
});
