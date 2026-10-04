// original: 0x00ba21a0 SET_LOAD_COLLISION_FOR_CHAR_FLAG
/// Script native `SET_LOAD_COLLISION_FOR_CHAR_FLAG` (hash 0x4AA762A4).
///
/// Forwards two script arguments (a character handle and a boolean flag) to
/// the engine. The flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful. No return slot is written.
export!(cdecl, rw_00ba21a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, flag)
    }
});
