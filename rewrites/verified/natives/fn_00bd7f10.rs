// original: 0x00bd7f10 IS_OBJECT_ON_PLAYER_MACHINE
/// Script native `IS_OBJECT_ON_PLAYER_MACHINE` (hash 0x00736569).
///
/// Forwards two script arguments (an object handle and a player index) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd7f10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
