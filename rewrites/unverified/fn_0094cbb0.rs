// original: 0x0094CBB0 clear_head_words_then_slots (proposed)

/// Zero the 32 head words of an object, then tail into the slot clearer.
///
/// Writes zero to the words at `this + HEAD_BASE + i * STRIDE` for
/// `i` in `0..32`, then tails into callee 1 (thiscall, `this` in ECX, no
/// stack words) and returns its answer. Written as a call that forwards
/// the argument and result; the original ends in a jump.
///
/// Original: 0x0094CBB0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_0094CBB0(this: u32) -> u32 {
    unsafe {
        const HEAD_BASE: u32 = 0x9600;
        const STRIDE: u32 = 0xC8;
        const COUNT: u32 = 0x20;
        const CLEAR_SLOTS: u32 = 1;
        let mut p = this.wrapping_add(HEAD_BASE);
        let mut i = 0u32;
        while i < COUNT {
            (p as *mut u16).write_unaligned(0);
            p = p.wrapping_add(STRIDE);
            i += 1;
        }
        lf_checker_rt::callee_thiscall!(CLEAR_SLOTS, u32, this)
    }
});
