// original: 0x00b9f400 GET_OBJECT_PED_IS_HOLDING
/// Script native `GET_OBJECT_PED_IS_HOLDING` (hash 0x45345838).
///
/// Forwards one script argument (a character handle) to the engine and
/// stores its full 32-bit answer (an object handle) into the return slot.
export!(cdecl, rw_00b9f400(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
