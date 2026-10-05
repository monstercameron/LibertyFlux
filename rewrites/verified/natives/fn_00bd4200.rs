// original: 0x00bd4200 SET_CURRENT_MOVIE
/// Script native `SET_CURRENT_MOVIE` (hash 0x5AF23F31).
///
/// Forwards one script argument (a movie name hash) to the engine. No
/// return slot is written. (The original cleans its one pushed argument
/// with `(an instruction of the original)`; the effect on the stack pointer is identical to the
/// plain cdecl return here.)
export!(cdecl, rw_00bd4200(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
