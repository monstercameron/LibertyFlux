// original: 0x00bb5fc0 GET_ID_OF_THIS_THREAD
/// Script native `GET_ID_OF_THIS_THREAD` (hash 0x051A131D).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the full 32-bit answer into the return slot.
export!(cdecl, rw_00bb5fc0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
