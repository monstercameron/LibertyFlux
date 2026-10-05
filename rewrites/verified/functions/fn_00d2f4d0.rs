// original: 0x00d2f4d0 task_route_segment_probe (proposed)

/// Scan route segments for the first one facing a probe plane, returning its index.
///
/// `this` points to the task object: dword at `+0x70` is the route table (i32
/// count at `[0]`, one point of three floats per 16 bytes from `+0x10`) and
/// the dword at `+0xd8` holds state flags. `arg` points to an object whose
/// dword at `+0x20` is a probe object holding three floats at
/// `+0x30/+0x34/+0x38`.
///
/// The scan runs only when flag `0x20000000` is clear, the table is non-null
/// and the count is at least 2; otherwise the result is 0. For each segment
/// from point `i - 1` to point `i` (`i` from 1 while `i < count`) the segment
/// direction is normalized (a zero-length segment normalizes to zero, exactly
/// as the original's zero-compare-then-divide does) and three gates are
/// tested in order: the projection of (probe minus previous point) onto the
/// direction must be ordered and at least `-0.5` and at most the segment
/// length; then two derived frames (each a normalized difference vector
/// crossed with the segment direction in the original's exact operation
/// order) must each place the probe within 2.0 of the segment point,
/// compared by absolute value. Any unordered (NaN) comparison fails its gate.
/// The first segment passing all gates returns its index `i`; if none does,
/// the result is 0. All float operations run in the original's operand order.
///
/// Original: 0x00d2f4d0 (thiscall: object in ECX, one stack word, callee pops
/// 4, index or 0 in EAX). True size 812 bytes, not the listed 803.
lf_checker_rt::export!(thiscall, rw_00d2f4d0(this: u32, arg: u32) -> u32 {
    unsafe {
        const TABLE_PTR: u32 = 0x70;
        const STATE_FLAGS: u32 = 0xd8;
        const FLAG_OFF: u32 = 0x20000000;
        const PROBE_LINK: u32 = 0x20;
        const PROBE_X: u32 = 0x30;
        const PROBE_Y: u32 = 0x34;
        const PROBE_Z: u32 = 0x38;
        const POINT_BASE: i32 = 0x10;
        const POINT_STRIDE: i32 = 0x10;
        const ONE: f32 = 1.0;
        const TWO: f32 = 2.0;
        const NEG_HALF: f32 = f32::from_bits(0xbf000000); // -0.5
        const SIGN: u32 = 0x8000_0000;
        const ABS: u32 = 0x7fff_ffff;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn sqrt(a: f32) -> f32 {
            core::hint::black_box(a).sqrt()
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }
        #[inline(always)]
        fn abs(a: f32) -> f32 {
            f32::from_bits(a.to_bits() & ABS)
        }
        // Reciprocal length exactly as the original: an exact zero (either
        // sign) maps to zero, anything else divides (NaN stays NaN).
        #[inline(always)]
        fn rlen(len2: f32) -> f32 {
            if len2 == 0.0 {
                0.0
            } else {
                div(ONE, sqrt(len2))
            }
        }

        if rd32(this + STATE_FLAGS) & FLAG_OFF != 0 {
            return 0;
        }
        let table = rd32(this + TABLE_PTR);
        if table == 0 {
            return 0;
        }
        let count = rd32(table) as i32;
        if count < 2 {
            return 0;
        }
        let probe = rd32(arg + PROBE_LINK);
        let s30 = rdf(probe + PROBE_X);
        let s34 = rdf(probe + PROBE_Y);
        let s38 = rdf(probe + PROBE_Z);
        let at = |i: i32| table.wrapping_add(i.wrapping_mul(POINT_STRIDE).wrapping_add(POINT_BASE) as u32);
        let mut qx = rdf(at(0));
        let mut qy = rdf(at(0) + 4);
        let mut qz = rdf(at(0) + 8);
        if count <= 1 {
            return 0;
        }
        let mut i = 1i32;
        loop {
            let p = at(i);
            let px = rdf(p);
            let py = rdf(p + 4);
            let pz = rdf(p + 8);
            let dx = sub(px, qx);
            let dy = sub(py, qy);
            let dz = sub(pz, qz);
            let len2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
            let len = sqrt(len2);
            let inv = rlen(len2);
            let nx = mul(dx, inv);
            let ny = mul(dy, inv);
            let nz = mul(dz, inv);
            // Gate 1: n.(s - q) within [-0.5, len], ordered.
            let dp = add(add(mul(nx, qx), mul(ny, qy)), mul(nz, qz));
            let sp = add(add(mul(s34, ny), mul(s30, nx)), mul(s38, nz));
            let g1 = add(neg(dp), sp);
            if !(g1 >= NEG_HALF) {
                qx = px;
                qy = py;
                qz = pz;
                i += 1;
                if i < count {
                    continue;
                }
                return 0;
            }
            if !(len >= g1) {
                qx = px;
                qy = py;
                qz = pz;
                i += 1;
                if i < count {
                    continue;
                }
                return 0;
            }
            // Second frame from the direction and a zero vector, normalized.
            let z0 = mul(nz, 0.0);
            let x0 = mul(nx, 0.0);
            let y0 = mul(ny, 0.0);
            let ay = sub(ny, z0);
            let ax = sub(x0, y0);
            let az = sub(z0, nx);
            let l2 = add(add(mul(az, az), mul(ay, ay)), mul(ax, ax));
            let inv2 = rlen(l2);
            let axn = mul(ax, inv2);
            let azn = mul(az, inv2);
            let ayn = mul(ay, inv2);
            let bx = sub(mul(azn, nz), mul(axn, ny));
            let by = sub(mul(axn, nx), mul(ayn, nz));
            let bz = sub(mul(ayn, ny), mul(azn, nx));
            // Gate 2: probe within 2.0 of p under frame b.
            let ep = add(add(mul(by, py), mul(bx, px)), mul(bz, pz));
            let fp = add(add(mul(s34, by), mul(s30, bx)), mul(s38, bz));
            if !(TWO > abs(add(neg(ep), fp))) {
                qx = px;
                qy = py;
                qz = pz;
                i += 1;
                if i < count {
                    continue;
                }
                return 0;
            }
            // Third frame from b crossed with the direction, in order.
            let cx = sub(mul(bz, ny), mul(by, nz));
            let cy = sub(mul(bx, nz), mul(bz, nx));
            let cz = sub(mul(by, nx), mul(bx, ny));
            // Gate 3: probe within 2.0 of p under frame c; a pass returns i.
            let hp = add(add(mul(cy, py), mul(cx, px)), mul(cz, pz));
            let ip = add(add(mul(s34, cy), mul(cx, s30)), mul(s38, cz));
            if TWO > abs(add(neg(hp), ip)) {
                return i as u32;
            }
            qx = px;
            qy = py;
            qz = pz;
            i += 1;
            if i >= count {
                return 0;
            }
        }
    }
});
