// original: 0x00b8d940 SET_TEXT_DROPSHADOW
/// Script native `SET_TEXT_DROPSHADOW` (hash 0x58F5023F).
///
/// Forwards five integer script arguments to the engine. No return slot
/// is written.
export!(cdecl, rw_00b8d940(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        )
    }
});
