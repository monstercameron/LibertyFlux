// original: 0x00b943b0 GET_CURRENT_LANGUAGE
/// Script native `GET_CURRENT_LANGUAGE` (hash 0x1105259C).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00b943b0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
