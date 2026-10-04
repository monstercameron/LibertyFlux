// original: 0x00ba0210 IS_PED_PINNED_DOWN
/// Script native `IS_PED_PINNED_DOWN` (hash 0x03B13377).
///
/// Forwards a character handle to the engine.
/// Stores the low byte of the engine answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00ba0210(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
