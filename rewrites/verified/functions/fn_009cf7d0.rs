// original: 0x009CF7D0 guarded_init_and_chain (proposed)

/// Guarded initialisation dispatch with a chained tail.
///
/// Calls singleton getter A (callee 1); when it returns non-zero the argument
/// is forwarded to the state-reset routine (callee 2, the function at
/// 0x009CF4A0, stubbed here). Then it calls singleton getter B (callee 3):
/// when that returns zero the function returns (with B's value); otherwise
/// the original jumps into a 1,562-byte sibling sequence elsewhere in the
/// program, which this rewrite does not implement (see the lane report for
/// the staged plan). The proof pins B to zero, so the chained tail never
/// runs on either side; that untaken branch is recorded in `narrowed`.
///
/// Original: 0x009CF7D0 (cdecl, one stack word, forwarded verbatim).
lf_checker_rt::export!(cdecl, rw_009CF7D0(arg0: u32) -> u32 {
    unsafe {
        const GET_A: u32 = 1;
        const STATE_RESET: u32 = 2;
        const GET_B: u32 = 3;

        let a = lf_checker_rt::callee_cdecl!(GET_A, u32,);
        if a != 0 {
            lf_checker_rt::callee_cdecl!(STATE_RESET, u32, arg0);
        }
        lf_checker_rt::callee_cdecl!(GET_B, u32,)
    }
});
