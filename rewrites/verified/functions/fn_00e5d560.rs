// original: 0x00e5d560 atexit_register_dtor_e5d560
/// Registers a teardown routine with the runtime registrar and returns the
/// registrar's answer (nonzero on success).
export!(cdecl, rw_e5d560() -> u32 {
    callee_cdecl!(2, u32, relocated(0x00E6E980))
});
