// original: 0x00a011a0 HAS_OBJECT_BEEN_DAMAGED_BY_CHAR
/// Script native `HAS_OBJECT_BEEN_DAMAGED_BY_CHAR` (hash 0x0B464BE8).
///
/// Forwards an object handle and a character handle to the engine.
/// Stores the low byte of the engine answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00a011a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1),);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
