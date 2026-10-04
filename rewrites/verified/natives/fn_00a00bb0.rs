// original: 0x00a00bb0 GET_CAR_OBJECT_IS_ATTACHED_TO
/// Script native `GET_CAR_OBJECT_IS_ATTACHED_TO` (hash 0x2D215414).
///
/// Forwards one object handle to the engine and stores the engine's full
/// 32-bit answer (the attached car handle) into the return slot.
export!(cdecl, rw_00a00bb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
