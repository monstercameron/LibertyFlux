// original: 0x00ba0890 LOCATE_CHAR_ON_FOOT_2D
/// Script native `LOCATE_CHAR_ON_FOOT_2D` (hash 0x50EE161F).
///
/// Forwards six script arguments (a character handle, four float
/// bit-patterns for the area bounds, and a boolean flag) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
/// The flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes a clean 0/1 and the contract masks
/// that call argument, so the leftover bytes are neither reproduced nor
/// compared.
export!(cdecl, rw_00ba0890(ctx: *const u8) -> u32 {
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
