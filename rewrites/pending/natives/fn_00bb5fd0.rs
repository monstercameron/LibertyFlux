// original: 0x00bb5fd0 GET_NUMBER_OF_INSTANCES_OF_STREAMED_SCRIPT
/// Native handler `GET_NUMBER_OF_INSTANCES_OF_STREAMED_SCRIPT`.
///
/// Report how many instances of a streamed script are running.
/// Forwards 1 argument(s) to the engine and stores its full
/// 32-bit answer into the return slot.
export!(cdecl, rw_00bb5fd0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let _ = arg0;
        let answer = callee_cdecl!(1, u32, arg0);
        *(*ctx as *mut u32) = answer;
        0
    }
});
