// original: 0x00bc8050 SKIP_TO_NEXT_ALLOWED_STATION
/// Script native `SKIP_TO_NEXT_ALLOWED_STATION` (hash 0x653B5374).
///
/// Forwards one script argument (a vehicle handle) to the engine.
/// /// No return slot is written.
/// /// (The original cleans its one pushed argument with `(an instruction of the original)`; the
/// /// effect on the stack pointer is identical to the plain cdecl
/// /// return here.)
export!(cdecl, rw_00bc8050(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
