// original: 0x00bd8670 NETWORK_HOST_GAME_CNC
/// Script native `NETWORK_HOST_GAME_CNC` (hash 0x26CF21AF).
///
/// Forwards two script arguments to the engine cops-n-crooks host trampoline and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd8670(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
