// original: 0x00D8AC10 render_phase_tree_imposters_init

/// Runs the base constructor with the single forwarded stack argument, installs
/// this class's vtable, sets its enabled flag, and returns the object pointer.
/// The object layout is the engine's 32-bit layout; the helper call is
/// intercepted by the proof contract and receives the original argument.
lf_checker_rt::export!(thiscall, rw_00d8ac10(this: u32, base_arg: u32) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const VTABLE: u32 = 0x00ee_ca94;
    const ENABLED: u32 = 0x8f4;

    let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this, base_arg);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(ENABLED) as *mut u32).write_unaligned(1);
    }
    this
});