// original: 0x0069b5a0 rage::crAnimChannelStaticQuaternion::vf3
/// Copy the 16-byte constant quaternion at the `this+8` pointer to `out`.
/// Returns `out`.
export!(thiscall, rw_0069b5a0(this: u32, _f: u32, out: u32) -> u32 {
    unsafe {
        let src = *((this + 8) as *const u32);
        core::ptr::copy_nonoverlapping(src as *const u8, out as *mut u8, 16);
        out
    }
});
