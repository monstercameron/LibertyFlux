// original: 0x009cc1f0 PRELOAD_STREAM
/// Script native `PRELOAD_STREAM` (hash 0x39DE515D).
///
/// Forwards one script argument (a stream name) to the engine and
/// /// stores the low byte of its answer (zero-extended) into the
/// /// return slot.
export!(cdecl, rw_009cc1f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
