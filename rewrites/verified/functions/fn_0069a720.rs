// original: 0x0069a720 rage::crAnimChannelStaticQuaternion::vf8
/// Copy the 16-byte constant quaternion at the `this+8` pointer to `out`.
/// Returns `out`.
export!(thiscall, rw_0069a720(this: u32, _a: u32, _b: u32, out: u32) -> u32 {
    unsafe {
        let src = *((this + 8) as *const u32);
        core::ptr::copy_nonoverlapping(src as *const u8, out as *mut u8, 16);
        out
    }
});
