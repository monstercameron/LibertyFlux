// original: 0x00AD8930 quad_batch_forward

/// Forward eight 32-bit values, in their original argument order, to the UI
/// quad batch emitter. The target is an intercepted cdecl callee; this wrapper
/// returns that callee's EAX value and does not alter the arguments.
lf_checker_rt::export!(cdecl, rw_00ad8930(
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    seventh: u32,
    eighth: u32,
) -> u32 {
    const UI_QUAD_BATCH_CALLEE: u32 = 1;
    lf_checker_rt::callee_cdecl!(
        UI_QUAD_BATCH_CALLEE,
        u32,
        first,
        second,
        third,
        fourth,
        fifth,
        sixth,
        seventh,
        eighth,
    )
});
