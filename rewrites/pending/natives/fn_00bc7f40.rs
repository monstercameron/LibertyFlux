// original: 0x00bc7f40 SET_VEH_ALARM_DURATION
// Rewrite of the SET_VEH_ALARM_DURATION native handler.

/// Script native `SET_VEH_ALARM_DURATION(veh, duration)`.
///
/// Forwards the two script arguments to the engine alarm routine. No return
/// slot is written; the engine's answer is left in the return register.
export!(cdecl, rw_bc7f40(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
