// original: 0x008ca880 stream_load_maybe_chain
/// Calls a helper with the argument, then either chains to a second routine
/// or returns the helper's answer.
///
/// Takes one stack word `arg`. It calls helper 1 with `arg` (cdecl, one
/// argument). If `arg` equals 1 it tail-chains to routine 2 with the same
/// word and returns routine 2's answer; otherwise it returns the helper's
/// answer unchanged. The frame carries no other state.
export!(cdecl, rw_008ca880(arg: u32) -> u32 {
    let ans: u32 = callee_cdecl!(1, u32, arg);
    if arg == 1 {
        callee_cdecl!(2, u32, arg)
    } else {
        ans
    }
});
