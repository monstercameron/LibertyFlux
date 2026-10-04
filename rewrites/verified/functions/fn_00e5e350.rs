// original: 0x00e5e350 net_reset_register_e350
/// Clear a 37-word network state block, then register its handler.
///
/// Zeroes the static block at `0x019D3A98` and registers the handler at
/// `0x00E6EEB0` with the registrar, returning the registrar's answer.
export!(cdecl, rw_00e5e350() -> u32 {
    unsafe {
        /// State block cleared by this reset (file VA).
        const STATE: u32 = 0x019D3A98;
        /// Handler registered for it (file VA).
        const HANDLER: u32 = 0x00E6EEB0;
        /// Words in the state block.
        const WORDS: usize = 37;
        let base = global::<u32>(STATE);
        let mut i = 0;
        while i < WORDS {
            base.add(i).write(0);
            i += 1;
        }
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});
