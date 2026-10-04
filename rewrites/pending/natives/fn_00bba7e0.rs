// original: 0x00bba7e0 TASK_SIT_DOWN_ON_NEAREST_OBJECT
/// Script native `TASK_SIT_DOWN_ON_NEAREST_OBJECT` (hash 0x725654F4).
///
/// Forwards ten script arguments to the engine, including float bit-patterns moved through stack temporaries. The last argument is a boolean flag coerced with `arg != 0`. No return slot is written.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the contract masks
/// that call argument (checks.call_skip).
export!(cdecl, rw_00bba7e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(9) != 0);
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
            *args.add(7),
            *args.add(8),
            flag,
        )
    }
});
