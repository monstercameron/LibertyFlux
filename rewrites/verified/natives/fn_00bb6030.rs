// original: 0x00bb6030 MARK_SCRIPT_AS_NO_LONGER_NEEDED
/// Script native `MARK_SCRIPT_AS_NO_LONGER_NEEDED` (hash 0x09E405DB).
///
/// Forwards one script argument (a script name hash) to the engine. No return slot is written. (The original cleans its one pushed argument with `(an instruction of the original)`; the effect on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bb6030(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
