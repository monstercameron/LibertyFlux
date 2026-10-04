// original: 0x00b93ff0 CLEAR_AREA
/// Script native `CLEAR_AREA` (hash 0x27722942).
///
/// Forwards five script arguments to the engine: four float coordinates (copied as raw bits) and a boolean flag coerced with `arg != 0`. No return slot is written.

/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful.
export!(cdecl, rw_00b93ff0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(4) != 0);
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            flag,
        )
    }
});
