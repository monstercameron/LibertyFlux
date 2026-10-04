// original: 0x00b9ff50 IS_CHAR_WAITING_FOR_WORLD_COLLISION
/// Script native `IS_CHAR_WAITING_FOR_WORLD_COLLISION` (hash 0x51453EA2).
///
/// Forwards one character handle to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9ff50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args.add(0));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
