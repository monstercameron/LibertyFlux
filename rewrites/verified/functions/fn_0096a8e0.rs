// original: 0x0096A8E0 audio_source_dir_quantize (proposed)

/// Quantize the direction from one 3D point to another into two small
/// integer indices plus fractional-blend floats.
///
/// `this` points to the source object (orientation triple at `+0x00`, three
/// more direction rows at `+0x60`/`+0x64`/`+0x68`). `pos_a` and `pos_b`
/// point at two three-float positions; the difference `d = b - a` drives
/// everything. The nine stack arguments are: two input vector pointers,
/// then seven out-pointers (`gain_a`, `idx_a`, `sub_a`, `gain_b`, `idx_b`,
/// `sub_b`, `angle_out`).
///
/// Two passes share one shape. Pass 1 takes `r1 = sqrt(dx*dx + dy*dy)`
/// (callee 1, cdecl, x87 float result), projects the scaled difference
/// onto the object's orientation rows, clamps the projection into the
/// `[CLAMP_LO, CLAMP_HI]` table range (an unordered comparison yields the
/// high end), maps it through the shaping callee (callee 2, argument in
/// the vector register, float answer) and scales by `ANGLE_SCALE`. When
/// the second projection sum is strictly below zero the result is mirrored
/// about `MIRROR`. Pass 2 does the same over the full length
/// `r2 = sqrt(dz*dz + (dx*dx + dy*dy))` with the three data scaling
/// factors, and its scaled angle is stored to `angle_out`.
///
/// Each pass then quantizes: the scaled value times `QUANT` is truncated
/// toward zero (x86 truncate semantics: NaN or out-of-range yields the
/// indefinite value, whose low byte is zero), the low byte is reduced
/// modulo 8 (pass 1) or 4 (pass 2) and stored to the index byte; the
/// fractional remainder picks one of two blend forms around the `STEP`
/// constant, stored to the gain float and the sub-index byte. The pass-1
/// gain is finally clamped into `[0, CLAMP_HI]`, preserving NaN. The
/// function returns the pass-2 sub-index counter value (index byte plus
/// or minus one, low byte).
///
/// The original's `and 0x80000007` remainder idioms only ever see
/// non-negative values (every input is zero-extended from a byte), so the
/// negative fix-up path is dead and plain masking is exactly equivalent.
/// Float operation order is the original's throughout.
///
/// Original: 0x0096A8E0 (thiscall, ECX = object, nine stack words).
lf_checker_rt::export!(thiscall, rw_0096A8E0(this: u32, pos_a: u32, pos_b: u32, gain_a: u32, idx_a: u32, sub_a: u32, gain_b: u32, idx_b: u32, sub_b: u32, angle_out: u32) -> u32 {
    unsafe {
        const ORIENT_X: u32 = 0x00;
        const ORIENT_Y: u32 = 0x04;
        const ORIENT_Z: u32 = 0x08;
        const ROW_B: u32 = 0x60;
        const ROW_C: u32 = 0x64;
        const ROW_D: u32 = 0x68;
        const G_Z_MIX: u32 = 0x00fe8628;
        const G_CLAMP_LO: u32 = 0x00fe8d94;
        const G_CLAMP_HI: u32 = 0x00fe88e8;
        const G_ANGLE_SCALE: u32 = 0x00e7c2a8;
        const G_MIRROR: u32 = 0x00e8b7fc;
        const G_DIR_X: u32 = 0x01038870;
        const G_DIR_Y: u32 = 0x01038874;
        const G_DIR_Z: u32 = 0x01038878;
        const G_QUANT: u32 = 0x00fe873c;
        const G_STEP: u32 = 0x00fe8830;
        const G_BLEND: u32 = 0x00fe8960;
        const SQRT_CALLEE: u32 = 1;
        const SHAPE_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { ((a as *mut u32)).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// x86 `cvttss2si` truncating conversion, including the indefinite
        /// `i32::MIN` result for NaN and out-of-range inputs (Rust `as`
        /// casts saturate instead, so they cannot be used here).
        #[inline(always)]
        fn trunc_i32(x: f32) -> i32 {
            if x.is_nan() {
                return i32::MIN;
            }
            let t = x.trunc();
            if t >= 2147483648.0 || t < -2147483648.0 {
                i32::MIN
            } else {
                t as i32
            }
        }
        /// The original's `comiss hi, x; jbe` clamp: the high end wins on
        /// unordered, otherwise the value is clamped into `[lo, hi]`.
        #[inline(always)]
        fn clamp_table(x: f32, lo: f32, hi: f32) -> f32 {
            if hi > x {
                if x > lo { x } else { lo }
            } else {
                hi
            }
        }
        #[inline(always)]
        unsafe fn g(addr: u32) -> f32 {
            unsafe { *lf_checker_rt::global::<f32>(addr) }
        }

        // Difference vector d = b - a.
        let dx = sub(rdf(pos_b), rdf(pos_a));
        let dy = sub(rdf(pos_b + 4), rdf(pos_a + 4));
        let dz = sub(rdf(pos_b + 8), rdf(pos_a + 8));
        // Pass 1: planar length and orientation projection.
        let planar = add(mul(dx, dx), mul(dy, dy));
        let r1: f32 = lf_checker_rt::callee_cdecl!(SQRT_CALLEE, f32, planar.to_bits());
        let r1_dy = mul(r1, dy);
        let dx_r1 = mul(dx, r1);
        let r1_mix = mul(r1, g(G_Z_MIX));
        let mut proj = mul(rdf(this + ORIENT_Y), r1_dy);
        proj = add(proj, mul(rdf(this + ORIENT_X), dx_r1));
        proj = add(proj, mul(rdf(this + ORIENT_Z), r1_mix));
        let shaped: f32 = {
            let c = clamp_table(proj, g(G_CLAMP_LO), g(G_CLAMP_HI));
            let bits: u32 = lf_checker_rt::callee_cdecl!(SHAPE_CALLEE, u32, c.to_bits());
            f32::from_bits(bits)
        };
        // Second sum deciding the mirror branch.
        let mut sum = mul(rdf(this + ROW_C), r1_dy);
        sum = add(sum, mul(dx_r1, rdf(this + ROW_B)));
        sum = add(sum, mul(rdf(this + ROW_D), r1_mix));
        let scaled = mul(shaped, g(G_ANGLE_SCALE));
        let w1 = if 0.0 > sum { sub(g(G_MIRROR), scaled) } else { scaled };
        // Pass 2: full length with the data scaling factors.
        let full = add(mul(dz, dz), planar);
        let r2: f32 = lf_checker_rt::callee_cdecl!(SQRT_CALLEE, f32, full.to_bits());
        let mut t2 = mul(mul(dx, r2), g(G_DIR_X));
        t2 = add(t2, mul(mul(r2, dy), g(G_DIR_Y)));
        t2 = add(t2, mul(mul(r2, dz), g(G_DIR_Z)));
        let shaped2: f32 = {
            let c = clamp_table(t2, g(G_CLAMP_LO), g(G_CLAMP_HI));
            let bits: u32 = lf_checker_rt::callee_cdecl!(SHAPE_CALLEE, u32, c.to_bits());
            f32::from_bits(bits)
        };
        let angle = mul(shaped2, g(G_ANGLE_SCALE));
        wrf(angle_out, angle);
        // Pass-1 quantization: modulo 8.
        let quant = g(G_QUANT);
        let step = g(G_STEP);
        let blend = g(G_BLEND);
        let q1 = mul(w1, quant);
        let b0 = (trunc_i32(q1) as u8) & 7;
        (idx_a as *mut u8).write(b0);
        (sub_a as *mut u8).write(0);
        let back0 = (idx_a as *const u8).read();
        let frac1 = sub(q1, back0 as f32);
        if step > frac1 {
            (sub_a as *mut u8).write((back0.wrapping_add(7)) & 7);
            wrf(gain_a, add(frac1, step));
        } else {
            (sub_a as *mut u8).write((back0.wrapping_add(1)) & 7);
            wrf(gain_a, sub(blend, frac1));
        }
        // Gain clamp into [0, hi], preserving NaN.
        let ga = rdf(gain_a);
        let hi = g(G_CLAMP_HI);
        let clamped = if 0.0 > ga { 0.0 } else if ga > hi { hi } else { ga };
        wrf(gain_a, clamped);
        // Pass-2 quantization: modulo 4.
        let q2 = mul(rdf(angle_out), quant);
        let b1 = (trunc_i32(q2) as u8) & 3;
        (idx_b as *mut u8).write(b1);
        (sub_b as *mut u8).write(0);
        let back1 = (idx_b as *const u8).read();
        let frac2 = sub(q2, back1 as f32);
        wrf(gain_b, 1.0);
        if step > frac2 {
            let down = (back1 as u32).wrapping_sub(1);
            let eax = (down & 0xff) as u8;
            let cl = if (down as i32) > 0 { eax } else { 0 };
            (sub_b as *mut u8).write(cl);
            wrf(gain_b, add(frac2, step));
            eax as u32
        } else {
            let up = (back1 as u32).wrapping_add(1);
            let eax = (up & 0xff) as u8;
            let cl = if (up as i32) < 3 { eax } else { 3 };
            (sub_b as *mut u8).write(cl);
            wrf(gain_b, sub(blend, frac2));
            eax as u32
        }
    }
});
