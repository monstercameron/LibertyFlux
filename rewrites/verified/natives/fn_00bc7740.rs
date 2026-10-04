// original: 0x00bc7740 SET_CAR_ON_GROUND_PROPERLY
/// Script native `SET_CAR_ON_GROUND_PROPERLY` (hash 0x0E717E98).
///
/// Forwards 1 script argument(s) to the engine: 1 integer(s).
/// Stores the low byte of the engine answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00bc7740(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
