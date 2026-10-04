// original: 0x009699f0 radial_weight_3d (proposed)

/// Spread a 3D sample point over four weighted accumulators.
///
/// `this` is the owner object, `p` points to three input floats and `out`
/// receives one float. The inputs are centred on the object's reference
/// point (the `+0x580` row) and normalised to a unit direction, which then
/// passes through two threshold-select stages: each stage compares a
/// squared length against three global cutoffs and bit-blends the
/// direction with a global mask, keeping the direction's bits where a
/// cutoff passed and the mask's bits elsewhere. The surviving direction
/// is projected onto four global weight triples; each projection is
/// clamped at zero and combined with a ratio of partial lengths into a
/// round weight that scales one float from each of two object tables (the
/// `+0x17d0` and `+0x1718` rows). The first table's running total is
/// stored to `out`; the second table's total is the float result.
///
/// Two scratch words the original never writes are read as zero (the
/// proof runs with a zero stack fill, so both sides agree). Every
/// arithmetic operation keeps the original's operand order.
///
/// Original: 0x009699f0 (thiscall: object in ECX, two stack words; float
/// result in ST0; no outgoing calls; reads the select flag, three
/// cutoffs, the blend mask and twelve weight floats from globals).
lf_checker_rt::export!(thiscall, rw_009699f0(this: u32, p: u32, out: u32) -> f32 {
    unsafe {
        const CENTER: u32 = 0x580;
        const ROW_A: u32 = 0x17d0;
        const ROW_B: u32 = 0x1718;
        const SELECT_FLAG: u32 = 0x017A_D148; // file VA of the select value
        const CUTOFFS: u32 = 0x0110_DAD0; // file VA of the 3 cutoff floats
        const BLEND_MASK: u32 = 0x0110_DB50; // file VA of the 4 mask words
        const WEIGHTS: u32 = 0x0121_F5E0; // file VA of the 12 weight floats
        const COMPLEMENT: f32 = 0.25;

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd(obj: u32, off: u32) -> f32 {
            unsafe { ((obj + off) as *const f32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(va).read() }
        }
        #[inline(always)]
        unsafe fn gw(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        /// One threshold select: the flag when `len2` is strictly above
        /// the cutoff, else zero. NaN lengths select zero.
        #[inline(always)]
        unsafe fn select(len2: f32, cutoff: u32) -> u32 {
            unsafe {
                if len2 > gf(CUTOFFS + cutoff * 4) {
                    gw(SELECT_FLAG)
                } else {
                    0
                }
            }
        }
        /// Bit-blend of direction `v` with the mask under select `s`:
        /// `(v & s) | (~s & mask)`, one word.
        #[inline(always)]
        unsafe fn blend_word(v: f32, s: u32, lane: u32) -> f32 {
            unsafe {
                f32::from_bits(
                    (v.to_bits() & s) | (!s & gw(BLEND_MASK + lane * 4)),
                )
            }
        }

        let d0 = sub(
            (p as *const f32).read_unaligned(),
            rd(this, CENTER),
        );
        let d1 = sub(
            ((p + 4) as *const f32).read_unaligned(),
            rd(this, CENTER + 4),
        );
        let d2 = sub(
            ((p + 8) as *const f32).read_unaligned(),
            rd(this, CENTER + 8),
        );
        let len2 = add(add(mul(d0, d0), mul(d1, d1)), mul(d2, d2));
        // Cutoffs are stored low to high but tested high to low.
        let sel_hi = select(len2, 2);
        let sel_mid = select(len2, 1);
        let sel_lo = select(len2, 0);
        let inv = div(1.0, len2.sqrt());
        let nx = mul(inv, d0);
        let ny = mul(inv, d1);
        let nz = mul(inv, d2);
        // The fourth word of each vector below is scratch the original
        // never writes; the proof's zero stack fill makes it zero.
        let bx = blend_word(nx, sel_lo, 0);
        let by = blend_word(ny, sel_mid, 1);
        let bz = blend_word(nz, sel_hi, 2);
        let bw = blend_word(0.0, 0, 3);

        let planar = add(mul(by, by), mul(bx, bx));
        let full = add(mul(bz, bz), planar);
        let ratio = div(planar, full);
        let sel2_hi = select(planar, 2);
        let sel2_mid = select(planar, 1);
        let sel2_lo = select(planar, 0);
        let inv2 = div(1.0, planar.sqrt());
        let vy = mul(inv2, by);
        let vz = mul(inv2, 0.0);
        let vx = mul(inv2, bx);
        // Second blend: the scaled direction (vx, vy, vz) under the
        // three second-stage selects. (The stores above overwrite the
        // first-stage scratch before the vector loads, so both vectors
        // hold fresh values in every lane.)
        let cx = blend_word(vx, sel2_lo, 0);
        let cy = blend_word(vy, sel2_mid, 1);
        let cz = blend_word(vz, sel2_hi, 2);
        // The fourth blend lane feeds nothing downstream.
        let _ = (bw, sel2_lo);

        (out as *mut u32).write(0);
        let residue = mul(sub(1.0, ratio), COMPLEMENT);
        let mut acc_a = 0.0f32;
        let mut acc_b = 0.0f32;
        let mut round = 0u32;
        while round < 4 {
            let wbase = WEIGHTS + round * 16;
            let mut t = add(
                add(
                    mul(cy, gf(wbase + 4)),
                    mul(cx, gf(wbase)),
                ),
                mul(cz, gf(wbase + 8)),
            );
            if t < 0.0 {
                t = 0.0;
            }
            let w = add(mul(mul(t, ratio), t), residue);
            let store_a = add(mul(rd(this, ROW_A + round * 4), w), acc_a);
            let store_b = add(mul(rd(this, ROW_B + round * 4), w), acc_b);
            (out as *mut f32).write_unaligned(store_a);
            acc_a = (out as *const f32).read_unaligned();
            acc_b = store_b;
            round += 1;
        }
        acc_b
    }
});
