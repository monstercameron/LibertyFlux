// original: 0x00ba1fb0 SET_DEFAULT_TARGET_SCORING_FUNCTION
// rw_set_default_target_scoring_function: native SET_DEFAULT_TARGET_SCORING_FUNCTION (handler 0x00BA1FB0).
//
// Forwards a ped handle and a scoring-function id. No return slot.
export!(cdecl, rw_set_default_target_scoring_function(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1));
        ans
    }
});
