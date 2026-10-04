// original: 0x00b9f1e0 GET_DAMAGE_TO_PED_BODY_PART
/// Script native `GET_DAMAGE_TO_PED_BODY_PART`.
///
/// Forwards two script arguments to the engine and stores the full 32-bit
/// answer into the return slot. Returns the engine answer.
export!(cdecl, rw_00b9f1e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});

