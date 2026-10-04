// original: 0x00b8bca0 CHANGE_BLIP_COLOUR
/// Script native `CHANGE_BLIP_COLOUR` (hash 0x1D8800E3).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00b8bca0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
