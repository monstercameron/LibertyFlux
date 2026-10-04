// original: 0x00E6FD30 timer_reset_forward_thunk
/// Forward to the shared timer reset routine with the fixed timer object.
///
/// Tail-jumps to the engine reset routine with the timer instance address;
/// the rewrite expresses the jump as a call that forwards the result.
export!(cdecl, rw_00e6fd30() -> u32 {
    unsafe {
        const TIMER_OBJ: u32 = 0x019FBAF0;
        callee_thiscall!(1, u32, relocated(TIMER_OBJ))
    }
});
