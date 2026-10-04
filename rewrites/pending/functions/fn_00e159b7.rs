// original: 0x00e159b7 forward_3arg_default
/// Forward to the 3-argument worker with the third argument fixed to zero.
///
/// The original pushes `0`, then its two stack arguments, and tail-returns
/// the callee's answer (cdecl/3, stubbed as id 1). Returns the answer.
export!(cdecl, rw_00e159b7(a: u32, b: u32) -> u32 {
    callee_cdecl!(1, u32, a, b, 0)
});
