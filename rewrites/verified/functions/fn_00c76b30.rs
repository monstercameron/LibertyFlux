// original: 0x00c76b30 CMloModelInfo::vf1

/// Release the object's current interior-model resource, then clear the five
/// cached resource words at offsets `+0x60` through `+0x70`. Return the
/// release routine's EAX value, which the original leaves in the return
/// register.
lf_checker_rt::export!(thiscall, rw_00c76b30(this: u32) -> u32 {
    const RELEASE_INTERIOR: u32 = 1;
    let result = lf_checker_rt::callee_thiscall!(RELEASE_INTERIOR, u32, this);
    unsafe {
        for offset in [0x60u32, 0x64, 0x68, 0x6c, 0x70] {
            ((this.wrapping_add(offset)) as *mut u32).write_unaligned(0);
        }
    }
    result
});
