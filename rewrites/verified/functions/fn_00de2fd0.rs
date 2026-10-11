// original: 0x00DE2FD0 scrolling_menu_forward_row_value

/// Load the delegated receiver from `+0x1E0` and forward the single 32-bit stack word through virtual slot `+0x23C`. The thiscall thunk returns the target's EAX value and removes its one argument.
lf_checker_rt::export!(thiscall, rw_00de2fd0(this_ptr: u32, forwarded_word: u32) -> u32 {
    const DELEGATE_OFFSET: u32 = 0x1E0;
    const DELEGATE_SLOT: u32 = 1;
    let delegate = unsafe { ((this_ptr.wrapping_add(DELEGATE_OFFSET)) as *const u32).read_unaligned() };
    lf_checker_rt::callee_thiscall!(DELEGATE_SLOT, u32, delegate, forwarded_word)
});
