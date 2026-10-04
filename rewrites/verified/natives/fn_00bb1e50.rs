// original: 0x00bb1e50 GET_NO_LAW_VEHICLES_DESTROYED_BY_LOCAL_PLAYER
/// Script native `GET_NO_LAW_VEHICLES_DESTROYED_BY_LOCAL_PLAYER` (hash
/// 0x63C50673).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00bb1e50(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
