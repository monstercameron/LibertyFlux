// original: 0x008CC6B0 stream_state_ready (proposed)

/// State readiness flag from the global state word: 0 when it is 0 or 2,
/// 1 for 1, 3, 4 and anything above 4 (including large unsigned values).
///
/// The original dispatches through a jump table; the rewrite is the
/// equivalent match. No arguments (cdecl); the byte result is in AL.
lf_checker_rt::export!(cdecl, rw_008CC6B0() -> u32 {
    unsafe {
        /// Global streaming state word.
        const STATE: u32 = 0x1173220;
        let state = (lf_checker_rt::relocated(STATE) as *const u32).read();
        u32::from(!matches!(state, 0 | 2))
    }
});
