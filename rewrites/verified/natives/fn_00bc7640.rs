// original: 0x00bc7640 SET_CAR_IN_CUTSCENE
/// Script native `SET_CAR_IN_CUTSCENE` (hash 0x32593711).
///
/// Forwards two script arguments to the engine: a vehicle handle and a boolean flag. The flag is coerced with `arg != 0`.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the pushed
/// argument is masked in the contract (call_skip).
export!(cdecl, rw_00bc7640(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
