// original: 0x00b9eff0 GET_CHAR_READY_TO_BE_EXECUTED
/// Script native `GET_CHAR_READY_TO_BE_EXECUTED` (hash 0x3FFF4DE9).
///
/// Forwards one script argument (a character handle) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9eff0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
