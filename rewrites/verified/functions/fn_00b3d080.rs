// original: 0x00B3D080 build_smoothed_point_sets (proposed)

/// Build smoothed point sets from a small input point cloud.
///
/// `hdr` points at a header dword whose bits 21..25 hold the point count `n`
/// (0..15). `pts` points at `n` input points, each 16 bytes with three
/// little-endian floats at +0/+4/+8 (the +12 lane is never read). `flags` is
/// either null or an array of `n` 8-byte flag slots; a slot whose halfword at
/// +2 is 0xFFFF marks its row skipped. `skip12` (low byte) skips the first
/// two output blocks when nonzero; `do34` (low byte) enables the last two
/// blocks when nonzero. `out` receives up to `4*n+1` rows of 16 bytes.
/// Returns the number of rows written. Cdecl, six stack words.
///
/// Stages, in order (all float arithmetic in the original's operand order;
/// the vectorised sum nests right, `acc = e3 + (e2 + (e1 + (e0 + acc)))`,
/// while the scalar tail nests left, `acc = acc + elem`):
/// 1. Centroid `m` of the `n` points, scaled by `1.0 / float(n)`.
/// 2. Unless skipped: `n` rows `(p[j] + m) * 0.5`.
/// 3. Unless skipped: `n` rows of neighbour smoothing with wraparound,
///    `((prev + cur) + m) / 3` (the z lane adds prev first, x and y cur
///    first), or a sentinel triple when the previous slot is flagged.
/// 4. One row holding the centroid itself.
/// 5. If enabled: `n` lerped rows `p[j] * 0.95 + m * 0.05`.
/// 6. If enabled: `n` weighted rows `(m * 0.2) + (((cur + prev) * 0.5) * 0.8)`
///    with the same flag rule (z again prev-first).
///
/// Every row's fourth lane (+12) is the original's uninitialised alignment
/// padding slot, reproduced here as 0.0 under the contract's `stack_fill: 0`
/// (see the result's `narrowed` field). The seven float constants live in the
/// original's read-only data and are embedded here by exact bits.
lf_checker_rt::export!(cdecl, rw_00B3D080(hdr: u32, pts: u32, flags: u32, skip12: u32, do34: u32, out: u32) -> u32 {
    unsafe {
        const C_ONE: f32 = f32::from_bits(0x3F80_0000); // 1.0
        const C_HALF: f32 = f32::from_bits(0x3F00_0000); // 0.5
        const C_THIRD: f32 = f32::from_bits(0x3EAA_AAAB); // 1/3
        const C_SMOOTH: f32 = f32::from_bits(0x3D4C_CCCD); // 0.05
        const C_KEEP: f32 = f32::from_bits(0x3F73_3333); // 0.95
        const C_BLEND: f32 = f32::from_bits(0x3E4C_CCCD); // 0.2
        const C_PAIR: f32 = f32::from_bits(0x3F4C_CCCD); // 0.8
        const SENTINEL: u32 = 0x4B18_9680;
        const FLAG_SKIP: u16 = 0xFFFF;
        const ROW: u32 = 16;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
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
        unsafe fn flagged(flags: u32, off: u32) -> bool {
            unsafe { flags != 0 && rd16(flags.wrapping_add(off).wrapping_add(2)) == FLAG_SKIP }
        }

        let n = ((rd32(hdr) >> 21) & 15) as i32;
        let inv_n = div(C_ONE, n as f32);

        // Stage 1: centroid.
        let mut sx = 0.0f32;
        let mut sy = 0.0f32;
        let mut sz = 0.0f32;
        let mut done = 0i32;
        if n >= 4 {
            let iters = (((n - 4) as u32 >> 2) + 1) as i32;
            let mut k = 0i32;
            while k < iters {
                let q = pts.wrapping_add((k as u32).wrapping_mul(64));
                let e0x = rdf(q);
                let e0y = rdf(q.wrapping_add(4));
                let e0z = rdf(q.wrapping_add(8));
                let e1x = rdf(q.wrapping_add(16));
                let e1y = rdf(q.wrapping_add(20));
                let e1z = rdf(q.wrapping_add(24));
                let e2x = rdf(q.wrapping_add(32));
                let e2y = rdf(q.wrapping_add(36));
                let e2z = rdf(q.wrapping_add(40));
                let e3x = rdf(q.wrapping_add(48));
                let e3y = rdf(q.wrapping_add(52));
                let e3z = rdf(q.wrapping_add(56));
                sx = add(e3x, add(e2x, add(e1x, add(e0x, sx))));
                sy = add(e3y, add(e2y, add(e1y, add(e0y, sy))));
                sz = add(e3z, add(e2z, add(e1z, add(e0z, sz))));
                k += 1;
            }
            done = iters * 4;
        }
        let mut i = done;
        while i < n {
            let p = pts.wrapping_add((i as u32).wrapping_mul(ROW));
            sx = add(sx, rdf(p));
            sy = add(sy, rdf(p.wrapping_add(4)));
            sz = add(sz, rdf(p.wrapping_add(8)));
            i += 1;
        }
        let mx = mul(sx, inv_n);
        let my = mul(sy, inv_n);
        let mz = mul(sz, inv_n);

        // The +12 lane reads the original's own uninitialised padding slot;
        // the contract pins it to zero on both sides (stack_fill).
        let w = 0.0f32;
        let mut rows = 0i32;

        if (skip12 as u8) == 0 {
            // Stage 2: scaled copies.
            if n > 0 {
                let mut j = 0i32;
                while j < n {
                    let p = pts.wrapping_add((j as u32).wrapping_mul(ROW));
                    let o = out.wrapping_add((j as u32).wrapping_mul(ROW));
                    wrf(o, mul(add(rdf(p), mx), C_HALF));
                    wrf(o.wrapping_add(4), mul(add(rdf(p.wrapping_add(4)), my), C_HALF));
                    wrf(o.wrapping_add(8), mul(add(rdf(p.wrapping_add(8)), mz), C_HALF));
                    wrf(o.wrapping_add(12), w);
                    j += 1;
                }
                rows = n;
                // Stage 3: neighbour smoothing with wraparound.
                let mut slot_off = ((n as u32).wrapping_mul(8)).wrapping_sub(8);
                let mut prev_off = ((n - 1) as u32).wrapping_mul(ROW);
                let mut k = 0i32;
                while k < n {
                    let ku = k as u32;
                    let cur = pts.wrapping_add(8).wrapping_add(ku.wrapping_mul(ROW));
                    let o = out.wrapping_add(((n as u32).wrapping_add(ku)).wrapping_mul(ROW));
                    if flagged(flags, slot_off) {
                        wr32(o, SENTINEL);
                        wr32(o.wrapping_add(4), SENTINEL);
                        wr32(o.wrapping_add(8), SENTINEL);
                        wrf(o.wrapping_add(12), w);
                    } else {
                        let pb = pts.wrapping_add(prev_off);
                        let ox = mul(add(add(rdf(cur.wrapping_sub(8)), rdf(pb)), mx), C_THIRD);
                        let oy = mul(
                            add(add(rdf(cur.wrapping_sub(4)), rdf(pb.wrapping_add(4))), my),
                            C_THIRD,
                        );
                        let oz = mul(
                            add(add(rdf(pb.wrapping_add(8)), rdf(cur)), mz),
                            C_THIRD,
                        );
                        wrf(o.wrapping_add(8), oz);
                        wrf(o, ox);
                        wrf(o.wrapping_add(4), oy);
                        wrf(o.wrapping_add(12), w);
                    }
                    rows += 1;
                    slot_off = ku.wrapping_mul(8);
                    prev_off = cur.wrapping_sub(pts).wrapping_sub(8);
                    k += 1;
                }
            }
        }

        // Stage 4: the centroid row.
        let o = out.wrapping_add((rows as u32).wrapping_mul(ROW));
        wrf(o, mx);
        wrf(o.wrapping_add(4), my);
        wrf(o.wrapping_add(8), mz);
        wrf(o.wrapping_add(12), w);
        rows += 1;

        if (do34 as u8) != 0 && n > 0 {
            // Stage 5: lerped rows.
            let bx = mul(mx, C_SMOOTH);
            let by = mul(my, C_SMOOTH);
            let bz = mul(mz, C_SMOOTH);
            let base3 = rows;
            let mut j = 0i32;
            while j < n {
                let p = pts.wrapping_add((j as u32).wrapping_mul(ROW));
                let oo = out.wrapping_add(((base3 + j) as u32).wrapping_mul(ROW));
                wrf(oo.wrapping_add(8), add(bz, mul(rdf(p.wrapping_add(8)), C_KEEP)));
                wrf(oo, add(bx, mul(rdf(p), C_KEEP)));
                wrf(oo.wrapping_add(4), add(by, mul(rdf(p.wrapping_add(4)), C_KEEP)));
                wrf(oo.wrapping_add(12), w);
                j += 1;
            }
            rows += n;
            // Stage 6: weighted smoothing.
            let mut slot_off = ((n as u32).wrapping_mul(8)).wrapping_sub(8);
            let mut prev_off = ((n - 1) as u32).wrapping_mul(ROW);
            let mut cur_base = pts.wrapping_add(8);
            let base6 = rows;
            let mut k = 0i32;
            while k < n {
                let ku = k as u32;
                let o6 = out.wrapping_add(((base6 + k) as u32).wrapping_mul(ROW));
                if flagged(flags, slot_off) {
                    wr32(o6, SENTINEL);
                    wr32(o6.wrapping_add(4), SENTINEL);
                    wr32(o6.wrapping_add(8), SENTINEL);
                } else {
                    let pb = pts.wrapping_add(prev_off);
                    let oy = add(
                        mul(my, C_BLEND),
                        mul(mul(add(rdf(cur_base.wrapping_sub(4)), rdf(pb.wrapping_add(4))), C_HALF), C_PAIR),
                    );
                    let oz = add(
                        mul(mz, C_BLEND),
                        mul(mul(add(rdf(pb.wrapping_add(8)), rdf(cur_base)), C_HALF), C_PAIR),
                    );
                    let ox = add(
                        mul(mx, C_BLEND),
                        mul(mul(add(rdf(cur_base.wrapping_sub(8)), rdf(pb)), C_HALF), C_PAIR),
                    );
                    wrf(o6.wrapping_add(4), oy);
                    wrf(o6.wrapping_add(8), oz);
                    wrf(o6, ox);
                }
                wrf(o6.wrapping_add(12), w);
                rows += 1;
                slot_off = ku.wrapping_mul(8);
                prev_off = cur_base.wrapping_sub(pts).wrapping_sub(8);
                cur_base = cur_base.wrapping_add(ROW);
                k += 1;
            }
        }

        rows as u32
    }
});
