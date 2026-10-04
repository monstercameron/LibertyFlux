// original: 0x009cbce0 GET_STREAM_PLAYTIME
/// Script native handler `GET_STREAM_PLAYTIME` (hash 0x4B6211F2).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores its full answer into the return slot.
export!(cdecl, rw_009cbce0(ctx: u32) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
