// original: 0x00bc7fe0 SHUT_CAR_DOOR
/// Script native `SHUT_CAR_DOOR` (hash 0x5E7A620E).
///
/// Forwards two script arguments (a vehicle handle and a door index) to the engine. No return slot is written.
export!(cdecl, rw_00bc7fe0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
