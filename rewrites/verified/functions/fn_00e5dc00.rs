// original: 0x00e5dc00 net_obj_init_thunk_dc00
/// Tail-dispatch to one fixed object initialiser (jump thunk).
///
/// Forwards to the zero-argument initialiser with the fixed global
/// instance and returns its result. Takes no inputs.
lf_checker_rt::export!(cdecl, rw_00e5dc00() -> u32 {
    lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(0x019D2E08))
});
