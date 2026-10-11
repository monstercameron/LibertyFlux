// original: 0x00CDF140 CTaskSimpleNMSit::vf27

/// Forward the ped and the zero-valued Sit variant to the shared task dispatcher. The method's object pointer stays in ECX, the ped
/// pointer is its one stack argument, and the shared callee's EAX is returned
/// unchanged. The real helper body is intercepted by the checker; this
/// rewrite specifies the constants, argument order, and forwarding behavior.
lf_checker_rt::export!(thiscall, rw_00cdf140(this: u32, ped: u32) -> u32 {
            const SIT_VARIANT: u32 = 0;
    lf_checker_rt::callee_thiscall!(
        1,
        u32,
        this,
        ped,
            SIT_VARIANT,
    )
});
