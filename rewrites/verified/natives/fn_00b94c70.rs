// original: 0x00b94c70 READ_KILL_FRENZY_STATUS
/// Script native `READ_KILL_FRENZY_STATUS` (hash 0x3F9F0CF5).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00b94c70(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
