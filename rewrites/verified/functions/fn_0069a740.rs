// original: 0x0069a740 rage::crAnimChannelStaticVector3::vf7
/// Copy the 16-byte constant vector at the `this+8` pointer to `out`,
/// returning the last (w) word, matching the original's EAX.
export!(thiscall, rw_0069a740(this: u32, _a: u32, _b: u32, out: u32) -> u32 {
    unsafe {
        let src = *((this + 8) as *const u32);
        core::ptr::copy_nonoverlapping(src as *const u8, out as *mut u8, 16);
        *((src + 12) as *const u32)
    }
});
