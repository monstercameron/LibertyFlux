// original: 0x00CDEC60 CTaskSimpleNMRelax::vf27

/// Forward the ped and the zero-valued Relax variant to the shared task dispatcher. The method's object pointer stays in ECX, the ped
/// pointer is its one stack argument, and the shared callee's EAX is returned
/// unchanged. The real helper body is intercepted by the checker; this
/// rewrite specifies the constants, argument order, and forwarding behavior.
lf_checker_rt::export!(thiscall, rw_00cdec60(this: u32, ped: u32) -> u32 {
            const RELAX_VARIANT: u32 = 0;
    lf_checker_rt::callee_thiscall!(
        1,
        u32,
        this,
        ped,
            RELAX_VARIANT,
    )
});
