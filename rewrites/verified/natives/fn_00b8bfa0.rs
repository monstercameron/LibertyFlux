// original: 0x00b8bfa0 DIM_BLIP
/// Script native `DIM_BLIP` (hash 0x272D15FD).
///
/// Forwards two script arguments (a blip handle and flags) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b8bfa0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1),)
    }
});
