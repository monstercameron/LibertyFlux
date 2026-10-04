// original: 0x00bd8520 NETWORK_GET_SERVER_NAME
/// Script native `NETWORK_GET_SERVER_NAME` (hash 0x03665B8D).
///
/// Takes no script arguments: calls the engine worker with no
/// arguments and stores its full 32-bit answer into the return slot.
///
/// Stores the engine answer's full 32 bits into the return slot
/// (`mov`, not `movzx`).
///
export!(cdecl, rw_00bd8520(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
