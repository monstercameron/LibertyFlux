// original: 0x00e60260 timer_domain_init_1
// Domain-root timer setup stub: runs the domain setup step (which pops the
// caller's 12 scratch bytes), publishes the domain head word, then hands the
// root descriptor to the shared registrar and returns its answer. Takes no
// arguments.
lf_checker_rt::export!(cdecl, rw_00e60260() -> u32 {
    const DESC: u32 = 0x00E6F7D0;
    const HEAD_SLOT: u32 = 0x019F3204;
    const HEAD_VALUE: u32 = 0x00FE49A8;
    // The original reserves 12 scratch bytes that the setup step pops
    // (stdcall/3); the words are never read, so the rewrite passes zeros and
    // the contract skips those call arguments.
    let _setup: u32 = lf_checker_rt::callee_stdcall!(1, u32, 0, 0, 0);
    unsafe {
        lf_checker_rt::global::<u32>(HEAD_SLOT).write(lf_checker_rt::relocated(HEAD_VALUE));
    }
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(DESC))
});
