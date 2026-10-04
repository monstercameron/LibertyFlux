// original: 0x00bc7600 SET_CAR_HEADING
/// Script native `SET_CAR_HEADING` (hash 0x75E40528).
///
/// Forwards two script arguments (a vehicle handle and a float heading,
/// copied as raw bits) to the engine. No return slot is written.
export!(cdecl, rw_00bc7600(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
