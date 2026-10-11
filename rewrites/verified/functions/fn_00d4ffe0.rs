// original: 0x00D4FFE0 initialize_swap_weapon_task
/// Initialize the swap-weapon task after its base constructor. The argument is
/// stored at `+0x14`; the object then receives its vtable and default state
/// words at `+0x18` through `+0x24`. The constructor returns the object pointer.
lf_checker_rt::export!(thiscall, rw_00d4ffe0(this: u32, target: u32) -> u32 {
    const VTABLE: u32 = 0x00ee_525c;
    const TARGET: u32 = 0x14;
    const STATE_A: u32 = 0x18;
    const STATE_B: u32 = 0x1c;
    const STATE_C: u32 = 0x20;
    const STATE_D: u32 = 0x24;
    const BASE_CONSTRUCTOR: u32 = 1;

    let _: u32 = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this);
    let target_value = target;
    unsafe {
        ((this + TARGET) as *mut u32).write_unaligned(target_value);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + STATE_A) as *mut u32).write_unaligned(0);
        ((this + STATE_B) as *mut u32).write_unaligned(u32::MAX);
        ((this + STATE_C) as *mut u32).write_unaligned(u32::MAX);
        ((this + STATE_D) as *mut u32).write_unaligned(0);
    }
    this
});
