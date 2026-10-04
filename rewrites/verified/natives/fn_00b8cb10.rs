// original: 0x00b8cb10 GET_STRING_FROM_HASH_KEY
/// Script native `GET_STRING_FROM_HASH_KEY` (hash 0x16E14EA4).
///
/// Forwards one script argument (a hash key) to the engine and stores the
/// full 32-bit answer into the return slot.
export!(cdecl, rw_00b8cb10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
