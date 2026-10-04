// original: 0x00bd1020 GIVE_WEAPON_TO_CHAR
/// Script native `GIVE_WEAPON_TO_CHAR` (hash 0x03E90416).
///
/// Forwards four script arguments (a character handle, a weapon id, ammo and
/// a boolean flag) to the engine. The flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred). The rewrite passes the clean 0/1 flag; the pushed
/// word carries context-pointer high bytes on the original side, so
/// the contract skips that call argument (v2 has no low-byte compare). No return slot is written.
export!(cdecl, rw_00bd1020(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(3) != 0);

        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), flag)
    }
});
