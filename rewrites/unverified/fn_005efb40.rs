// original: 0x005EFB40 child_state_is_single (proposed)

/// True when the looked-up record's state field equals 1.
///
/// Passes the child pointer at `this + 0x18` to a lookup helper and reports
/// whether the returned record's word at `+ 0xc` equals 1. The original is
/// thiscall with no stack arguments (the helper takes its argument on the
/// stack and the caller cleans up), returning the result in AL.
lf_checker_rt::export!(thiscall, rw_005efb40(this: u32) -> u32 {
    unsafe {
        const CHILD_OFFSET: u32 = 0x18;
        const STATE_OFFSET: u32 = 0x0c;
        const LOOKUP_CALLEE: u32 = 1;
        const SINGLE_STATE: u32 = 1;
        let child =
            (this.wrapping_add(CHILD_OFFSET) as *const u32).read_unaligned();
        let found = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, child);
        let state =
            (found.wrapping_add(STATE_OFFSET) as *const u32).read_unaligned();
        u32::from(state == SINGLE_STATE)
    }
});
