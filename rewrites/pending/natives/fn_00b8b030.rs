// original: 0x00b8b030 GET_CONSOLE_COMMAND
/// Script native `GET_CONSOLE_COMMAND` (hash 0x3BC51157).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00b8b030(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
