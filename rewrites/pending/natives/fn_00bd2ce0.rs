// original: 0x00bd2ce0 GET_CURRENT_POPULATION_ZONE_TYPE
/// Script native `GET_CURRENT_POPULATION_ZONE_TYPE` (hash 0x30516A11).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00bd2ce0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
