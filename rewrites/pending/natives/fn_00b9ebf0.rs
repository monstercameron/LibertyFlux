// original: 0x00b9ebf0 FREEZE_CHAR_POSITION_AND_DONT_LOAD_COLLISION
/// Freeze a character in place without loading collision.
///
/// Forwards the character handle (argument 0) and the frozen flag (argument
/// 1, normalised to 0/1) to the engine implementation. The original builds the
/// pushed flag dword by writing the 0/1 byte into the low byte of its own
/// incoming stack slot, so the high three bytes are the context pointer's;
/// that is reproduced exactly below. Returns whatever the engine returned.
export!(cdecl, rw_00b9ebf0(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let frozen = u32::from(*args.add(1) != 0);
        let pushed = (ctx & 0xFFFF_FF00) | frozen;
        callee_cdecl!(1, u32, *args, pushed)
    }
});
