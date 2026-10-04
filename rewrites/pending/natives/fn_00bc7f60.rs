// original: 0x00bc7f60 SET_VEH_HAS_STRONG_AXLES
/// Set whether a vehicle has strong axles.
///
/// Forwards the vehicle handle (argument 0) and the flag (argument 1,
/// normalised to 0/1) to the engine. The pushed flag dword carries the context
/// pointer's high bytes (see `rw_00b9ebf0`), reproduced exactly. Returns
/// whatever the engine returned.
export!(cdecl, rw_00bc7f60(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let pushed = (ctx & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, pushed)
    }
});
