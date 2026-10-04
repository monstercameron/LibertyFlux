// original: 0x00bd8570 NETWORK_GET_UNACCEPTED_INVITE_EPISODE
/// NETWORK_GET_UNACCEPTED_INVITE_EPISODE: query pending invite episode.
///
/// Native handler. Forwards the player index, stores the episode id through the return slot.
export!(cdecl, rw_00bd8570(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let ret_slot = *ctx_words as *mut u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let answer = callee_cdecl!(1, u32, a0);
        *ret_slot = answer;
        answer
    }
});
