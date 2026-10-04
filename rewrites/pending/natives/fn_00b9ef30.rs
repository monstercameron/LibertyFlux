// original: 0x00b9ef30 GET_CHAR_MELEE_ACTION_FLAG2
/// Script native `GET_CHAR_MELEE_ACTION_FLAG2` (hash 0x032F729B).
///
/// Forwards one script word (a character handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9ef30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
