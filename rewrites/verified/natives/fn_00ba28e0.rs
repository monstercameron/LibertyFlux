// original: 0x00ba28e0 STOP_PED_DOING_FALL_OFF_TESTS_WHEN_SHOT
/// Script native `STOP_PED_DOING_FALL_OFF_TESTS_WHEN_SHOT` (hash 0x4E386C7B).
///
/// Forwards one script argument (a character handle) to the engine.
/// /// No return slot is written.
/// /// (The original cleans its one pushed argument with `(an instruction of the original)`; the
/// /// effect on the stack pointer is identical to the plain cdecl
/// /// return here.)
export!(cdecl, rw_00ba28e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
