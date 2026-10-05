// original: 0x009cba50 DISABLE_POLICE_SCANNER
/// Script-native handler thunk.
///
/// Takes one word, the script context pointer. Hands that pointer to the
/// shared implementation unchanged and returns whatever that call answers.
/// Control never comes back here: the transfer at the entry leaves this
/// address for good. The bytes after the entry hold padding, not code.
export!(cdecl, rw_009cba50(ctx: u32) -> u32 {
    callee_cdecl!(1, u32, ctx)
});
