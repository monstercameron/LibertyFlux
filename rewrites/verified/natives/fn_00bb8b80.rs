// original: 0x00bb8b80 GET_NAVMESH_ROUTE_RESULT
/// Script native `GET_NAVMESH_ROUTE_RESULT` (hash 0x4EFE6B67).
///
/// Forwards one script argument (a route handle) to the engine and stores
/// its full 32-bit answer into the return slot.
export!(cdecl, rw_00bb8b80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
