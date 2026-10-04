// original: 0x00b9f560 GIVE_PED_AMBIENT_OBJECT
/// Script native `GIVE_PED_AMBIENT_OBJECT` (hash 0x44AA71F9).
///
/// Forwards two script arguments (a ped handle and an object handle) to
/// the engine. No return slot is written.
export!(cdecl, rw_00b9f560(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
