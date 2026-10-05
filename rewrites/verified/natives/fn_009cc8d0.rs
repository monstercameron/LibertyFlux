// original: 0x009cc8d0 STOP_SOUND
/// Script native `STOP_SOUND` (hash 0x09DB00B9).
///
/// Forwards one script argument (a sound handle) to the engine. No return
/// slot is written. (The original cleans its one pushed argument with
/// `(an instruction of the original)`; the effect on the stack pointer is identical to the plain
/// cdecl return here.)
export!(cdecl, rw_009cc8d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
