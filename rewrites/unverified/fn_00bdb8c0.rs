// original: 0x00bdb8c0 NETWORK_STORE_GAME_CONFIG
/// Script-native handler thunk with an argument rewrite.
///
/// Takes one word, the script context pointer. Reads the argument block
/// through two pointer levels (the table word past the context header,
/// then the block the table points at), advances past the block's header
/// word, and hands the resulting pointer to the shared implementation,
/// returning whatever that call answers. The incoming argument slot is
/// overwritten in place before the transfer, so the stack comparison is
/// off for this contract and the forwarded pointer is compared through
/// the call log instead.
export!(cdecl, rw_00bdb8c0(ctx: u32) -> u32 {
    unsafe {
        let table = *(ctx.wrapping_add(8) as *const u32);
        let block = *(table as *const u32);
        let forwarded = block.wrapping_add(4);
        callee_cdecl!(1, u32, forwarded)
    }
});
