// original: 0x00b9fae0 IS_CHAR_IN_AREA_ON_FOOT_2D
/// Script native `IS_CHAR_IN_AREA_ON_FOOT_2D` (hash 0x3F2D7D06).
///
/// Forwards six script arguments to the engine: a character handle, four float bit-patterns (an area rectangle) and a boolean flag coerced with `arg != 0`. Floats are copied as raw bits. Stores the low byte of the engine answer (zero-extended) into the return slot.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred). The rewrite passes the clean 0/1 flag; the pushed
/// word carries context-pointer high bytes on the original side, so
/// the contract skips that call argument (v2 has no low-byte compare).
export!(cdecl, rw_00b9fae0(ctx: *const u8) -> u32 {
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
