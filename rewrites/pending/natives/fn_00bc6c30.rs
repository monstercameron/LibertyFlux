// original: 0x00bc6c30 IS_GARAGE_CLOSED
/// Script native `IS_GARAGE_CLOSED` (hash 0x26BC1939).
///
/// Forwards one script argument (a garage handle) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc6c30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
