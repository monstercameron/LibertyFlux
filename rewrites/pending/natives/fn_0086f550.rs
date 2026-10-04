// original: 0x0086f550 GET_NUM_CONSOLE_COMMAND_TOKENS
/// Script native `GET_NUM_CONSOLE_COMMAND_TOKENS` (hash 0x205E2D92).
///
/// Takes no script arguments and makes no engine call: it writes zero
/// directly into the return slot.
export!(cdecl, rw_0086f550(ctx: *const u8) -> u32 {
    unsafe {
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = 0;
        slot as u32
    }
});
