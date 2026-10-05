// original: 0x00b952b0 THIS_SCRIPT_IS_SAFE_FOR_NETWORK_GAME
/// Script-native handler thunk.
///
/// Takes one word, the script context pointer. Hands that pointer to the
/// shared implementation unchanged and returns whatever that call answers.
/// Control never comes back here: the transfer at the entry leaves this
/// address for good. The bytes after the entry hold padding, not code.
export!(cdecl, rw_00b952b0(ctx: u32) -> u32 {
    callee_cdecl!(1, u32, ctx)
});
