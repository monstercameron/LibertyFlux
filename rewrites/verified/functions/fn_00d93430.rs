// original: 0x00D93430 spatial_sphere_pick (proposed)

/// Pick the nearest table point to a query sphere and report it through an
/// out-word.
///
/// `pos` points to the sphere centre (three floats), `radius` is its radius,
/// `out` points to two words receiving the result, `flag` is a filter byte
/// (only its low byte is read) and `id` selects the query object. `this`
/// points to an object with a resync switch at `+0xc18` and a 16-bit tag at
/// `+0xc42`.
///
/// Resolution: id `0xfff` means "look it up": with the filter byte set the id
/// becomes `0xe74`, otherwise the classify callee maps the centre to an id.
/// (The original also writes the resolved id back over the caller's id slot;
/// a rewrite cannot touch caller stack, so the stack check is off for this
/// function.) The table callee then maps the id to a query object, or null.
///
/// The out-word starts as `0xffff0fff` with bits merged into its second word.
/// A null query object ends the call. Otherwise two paths split on the
/// query's mode word at `+0x70`:
/// - Direct path: the probe callee (thiscall, centre, radius, frame buffer,
///   five zero words) returns a hit pointer or null. Null, or a squared
///   distance at or above the float maximum, ends the call. Otherwise the
///   low 12 bits of the out-word become the id and the word after it the hit
///   index divided by 40 (signed, by the multiply-shift sequence).
/// - Scan path: centre minus/plus radius, scaled by 8 and truncated toward
///   zero (exact `cvttss2si`, including its `0x80000000` invalid result),
///   gives float bounds kept as integers. Each of the query's entries (40
///   bytes, count at `+0x7c`) is skipped unless its six 16-bit bound words
///   (compared signed) contain the query bounds, and, with the filter byte
///   set, unless its skip bit (bit 18) is clear. A set resync switch refreshes
///   a stale entry tag (clearing bits, zeroing a word, bumping a counter).
///   Surviving entries with point-count bits (21-24) resolve that many points
///   through the point callee into shared slots, then test every slot against
///   the previous one (the first against the last): a closest-approach factor
///   below three 1e-12 thresholds keeps the raw delta, otherwise the delta is
///   projected along the slot pair. A point nearer than the radius squared and
///   nearer than every earlier point rewrites the best distance, the out-word
///   id bits and the out index.
///
/// The call returns 1 when the out-word names an id (low 12 bits set by
/// either path) and the index word is not `0xffff`, else 0. Only the low byte
/// is a real return; the rest of eax is leftover.
///
/// Original: 0x00D93430 (thiscall, five stack words; callee pops 20).
lf_checker_rt::export!(thiscall, rw_00D93430(this: u32, pos: u32, radius: u32, out: u32, flag: u32, id: u32) -> u32 {
    unsafe {
        const ID_LOOKUP: u32 = 0xfff;
        const ID_FILTERED: u32 = 0xe74;
        const OUT_INIT: u32 = 0xffff0fff;
        const OUT_KEEP: u32 = 0xefffffff;
        const OUT_SET: u32 = 0x0fffffff;
        const SCALE: f32 = 8.0;
        const KEY_MASK: u32 = 0x1ffff;
        const ENTRY_STRIDE: u32 = 0x28;
        const SKIP_BIT: u32 = 0x40000;
        const COUNT_BITS: u32 = 0x1e00000;
        const RESYNC_KEEP: u32 = 0xffe530ff;
        const THIS_RESYNC: u32 = 0xc18;
        const THIS_TAG: u32 = 0xc42;
        const Q_UTABLE: u32 = 0x60;
        const Q_BASE: u32 = 0x6c;
        const Q_MODE: u32 = 0x70;
        const Q_COUNT: u32 = 0x7c;
        const E_PACKED: u32 = 0x04;
        const E_TAG: u32 = 0x0a;
        const E_ZERO: u32 = 0x0c;
        const G_INITFLAG: u32 = 0x017a3500;
        const G_COUNTER: u32 = 0x017a3374;
        const G_SLOTS: u32 = 0x017a3400;
        const C_SCALE: u32 = 0x00fe8afc;
        const C_FMAX: u32 = 0x00fe8d18;
        const C_EPS0: u32 = 0x0110db40;
        const C_EPS1: u32 = 0x0110db44;
        const C_EPS2: u32 = 0x0110db48;
        const SIGN_BIT: u32 = 0x80000000;
        const CALLEE_CLASSIFY: u32 = 1;
        const CALLEE_TABLE: u32 = 2;
        const CALLEE_PROBE: u32 = 3;
        const CALLEE_POINT: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        /// Exact `cvttss2si`: truncation with the hardware invalid result,
        /// which a saturating float-to-int cast would not reproduce.
        #[target_feature(enable = "sse2")]
        unsafe fn cvtt(x: f32) -> i32 {
            unsafe {
                core::arch::x86::_mm_cvttss_si32(core::arch::x86::_mm_set_ss(x))
            }
        }
        /// `comiss e, t` followed by `jb`: taken when e is strictly below t
        /// or the comparison is unordered.
        #[inline(always)]
        fn below(e: f32, t: f32) -> bool {
            !(e >= t)
        }
        /// Signed division by 40 by the original's multiply-shift sequence.
        #[inline(always)]
        fn div40(diff: u32) -> u16 {
            let prod = (diff as i32 as i64).wrapping_mul(0x66666667);
            let q = ((prod >> 32) as i32) >> 4;
            q.wrapping_add(((q as u32) >> 31) as i32) as u16
        }
        #[inline(always)]
        unsafe fn slot(v: u32) -> f32 {
            unsafe { f32::from_bits(rd32(v)) }
        }

        let filter = (flag & 0xff) != 0;
        let mut cur = id;
        if id == ID_LOOKUP {
            if filter {
                cur = ID_FILTERED;
            } else {
                cur = lf_checker_rt::callee_cdecl!(CALLEE_CLASSIFY, u32, pos);
            }
        }
        let gf = lf_checker_rt::relocated(G_INITFLAG);
        let f = rd32(gf);
        if f & 1 == 0 {
            wr32(gf, f | 1);
        }
        let r = f32::from_bits(radius);
        let px = rdf(pos);
        let py = rdf(pos.wrapping_add(4));
        let pz = rdf(pos.wrapping_add(8));
        let scale = rdf(lf_checker_rt::relocated(C_SCALE));
        let lo_x = cvtt(mul(sub(px, r), scale));
        let hi_x = cvtt(mul(add(px, r), scale));
        let lo_y = cvtt(mul(sub(py, r), scale));
        let hi_y = cvtt(mul(add(py, r), scale));
        let lo_z = cvtt(mul(sub(pz, r), scale));
        let hi_z = cvtt(mul(add(pz, r), scale));
        let r2 = mul(r, r);
        wr32(out, OUT_INIT);
        wr32(out.wrapping_add(4), (rd32(out.wrapping_add(4)) & OUT_KEEP) | OUT_SET);
        let query = lf_checker_rt::callee_cdecl!(CALLEE_TABLE, u32, cur);
        if query == 0 {
            return if rd32(out) & 0xfff == 0xfff || rd16(out.wrapping_add(2)) == 0xffff {
                0
            } else {
                1
            };
        }
        let epilogue = |out: u32| -> u32 {
            if rd32(out) & 0xfff == 0xfff || rd16(out.wrapping_add(2)) == 0xffff {
                0
            } else {
                1
            }
        };
        if rd32(query.wrapping_add(Q_MODE)) != 0 {
            let mut buf = [0.0f32; 3];
            let hit = lf_checker_rt::callee_thiscall!(
                CALLEE_PROBE, u32, query, pos, radius, buf.as_mut_ptr() as u32,
                0, 0, 0, 0, 0
            );
            if hit == 0 {
                return epilogue(out);
            }
            let dx = sub(buf[0], px);
            let dy = sub(buf[1], py);
            let dz = sub(buf[2], pz);
            let dist2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
            if !(rdf(lf_checker_rt::relocated(C_FMAX)) > dist2) {
                return epilogue(out);
            }
            wr32(out, rd32(out) ^ ((rd32(out) ^ cur) & 0xfff));
            let base = rd32(query.wrapping_add(Q_BASE));
            wr16(out.wrapping_add(2), div40(hit.wrapping_sub(base)));
            return epilogue(out);
        }
        let qcount = rd32(query.wrapping_add(Q_COUNT));
        if qcount == 0 {
            return epilogue(out);
        }
        let e0 = rdf(lf_checker_rt::relocated(C_EPS0));
        let e1 = rdf(lf_checker_rt::relocated(C_EPS1));
        let e2 = rdf(lf_checker_rt::relocated(C_EPS2));
        let mut best = rdf(lf_checker_rt::relocated(C_FMAX));
        let qbase = rd32(query.wrapping_add(Q_BASE));
        let utable = rd32(query.wrapping_add(Q_UTABLE));
        let mut oi: u32 = 0;
        while oi < rd32(query.wrapping_add(Q_COUNT)) {
            let entry = qbase.wrapping_add(oi.wrapping_mul(ENTRY_STRIDE));
            if rd32(this.wrapping_add(THIS_RESYNC)) != 0 {
                let tag = rd16(this.wrapping_add(THIS_TAG));
                if rd16(entry.wrapping_add(E_TAG)) != tag {
                    wr32(entry, rd32(entry) & RESYNC_KEEP);
                    wr16(entry.wrapping_add(E_TAG), tag);
                    wr32(entry.wrapping_add(E_ZERO), 0);
                    let gc = lf_checker_rt::relocated(G_COUNTER);
                    wr32(gc, rd32(gc).wrapping_add(1));
                }
            }
            // The first three bound tests skip when the query bound is
            // strictly above the entry word (jg); the last three skip when
            // it is strictly below (jl); all compare signed 16-bit.
            let restart = (filter && ((rd32(entry) >> 0x12) & 1 != 0))
                || (lo_x as u16) as i16 > (rd16(entry.wrapping_add(0x12)) as i16)
                || (lo_y as u16) as i16 > (rd16(entry.wrapping_add(0x16)) as i16)
                || (lo_z as u16) as i16 > (rd16(entry.wrapping_add(0x1a)) as i16)
                || ((hi_x as u16) as i16) < (rd16(entry.wrapping_add(0x10)) as i16)
                || ((hi_y as u16) as i16) < (rd16(entry.wrapping_add(0x14)) as i16)
                || ((hi_z as u16) as i16) < (rd16(entry.wrapping_add(0x18)) as i16);
            if restart {
                oi = oi.wrapping_add(1);
                continue;
            }
            let flags = rd32(entry);
            let count2 = (flags >> 21) & 0xf;
            if flags & COUNT_BITS != 0 {
                let mut k: u32 = 0;
                while k < count2 {
                    let key = (rd32(entry.wrapping_add(E_PACKED)) & KEY_MASK).wrapping_add(k);
                    let v = rd16(utable.wrapping_add(key.wrapping_mul(2))) as u32;
                    lf_checker_rt::callee_thiscall!(
                        CALLEE_POINT, u32, query, v,
                        lf_checker_rt::relocated(G_SLOTS).wrapping_add(k.wrapping_mul(16))
                    );
                    k = k.wrapping_add(1);
                }
            }
            // The jbe below the count computation tests the AND that formed
            // count2 (lea sets no flags), so the inner loop runs only when
            // count2 != 0, for exactly count2 iterations.
            if count2 != 0 {
                let n = count2;
                let slots = lf_checker_rt::relocated(G_SLOTS);
                let mut edx_slot = (count2 - 1).wrapping_mul(16);
                let mut ecx_slot: u32 = 0;
                let mut j: u32 = 0;
                while j < n {
                    let dhx = slot(slots.wrapping_add(edx_slot));
                    let dhy = slot(slots.wrapping_add(edx_slot).wrapping_add(4));
                    let dhz = slot(slots.wrapping_add(edx_slot).wrapping_add(8));
                    let djx = slot(slots.wrapping_add(ecx_slot));
                    let djy = slot(slots.wrapping_add(ecx_slot).wrapping_add(4));
                    let djz = slot(slots.wrapping_add(ecx_slot).wrapping_add(8));
                    let dx = sub(djx, dhx);
                    let hx = sub(dhx, px);
                    let dy = sub(djy, dhy);
                    let hy = sub(dhy, py);
                    let dz = sub(djz, dhz);
                    let hz = sub(dhz, pz);
                    let t = f32::from_bits(
                        add(add(mul(hy, dy), mul(hx, dx)), mul(hz, dz)).to_bits() ^ SIGN_BIT,
                    );
                    let (fx, fy, fz);
                    if below(e0, t) || below(e1, t) || below(e2, t) {
                        let len2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                        let d = sub(len2, t);
                        if below(e0, d) || below(e1, d) || below(e2, d) {
                            let len_a = add(d, t);
                            let len_b = add(d, t);
                            let len_c = add(d, t);
                            let q0 = div(t, len_a);
                            let q1 = div(t, len_b);
                            let q3 = div(t, len_c);
                            fx = add(mul(q0, dx), hx);
                            fy = add(mul(q1, dy), hy);
                            fz = add(hz, mul(q3, dz));
                        } else {
                            fx = add(hx, dx);
                            fy = add(hy, dy);
                            fz = add(hz, dz);
                        }
                    } else {
                        fx = hx;
                        fy = hy;
                        fz = hz;
                    }
                    let qx = sub(add(px, fx), px);
                    let qy = sub(add(py, fy), py);
                    let qz = sub(add(pz, fz), pz);
                    let dist2 = add(add(mul(qy, qy), mul(qx, qx)), mul(qz, qz));
                    if r2 > dist2 && best > dist2 {
                        best = dist2;
                        wr32(out, rd32(out) ^ ((rd32(out) ^ cur) & 0xfff));
                        wr16(out.wrapping_add(2), oi as u16);
                    }
                    edx_slot = ecx_slot;
                    ecx_slot = ecx_slot.wrapping_add(16);
                    j = j.wrapping_add(1);
                }
            }
            oi = oi.wrapping_add(1);
        }
        epilogue(out)
    }
});
