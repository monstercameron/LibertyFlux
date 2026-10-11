// original: 0x00DE21B0 UIScrollingMenu::vf107

/// Load the delegated receiver from `+0x1E0` and forward it through virtual slot `+0x1AC`. This thunk takes no stack arguments and returns the target's EAX result.
lf_checker_rt::export!(thiscall, rw_00de21b0(this_ptr: u32) -> u32 {
    const DELEGATE_OFFSET: u32 = 0x1E0;
    const DELEGATE_SLOT: u32 = 1;
    let delegate = unsafe { ((this_ptr.wrapping_add(DELEGATE_OFFSET)) as *const u32).read_unaligned() };
    lf_checker_rt::callee_thiscall!(DELEGATE_SLOT, u32, delegate)
});
