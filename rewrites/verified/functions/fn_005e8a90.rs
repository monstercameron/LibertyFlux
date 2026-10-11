// original: 0x005E8A90 CTaskComplexWanderCop::CTaskComplexWanderCop
const WANDER_COP_VTABLE_FILE_VA: u32 = 0x00FE1314;
const WANDER_MAX_STEP_BITS: u32 = 0x41000000;
const WANDER_MAX_DISTANCE_BITS: u32 = 0x41F00000;

/// Initializes a wander task after its base initializer. The method takes
/// four stack words and the low 32-bit lane of XMM2, passes the first stack
/// word, that lane, two flags, and the configured maximum distance to the
/// base initializer, then installs the task vtable and clears its owned
/// subtask and state fields. The fixed step and distance values are stored
/// as their exact single-precision bit patterns. It returns `this` in EAX.
lf_checker_rt::export!(thiscall, rw_005e8a90(
    this: u32,
    first_arg: u32,
    _arg1: u32,
    _arg2: u32,
    _arg3: u32
) -> u32 {
    let max_distance = unsafe {
        lf_checker_rt::global::<u32>(0x00ed7ee4).read_unaligned()
    };
    let vector_arg = lf_checker_rt::xmm_word(2, 0);
    lf_checker_rt::callee_thiscall!(1, u32, this, first_arg, vector_arg, 1, max_distance, 1);

    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(WANDER_COP_VTABLE_FILE_VA));
        (this.wrapping_add(0x80) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x84) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x88) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x8c) as *mut u16).write_unaligned(0);
        (this.wrapping_add(0x90) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x94) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x98) as *mut u16).write_unaligned(0);
        (this.wrapping_add(0x9c) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0xa0) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0xa4) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x24) as *mut u32).write_unaligned(WANDER_MAX_STEP_BITS);
        (this.wrapping_add(0x28) as *mut u32).write_unaligned(WANDER_MAX_DISTANCE_BITS);
    }
    this
});
