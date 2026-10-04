// original: 0x00bc7a10 SET_HELI_FORCE_ENGINE_ON
/// Script native `SET_HELI_FORCE_ENGINE_ON` (hash 0x3B8F5E20).
///
/// Forwards a helicopter handle and coerces script argument 1 to a 0/1 engine-on flag.
/// No return slot is written.
/// The original pushes the flag byte through its own stack slot; the contract masks that call argument to the low byte.
export!(cdecl, rw_00bc7a10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, flag)
    }
});
