// original: 0x00D2B300 navmesh_point_blend (proposed)

/// Relax one navigation-route point toward a goal, or report that the stored
/// point should be kept as is.
///
/// `this` is the route follower object (flag byte at `+0x98`, stride-16
/// waypoint array base at `+0x70`, state flags at `+0xDC`). `route` is the
/// route object: a dword at `+0xA80` points at an auxiliary record whose
/// flag word at `+0x50` (bit 1 = finished) and threshold float at `+8` gate
/// the work, and a dword at `+0x20` points at a record holding a reference
/// point at `+0x30/+0x34/+0x38`. `cur` points at the current point
/// (three floats and a trailing word), `goal` at a target point (three
/// floats), and `out` receives four words: the blended point plus a trailing
/// word.
///
/// Behaviour: when the finished bit is set, or the auxiliary threshold does
/// not exceed 1.0, `out` is a plain copy of `cur` and the result is 0. Else
/// the distance from `cur` to the reference point must be below 0.75, and
/// the distance from `goal` to `cur` must reach 0.75, or `out` is again a
/// copy of `cur` with result 0. Otherwise, when the flag byte is positive,
/// the waypoint entry it selects must also lie at least 0.75 from `cur`.
/// Finally a score decides the blend factor: the dot of the scaled goal
/// offset with the reference point, minus the dot of `cur` with the scaled
/// offset. A non-positive score blends with `0.75 - first_distance`; a
/// positive score blends with 0.75 but only when state bit 3 is already set,
/// otherwise it also falls back to the copy. On the blend path the state
/// bit is set and the result is 1.
///
/// All distances go through a reciprocal-of-length helper that yields 0 for
/// a zero length (so the doubled reciprocal is +infinity) and NaN for NaN.
/// Every comparison treats an unordered (NaN) result as taking the
/// "greater-or-equal / copy" side, exactly as the original's comiss+jumps.
/// The trailing word on the blend path is whatever the original's
/// uninitialised frame slot holds; the contract pins the stack fill to zero,
/// so the rewrite writes zero there.
///
/// Original: 0x00D2B300 (thiscall, four stack words, returns a byte in AL;
/// the upper bytes of EAX are leftovers and only AL is compared).
lf_checker_rt::export!(thiscall, rw_00d2b300(this: u32, route: u32, cur: u32, goal: u32, out: u32) -> u32 {
    unsafe {
        const AUX_OFF: u32 = 0xA80;
        const AUX_FLAGS: u32 = 0x50;
        const AUX_LIMIT: u32 = 8;
        const REF_OFF: u32 = 0x20;
        const REF_X: u32 = 0x30;
        const REF_Y: u32 = 0x34;
        const REF_Z: u32 = 0x38;
        const FLAG_BYTE: u32 = 0x98;
        const ENTRY_BASE: u32 = 0x70;
        const STATE: u32 = 0xDC;
        const STATE_READY: u32 = 8;
        const FINISHED_BIT: u32 = 2;
        const RADIUS: f32 = 0.75;
        const ONE: f32 = 1.0;
        /// Value of the original's uninitialised frame slot under the
        /// contract's `stack_fill: 0`; the original reads a word it never
        /// wrote (its `[esp+0x2c]`) as the trailing word on the blend path.
        const UNINIT_SLOT: u32 = 0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            (a as *const u32).read_unaligned()
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            f32::from_bits(rd32(a))
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            (a as *mut u32).write_unaligned(v)
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            wr32(a, v.to_bits())
        }
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
        /// Squared length in the original's accumulation order:
        /// (dy*dy + dx*dx) + dz*dz.
        #[inline(always)]
        fn lensq(dx: f32, dy: f32, dz: f32) -> f32 {
            let xx = mul(dx, dx);
            let yy = mul(dy, dy);
            let zz = mul(dz, dz);
            add(add(yy, xx), zz)
        }
        /// Reciprocal length: 0 for a zero length, else 1/sqrt.
        /// (The original tests the flags of an unordered-compare against
        /// +0.0; zero of either sign takes the zero side, everything else
        /// including NaN takes the divide side.)
        #[inline(always)]
        fn rlen(sq: f32) -> f32 {
            if core::hint::black_box(sq) == 0.0 {
                0.0
            } else {
                div(ONE, core::hint::black_box(sq).sqrt())
            }
        }
        /// Copy the current point to the output, result 0.
        #[inline(always)]
        unsafe fn keep(cur: u32, out: u32) -> u32 {
            unsafe {
                wrf(out, rdf(cur));
                wrf(out + 4, rdf(cur + 4));
                wrf(out + 8, rdf(cur + 8));
                wr32(out + 12, rd32(cur + 12));
                0
            }
        }

        let aux = rd32(route + AUX_OFF);
        if rd32(aux + AUX_FLAGS) & FINISHED_BIT != 0 {
            return keep(cur, out);
        }
        // Proceed only when the limit strictly exceeds 1.0 (NaN keeps).
        if !(rdf(aux + AUX_LIMIT) > ONE) {
            return keep(cur, out);
        }
        let base = rd32(route + REF_OFF);
        let cx = rdf(cur);
        let cy = rdf(cur + 4);
        let cz = rdf(cur + 8);
        let d1 = lensq(
            sub(cx, rdf(base + REF_X)),
            sub(cy, rdf(base + REF_Y)),
            sub(cz, rdf(base + REF_Z)),
        );
        let len1 = div(ONE, rlen(d1));
        // Proceed only when the first distance is strictly below 0.75.
        if !(len1 < RADIUS) {
            return keep(cur, out);
        }
        let mut ox = sub(rdf(goal), cx);
        let mut oy = sub(rdf(goal + 4), cy);
        let mut oz = sub(rdf(goal + 8), cz);
        let d2 = lensq(ox, oy, oz);
        let k2 = rlen(d2);
        oy = mul(oy, k2);
        ox = mul(ox, k2);
        oz = mul(oz, k2);
        let len2 = div(ONE, k2);
        let blend0 = sub(RADIUS, len1);
        // The second distance must reach 0.75 (NaN passes).
        if len2 < RADIUS {
            return keep(cur, out);
        }
        let idx = (this as *const u8).add(FLAG_BYTE as usize).read() as i8 as i32;
        if idx > 0 {
            let e = rd32(this + ENTRY_BASE).wrapping_add((idx as u32) << 4);
            let d3 = lensq(
                sub(cx, rdf(e)),
                sub(cy, rdf(e + 4)),
                sub(cz, rdf(e + 8)),
            );
            // Direct square root here, not the doubled reciprocal.
            let len3 = core::hint::black_box(d3).sqrt();
            if len3 < RADIUS {
                return keep(cur, out);
            }
        }
        // Score: dot(scaled offset, reference) minus dot(cur, scaled offset).
        // The first dot accumulates ((x + y) + z), the second ((y + x) + z).
        let s = add(
            add(
                add(mul(ox, rdf(base + REF_X)), mul(oy, rdf(base + REF_Y))),
                mul(oz, rdf(base + REF_Z)),
            ),
            f32::from_bits(
                add(
                    add(mul(cy, oy), mul(cx, ox)),
                    mul(cz, oz),
                )
                .to_bits()
                    ^ 0x8000_0000,
            ),
        );
        let t = if !(s > 0.0) {
            blend0
        } else {
            if rd32(this + STATE) & STATE_READY == 0 {
                return keep(cur, out);
            }
            RADIUS
        };
        wrf(out, add(mul(ox, t), cx));
        wrf(out + 4, add(mul(oy, t), cy));
        wrf(out + 8, add(mul(oz, t), cz));
        wr32(out + 12, UNINIT_SLOT);
        wr32(this + STATE, rd32(this + STATE) | STATE_READY);
        1
    }
});
