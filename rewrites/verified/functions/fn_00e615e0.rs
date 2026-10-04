// original: 0x00e615e0 timing_counters_zero_and_register
/// Zero six timing counters, then register the callback.
///
/// Returns the registrar's answer.
export!(cdecl, rw_00e615e0() -> u32 {
    unsafe {
        let base = global::<u32>(0x01A0221C);
        for i in 0..6usize {
            *base.add(i) = 0;
        }
        callee_cdecl!(1, u32, relocated(0x00E70210))
    }
});
