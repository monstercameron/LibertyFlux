// original: 0x00b9ef70 GET_CHAR_MONEY
/// Script native `GET_CHAR_MONEY` (hash 0x7D675993).
///
/// Forwards one script argument (a character handle) to the engine and
/// stores its full 32-bit answer into the return slot. Unlike the
/// boolean natives, this handler keeps the whole answer.
export!(cdecl, rw_00b9ef70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
