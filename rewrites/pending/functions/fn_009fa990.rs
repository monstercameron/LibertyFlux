// original: 0x009fa990 playstat_notifier_emit_k13
/// Emit a kind-19 notifier report for the given stat token.
///
/// Builds a temporary notifier of kind 19 on the stack, resolves the
/// stat token through the name service, attaches it, emits the report,
/// and tears the notifier down. Returns nothing meaningful.
export!(cdecl, rw_009fa990(stat: u32) -> u32 {
    unsafe {
        let mut temp = [0u32; 8];
        let temp_ptr = temp.as_mut_ptr() as u32;
        callee_thiscall!(1, u32, temp_ptr, 0x13);
        let token = callee_cdecl!(2, u32, stat, 0);
        callee_thiscall!(3, u32, temp_ptr, token, stat);
        callee_cdecl!(4, u32, temp_ptr, 0x34);
        callee_thiscall!(5, u32, temp_ptr);
        // Stack-cookie check: argument excluded from comparison (see 740).
        callee_thiscall!(6, u32, 0);
        0
    }
});
