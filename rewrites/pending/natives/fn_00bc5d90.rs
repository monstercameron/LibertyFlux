// original: 0x00bc5d90 GET_CURRENT_PLAYBACK_NUMBER_FOR_CAR
/// Script native `GET_CURRENT_PLAYBACK_NUMBER_FOR_CAR` (hash 0x678813A4).
///
/// Forwards one script argument (a vehicle handle) to the engine.
///
/// Stores the engine answer's full 32 bits into the return slot
/// (`mov`, not `movzx`).
///
export!(cdecl, rw_00bc5d90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
