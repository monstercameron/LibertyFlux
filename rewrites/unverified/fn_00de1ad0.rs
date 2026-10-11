// original: 0x00DE1AD0 UIScrollingMenu::vf94

/// Query the delegated count through virtual slot `+0x4C`; return that EAX value when it differs from the incoming word. On equality, query slot `+0x124` and return its EAX result when AL is zero. When AL is nonzero, load the object pointer from the global navigation context and tail-dispatch the helper with argument two.
lf_checker_rt::export!(thiscall, rw_00de1ad0(this_ptr: u32, expected_count: u32) -> u32 {
    const DELEGATE_OFFSET: u32 = 0x1EC;
    const COUNT_QUERY: u32 = 1;
    const ACTIVE_QUERY: u32 = 2;
    const NAVIGATION_CONTEXT_VA: u32 = 0x018B6C8C;
    const NAVIGATION_DISPATCH: u32 = 3;
    let delegate = unsafe { ((this_ptr.wrapping_add(DELEGATE_OFFSET)) as *const u32).read_unaligned() };
    let count = lf_checker_rt::callee_thiscall!(COUNT_QUERY, u32, delegate);
    if expected_count != count { return count; }
    let active = lf_checker_rt::callee_thiscall!(ACTIVE_QUERY, u32, delegate);
    if active as u8 == 0 { return active; }
    let global_slot = lf_checker_rt::relocated(NAVIGATION_CONTEXT_VA) as *const u32;
    let context = unsafe { global_slot.read_unaligned() };
    lf_checker_rt::callee_thiscall!(NAVIGATION_DISPATCH, u32, context, 2)
});
