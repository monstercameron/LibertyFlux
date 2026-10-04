// original: 0x00a71cb0 copy_pose_and_bias (proposed)
/// Copies four words from the object (`+0x30`, `+0x34`, `+0x38`, `+0x3c`)
/// into `out4`, stores `float_at(+0x44) - global_float` into `outf`, and
/// returns the byte at `+0x40`.
///
/// `thiscall`: object in ECX, two stack words, callee pops 8. The middle
/// two copied words move through vector registers in the original but are
/// plain copies, bit-exact either way. The subtraction is the original's
/// single `a - b` in operand order, pinned against commutation; the global
/// is a 32-bit float the harness fills per trial.
lf_checker_rt::export!(thiscall, rw_00a71cb0(this: u32, out4: u32, outf: u32) -> u8 {
    unsafe {
        const W0: u32 = 0x30;
        const F1: u32 = 0x34;
        const F2: u32 = 0x38;
        const W3: u32 = 0x3c;
        const TAG: u32 = 0x40;
        const BIAS_SRC: u32 = 0x44;
        const GLOBAL_BIAS: u32 = 0x0103ceb8;
        let r = |o: u32| ((this + o) as *const u32).read_unaligned();
        ((out4) as *mut u32).write_unaligned(r(W0));
        ((out4 + 4) as *mut u32).write_unaligned(r(F1));
        ((out4 + 8) as *mut u32).write_unaligned(r(F2));
        ((out4 + 12) as *mut u32).write_unaligned(r(W3));
        let a = f32::from_bits(r(BIAS_SRC));
        let b = f32::from_bits((lf_checker_rt::global::<u32>(GLOBAL_BIAS)).read_unaligned());
        let d = core::hint::black_box(a) - core::hint::black_box(b);
        ((outf) as *mut u32).write_unaligned(d.to_bits());
        ((this + TAG) as *const u8).read()
    }
});
