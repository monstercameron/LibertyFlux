// original: 0x00a88150 global_mode_select
/// Conditionally fold global mode 5 down to 2.
///
/// Calls the liveness helper (intercepted); when its low byte is nonzero
/// and the inhibit flag is clear, replaces global mode 5 with 2.
/// Returns the helper's full answer.
export!(cdecl, rw_00a88150() -> u32 {
    unsafe {
        let alive: u32 = callee_cdecl!(1, u32,);
        if alive & 0xff != 0 && *global::<u8>(0x17f5eb3) == 0 {
            let cur = *global::<u8>(0x103b924);
            *global::<u8>(0x103b924) = if cur == 5 { 2 } else { cur };
        }
        alive
    }
});
