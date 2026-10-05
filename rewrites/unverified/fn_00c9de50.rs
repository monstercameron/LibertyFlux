// original: 0x00C9DE50 ik_joint_step_and_clamp (proposed)

/// Step two object channels and two pointed-to floats toward their targets,
/// then clamp the pointed-to floats into the ranges the object now holds.
///
/// `this` points to an object whose floats at `+0xb4` (X) and `+0xb8` (Y) are
/// read and written. `p5` points to a six-float table (offsets `0x00`..`0x14`,
/// read only). `p3` and `p4` point to single floats that are read and written.
/// `a1` and `a2` are float arguments passed by value. A global float rate `G`
/// scales three step sizes. All comparisons are `comiss` semantics: an
/// unordered (NaN) comparison takes the `jbe` edge and not the `ja` edge.
///
/// Each of the four stepping blocks has the same shape: if the target is
/// within the step of the current value (absolute difference not above the
/// step), snap to the target; else if the target is above, add the step; else
/// compare against a witness (for X and Y the `*p4` float, for the others the
/// values themselves) and either leave the value alone or subtract the step.
/// The steps are `G * pi` for X, `p5[0x14] * G` for Y, `p5[8] * G` for `*p3`
/// and `p5[0x14] * G` for `*p4`; the targets are `p5[0x10]` for X, `p5[0x0c]`
/// for Y, `a1` for `*p3` and `a2` for `*p4`.
///
/// Afterwards `*p3` is clamped into `[p5[4], p5[0]]` (`p5[0]` is the upper
/// bound: a value strictly above it is replaced by it, else a value strictly
/// below `p5[4]` is replaced by that) and `*p4` into `[this.X, this.Y]` the
/// same way, using the freshly stepped X and Y. A flag word records which
/// clamps fired (bit 0: `*p3`
/// snapped to `a1`; bit 2: `*p3` clamped; bit 3: `*p4` snapped to `a2`,
/// cleared again when the `*p4` clamp fires; bit 4: `*p4` clamped) and the
/// return value folds it: 0 when bit 2 or 4 is set, 2 when exactly bits 0
/// and 3 are set, 1 otherwise.
///
/// Original: 0x00C9DE50 (thiscall, five stack words: two floats, three pointers).
lf_checker_rt::export!(thiscall, rw_00C9DE50(this: u32, a1: u32, a2: u32, p3: u32, p4: u32, p5: u32) -> u32 {
    unsafe {
        const THIS_X: u32 = 0xB4;
        const THIS_Y: u32 = 0xB8;
        const GLOBAL_RATE: u32 = 0x11735BC;
        const FABS_MASK: u32 = 0x7FFF_FFFF;
        const PI_BITS: u32 = 0x40490FDB;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        /// Bitwise absolute value, the original's `andps` with the mask.
        #[inline(always)]
        fn fabsb(x: f32) -> f32 {
            f32::from_bits(x.to_bits() & FABS_MASK)
        }
        /// `comiss a, b` followed by `jbe`: taken unless a > b, so NaN takes it.
        #[inline(always)]
        fn jbe(a: f32, b: f32) -> bool {
            !(a > b)
        }
        /// `comiss a, b` followed by `ja`: taken only when a > b, never for NaN.
        #[inline(always)]
        fn ja(a: f32, b: f32) -> bool {
            a > b
        }

        // Steps derived from the global rate.
        let g = f32::from_bits(rd32(lf_checker_rt::relocated(GLOBAL_RATE)));
        let x = rdf(this + THIS_X);
        let x_target = rdf(p5 + 0x10);
        let mut p3_step = rdf(p5 + 8);
        let mut y_step = rdf(p5 + 0x14);
        let mut gap = fabsb(sub(x, x_target));
        p3_step = mul(p3_step, g);
        y_step = mul(y_step, g);
        let x_step = mul(g, f32::from_bits(PI_BITS));
        let mut eax: u32 = 0;

        // Step X toward p5[0x10].
        if jbe(x_step, gap) {
            wrf(this + THIS_X, x_target);
        } else if jbe(x_target, x) {
            wrf(this + THIS_X, add(x, x_step));
        } else {
            gap = rdf(p4);
            if !jbe(gap, x_target) {
                wrf(this + THIS_X, sub(x, x_step));
            }
        }

        // Step Y toward p5[0x0c].
        let y = rdf(this + THIS_Y);
        let y_target = rdf(p5 + 0x0C);
        gap = fabsb(sub(y, y_target));
        if jbe(y_step, gap) {
            wrf(this + THIS_Y, y_target);
        } else if jbe(y_target, y) {
            wrf(this + THIS_Y, add(y, y_step));
        } else {
            gap = rdf(p4);
            if !jbe(gap, y_target) {
                wrf(this + THIS_Y, sub(y, y_step));
            }
        }

        // Step *p3 toward a1.
        let goal3 = f32::from_bits(a1);
        let cur3 = rdf(p3);
        let goal4 = f32::from_bits(a2);
        let gap4 = fabsb(sub(rdf(p4), goal4));
        gap = fabsb(sub(cur3, goal3));
        if jbe(p3_step, gap) {
            wrf(p3, goal3);
            eax = 1;
        } else if jbe(goal3, cur3) {
            wrf(p3, add(cur3, p3_step));
        } else if !jbe(cur3, goal3) {
            wrf(p3, sub(cur3, p3_step));
        }

        // Clamp *p3 into [p5[0], p5[4]].
        let lo3 = rdf(p5);
        let v3 = rdf(p3);
        if ja(v3, lo3) {
            eax = (eax & 0xFFFF_FFFE) | 4;
            wrf(p3, lo3);
        } else {
            let hi3 = rdf(p5 + 4);
            if !jbe(hi3, v3) {
                eax = (eax & 0xFFFF_FFFE) | 4;
                wrf(p3, hi3);
            }
        }

        // Step *p4 toward a2.
        if jbe(y_step, gap4) {
            wrf(p4, goal4);
            eax |= 8;
        } else {
            let cur4 = rdf(p4);
            if jbe(goal4, cur4) {
                wrf(p4, add(cur4, y_step));
            } else if !jbe(cur4, goal4) {
                wrf(p4, sub(cur4, y_step));
            }
        }

        // Clamp *p4 into [this.X, this.Y] with the stepped bounds.
        let bound_y = rdf(this + THIS_Y);
        let v4 = rdf(p4);
        if ja(v4, bound_y) {
            eax = (eax & 0xFFFF_FFF7) | 0x10;
            wrf(p4, bound_y);
        } else {
            let bound_x = rdf(this + THIS_X);
            if !jbe(bound_x, v4) {
                eax = (eax & 0xFFFF_FFF7) | 0x10;
                wrf(p4, bound_x);
            }
        }

        // Fold the flag word into the 0/1/2 result.
        let al = (eax & 0xFF) as u8;
        if al & 0x14 != 0 {
            0
        } else if al & 8 == 0 {
            1
        } else if al & 1 == 0 {
            1
        } else {
            2
        }
    }
});
