// original: 0x00DE1840 UIScrollingMenu::vf121

/// Load the delegated receiver from `+0x1E0` and forward the single 32-bit stack argument through its virtual slot. The target's EAX result and thiscall stack cleanup pass through unchanged.
lf_checker_rt::export!(thiscall, rw_00de1840(this_ptr: u32, forwarded_word: u32) -> u32 {
    const DELEGATE_OFFSET: u32 = 0x1E0;
    const DELEGATE_SLOT: u32 = 1;
    let delegate = unsafe { ((this_ptr.wrapping_add(DELEGATE_OFFSET)) as *const u32).read_unaligned() };
    lf_checker_rt::callee_thiscall!(DELEGATE_SLOT, u32, delegate, forwarded_word)
});
