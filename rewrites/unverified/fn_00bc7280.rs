// original: 0x00bc7280 SET_ALL_CAR_GENERATORS_BACK_TO_ACTIVE
/// Script-native handler thunk.
///
/// Takes one word, the script context pointer. Hands that pointer to the
/// shared implementation unchanged and returns whatever that call answers.
/// Control never comes back here: the transfer at the entry leaves this
/// address for good. The bytes after the entry hold padding, not code.
export!(cdecl, rw_00bc7280(ctx: u32) -> u32 {
    callee_cdecl!(1, u32, ctx)
});
