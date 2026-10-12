// original: 0x008C50F0 file_handle_validate

/// Forward the receiver and one stack word to the intercepted thiscall validation helper and return its result.
lf_checker_rt::export!(cdecl, rw_008c50f0(receiver: u32, value: u32) -> u32 {
    const CALLEE_ID: u32 = 32;
    let _ = CALLEE_ID;
    unsafe { lf_checker_rt::callee_thiscall!(32, u32, receiver, value) }
});
