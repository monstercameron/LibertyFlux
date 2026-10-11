// original: 0x00CBFF30 CPedMoveBlendOnFoot::vf7
/// Read the blend object stored in the global lookup slot, call its lookup
/// method, and tail-call the returned object when non-null. A null lookup
/// returns zero. The direct lookup and conditional tail call are both
/// intercepted by the contract; the method has no stack arguments.
lf_checker_rt::export!(thiscall, rw_00cbff30(_this: u32) -> u32 {
    const GLOBAL_LOOKUP: u32 = 0x0167E2A0;
    let global_object = unsafe { lf_checker_rt::global::<u32>(GLOBAL_LOOKUP).read_unaligned() };
    let result = lf_checker_rt::callee_thiscall!(1, u32, global_object);
    if result == 0 {
        0
    } else {
        lf_checker_rt::callee_thiscall!(2, u32, result)
    }
});
