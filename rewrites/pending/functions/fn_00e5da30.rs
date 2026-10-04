// original: 0x00e5da30 net_static_init_09
/// Run one subsystem initializer, then register its handler block.
///
/// The initializer's result is discarded; the registration block address
/// goes to the shared registrar and this returns the registrar's answer.
lf_checker_rt::export!(cdecl, rw_00e5da30() -> u32 {
    lf_checker_rt::callee_cdecl!(0, u32,);
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00e6eb80))
});
