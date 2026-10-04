// original: 0x00b9fb60 IS_CHAR_IN_FLYING_VEHICLE
/// Script native `IS_CHAR_IN_FLYING_VEHICLE` (hash 0x7FA763E8).
///
/// Forwards a character handle to the engine. Stores the low byte of
/// the engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9fb60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
