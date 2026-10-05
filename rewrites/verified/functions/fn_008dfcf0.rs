// original: 0x008dfcf0 pair_subinit (proposed)

/// Initialise the counter block and its two sub-objects.
///
/// Zeroes the words at `this + 0x18` and `this + 0x1c`, writes 1 and 0 to
/// `this + 0x10` and `this + 0x14`, runs the sub-initialiser (callee 1) on
/// the objects at `this + 0x30` and `this + 0x3c`, and clears the half-word
/// at `this + 8`. Thiscall, no stack arguments, two outgoing calls.
lf_checker_rt::export!(thiscall, rw_008dfcf0(this: u32) -> u32 {
    unsafe {
        const SUB_A_OFF: u32 = 0x30;
        const SUB_B_OFF: u32 = 0x3c;
        const CALLEE_SUBINIT: u32 = 1;
        ((this + 0x1c) as *mut u32).write_unaligned(0);
        ((this + 0x18) as *mut u32).write_unaligned(0);
        ((this + 0x10) as *mut u32).write_unaligned(1);
        ((this + 0x14) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(CALLEE_SUBINIT, u32, this + SUB_A_OFF);
        lf_checker_rt::callee_thiscall!(CALLEE_SUBINIT, u32, this + SUB_B_OFF);
        ((this + 8) as *mut u16).write_unaligned(0);
        0
    }
});
