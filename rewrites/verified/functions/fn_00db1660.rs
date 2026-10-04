// original: 0x00db1660 UILayoutFrame::vf23
/// Resolve the frame's token to an object, run its finalizer, count one more.
///
/// Asks the frame for its current token, resolves the token to a target
/// object through the shared resolver, runs the target's follow-up step,
/// and returns that step's result plus one.
export!(thiscall, rw_00db1660(this_ptr: u32) -> u32 {
    unsafe {
        const TOKEN_SLOT: usize = 0x54;
        const FOLLOW_UP_SLOT: usize = 0x5c;

        let table = *(this_ptr as *const u32) as usize;
        let token_of = *((table + TOKEN_SLOT) as *const u32) as usize;
        let ask_token: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(token_of);
        let token = ask_token(this_ptr);

        let target = callee_cdecl!(2, u32, token);

        let target_table = *(target as *const u32) as usize;
        let follow_up_at = *((target_table + FOLLOW_UP_SLOT) as *const u32) as usize;
        let follow_up: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(follow_up_at);
        follow_up(target).wrapping_add(1)
    }
});
