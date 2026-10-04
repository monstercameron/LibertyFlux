// original: 0x00b9f990 IS_CHAR_IN_ANY_HELI
/// Script native `IS_CHAR_IN_ANY_HELI` (hash 0x0FC40275).
///
/// Forwards one character handle to the engine and stores the low byte
/// of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9f990(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
