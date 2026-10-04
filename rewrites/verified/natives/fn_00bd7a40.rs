// original: 0x00bd7a40 GET_LAST_TIME_NETWORK_ID_DAMAGED
/// Script native `GET_LAST_TIME_NETWORK_ID_DAMAGED` (hash 0x3A8D7BA4).
///
/// Forwards 1 script argument(s) to the engine: 1 integer(s).
/// Stores the full engine answer into the return slot.
export!(cdecl, rw_00bd7a40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
