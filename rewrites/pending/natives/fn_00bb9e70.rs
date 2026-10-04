// original: 0x00bb9e70 TASK_JUMP
/// Script native `TASK_JUMP` (hash 0x5E97106E).
///
/// Forwards two script arguments (a character handle and a boolean flag) to the engine. The flag is coerced with `arg != 0`.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred). The rewrite passes the clean 0/1 flag; the pushed
/// word carries context-pointer high bytes on the original side, so
/// the contract skips that call argument (v2 has no low-byte compare).
/// No return slot is written.
export!(cdecl, rw_00bb9e70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);

        callee_cdecl!(1, u32, *args, flag,)
    }
});
