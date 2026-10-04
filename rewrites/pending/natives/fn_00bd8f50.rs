// original: 0x00bd8f50 READ_LOBBY_PREFERENCE
/// Script native `READ_LOBBY_PREFERENCE` (hash 0x177A3DA4).
///
/// Forwards one script argument (a preference index) to the engine and
/// stores its full 32-bit answer into the return slot. Unlike the boolean
/// natives, this handler keeps the whole answer (`mov`, not `movzx`).
export!(cdecl, rw_00bd8f50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
