// original: 0x00DE3E60 ui_clip_transition_reset

/// Query the object's virtual state method. When its low return byte is zero,
/// clear the stored time, set the transition flag, then invoke the action
/// method with one. Preserve the full EAX result from whichever virtual call
/// runs last; only the query's low byte controls the branch.
lf_checker_rt::export!(thiscall, rw_00DE3E60(this: u32) -> u32 {
    const STORED_TIME: u32 = 0x1e8;
    const TRANSITION_FLAG: u32 = 0x1ec;
    const CALLEE_QUERY_STATE: u32 = 1;
    const CALLEE_APPLY_ACTION: u32 = 2;

    let state = lf_checker_rt::callee_thiscall!(CALLEE_QUERY_STATE, u32, this);
    if state as u8 == 0 {
        unsafe {
            (this.wrapping_add(STORED_TIME) as *mut u32).write_unaligned(0);
            (this.wrapping_add(TRANSITION_FLAG) as *mut u8).write(1);
        }
        lf_checker_rt::callee_thiscall!(CALLEE_APPLY_ACTION, u32, this, 1u32)
    } else {
        state
    }
});
