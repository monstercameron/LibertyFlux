// original: 0x006996e0 rage::crAnimChannelDeltaFloat::vf20
/// Delta-float serializer: moves the three parameter words (+0x30, +0x34,
/// +0x2c) through the resource stream via one of two helpers selected by
/// the resource flag bit, then serializes the three sub-objects.
/// Returns the last helper answer.
lf_k2_rt::export!(thiscall, rw_006996e0(this: *mut u8, res: *const u8) -> u32 {
    unsafe {
        for off in [0x30usize, 0x34, 0x2c] {
            let flag_clear = (*res & 1) == 0;
            let stream = *((res.add(4)) as *const u32);
            let p = (this as u32).wrapping_add(off as u32);
            if flag_clear {
                lf_k2_rt::callee_thiscall!(2, u32, stream, p, 4);
            } else {
                lf_k2_rt::callee_thiscall!(1, u32, stream, p, 4);
            }
        }
        lf_k2_rt::callee_thiscall!(3, u32, (this as u32).wrapping_add(8), res as u32);
        lf_k2_rt::callee_thiscall!(3, u32, (this as u32).wrapping_add(0x14), res as u32);
        lf_k2_rt::callee_thiscall!(4, u32, (this as u32).wrapping_add(0x20), res as u32)
    }
});
