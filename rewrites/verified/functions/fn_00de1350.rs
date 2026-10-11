// original: 0x00DE1350 UIScrollingMenu::~UIScrollingMenu__deleting

/// Destroy the delegated UI base object, then perform the flag-gated cleanup call when flag bit zero is set. The `thiscall` receiver is ECX, one 32-bit flags word is popped by the function, and the original receiver is returned in EAX.
lf_checker_rt::export!(thiscall, rw_00de1350(this_ptr: u32, flags: u32) -> u32 {
    const DELETE_BASE: u32 = 1;
    const OPTIONAL_CLEANUP: u32 = 2;
    let _base_result = lf_checker_rt::callee_thiscall!(DELETE_BASE, u32, this_ptr);
    if flags & 1 != 0 {
        let _cleanup_result = lf_checker_rt::callee_cdecl!(OPTIONAL_CLEANUP, u32, this_ptr);
    }
    this_ptr
});
