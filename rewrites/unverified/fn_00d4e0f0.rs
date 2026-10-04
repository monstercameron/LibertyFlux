// original: 0x00D4E0F0 task_timer_callback_b (proposed)

// Timer/condition callback, variant B: refreshes a -8.0 threshold then clears a handle.
///
/// When the word at `+0x10` is non-zero the object pointer goes in ECX with
/// the float -8.0 to intercepted callee 1. Then the stack argument is taken
/// as an object pointer in ECX for intercepted callee 2 with a zero argument.
/// Returns nothing.
///
/// Original: 0x00D4E0F0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d4e0f0(this: u32, handle: u32) -> u32 {
    unsafe {
        const NEG_EIGHT_BITS: u32 = 0xC1000000; // -8.0f
        const C1: u32 = 1;
        const C2: u32 = 2;
        if ((this + 0x10) as *const u32).read_unaligned() != 0 {
            lf_checker_rt::callee_thiscall!(C1, u32, this, NEG_EIGHT_BITS);
        }
        lf_checker_rt::callee_thiscall!(C2, u32, handle, 0);
        0
    }
});
