// original: 0x00e5d700 run_init_register_dtor_e5d700
/// Runs the pool's init routine, then registers a teardown routine with the
/// runtime registrar and returns the registrar's answer.
export!(cdecl, rw_e5d700() -> u32 {
    let _: u32 = callee_cdecl!(2, u32,);
    callee_cdecl!(3, u32, relocated(0x00E6EA10))
});
