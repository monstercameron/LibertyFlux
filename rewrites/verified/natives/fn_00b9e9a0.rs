// original: 0x00b9e9a0 DOES_CHAR_EXIST
/// Script native `DOES_CHAR_EXIST` (hash 0x46531797).
///
/// Forwards one script argument (a character handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9e9a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
