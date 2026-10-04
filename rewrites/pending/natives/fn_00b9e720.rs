// original: 0x00b9e720 CREATE_CHAR_INSIDE_CAR
/// Script native `CREATE_CHAR_INSIDE_CAR` (hash 0x2702274D).
///
/// Forwards four script arguments (a vehicle handle and three spawn parameters) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b9e720(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
