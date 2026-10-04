// original: 0x00b8b9b0 ADD_BLIP_FOR_CAR
/// Script native `ADD_BLIP_FOR_CAR` (hash 0x6D21564D).
///
/// Forwards two script arguments (a vehicle handle and an out-pointer slot
/// for the new blip handle) to the engine. No return slot is written.
export!(cdecl, rw_00b8b9b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
