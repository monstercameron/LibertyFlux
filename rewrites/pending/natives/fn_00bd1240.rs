// original: 0x00bd1240 SET_MIN_MAX_PED_ACCURACY
/// Script native `SET_MIN_MAX_PED_ACCURACY` (hash 0x74627538).
///
/// Forwards three script arguments (a ped handle and two accuracy bounds, forwarded as raw bits) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bd1240(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
