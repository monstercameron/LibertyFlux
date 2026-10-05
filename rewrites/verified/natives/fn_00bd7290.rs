// original: 0x00bd7290 SET_TIME_OF_NEXT_APPOINTMENT
/// Script native `SET_TIME_OF_NEXT_APPOINTMENT` (hash 0x0A7D3AF9).
///
/// Forwards one script argument to the engine. No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bd7290(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
