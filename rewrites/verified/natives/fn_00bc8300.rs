// original: 0x00bc8300 TRIGGER_VEH_ALARM
/// Script native `TRIGGER_VEH_ALARM` (hash 0x5E5047AC).
///
/// Forwards one script argument (a vehicle handle) to the engine.
/// No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bc8300(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
