// original: 0x009cbc40 GET_PLAYER_RADIO_STATION_NAME
/// Script native `GET_PLAYER_RADIO_STATION_NAME` (hash 0x25136AC2).
///
/// Calls the engine worker with no arguments and stores its full 32-bit
/// answer into the return slot.
export!(cdecl, rw_009cbc40(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
