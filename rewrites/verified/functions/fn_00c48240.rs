// original: 0x00c48240 ccamscripted_set_floats (proposed)
/// Store three parameters and clear the derived triple when `a` is not positive.
///
/// `a`, `b`, `c` are stored at `this + 0x258/0x25c/0x260`. Then, unless
/// `a` compares above zero (the original's `comiss 0, a; jb` also skips
/// the clear for a NaN `a`, which a plain `<=` reproduces), the words at
/// `this + 0x230/0x234/0x238` are zeroed. No value is returned.
///
/// Original: thiscall, three stack words, callee cleanup (the callee pops 12 bytes).
lf_checker_rt::export!(thiscall, rw_00c48240(this: u32, a: u32, b: u32, c: u32) -> u32 {
    const SLOT_A: u32 = 0x258;
    const SLOT_B: u32 = 0x25c;
    const SLOT_C: u32 = 0x260;
    const DERIVED: [u32; 3] = [0x230, 0x234, 0x238];
    unsafe {
        ((this + SLOT_A) as *mut u32).write_unaligned(a);
        ((this + SLOT_B) as *mut u32).write_unaligned(b);
        ((this + SLOT_C) as *mut u32).write_unaligned(c);
        // `comiss 0.0, a; jb` jumps (skips the clear) exactly when
        // `a <= 0.0` is false, including NaN. No arithmetic: bit-exact.
        if f32::from_bits(a) <= 0.0 {
            for off in DERIVED {
                ((this + off) as *mut u32).write_unaligned(0);
            }
        }
    }
    0
});
