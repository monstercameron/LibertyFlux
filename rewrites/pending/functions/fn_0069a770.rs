// original: 0x0069a770 unknown (four-word record serializer)
/// Moves a four-word record between a buffer and the resource stream, one
/// word per helper call, via one of two helpers selected by the resource
/// flag bit. Returns the last helper answer.
lf_k2_rt::export!(thiscall, rw_0069a770(this: *mut u8, out: *mut u8) -> u32 {
    unsafe {
        let mut ans = 0u32;
        for k in 0..4u32 {
            let flag_clear = (*this & 1) == 0;
            let stream = *((this.add(4)) as *const u32);
            let p = (out as u32).wrapping_add(k.wrapping_mul(4));
            ans = if flag_clear {
                lf_k2_rt::callee_thiscall!(2, u32, stream, p, 4)
            } else {
                lf_k2_rt::callee_thiscall!(1, u32, stream, p, 4)
            };
        }
        ans
    }
});
