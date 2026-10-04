// original: 0x0069b3e0 rage::crAnimChannelStaticFloat::vf20
/// Static-float serializer: moves the single value at +0x08 through the
/// resource stream via one of two helpers selected by the resource flag
/// bit. Returns the helper answer.
lf_k2_rt::export!(thiscall, rw_0069b3e0(this: *mut u8, res: *const u8) -> u32 {
    unsafe {
        let flag_clear = (*res & 1) == 0;
        let stream = *((res.add(4)) as *const u32);
        let p = (this as u32).wrapping_add(8);
        if flag_clear {
            lf_k2_rt::callee_thiscall!(2, u32, stream, p, 4)
        } else {
            lf_k2_rt::callee_thiscall!(1, u32, stream, p, 4)
        }
    }
});
