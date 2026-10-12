// original: 0x00D01A10 task_mirror_triplet_around_scalar

/// Mirror a coordinate triplet around a margin into two bound triplets.
///
/// Copies the triplet at `+0x13e0` to `+0x60` and subtracts the scalar at
/// `+0x1440` from each lane, then copies the same triplet to `+0x70` and adds
/// the scalar to each lane. The last lane adds in scalar-first order, as the
/// original's instructions do; all float operations keep the original's
/// operand order bit-exactly.
///
/// Original: 0x00D01A10 (thiscall, no stack words). Name proposed: no merged
/// symbol.
lf_checker_rt::export!(thiscall, rw_00d01a10(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        const SRC: u32 = 0x13E0;
        const LO: u32 = 0x60;
        const HI: u32 = 0x70;
        const MARGIN: u32 = 0x1440;

        let x = f32::from_bits(((this + SRC) as *const u32).read_unaligned());
        let y = f32::from_bits(((this + SRC + 4) as *const u32).read_unaligned());
        let z = f32::from_bits(((this + SRC + 8) as *const u32).read_unaligned());
        let m = f32::from_bits(((this + MARGIN) as *const u32).read_unaligned());
        ((this + LO) as *mut u32).write_unaligned(sub(x, m).to_bits());
        ((this + LO + 4) as *mut u32).write_unaligned(sub(y, m).to_bits());
        ((this + LO + 8) as *mut u32).write_unaligned(sub(z, m).to_bits());
        ((this + HI) as *mut u32).write_unaligned(add(x, m).to_bits());
        ((this + HI + 4) as *mut u32).write_unaligned(add(y, m).to_bits());
        ((this + HI + 8) as *mut u32).write_unaligned(add(m, z).to_bits());
        0
    }
});
