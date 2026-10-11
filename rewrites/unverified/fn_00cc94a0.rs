// original: 0x00CC94A0 release_physics_link_if_present

/// Calls the stored link's thiscall helper with the receiver when the receiver's
/// first word is nonnull, then clears that first word. The helper's register
/// and stack arguments are compared by the contract; its return is unused.
lf_checker_rt::export!(thiscall, rw_00cc94a0(this: u32) -> () {
    const LINK_POINTER: u32 = 0;
    const RELEASE_LINK: u32 = 1;

    let link = unsafe { (this as *const u32).read_unaligned() };
    if link != 0 {
        let _ = lf_checker_rt::callee_thiscall!(RELEASE_LINK, u32, link, this);
    }
    unsafe {
        (this as *mut u32).write_unaligned(0);
    }
});
