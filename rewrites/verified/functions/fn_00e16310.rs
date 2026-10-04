// original: 0x00e16310 forward_6arg_reordered
/// Forward five arguments plus a constant flag to the 6-argument worker.
///
/// The original reorders its stack arguments as (b, c, d, e, a) and appends
/// the constant flag `1` (cdecl/6, stubbed as id 1), returning the answer.
export!(cdecl, rw_00e16310(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    const FLAG: u32 = 1;
    callee_cdecl!(1, u32, b, c, d, e, a, FLAG)
});
