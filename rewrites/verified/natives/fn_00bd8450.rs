// original: 0x00bd8450 NETWORK_GET_MAX_PRIVATE_SLOTS
/// Script native `NETWORK_GET_MAX_PRIVATE_SLOTS` (hash 0x2EF80425).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer (the maximum private slot count) into the return slot.
export!(cdecl, rw_00bd8450(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});

