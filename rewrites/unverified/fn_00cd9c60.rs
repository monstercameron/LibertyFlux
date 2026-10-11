// original: 0x00CD9C60 CTaskComplexNM::CTaskComplexNM

/// Initialize the common `CTaskComplexNM` state after calling its base task
/// constructor. It stores the first three integer arguments at offsets 0x14,
/// 0x18 and 0x28, copies the fourth argument's exact float bits to offset
/// 0x20, clears the word at 0x1C and the halfword at 0x24, installs the
/// derived vtable, and returns `this`.
///
/// Calling convention: thiscall with four 32-bit stack words; the final word
/// is a single-precision value transported unchanged. The base constructor is
/// thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00cd9c60(this: u32, animation: u32, blend_state: u32, mode: u32, rate_bits: u32) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const ANIMATION: u32 = 0x14;
    const BLEND_STATE: u32 = 0x18;
    const FLAGS: u32 = 0x1C;
    const RATE: u32 = 0x20;
    const RESERVED: u32 = 0x24;
    const MODE: u32 = 0x28;
    const VTABLE: u32 = 0x00ED_D6E4;

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this);
        ((this + ANIMATION) as *mut u32).write_unaligned(animation);
        ((this + BLEND_STATE) as *mut u32).write_unaligned(blend_state);
        ((this + MODE) as *mut u32).write_unaligned(mode);
        ((this + FLAGS) as *mut u32).write_unaligned(0);
        ((this + RATE) as *mut u32).write_unaligned(rate_bits);
        ((this + RESERVED) as *mut u16).write_unaligned(0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        this
    }
});
