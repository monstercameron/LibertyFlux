// original: 0x00ba1c90 SET_CHAR_WILL_LEAVE_CAR_IN_COMBAT
/// Set whether a character leaves cars during combat.
///
/// Forwards the character handle (argument 0) and the flag (argument 1,
/// normalised to 0/1) to the engine. The pushed flag dword carries the
/// context pointer's high bytes (see `rw_00b9ebf0`), reproduced exactly.
/// Returns whatever the engine returned.
export!(cdecl, rw_00ba1c90(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let pushed = (ctx & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, pushed)
    }
});
