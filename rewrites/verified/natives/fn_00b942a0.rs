// original: 0x00b942a0 FORCE_WEATHER_NOW
/// Script native `FORCE_WEATHER_NOW` (hash 0x63737D31).
///
/// Forwards one script argument (a weather id) to the engine worker.
/// (The original drops its pushed word with `(an instruction of the original)`; the effect on the
/// stack pointer is identical to the plain cdecl return here.)
/// No return slot is written.
export!(cdecl, rw_00b942a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
