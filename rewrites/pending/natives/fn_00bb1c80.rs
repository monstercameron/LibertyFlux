// original: 0x00bb1c80 CREATE_PLAYER
/// Script native `CREATE_PLAYER` (hash 0x335E3951).
///
/// Forwards five script arguments to the engine: a player index, three float
/// bit-patterns (spawn coordinates) moved through vector registers, and a
/// model hash. Floats are forwarded as raw `u32` bits, so they match
/// bit-exactly by construction. No return slot is written.
export!(cdecl, rw_00bb1c80(ctx: *const u8) -> u32 {
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
