// original: 0x00b40650 forward_4arg_flag1
/// Forward three arguments to the shared four-argument worker with flag 1.
export!(cdecl, rw_b40650(a: u32, b: u32, c: u32) -> u32 {
    callee_cdecl!(2, u32, a, b, 1, c)
});
