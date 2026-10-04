// original: 0x00b9f110 GET_CHAR_WILL_TRY_TO_LEAVE_WATER
/// Script native `GET_CHAR_WILL_TRY_TO_LEAVE_WATER` (hash 0x7BC85E73).
///
/// Forwards one script argument (a character handle) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9f110(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
