// original: 0x00bd4700 WASH_VEHICLE_TEXTURES
/// Script native `WASH_VEHICLE_TEXTURES` (hash 0x69491CFA).
///
/// Forwards two script arguments (a vehicle handle and an integer)
/// to the engine. No return slot is written.
export!(cdecl, rw_00bd4700(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
