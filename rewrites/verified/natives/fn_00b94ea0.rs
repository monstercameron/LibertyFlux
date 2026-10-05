// original: 0x00b94ea0 SET_GLOBAL_INSTANCE_PRIORITY
/// Script native `SET_GLOBAL_INSTANCE_PRIORITY` (hash 0x573F5B48).
///
/// Forwards one script argument (a priority value) to the engine.
/// No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b94ea0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
