// original: 0x00e6b0e0 task_single_init_02 (proposed)

/// Initialise one record, then notify done.
///
/// Calls the per-record routine once with a fixed record address
/// (object call, no stack arguments), then the shared notifier
/// with this unit's fixed address (cdecl, one stack word). No
/// arguments, no meaningful return value.
lf_checker_rt::export!(cdecl, rw_00e6b0e0() -> u32 {
    const THIS_ADDR: u32 = 0x016dcc30;
    const ARG_ADDR: u32 = 0x00e72af0;
    let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(THIS_ADDR));
    let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(ARG_ADDR));
    0
});
