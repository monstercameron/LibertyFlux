// original: 0x00b40630 forward_4arg_flag0
/// Forward three arguments to the shared four-argument worker with flag 0.
export!(cdecl, rw_b40630(a: u32, b: u32, c: u32) -> u32 {
    callee_cdecl!(2, u32, a, b, 0, c)
});
