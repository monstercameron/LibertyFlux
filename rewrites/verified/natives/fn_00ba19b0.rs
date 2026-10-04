// original: 0x00ba19b0 SET_CHAR_READY_TO_BE_STUNNED
/// Script native `SET_CHAR_READY_TO_BE_STUNNED` (hash 0x2B416A06).
///
/// Forwards a character handle and a boolean flag (`arg != 0`) to the
/// engine. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred). The rewrite passes the clean 0/1 flag; the pushed
/// word carries context-pointer high bytes on the original side, so
/// the contract skips that call argument (v2 has no low-byte compare).
export!(cdecl, rw_00ba19b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);

        callee_cdecl!(1, u32, *args, flag)
    }
});
