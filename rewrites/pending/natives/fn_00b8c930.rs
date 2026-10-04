// original: 0x00b8c930 GET_LENGTH_OF_STRING_WITH_THIS_HASH_KEY
/// Script native `GET_LENGTH_OF_STRING_WITH_THIS_HASH_KEY` (hash 0x6C013A17).
///
/// Hash key; stores full answer.
///
/// Stores the engine's full 32-bit answer into the return slot.
export!(cdecl, rw_00b8c930(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
