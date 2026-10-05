// original: 0x00b9e5a0 CLEAR_ROOM_FOR_CHAR
/// Script native `CLEAR_ROOM_FOR_CHAR` (hash 0x405B16CF).
///
/// Forwards one script argument (a character handle) to the engine.
/// /// No return slot is written.
/// /// (The original cleans its one pushed argument with `(an instruction of the original)`; the
/// /// effect on the stack pointer is identical to the plain cdecl
/// /// return here.)
export!(cdecl, rw_00b9e5a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
