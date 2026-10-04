// original: 0x00a00780 CREATE_PICKUP
/// Script native `CREATE_PICKUP` (hash 0x7E2868D4).
///
/// Forwards seven script arguments to the engine: two integers, three float
/// bit-patterns (spawn coordinates), one integer, and a boolean flag. The
/// flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the contract masks
/// that call argument (checks.call_skip). No return slot is written.
export!(cdecl, rw_00a00780(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(6) != 0);
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            flag,
        )
    }
});
