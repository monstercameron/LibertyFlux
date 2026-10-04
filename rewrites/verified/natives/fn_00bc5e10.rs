// original: 0x00bc5e10 GET_DISPLAY_NAME_FROM_VEHICLE_MODEL
/// Script native `GET_DISPLAY_NAME_FROM_VEHICLE_MODEL` (hash 0x404E0056).
///
/// Forwards one script argument to the engine and stores the engine's full
/// 32-bit answer into the return slot.
export!(cdecl, rw_00bc5e10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
