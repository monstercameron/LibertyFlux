// original: 0x00bba860 TASK_SIT_DOWN_ON_OBJECT
/// Script native `TASK_SIT_DOWN_ON_OBJECT` (hash 0x515C3218).
///
/// Forwards 10 script arguments to the engine (raw bit patterns, so the forward is bit-exact); the boolean argument is coerced with `arg != 0`. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the plain 0/1 flag; the argument sits beyond
/// the checker's compared call window so no masking is needed.
export!(cdecl, rw_00bba860(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(9) != 0);
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6), *args.add(7), *args.add(8), flag)
    }
});
