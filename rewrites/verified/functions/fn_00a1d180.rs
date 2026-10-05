// original: 0x00A1D180 task_compute_bounding_radius (proposed)

/// Recompute the task's bounding radius from its four point arrays.
///
/// `this` is the task object. The function first zeroes the spare dword at
/// `+0x494`, runs the centroid callee (which refreshes the centre point at
/// `+0x420`, `+0x424`, `+0x428`), zeroes the radius at `+0x490`, and then
/// takes the maximum over every live point of the four arrays below, storing
/// the running best back to `+0x490` after each candidate. A candidate whose
/// distance is unordered (NaN) against the best replaces it, because the
/// original's unsigned-above branch is not taken then.
///
/// The four arrays (count, first record, stride, flag row) are:
/// - A: count `+0x400`, records at `+0x108`, stride `0xB0`, flags `+0x410`.
///   Each record holds two points: `(base-0x18, base-0x14, base-0x10)` and
///   `(base-8, base-4, base+0)`; each contributes its plain distance.
/// - B: count `+0x404`, records at `+0x1d8`, stride `0xC0`, flags `+0x411`.
///   One point `(base-8, base-4, base+0)` per record; the candidate is its
///   distance plus the extra at `base+0x18`.
/// - C: count `+0x408`, records at `+0x2a8`, stride `0xE0`, flags `+0x412`.
///   Two points like array A with a per-record extra at `base+0x28` added
///   to each distance.
/// - D: count `+0x40c`, records at `+0x3a8`, stride `0x120`, flags `+0x413`.
///   One candidate per record: the length of `(base+0x48, base+0x4c,
///   base+0x50)` plus the distance of `(base-8, base-4, base+0)`.
///
/// A record counts only when its flag byte is non-zero. Counts are signed:
/// a non-positive count skips its loop. Every distance is
/// `sqrt((dy*dy + dx*dx) + dz*dz)` with the subtractions `(point - centre)`
/// in x, y, z order; the float operation order is the original's.
///
/// Original: 0x00A1D180 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00A1D180(this: u32) -> u32 {
    unsafe {
        const CENTRE: u32 = 0x420;
        const RADIUS: u32 = 0x490;
        const SPARE: u32 = 0x494;
        const CENTROID: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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

        /// Distance of the point at (px, py, pz) from the centre.
        #[inline(always)]
        unsafe fn dist(px: u32, py: u32, pz: u32, cx: f32, cy: f32, cz: f32) -> f32 {
            unsafe {
                let dx = sub(rdf(px), cx);
                let dy = sub(rdf(py), cy);
                let dz = sub(rdf(pz), cz);
                add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz)).sqrt()
            }
        }
        /// Fold one candidate into the running best, like the original's
        /// compare-and-conditional-move (NaN candidates replace the best).
        #[inline(always)]
        fn consider(best: f32, d: f32) -> f32 {
            if !(best > d) { d } else { best }
        }

        wr32(this + SPARE, 0);
        lf_checker_rt::callee_thiscall!(CENTROID, u32, this);
        wr32(this + RADIUS, 0);
        let cx = rdf(this + CENTRE);
        let cy = rdf(this + CENTRE + 4);
        let cz = rdf(this + CENTRE + 8);
        let mut best = 0.0f32;

        // Array A: two plain distances per live record.
        let n = rd32(this + 0x400) as i32;
        if n > 0 {
            let mut i = 0i32;
            while i < n {
                let base = this + 0x108 + (i as u32) * 0xb0;
                if rd8(this + 0x410 + (i as u32)) != 0 {
                    best = consider(best, dist(base - 0x18, base - 0x14, base - 0x10, cx, cy, cz));
                    wrf(this + RADIUS, best);
                    best = consider(best, dist(base - 8, base - 4, base, cx, cy, cz));
                    wrf(this + RADIUS, best);
                }
                i += 1;
            }
        }
        // Array B: one distance plus the record extra per live record.
        let n = rd32(this + 0x404) as i32;
        if n > 0 {
            let mut i = 0i32;
            while i < n {
                let base = this + 0x1d8 + (i as u32) * 0xc0;
                if rd8(this + 0x411 + (i as u32)) != 0 {
                    let d = add(dist(base - 8, base - 4, base, cx, cy, cz), rdf(base + 0x18));
                    best = consider(best, d);
                    wrf(this + RADIUS, best);
                }
                i += 1;
            }
        }
        // Array C: two distances with a shared per-record extra.
        let n = rd32(this + 0x408) as i32;
        if n > 0 {
            let mut i = 0i32;
            while i < n {
                let base = this + 0x2a8 + (i as u32) * 0xe0;
                if rd8(this + 0x412 + (i as u32)) != 0 {
                    let extra = rdf(base + 0x28);
                    let d = add(dist(base - 0x18, base - 0x14, base - 0x10, cx, cy, cz), extra);
                    best = consider(best, d);
                    wrf(this + RADIUS, best);
                    let d = add(dist(base - 8, base - 4, base, cx, cy, cz), extra);
                    best = consider(best, d);
                    wrf(this + RADIUS, best);
                }
                i += 1;
            }
        }
        // Array D: record-vector length plus point distance.
        let n = rd32(this + 0x40c) as i32;
        if n > 0 {
            let mut i = 0i32;
            while i < n {
                let base = this + 0x3a8 + (i as u32) * 0x120;
                if rd8(this + 0x413 + (i as u32)) != 0 {
                    let vx = rdf(base + 0x48);
                    let vy = rdf(base + 0x4c);
                    let vz = rdf(base + 0x50);
                    let vlen = add(add(mul(vx, vx), mul(vy, vy)), mul(vz, vz)).sqrt();
                    let d = add(vlen, dist(base - 8, base - 4, base, cx, cy, cz));
                    best = consider(best, d);
                    wrf(this + RADIUS, best);
                }
                i += 1;
            }
        }
        0
    }
});
