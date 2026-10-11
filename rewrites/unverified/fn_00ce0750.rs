// original: 0x00CE0750 CTaskSimpleNMRollUpAndRelax::vf28

/// Forward the ped, mode 0, and action 1 to the shared NaturalMotion dispatcher. The method's object pointer stays in ECX, the ped
/// pointer is its one stack argument, and the shared callee's EAX is returned
/// unchanged. The real helper body is intercepted by the checker; this
/// rewrite specifies the constants, argument order, and forwarding behavior.
lf_checker_rt::export!(thiscall, rw_00ce0750(this: u32, ped: u32) -> u32 {
            const NM_MODE: u32 = 0;
        const NM_ACTION: u32 = 1;
    lf_checker_rt::callee_thiscall!(
        1,
        u32,
        this,
        ped,
            NM_MODE,
            NM_ACTION,
    )
});
