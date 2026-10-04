// original: 0x00ba0b90 MP_GET_PREFERENCE_VALUE
/// Script native `MP_GET_PREFERENCE_VALUE` (hash 0x54F61C99).
///
/// Forwards one preference index to the engine and stores the engine's
/// full 32-bit answer into the return slot.
export!(cdecl, rw_00ba0b90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
