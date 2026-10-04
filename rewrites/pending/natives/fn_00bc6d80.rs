// original: 0x00bc6d80 IS_THIS_MODEL_A_HELI
/// Script native `IS_THIS_MODEL_A_HELI` (hash 0x62EA75E0).
///
/// Forwards one script argument (a model hash) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc6d80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
