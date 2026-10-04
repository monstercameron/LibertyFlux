// original: 0x00e620e0 timer_obj_init_thunk
// Object initialiser thunk: binds the object pointer and tail-calls the
// shared object initialiser, forwarding its result. Takes no arguments.
lf_checker_rt::export!(cdecl, rw_00e620e0() -> u32 {
    const OBJECT: u32 = 0x01B491D0;
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJECT))
});
