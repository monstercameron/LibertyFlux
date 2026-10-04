// original: 0x00b9fd50 IS_CHAR_SHOOTING_IN_AREA
/// Script native `IS_CHAR_SHOOTING_IN_AREA` (hash 0x42941472).
///
/// Forwards six script arguments to the engine: a character handle, four
/// float bit-patterns (area bounds) and a boolean flag. The flag is
/// coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the contract masks
/// that call argument (checks.call_skip).
///
/// Stores the low byte of the engine answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00b9fd50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(5) != 0);
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            flag,
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
