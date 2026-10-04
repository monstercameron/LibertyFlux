// original: 0x009cbc30 GET_PLAYER_RADIO_STATION_INDEX
/// Script native `GET_PLAYER_RADIO_STATION_INDEX` (hash 0x4E493AAF).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer into the return slot.
export!(cdecl, rw_009cbc30(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
