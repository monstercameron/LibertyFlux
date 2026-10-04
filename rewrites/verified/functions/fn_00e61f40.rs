// original: 0x00e61f40 timer_reg_domain
// Domain-root registration stub: runs the domain setup step (which pops the
// caller's 12 scratch bytes), publishes the domain head word, then hands the
// root descriptor to the shared registrar and returns its answer.
lf_checker_rt::export!(cdecl, rw_00e61f40() -> u32 {
    const DESC: u32 = 0x00E70830;
    const HEAD_SLOT: u32 = 0x01B492A8;
    const HEAD_VALUE: u32 = 0x00FE5B60;
    // The original reserves 12 scratch bytes that the setup step pops
    // (stdcall/3); the words are never read, so the rewrite passes zeros and
    // the contract skips those call arguments.
    let _setup: u32 = lf_checker_rt::callee_stdcall!(1, u32, 0, 0, 0);
    unsafe {
        lf_checker_rt::global::<u32>(HEAD_SLOT).write(lf_checker_rt::relocated(HEAD_VALUE));
    }
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(DESC))
});
