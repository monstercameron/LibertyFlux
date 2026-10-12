// original: 0x008C4E50 file_handle_read

/// Forward the receiver and two stack words, in call order, to the intercepted thiscall file-read helper and return its result.
lf_checker_rt::export!(cdecl, rw_008c4e50(receiver: u32, buffer: u32, amount: u32) -> u32 {
    const CALLEE_ID: u32 = 31;
    let _ = CALLEE_ID;
    unsafe { lf_checker_rt::callee_thiscall!(31, u32, receiver, buffer, amount) }
});
