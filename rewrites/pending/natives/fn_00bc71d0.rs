// original: 0x00bc71d0 REMOVE_CAR_WINDOW
/// Script native `REMOVE_CAR_WINDOW` (hash 0x038A7526).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bc71d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
