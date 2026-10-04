// original: 0x00bb1df0 GET_LOCAL_PLAYER_MP_CASH
/// Script native `GET_LOCAL_PLAYER_MP_CASH` (hash 0x76B068CA).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the full 32-bit answer into the return slot.
export!(cdecl, rw_00bb1df0(ctx: *const u8) -> u32 {
    unsafe {
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(1, u32,);
        *slot = answer;
        answer
    }
});
