// original: 0x00bc5280 CHANGE_CAR_COLOUR
/// Script native `CHANGE_CAR_COLOUR` (hash 0x06441EAF).
///
/// Forwards three script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bc5280(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
