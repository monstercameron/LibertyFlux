// original: 0x009BF120 rot_axis_angle_between (proposed)

/// Rotation taking vector `a` to vector `b`, as a unit axis plus an angle.
///
/// `a` and `b` point to three-float vectors, `out_axis` receives the unit
/// rotation axis (three floats) and `out_angle` the angle in radians (one
/// float). Returns 1 with both outputs written, or 0 with nothing written
/// when the inputs are already aligned (dot product at least `DOT_ALIGNED`,
/// or a NaN dot product, which takes the same branch).
///
/// Algorithm: `dot = a.y*b.y + a.x*b.x + a.z*b.z` in that order. A copy of
/// `a` is stored to the output and the angle slot is seeded with pi/2; both
/// are overwritten on the paths below except that the near-opposite path
/// keeps the pi/2 angle.
/// - Aligned (`dot >= 0.99800104`): return 0.
/// - General (`dot > -0.99800104`): axis is `a x b` normalised by an
///   `asin`-like helper call on the length: angle is the helper's answer for
///   positive dot, pi minus it otherwise. A zero-length cross product (or a
///   zero input) falls back to the +X unit vector from the game's axis table
///   with angle 0.
/// - Near-opposite (`dot <= -0.99800104`): axis is `a` crossed with the
///   least-aligned unit axis (+X unless `|a.x|` exceeds 1/sqrt(3), else +Y
///   unless `|a.y|` exceeds it, else +Z), normalised; angle stays pi/2.
///   A zero-length cross product there yields a zero axis.
///
/// The helper takes the length pushed as a float word and returns a float in
/// ST0 (cdecl, one stack word). The original spills one cross-product
/// temporary over its own incoming arg0 stack slot; that clobber is outside
/// this rewrite's reach, so the contract runs with the stack check off.
///
/// Original: 0x009BF120 (cdecl, four stack words). Returns its flag in AL;
/// the upper bytes of EAX keep the caller's value on the 1 paths.
lf_checker_rt::export!(cdecl, rw_009BF120(a: u32, b: u32, out_axis: u32, out_angle: u32) -> u32 {
    unsafe {
        const DOT_ALIGNED: f32 = f32::from_bits(0x3f7f_7cff); // 0.99800104
        const DOT_OPPOSITE: f32 = f32::from_bits(0xbf7f_7cff); // -0.99800104
        const ONE: f32 = 1.0;
        const PI: f32 = f32::from_bits(0x4049_0fdb);
        const HALF_PI_BITS: u32 = 0x3fc9_0fdb;
        const AXIS_PICK: f32 = f32::from_bits(0x3f13_cd3a); // 1/sqrt(3)
        const ABS_MASK: u32 = 0x7fff_ffff;
        const ASIN_CALLEE: u32 = 0;
        // Game's unit-axis table (read-only data): +X, +Y, +Z triples.
        const AXIS_X: u32 = 0x0110_db00;
        const AXIS_Y: u32 = 0x0110_db50;
        const AXIS_Z: u32 = 0x0110_db70;

        #[inline(always)]
        unsafe fn rd32(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(p: u32) -> f32 {
            unsafe { f32::from_bits(rd32(p)) }
        }
        #[inline(always)]
        unsafe fn wr32(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(p: u32, v: f32) {
            unsafe { wr32(p, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        unsafe fn fabs_bits(p: u32) -> f32 {
            unsafe { f32::from_bits(rd32(p) & ABS_MASK) }
        }

        let (ax, ay, az) = (rdf(a), rdf(a + 4), rdf(a + 8));
        let (bx, by, bz) = (rdf(b), rdf(b + 4), rdf(b + 8));
        // Dot product in the original's order: y first, then x, then z.
        let dot = add(add(mul(ay, by), mul(ax, bx)), mul(az, bz));
        // comiss+jbe: proceed only on a strict less-than (NaN returns 0).
        if !(dot < DOT_ALIGNED) {
            return 0;
        }
        // Seed the outputs: a copy of `a` and a half-pi angle.
        wrf(out_axis, ax);
        wrf(out_axis + 4, ay);
        wrf(out_axis + 8, az);
        wr32(out_angle, HALF_PI_BITS);
        if dot > DOT_OPPOSITE {
            // General path: axis is a x b, normalised.
            let cx = sub(mul(ay, bz), mul(az, by));
            let cy = sub(mul(bx, az), mul(ax, bz));
            let cz = sub(mul(ax, by), mul(ay, bx));
            wrf(out_axis, cx);
            wrf(out_axis + 4, cy);
            wrf(out_axis + 8, cz);
            let len = add(add(mul(cx, cx), mul(cy, cy)), mul(cz, cz)).sqrt();
            // ucomiss+lahf idiom: exactly +0/-0 takes the fallback, NaN does not.
            if len == 0.0 {
                let x = lf_checker_rt::relocated(AXIS_X);
                wr32(out_axis, rd32(x));
                wr32(out_axis + 4, rd32(x + 4));
                wr32(out_axis + 8, rd32(x + 8));
                wr32(out_angle, 0);
                return 1;
            }
            let inv = core::hint::black_box(ONE) / core::hint::black_box(len);
            wrf(out_axis, mul(cx, inv));
            wrf(out_axis + 4, mul(cy, inv));
            wrf(out_axis + 8, mul(cz, inv));
            let base: f32 = lf_checker_rt::callee_cdecl!(ASIN_CALLEE, f32, len.to_bits());
            // comiss+jbe against +0: NaN dot takes the pi-minus branch.
            let ang = if dot > 0.0 { base } else { sub(PI, base) };
            wrf(out_angle, ang);
            return 1;
        }
        // Near-opposite path: cross `a` with the least-aligned unit axis.
        let tab = if AXIS_PICK > fabs_bits(a) {
            AXIS_X
        } else if AXIS_PICK > fabs_bits(a + 4) {
            AXIS_Y
        } else {
            AXIS_Z
        };
        let t = lf_checker_rt::relocated(tab);
        let (fx, fy, fz) = (rdf(t), rdf(t + 4), rdf(t + 8));
        let cx = sub(mul(ay, fz), mul(az, fy));
        let cy = sub(mul(az, fx), mul(ax, fz));
        let cz = sub(mul(ax, fy), mul(ay, fx));
        wrf(out_axis, cx);
        wrf(out_axis + 4, cy);
        wrf(out_axis + 8, cz);
        // Length-squared folds y first here, unlike the general path.
        let len2 = add(add(mul(cy, cy), mul(cx, cx)), mul(cz, cz));
        let mut inv = 0.0f32;
        if len2 != 0.0 {
            let len = len2.sqrt();
            inv = core::hint::black_box(ONE) / core::hint::black_box(len);
        }
        // Note the mixed operand order: x and y are inv*v, z is v*inv.
        wrf(out_axis, mul(inv, cx));
        wrf(out_axis + 4, mul(inv, cy));
        wrf(out_axis + 8, mul(cz, inv));
        1
    }
});
