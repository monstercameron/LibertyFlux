// original: 0x00bc7e40 SET_VEHICLE_DIRT_LEVEL
/// Script native `SET_VEHICLE_DIRT_LEVEL` (hash 0x02A57428).
///
/// Forwards two script arguments (a vehicle handle and a dirt level, forwarded as raw bits) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bc7e40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
