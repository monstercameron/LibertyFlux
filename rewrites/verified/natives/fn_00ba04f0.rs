// original: 0x00ba04f0 LOCATE_CHAR_ANY_MEANS_CHAR_3D
/// Script native `LOCATE_CHAR_ANY_MEANS_CHAR_3D` (hash 0x3E441A58).
///
/// Forwards two character handles, a position (three floats passed as raw
/// bits) and a boolean flag to the engine, and stores the low byte of its
/// answer (zero-extended) into the return slot. The flag is coerced with
/// `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the pushed
/// argument is masked in the contract (call_skip).
export!(cdecl, rw_00ba04f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(5) != 0);
        let quirked = flag;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), quirked);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
