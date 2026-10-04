// original: 0x00bd2cf0 GET_CURRENT_ZONE_SCUMMINESS
/// Script native `GET_CURRENT_ZONE_SCUMMINESS` (hash 0x4B7B5F77).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00bd2cf0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
