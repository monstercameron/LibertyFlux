// original: 0x00D4F7E0 check_global_and_object_predicates
/// Evaluate a global readiness predicate, then consult two object predicates
/// in order. If global readiness is false, the result is true. Otherwise the
/// object check must be true and the final object check true. The argument
/// pointer is passed to both object methods through ECX; all results are low
/// byte booleans.
lf_checker_rt::export!(cdecl, rw_00d4f7e0(object: u32) -> u8 {
    const GLOBAL_READY: u32 = 1;
    const OBJECT_ELIGIBLE: u32 = 2;
    const OBJECT_BLOCKED: u32 = 3;

    let ready: u8 = lf_checker_rt::callee_cdecl!(GLOBAL_READY, u8,);
    if ready == 0 {
        return 1;
    }

    let eligible: u8 = lf_checker_rt::callee_thiscall!(OBJECT_ELIGIBLE, u8, object);
    if eligible == 0 {
        return 0;
    }

    let blocked: u8 = lf_checker_rt::callee_thiscall!(OBJECT_BLOCKED, u8, object);
    u8::from(blocked != 0)
});
