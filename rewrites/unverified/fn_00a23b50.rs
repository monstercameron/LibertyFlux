// original: 0x00a23b50 camera_aim_object_initialize

/// Initialize the object through two helpers, then install its relocated
/// vtable and clear the two state words at offsets 0x258 and 0x25c before
/// returning the original object pointer.
lf_checker_rt::export!(thiscall, rw_00a23b50(this: u32) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00e9_b98c));
        (this.wrapping_add(0x258) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x25c) as *mut u32).write_unaligned(0);
    }
    let _ = lf_checker_rt::callee_thiscall!(2, u32, this);
    this
});
