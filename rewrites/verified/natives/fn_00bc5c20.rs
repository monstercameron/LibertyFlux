// original: 0x00bc5c20 GET_CAR_SIREN_HEALTH
/// Script native `GET_CAR_SIREN_HEALTH` (hash 0x0896249A).
///
/// Forwards one script argument (a vehicle handle) to the engine and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00bc5c20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
