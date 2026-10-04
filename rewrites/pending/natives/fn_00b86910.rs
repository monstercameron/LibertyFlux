// original: 0x00b86910 COUNT_SCRIPT_CAMS_BY_TYPE_AND_OR_STATE
/// Script native `COUNT_SCRIPT_CAMS_BY_TYPE_AND_OR_STATE`.
///
/// Forwards three script arguments to the engine and stores the full 32-bit
/// answer into the return slot. Returns the engine answer.
export!(cdecl, rw_00b86910(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});

