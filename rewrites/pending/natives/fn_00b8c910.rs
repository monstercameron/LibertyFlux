// original: 0x00b8c910 GET_LENGTH_OF_LITERAL_STRING
/// Script native `GET_LENGTH_OF_LITERAL_STRING` (hash 0x02BE2D97).
///
/// Forwards one script argument (a string pointer) to the engine and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00b8c910(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
