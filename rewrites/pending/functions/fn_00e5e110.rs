// original: 0x00e5e110 net_reset_register_e110
/// Clear an 8-word network state block, then register its handler.
///
/// Zeroes the static block at `0x019D2FF8` and registers the handler at
/// `0x00E6ED70` with the registrar, returning the registrar's answer.
export!(cdecl, rw_00e5e110() -> u32 {
    unsafe {
        /// State block cleared by this reset (file VA).
        const STATE: u32 = 0x019D2FF8;
        /// Handler registered for it (file VA).
        const HANDLER: u32 = 0x00E6ED70;
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
