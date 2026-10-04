// original: 0x00b8cd70 IS_STREAMING_ADDITIONAL_TEXT
/// Script native `IS_STREAMING_ADDITIONAL_TEXT` (hash 0x23B00129).
///
/// Forwards one script argument (a text-slot index) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b8cd70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
