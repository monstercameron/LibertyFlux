// original: 0x00E466F0 UIBenchMark::UIBenchMark_2

/// Install this class's vtable, run its class-specific initialization helper, then transfer to the shared UI child-cleanup routine with the same object in ECX. The initializer is a thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00e466f0(this: u32) -> u32 {
    const CLASS_VTABLE: u32 = 0x00F166EC;
    unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(CLASS_VTABLE)); }
    let _init_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    lf_checker_rt::callee_thiscall!(2, u32, this)
});
