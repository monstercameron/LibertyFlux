// original: 0x00bd84c0 NETWORK_GET_NUM_PARTY_MEMBERS
/// Script native `NETWORK_GET_NUM_PARTY_MEMBERS`.
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00bd84c0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
