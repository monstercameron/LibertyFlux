// original: 0x00bb2270 HAS_PLAYER_DAMAGED_AT_LEAST_ONE_PED
/// Script native `HAS_PLAYER_DAMAGED_AT_LEAST_ONE_PED` (hash 0x64E06CBB).
///
/// Forwards one script argument (a player index) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb2270(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
