// original: 0x00bd7210 GET_MINUTES_OF_DAY
/// Script native `GET_MINUTES_OF_DAY` (hash 0x3DFE691D).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00bd7210(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
