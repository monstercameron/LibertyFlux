// original: 0x00b9f010 GET_CHAR_READY_TO_BE_STUNNED
/// Script native `GET_CHAR_READY_TO_BE_STUNNED` (hash 0x5C422066).
///
/// Forwards arg0 to the engine.
/// Writes the zero-extended low byte of the engine answer into the return slot.
export!(cdecl, rw_00b9f010(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0));
        let slot = (*(ctx as *const u32)) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
