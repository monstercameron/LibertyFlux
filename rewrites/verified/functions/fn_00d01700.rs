// original: 0x00D01700 cover_slot_bound_radius (proposed)

/// Recompute the bounding radius of a cover-slot group from its centroid.
///
/// `this` points to a large (~5 KiB) group object holding four element
/// arrays. The sole callee first refreshes the centroid stored at `+0x13D0`
/// (three floats); this function then scans every active element, measures
/// its distance from that centroid, and keeps the largest distance seen in
/// the radius cell at `+0x1440`, which starts cleared (a flag word at
/// `+0x1444` is cleared first, the radius right after the callee returns).
///
/// Layouts read (counts are signed 32-bit, arrays are embedded in the
/// object, one active-flag byte per element):
///
/// * group 1: count at `+0x13A0`, elements of 0xB0 bytes from `+0x108`,
///   flags from `+0x13B0`; two points per element at element offsets
///   `-0x18` and `-0x08` (i.e. array base `+0xF0`, points at `+0x00` and
///   `+0x10`), each contributing its plain distance.
/// * group 2: count at `+0x13A4`, elements of 0xC0 bytes from `+0x8B8`,
///   flags from `+0x13BB`; one point per element at offset `-0x08`
///   (base `+0x8B0`) whose distance grows by the element radius at `+0x18`.
/// * group 3: count at `+0x13A8`, elements of 0xE0 bytes from `+0x988`,
///   flags from `+0x13BC`; two points per element (base `+0x970`, at
///   `+0x00` and `+0x10`), each distance grown by the word at `+0x28`.
/// * group 4: count at `+0x13AC`, elements of 0x120 bytes from `+0x1348`,
///   flags from `+0x13C7`; one entry per element combining the distance of
///   the point at `-0x08` (base `+0x1340`) with the length of the offset
///   vector at `+0x48`.
///
/// A distance is `sqrt((dy*dy + dx*dx) + dz*dz)` with `dx = px - cx` and so
/// on; group 4 adds `sqrt((vx*vx + vy*vy) + vz*vz)` to it afterwards. The
/// radius update keeps the old value only when it is strictly greater than
/// the candidate (`comiss` + `ja`), so an unordered (NaN) candidate always
/// replaces it. Groups with a non-positive count are skipped. The float
/// operation order below is the original's.
///
/// Original: 0x00D01700 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00d01700(this: u32) -> u32 {
    unsafe {
        const COUNT1: u32 = 0x13a0;
        const COUNT2: u32 = 0x13a4;
        const COUNT3: u32 = 0x13a8;
        const COUNT4: u32 = 0x13ac;
        const FLAG1: u32 = 0x13b0;
        const FLAG2: u32 = 0x13bb;
        const FLAG3: u32 = 0x13bc;
        const FLAG4: u32 = 0x13c7;
        const CX: u32 = 0x13d0;
        const CY: u32 = 0x13d4;
        const CZ: u32 = 0x13d8;
        const RADIUS: u32 = 0x1440;
        const DONE: u32 = 0x1444;
        const BASE1: u32 = 0x108;
        const STRIDE1: u32 = 0xb0;
        const BASE2: u32 = 0x8b8;
        const STRIDE2: u32 = 0xc0;
        const BASE3: u32 = 0x988;
        const STRIDE3: u32 = 0xe0;
        const BASE4: u32 = 0x1348;
        const STRIDE4: u32 = 0x120;
        const CENTROID_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        /// Squared length in the original's accumulation order.
        #[inline(always)]
        fn len2(x: f32, y: f32, z: f32) -> f32 {
            add(add(mul(y, y), mul(x, x)), mul(z, z))
        }
        /// Radius update: keep the old radius only when strictly greater.
        #[inline(always)]
        unsafe fn relax(this: u32, d: f32) {
            unsafe {
                let m = rdf(this.wrapping_add(RADIUS));
                if !(m > d) {
                    wrf(this.wrapping_add(RADIUS), d);
                }
            }
        }

        wr32(this.wrapping_add(DONE), 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(CENTROID_CALLEE, u32, this);
        // The original zeroes the radius cell here, before the first loop.
        wr32(this.wrapping_add(RADIUS), 0);
        let (cx, cy, cz) = (
            rdf(this.wrapping_add(CX)),
            rdf(this.wrapping_add(CY)),
            rdf(this.wrapping_add(CZ)),
        );

        // Group 1: two plain points per element.
        let n1 = rd32(this.wrapping_add(COUNT1)) as i32;
        if n1 > 0 {
            let mut p = this.wrapping_add(BASE1);
            let mut i = 0i32;
            while i < n1 {
                if rd8(this.wrapping_add(i as u32).wrapping_add(FLAG1)) != 0 {
                    for off in [0x18u32, 8] {
                        let q = p.wrapping_sub(off);
                        let dx = sub(rdf(q), cx);
                        let dy = sub(rdf(q.wrapping_add(4)), cy);
                        let dz = sub(rdf(q.wrapping_add(8)), cz);
                        relax(this, len2(dx, dy, dz).sqrt());
                    }
                }
                i += 1;
                p = p.wrapping_add(STRIDE1);
            }
        }

        // Group 2: one point per element plus the element radius.
        let n2 = rd32(this.wrapping_add(COUNT2)) as i32;
        if n2 > 0 {
            let mut p = this.wrapping_add(BASE2);
            let mut i = 0i32;
            while i < n2 {
                if rd8(this.wrapping_add(i as u32).wrapping_add(FLAG2)) != 0 {
                    let q = p.wrapping_sub(8);
                    let dx = sub(rdf(q), cx);
                    let dy = sub(rdf(q.wrapping_add(4)), cy);
                    let dz = sub(rdf(q.wrapping_add(8)), cz);
                    let d = add(
                        len2(dx, dy, dz).sqrt(),
                        rdf(p.wrapping_add(0x18)),
                    );
                    relax(this, d);
                }
                i += 1;
                p = p.wrapping_add(STRIDE2);
            }
        }

        // Group 3: two points per element plus the shared extra word.
        let n3 = rd32(this.wrapping_add(COUNT3)) as i32;
        if n3 > 0 {
            let mut p = this.wrapping_add(BASE3);
            let mut i = 0i32;
            while i < n3 {
                if rd8(this.wrapping_add(i as u32).wrapping_add(FLAG3)) != 0 {
                    let extra = rdf(p.wrapping_add(0x28));
                    for off in [0x18u32, 8] {
                        let q = p.wrapping_sub(off);
                        let dx = sub(rdf(q), cx);
                        let dy = sub(rdf(q.wrapping_add(4)), cy);
                        let dz = sub(rdf(q.wrapping_add(8)), cz);
                        relax(this, add(len2(dx, dy, dz).sqrt(), extra));
                    }
                }
                i += 1;
                p = p.wrapping_add(STRIDE3);
            }
        }

        // Group 4: point distance plus offset-vector length.
        let n4 = rd32(this.wrapping_add(COUNT4)) as i32;
        if n4 > 0 {
            let mut p = this.wrapping_add(BASE4);
            let mut i = 0i32;
            while i < n4 {
                if rd8(this.wrapping_add(i as u32).wrapping_add(FLAG4)) != 0 {
                    let q = p.wrapping_sub(8);
                    let dx = sub(rdf(q), cx);
                    let dy = sub(rdf(q.wrapping_add(4)), cy);
                    let dz = sub(rdf(q.wrapping_add(8)), cz);
                    let lv = len2(
                        rdf(p.wrapping_add(0x48)),
                        rdf(p.wrapping_add(0x4c)),
                        rdf(p.wrapping_add(0x50)),
                    );
                    let lp = len2(dx, dy, dz);
                    relax(this, add(lv.sqrt(), lp.sqrt()));
                }
                i += 1;
                p = p.wrapping_add(STRIDE4);
            }
        }
        0
    }
});
