// original: 0x00bc6cc0 IS_NEXT_STATION_ALLOWED
/// Script native `IS_NEXT_STATION_ALLOWED` (hash 0x7B8B1D10).
///
/// Forwards one script argument (a station index) to the engine.
///
/// Stores the low byte of the engine answer (zero-extended)
/// into the return slot.
///
export!(cdecl, rw_00bc6cc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
