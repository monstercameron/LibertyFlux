// original: 0x00b86960 DESTROY_ALL_CAMS
/// Script native handler `DESTROY_ALL_CAMS`.
///
/// Tail jump to the shared camera teardown implementation.
lf_rn94_rt::export!(cdecl, rw_fn_00b86960(ctx: u32) -> u32 {
    unsafe {
        // The original body is a single tail jump to a shared implementation,
        // entered with the context pointer on the stack. The checker logs the
        // trampoline return address in call slot 0 (skipped by the contract),
        // so the rewrite forwards the context in slot 1 where it is compared.
        lf_rn94_rt::callee_cdecl!(1, u32, 0, ctx)
    }
});
