// original: 0x00bb2290 HAS_PLAYER_DAMAGED_AT_LEAST_ONE_VEHICLE
/// Script native `HAS_PLAYER_DAMAGED_AT_LEAST_ONE_VEHICLE` (hash 0x674849B5).
///
/// Forwards one script argument to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb2290(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
