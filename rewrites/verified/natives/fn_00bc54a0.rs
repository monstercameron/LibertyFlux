// original: 0x00bc54a0 CREATE_MISSION_TRAIN
/// Script native `CREATE_MISSION_TRAIN` (hash 0x0DDD70AE).
///
/// Forwards six script arguments to the engine: an integer, three float bit-patterns (spawn coordinates), a boolean flag, and an integer. The flag is coerced with `arg != 0`.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the pushed
/// argument is masked in the contract (call_skip).
export!(cdecl, rw_00bc54a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(4) != 0);
        let quirked = flag;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), quirked, *args.add(5))
    }
});
