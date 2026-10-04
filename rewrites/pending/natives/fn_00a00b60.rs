// original: 0x00a00b60 FREEZE_POSITION_OF_CLOSEST_OBJECT_OF_TYPE
/// Script native `FREEZE_POSITION_OF_CLOSEST_OBJECT_OF_TYPE` (hash 0x5A196B79).
///
/// Forwards six script arguments to the engine: four float bit-patterns
/// (a position and radius), a model hash, and a boolean flag coerced with
/// `arg != 0`. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred). The rewrite passes the clean 0/1 flag; the pushed
/// word carries context-pointer high bytes on the original side, so
/// the contract skips that call argument (v2 has no low-byte compare).
export!(cdecl, rw_00a00b60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(5) != 0);

        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            flag,
        )
    }
});
