// original: 0x008895C0 stream_find_dispatch (proposed)

/// Dispatch a stream find to the table or the overflow entry.
///
/// Reads the table kind at `a0 + 0x24`: kind 1 goes to the overflow find
/// entry (callee 2), any other kind to the table find entry (callee 1),
/// both with `a0` in `ecx` and (`a1`, `a2`) on the stack. The answer is
/// the chosen entry's answer.
///
/// Original: 0x008895C0 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_008895C0(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x24;
        const FIND: u32 = 1;
        const FIND_OVERFLOW: u32 = 2;
        let k = ((a0 + KIND) as *const u32).read_unaligned();
        if k.wrapping_sub(1) == 0 {
            lf_checker_rt::callee_thiscall!(FIND_OVERFLOW, u32, a0, a1, a2)
        } else {
            lf_checker_rt::callee_thiscall!(FIND, u32, a0, a1, a2)
        }
    }
});
