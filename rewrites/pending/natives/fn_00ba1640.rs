// original: 0x00ba1640 SET_CHAR_KEEP_TASK
/// Script native `SET_CHAR_KEEP_TASK` (hash 0x264009D3).
///
/// Forwards two script arguments (a character handle and a boolean flag) to
/// the engine. The flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the pushed
/// argument is masked in the contract (call_skip). No return slot is written.
export!(cdecl, rw_00ba1640(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
