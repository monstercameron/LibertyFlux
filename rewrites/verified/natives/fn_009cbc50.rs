// original: 0x009cbc50 GET_PLAYER_RADIO_STATION_NAME_ROLL
/// Script native `GET_PLAYER_RADIO_STATION_NAME_ROLL` (hash 0x1A936344).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_009cbc50(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
