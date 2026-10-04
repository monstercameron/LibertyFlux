// original: 0x009fa740 playstat_notifier_emit_bare_k0d
/// Emit a bare kind-13 notifier report.
///
/// Builds a temporary notifier of kind 13 on the stack, emits its report,
/// and tears it down. Returns nothing meaningful.
export!(cdecl, rw_009fa740() -> u32 {
    unsafe {
        let mut temp = [0u32; 8];
        let temp_ptr = temp.as_mut_ptr() as u32;
        callee_thiscall!(1, u32, temp_ptr, 0x0d);
        callee_cdecl!(2, u32, temp_ptr, 0x34);
        callee_thiscall!(3, u32, temp_ptr);
        // Stack-cookie check: its argument derives from the stack pointer,
        // which legitimately differs between sides, so the contract excludes
        // it from the call comparison.
        callee_thiscall!(4, u32, 0);
        0
    }
});
