// original: 0x00e5dc50 net_zero_state_register_dc50
/// Clear a fixed state block, then register one fixed handler address.
///
/// Zeroes the dedicated global words, then calls the central registrar
/// with its constant address argument and returns the registrar answer.
/// Takes no inputs.
lf_checker_rt::export!(cdecl, rw_00e5dc50() -> u32 {
    unsafe {
        let base = lf_checker_rt::global::<u32>(0x019D2F20);
        for i in 0..10 {
            *base.add(i) = 0;
        }
    }
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E6ECA0))
});
