// original: 0x00D939A0 spatial_sphere_pick_multi (proposed)

/// Pick the nearest table point to a query sphere over several query objects.
///
/// This is the multi-id sibling of `0x00D93430`: `pos` (sphere centre),
/// `radius`, `out` (two result words), `flag` (filter byte, low byte only)
/// and `this` (resync switch `+0xc18`, tag `+0xc42`) match, with two extras:
/// `extra` points to four words receiving the hit point, and `id` (the sixth
/// stack word) selects one id or, as `0xfff` with the filter byte clear, a
/// whole id list from the list callee (which returns the list length).
///
/// Resolution: a plain id queries once; `0xfff` with the filter byte set
/// queries `0xe74` once; otherwise the list callee fills a frame list and
/// returns its length (zero or negative ends the call). Each listed id maps
/// through the table callee to a query object or is skipped when null.
///
/// Per query object the logic mirrors the sibling: the out-word starts
/// `0xffff0fff` with merged flag bits; a set mode word takes the probe path
/// (the probe callee returns a hit pointer or null; a hit nearer than the
/// running best updates the best, the out id bits and the hit index divided
/// by 40, and stores the four hit words through `extra`); a clear mode word
/// scans entries exactly as the sibling does (bounds, resync, filter bit,
/// point slots at another shared base, closest-approach factor), except a
/// surviving point also calls the project callee for an interpolation factor
/// along the slot pair and stores the interpolated point through `extra`.
/// The best distance starts at the float maximum and is shared across every
/// id and both paths, so later ids only narrow it.
///
/// The centre and out pointers stay valid across ids: the scan clobbers its
/// registers but restores both before the next id on every path, so no stale
/// pointer can reach the probe. The fourth stored word on the scan path is
/// uninitialized stack in the original; the contract fills it with zero and
/// this rewrite stores zero. Both exits run the C runtime security check,
/// which genuinely preserves every register including the al result.
///
/// The call returns 1 when the out-word names an id and the index word is not
/// `0xffff`, else 0; only al is a real return.
///
/// Original: 0x00D939A0 (thiscall, six stack words; callee pops 24).
lf_checker_rt::export!(thiscall, rw_00D939A0(this: u32, pos: u32, radius: u32, out: u32, flag: u32, extra: u32, id: u32) -> u32 {
    unsafe {
        const ID_LOOKUP: u32 = 0xfff;
        const ID_FILTERED: u32 = 0xe74;
        const OUT_INIT: u32 = 0xffff0fff;
        const OUT_KEEP: u32 = 0xefffffff;
        const OUT_SET: u32 = 0x0fffffff;
        const KEY_MASK: u32 = 0x1ffff;
        const ENTRY_STRIDE: u32 = 0x28;
        const SKIP_BIT_SHIFT: u32 = 0x12;
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
        const G_INITFLAG: u32 = 0x017a3610;
        const G_COUNTER: u32 = 0x017a3374;
        const G_SLOTS: u32 = 0x017a3510;
        const C_SCALE: u32 = 0x00fe8afc;
        const C_FMAX: u32 = 0x00fe8d18;
        const C_EPS0: u32 = 0x0110db40;
        const C_EPS1: u32 = 0x0110db44;
        const C_EPS2: u32 = 0x0110db48;
        const SIGN_BIT: u32 = 0x80000000;
        const CALLEE_LIST: u32 = 1;
        const CALLEE_TABLE: u32 = 2;
        const CALLEE_PROBE: u32 = 3;
        const CALLEE_POINT: u32 = 4;
        const CALLEE_PROJECT: u32 = 5;
        const CALLEE_COOKIE: u32 = 6;

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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        /// Exact `cvttss2si` (see the sibling rewrite).
        #[target_feature(enable = "sse2")]
        unsafe fn cvtt(x: f32) -> i32 {
            unsafe {
                core::arch::x86::_mm_cvttss_si32(core::arch::x86::_mm_set_ss(x))
            }
        }
        /// `comiss e, t` followed by `jb`.
        #[inline(always)]
        fn below(e: f32, t: f32) -> bool {
            !(e >= t)
        }
        /// Signed division by 40 by the multiply-shift sequence.
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
        let mut idlist = [0u32; 4];
        let count: i32;
        if id != ID_LOOKUP {
            idlist[0] = id;
            count = 1;
        } else if filter {
            idlist[0] = ID_FILTERED;
            count = 1;
        } else {
            count = lf_checker_rt::callee_cdecl!(
                CALLEE_LIST, u32, pos, radius, idlist.as_mut_ptr() as u32
            ) as i32;
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
        let e0 = rdf(lf_checker_rt::relocated(C_EPS0));
        let e1 = rdf(lf_checker_rt::relocated(C_EPS1));
        let e2 = rdf(lf_checker_rt::relocated(C_EPS2));
        let mut best = rdf(lf_checker_rt::relocated(C_FMAX));
        let slots = lf_checker_rt::relocated(G_SLOTS);
        if count > 0 {
            let mut li: u32 = 0;
            while (li as i32) < count {
                let cur = idlist[li as usize];
                let query = lf_checker_rt::callee_cdecl!(CALLEE_TABLE, u32, cur);
                if query != 0 {
                    if rd32(query.wrapping_add(Q_MODE)) != 0 {
                        let mut buf = [0.0f32; 4];
                        let hit = lf_checker_rt::callee_thiscall!(
                            CALLEE_PROBE, u32, query, pos, radius, buf.as_mut_ptr() as u32,
                            0, 0, 0, 0, 0
                        );
                        if hit != 0 {
                            let dx = sub(buf[0], px);
                            let dy = sub(buf[1], py);
                            let dz = sub(buf[2], pz);
                            let dist2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
                            if best > dist2 {
                                best = dist2;
                                wr32(out, rd32(out) ^ ((rd32(out) ^ cur) & 0xfff));
                                let base = rd32(query.wrapping_add(Q_BASE));
                                wr16(out.wrapping_add(2), div40(hit.wrapping_sub(base)));
                                wrf(extra, buf[0]);
                                wrf(extra.wrapping_add(4), buf[1]);
                                wrf(extra.wrapping_add(8), buf[2]);
                                wrf(extra.wrapping_add(12), buf[3]);
                            }
                        }
                    } else {
                        let qcount = rd32(query.wrapping_add(Q_COUNT));
                        if qcount != 0 {
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
                                let restart = (filter && ((rd32(entry) >> SKIP_BIT_SHIFT) & 1 != 0))
                                    || (lo_x as u16) as i16 > (rd16(entry.wrapping_add(0x12)) as i16)
                                    || (lo_y as u16) as i16 > (rd16(entry.wrapping_add(0x16)) as i16)
                                    || (lo_z as u16) as i16 > (rd16(entry.wrapping_add(0x1a)) as i16)
                                    || ((hi_x as u16) as i16) < (rd16(entry.wrapping_add(0x10)) as i16)
                                    || ((hi_y as u16) as i16) < (rd16(entry.wrapping_add(0x14)) as i16)
                                    || ((hi_z as u16) as i16) < (rd16(entry.wrapping_add(0x18)) as i16);
                                if !restart {
                                    let flags = rd32(entry);
                                    let count2 = (flags >> 21) & 0xf;
                                    if flags & COUNT_BITS != 0 {
                                        let mut k: u32 = 0;
                                        while k < count2 {
                                            let key = (rd32(entry.wrapping_add(E_PACKED)) & KEY_MASK)
                                                .wrapping_add(k);
                                            let v = rd16(utable.wrapping_add(key.wrapping_mul(2))) as u32;
                                            lf_checker_rt::callee_thiscall!(
                                                CALLEE_POINT, u32, query, v,
                                                slots.wrapping_add(k.wrapping_mul(16))
                                            );
                                            k = k.wrapping_add(1);
                                        }
                                    }
                                    // The jbe tests the AND that formed count2
                                    // (lea sets no flags): count2 iterations.
                                    if count2 != 0 {
                                        let mut edx_slot = (count2 - 1).wrapping_mul(16);
                                        let mut ecx_slot: u32 = 0;
                                        let mut j: u32 = 0;
                                        while j < count2 {
                                            let dhx = slot(slots.wrapping_add(edx_slot));
                                            let dhy = slot(slots.wrapping_add(edx_slot).wrapping_add(4));
                                            let dhz = slot(slots.wrapping_add(edx_slot).wrapping_add(8));
                                            let djx = slot(slots.wrapping_add(ecx_slot));
                                            let djy =
                                                slot(slots.wrapping_add(ecx_slot).wrapping_add(4));
                                            let djz =
                                                slot(slots.wrapping_add(ecx_slot).wrapping_add(8));
                                            let high_addr = slots.wrapping_add(edx_slot);
                                            let dx = sub(djx, dhx);
                                            let hx = sub(dhx, px);
                                            let dy = sub(djy, dhy);
                                            let hy = sub(dhy, py);
                                            let dz = sub(djz, dhz);
                                            let hz = sub(dhz, pz);
                                            let t = f32::from_bits(
                                                add(add(mul(hy, dy), mul(hx, dx)), mul(hz, dz)).to_bits()
                                                    ^ SIGN_BIT,
                                            );
                                            let (fx, fy, fz);
                                            if below(e0, t) || below(e1, t) || below(e2, t) {
                                                let len2 =
                                                    add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
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
                                            let dist2 =
                                                add(add(mul(qy, qy), mul(qx, qx)), mul(qz, qz));
                                            if r2 > dist2 && best > dist2 {
                                                best = dist2;
                                                wr32(out, rd32(out) ^ ((rd32(out) ^ cur) & 0xfff));
                                                wr16(out.wrapping_add(2), oi as u16);
                                                let mut pbuf = [0.0f32; 3];
                                                pbuf[0] = sub(djx, dhx);
                                                pbuf[1] = sub(djy, dhy);
                                                pbuf[2] = sub(djz, dhz);
                                                let factor: f32 = lf_checker_rt::callee_cdecl!(
                                                    CALLEE_PROJECT, f32, high_addr,
                                                    pbuf.as_mut_ptr() as u32, pos
                                                );
                                                let sx = sub(djx, dhx);
                                                let sy = sub(djy, dhy);
                                                let sz = sub(djz, dhz);
                                                wrf(extra.wrapping_add(12), 0.0);
                                                wrf(extra, add(dhx, mul(sx, factor)));
                                                wrf(extra.wrapping_add(4), add(dhy, mul(sy, factor)));
                                                wrf(extra.wrapping_add(8), add(dhz, mul(sz, factor)));
                                            }
                                            edx_slot = ecx_slot;
                                            ecx_slot = ecx_slot.wrapping_add(16);
                                            j = j.wrapping_add(1);
                                        }
                                    }
                                }
                                oi = oi.wrapping_add(1);
                            }
                        }
                    }
                }
                li = li.wrapping_add(1);
            }
        }
        let ret = if rd32(out) & 0xfff == 0xfff || rd16(out.wrapping_add(2)) == 0xffff {
            0u32
        } else {
            1u32
        };
        lf_checker_rt::callee_cdecl!(CALLEE_COOKIE, u32,);
        ret
    }
});
