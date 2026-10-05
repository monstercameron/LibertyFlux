// original: 0x00DAAB80 probe_slots_and_dispatch (proposed)

/// Probe eleven candidate slots with a worker callee, pick the best one by a
/// distance metric, and dispatch through two more callees.
///
/// Arguments (cdecl, nine stack words): `ctx` is an opaque context pointer
/// passed to every callee; `src` points to an input record of floats (x/y/z
/// direction at `+0x10`, position-ish triple at `+0x30`); `out_id`, `out_vec`
/// and `out_flag` are caller out-slots (one word, four words, one word);
/// `aux` is an opaque word passed on to the dispatch callee; `rf` and `cf`
/// are float parameters; only the low byte of `mode` is read.
///
/// Behaviour: `rmax` is `max(1.8, |rf|)` and `csum` is `clamp(cf, -1.8, 0.3)
/// + rmax`. Eleven 0x60-byte slots are built from the global triple at
/// `0x1B4B320` (three copies at `+0x10`, `+0x20`, `+0x30`, zeros and `0xffff`
/// in the header words) and each is offered, together with a position triple
/// derived from `src`, to the probe callee, which stamps a status word at
/// slot `+0x00`. The scan takes the first stamped slot whose index is below
/// 10 (a pair gate on slots `i`/`i+1` and a metric refinement exist but can
/// only fire when slots differ within a trial, which scripted answers cannot
/// express). The check callee vetoes, then the dispatch callee runs with
/// eleven arguments; when it reports failure and `mode` is nonzero, the
/// slot's triple at `+0x10` is copied to `out_vec`, `out_flag` is zeroed and,
/// when `out_id` is non-null, it receives the dereference callee's answer.
/// Returns 1 on success, 0 when no slot was usable, vetoed, or refused.
///
/// Float order is the original's scalar SSE order; every `comiss` branch
/// keeps NaN semantics (`ja` is a direct `>`, `jbe` is a negated `>`).
/// Uninitialised slot bytes read as the checker's defined stack fill (0),
/// which the rewrite writes explicitly.
lf_checker_rt::export!(cdecl, rw_00DAAB80(
    ctx: u32,
    src: u32,
    out_id: u32,
    out_vec: u32,
    out_flag: u32,
    aux: u32,
    rf: u32,
    cf: u32,
    mode: u32,
) -> u32 {
    unsafe {
        const C_RMAX: f32 = f32::from_bits(0x3FE6_6666); // 1.8
        const C_CMIN: f32 = f32::from_bits(0xBFE6_6666); // -1.8
        const C_CHI: f32 = f32::from_bits(0x3E99_999A); // 0.3
        const C_SCALE: f32 = f32::from_bits(0xBDCC_CCCD); // -0.1
        const C_POS: f32 = f32::from_bits(0x3DCC_CCCD); // 0.1
        const C_GATE: f32 = f32::from_bits(0x3E19_999A); // 0.15
        const C_METRIC: f32 = f32::from_bits(0x3E99_999A); // 0.3
        const C_ONE: f32 = 1.0;
        const G_VEC: u32 = 0x01B4_B320;
        const G_DISPATCH_ARG: u32 = 0x01B4_B2A0;
        const CAL_PROBE: u32 = 1;
        const CAL_CHECK: u32 = 2;
        const CAL_DISPATCH: u32 = 3;
        const CAL_DEREF: u32 = 4;
        const CAL_COOKIE: u32 = 5;
        const SLOT_WORDS: usize = 24;
        const N_SLOTS: usize = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        // rmax = max(1.8, |rf|); comiss/ja: keep on strictly greater.
        let rf_abs = f32::from_bits(rf & 0x7FFF_FFFF);
        let mut rmax = C_RMAX;
        if !(C_RMAX > rf_abs) {
            rmax = rf_abs;
        }
        // csum = clamp(cf, -1.8, 0.3) + rmax.
        let cfv = f32::from_bits(cf);
        let mut csum = C_CMIN;
        if !(C_CMIN > cfv) {
            csum = C_CHI;
            if !(cfv > C_CHI) {
                csum = cfv;
            }
        }
        csum = add(csum, rmax);

        let dir_x = rdf(src.wrapping_add(0x10));
        let dir_y = rdf(src.wrapping_add(0x14));
        let dir_z = rdf(src.wrapping_add(0x18));
        let mut reach = rdf(src.wrapping_add(0x30));
        let mut side = rdf(src.wrapping_add(0x34));
        let mut up = rdf(src.wrapping_add(0x38));
        let sx = mul(dir_x, C_SCALE);
        let sy = mul(dir_y, C_SCALE);
        let sz = mul(dir_z, C_SCALE);
        reach = add(reach, sx);
        side = add(side, sy);
        up = add(up, rmax);
        up = add(up, sz);

        let gx = f32::from_bits(rd32(lf_checker_rt::relocated(G_VEC)));
        let gy = f32::from_bits(rd32(lf_checker_rt::relocated(G_VEC.wrapping_add(4))));
        let gz = f32::from_bits(rd32(lf_checker_rt::relocated(G_VEC.wrapping_add(8))));

        // Eleven slots, all initialised (the dec/jns loop runs eleven times).
        let mut slots = [[0u32; SLOT_WORDS]; N_SLOTS];
        for s in slots.iter_mut() {
            s[4] = gx.to_bits();
            s[5] = gy.to_bits();
            s[6] = gz.to_bits();
            s[8] = gx.to_bits();
            s[9] = gy.to_bits();
            s[10] = gz.to_bits();
            s[12] = gx.to_bits();
            s[13] = gy.to_bits();
            s[14] = gz.to_bits();
            s[19] = 0xFFFF;
        }

        let step_x = mul(dir_x, C_POS);
        let step_y = mul(dir_y, C_POS);
        let step_z = mul(dir_z, C_POS);

        // Offer every slot to the probe callee with its position triple.
        for (i, slot) in slots.iter_mut().enumerate() {
            let t = i as f32;
            let mut px = mul(t, step_x);
            let mut py = mul(t, step_y);
            let mut pz = mul(t, step_z);
            px = add(px, reach);
            py = add(py, side);
            pz = add(pz, up);
            let mut pos = [px.to_bits(), py.to_bits(), pz.to_bits()];
            slot[0] = 0;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                CAL_PROBE,
                u32,
                ctx,
                pos.as_mut_ptr() as u32,
                csum.to_bits(),
                src,
                rmax.to_bits(),
                slot.as_mut_ptr() as u32,
                C_POS.to_bits()
            );
        }

        // Scan for the first stamped slot below index 10.
        let mut best: i32 = -1;
        let mut dx = 0.0f32;
        let mut dy = 0.0f32;
        let mut dz_slot = 0.0f32;
        let mut dz_prev = 0.0f32;
        for i in 0..N_SLOTS {
            let s = &slots[i];
            if s[0] == 0 {
                continue;
            }
            if i == 10 {
                continue;
            }
            if s[18] != 0 {
                let here = f32::from_bits(s[6]);
                let next = f32::from_bits(slots[i + 1][6]);
                if sub(next, here) > C_GATE {
                    continue;
                }
            }
            let vx = f32::from_bits(s[4]);
            let vy = f32::from_bits(s[5]);
            let vz = f32::from_bits(s[6]);
            let qx = sub(vx, dx);
            let mut qy = sub(vy, dy);
            let qz = sub(vz, dz_slot);
            if best != -1 {
                if !(qz > C_METRIC) {
                    dx = dz_prev;
                    continue;
                }
                qy = mul(qy, qy);
                let qxx = mul(qx, qx);
                let m = add(qy, qxx);
                if !(C_METRIC > m) {
                    dx = dz_prev;
                    continue;
                }
            }
            dx = vx;
            best = i as i32;
            dz_prev = vx;
            dy = vy;
            dz_slot = vz;
        }

        let ok = (|| -> bool {
            if best == -1 || best >= 10 {
                return false;
            }
            let slot = &mut slots[best as usize];
            let vto: u32 = lf_checker_rt::callee_cdecl!(CAL_CHECK, u32, ctx, slot.as_mut_ptr() as u32);
            if (vto as u8) != 0 {
                return false;
            }
            let entry = slot.as_mut_ptr() as u32;
            let dlv: u32 = lf_checker_rt::callee_cdecl!(
                CAL_DISPATCH,
                u32,
                ctx,
                out_vec,
                out_flag,
                entry.wrapping_add(0x10),
                out_id,
                aux,
                0,
                0,
                C_ONE.to_bits(),
                lf_checker_rt::relocated(G_DISPATCH_ARG),
                entry
            );
            if (dlv as u8) != 0 {
                return true;
            }
            if (mode as u8) == 0 {
                return false;
            }
            // Copy path: slot triple to out_vec, zero out_flag, answer out_id.
            let base = entry.wrapping_add(0x10);
            wr32(out_vec, rd32(base));
            wr32(out_vec.wrapping_add(4), rd32(base.wrapping_add(4)));
            wr32(out_vec.wrapping_add(8), rd32(base.wrapping_add(8)));
            wr32(out_vec.wrapping_add(12), rd32(base.wrapping_add(12)));
            wr32(out_flag, 0);
            if out_id != 0 {
                // The original pushes the slot's status word (its value, not
                // its address); the scripted callee only observes that value.
                let status0 = rd32(entry);
                let ans: u32 = lf_checker_rt::callee_cdecl!(CAL_DEREF, u32, status0);
                wr32(out_id, ans);
            }
            true
        })();

        let _: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
        if ok { 1 } else { 0 }
    }
});
