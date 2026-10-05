// original: 0x00c08b90 stream_range_check (proposed)

/// Test whether the argument falls inside the streaming window.
///
/// `this` points to the window (`LO` its lower edge, `SPAN` its width); the
/// argument must be at least `LO` and at most `LO + SPAN` (the addition in
/// that operand order), and the gate helper (callee 1, no arguments) must
/// answer zero. Unordered (NaN) comparisons fail. Returns 1 when all three
/// hold, else 0.
///
/// Original: 0x00c08b90 (thiscall, one stack word, byte result).
lf_checker_rt::export!(thiscall, rw_00c08b90(this: u32, arg: u32) -> u32 {
    unsafe {
        const LO: u32 = 0x04;
        const SPAN: u32 = 0x08;
        const GATE: u32 = 1;
        let lo = f32::from_bits((this.wrapping_add(LO) as *const u32).read_unaligned());
        let a = f32::from_bits(arg);
        if !(a >= lo) {
            return 0;
        }
        let span = f32::from_bits((this.wrapping_add(SPAN) as *const u32).read_unaligned());
        let hi = core::hint::black_box(span) + core::hint::black_box(lo);
        if !(hi >= a) {
            return 0;
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, this);
        if r & 0xff != 0 { 0 } else { 1 }
    }
});
