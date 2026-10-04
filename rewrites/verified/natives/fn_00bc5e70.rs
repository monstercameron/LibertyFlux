// original: 0x00bc5e70 GET_ENGINE_HEALTH
/// Script native `GET_ENGINE_HEALTH` (hash 0x2B0A05E0).
///
/// Forwards one script argument (a vehicle handle) to the engine, which answers with a float in ST0, and stores the answer bit-pattern into the return slot.
export!(cdecl, rw_00bc5e70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: f32 = callee_cdecl!(1, f32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer.to_bits();
        slot as u32
    }
});
