// original: 0x00b8cb00 GET_SIMPLE_BLIP_ID
/// Script native `GET_SIMPLE_BLIP_ID` (hash 0x047B0898).
///
/// Takes no script arguments: calls the engine worker and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00b8cb00(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
