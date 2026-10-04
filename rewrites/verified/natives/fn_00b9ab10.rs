// original: 0x00b9ab10 GET_SAFE_POSITION_FOR_CHAR
/// Script native `GET_SAFE_POSITION_FOR_CHAR` (hash 0x5D877285).
///
/// Forwards seven script arguments to the engine and stores the low byte of its answer (zero-extended) into the return slot. The fourth argument is a boolean flag coerced with `arg != 0`.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the contract masks
/// that call argument (checks.call_skip).
export!(cdecl, rw_00b9ab10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(3) != 0);
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            flag,
            *args.add(4),
            *args.add(5),
            *args.add(6),
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
