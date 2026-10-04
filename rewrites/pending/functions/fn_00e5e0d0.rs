// original: 0x00e5e0d0 net_reset_register_e0d0
/// Clear an 8-word network state block, then register its handler.
///
/// Zeroes the static block at `0x019D2F84` and registers the handler at
/// `0x00E6ED40` with the registrar, returning the registrar's answer.
export!(cdecl, rw_00e5e0d0() -> u32 {
    unsafe {
        /// State block cleared by this reset (file VA).
        const STATE: u32 = 0x019D2F84;
        /// Handler registered for it (file VA).
        const HANDLER: u32 = 0x00E6ED40;
        /// Words in the state block.
        const WORDS: usize = 8;
        let base = global::<u32>(STATE);
        let mut i = 0;
        while i < WORDS {
            base.add(i).write(0);
            i += 1;
        }
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});
