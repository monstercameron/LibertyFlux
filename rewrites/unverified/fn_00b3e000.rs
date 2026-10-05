// original: 0x00B3E000 query_publish_and_run (proposed)

/// Query setup that resolves candidate scopes, publishes the query state to
/// the shared globals and runs the worker scan.
///
/// Arguments (cdecl, six stack words): `center` points at the query vec3
/// (a fourth float follows it), `out` receives the result vec4, `radius`
/// is the search radius as float bits, `flags` packs five option bits
/// (bits 0, 2, 5, 6 and 1), and `aux0`/`aux1` are an entry count and a base
/// pointer for the worker. Returns 0 when nothing was found, else 1 or 2
/// (one plus a flag bit of the recorded member).
///
/// The radius is clamped to the stored raw radius when that is not above
/// it. Five sample points (the center and the center shifted by the radius
/// along x and y in both directions) are resolved to scope ids through a
/// callee and deduplicated. A first loop takes the first scope whose probe
/// callee reports a hit passing every enabled flag-gated bit test. When no
/// scope is taken, a second loop keeps the nearest hit reported by a
/// measuring callee (distance from the callee's out-vector to the center).
/// The taken scope id and member index select a member slot; the center,
/// radius, flags, bounds (center plus/minus radius scaled by 8, truncated
/// to 16-bit words) and fresh result cells are published to the globals, a
/// setup callee runs, the member is stamped, a one-entry work list is
/// pushed and the worker is called. When the worker recorded a member, its
/// result vector is copied to `out`.
///
/// Two quirks are reproduced literally. The selector the publish step
/// resolves is a stale slot: it still holds the last scope pointer of the
/// first loop (or the preset constant when the first loop accepted), never
/// the second loop's pointer. And the scope id the worker runs under is the
/// member index the measuring callee's return implies ((return - base) / 40
/// through the multiply-and-shift idiom). The worker call site is patched,
/// so with a stubbed worker the result cell keeps its preset value and the
/// copy/return-1-or-2 path never runs; it is implemented for composition
/// with a natively run worker and noted as uncovered.
///
/// Float order is the original's throughout. The stack cookie is checked
/// through the intercepted checker callee on both exits; the value passed
/// on cannot reproduce the original's stack-derived mix and is left
/// uncompared.
///
/// Original: 0x00B3E000 (cdecl, six stack words).
lf_checker_rt::export!(cdecl, rw_00B3E000(center: u32, out: u32, radius: u32, flags: u32, aux0: u32, aux1: u32) -> u32 {
    unsafe {
        const COOKIE: u32 = 0x1057FB4;
        const RAW_RADIUS_SRC: u32 = 0x10482AC;
        const FLAG0: u32 = 0x16646A0;
        const FLAG1: u32 = 0x16646A1;
        const FLAG2: u32 = 0x16646A2;
        const FLAG3: u32 = 0x16646A3;
        const FLAG4: u32 = 0x16646A4;
        const SAVED_FLAGS: u32 = 0x1664680;
        const QUERY_FLAGS: u32 = 0x166469C;
        const QUERY_AUX0: u32 = 0x16646A8;
        const QUERY_AUX1: u32 = 0x16646AC;
        const QUERY_POS_X: u32 = 0x16653D0;
        const QUERY_POS_Y: u32 = 0x16653D4;
        const QUERY_POS_Z: u32 = 0x16653D8;
        const QUERY_POS_W: u32 = 0x16653DC;
        const QUERY_RAD: u32 = 0x1664684;
        const QUERY_RAD2: u32 = 0x1664688;
        const BOUND_LO_X: u32 = 0x16653F0;
        const BOUND_LO_Y: u32 = 0x16653F4;
        const BOUND_LO_Z: u32 = 0x16653F8;
        const BOUND_HI_X: u32 = 0x16653F2;
        const BOUND_HI_Y: u32 = 0x16653F6;
        const BOUND_HI_Z: u32 = 0x16653FA;
        const BEST_DIST: u32 = 0x166468C;
        const RESULT_ID12: u32 = 0x1664690;
        const RESULT_IDX: u32 = 0x1664694;
        const RESULT_SLOT: u32 = 0x1664698;
        const RESULT_X: u32 = 0x16653E0;
        const RESULT_Y: u32 = 0x16653E4;
        const RESULT_Z: u32 = 0x16653E8;
        const RESULT_W: u32 = 0x16653EC;
        const WORK_COUNT: u32 = 0x16646B0;
        const WORK_PTR: u32 = 0x16646B8;
        const WORK_SLOT: u32 = 0x16646BC;
        const STAMP: u32 = 0x16C7472;
        const SETUP_THIS: u32 = 0x16C6830;
        const F32_MAX_BITS: u32 = 0xFE8D18;
        const RADIUS_GAIN: u32 = 0xFE8B68;
        const BOUND_GAIN: u32 = 0xFE8AFc;
        const MEASURE_CB: u32 = 0xB3EC90;
        const SCOPE_MEMBERS: u32 = 0x6C;
        const MEMBER_STRIDE: u32 = 40;
        const MEMBER_BITS: u32 = 0x1C;
        const MISS: u32 = 0xFFFF;
        const NO_ID12: u32 = 0xFFF;
        const RESOLVE_ID: u32 = 1;
        const LOOKUP_SCOPE: u32 = 2;
        const PROBE_SCOPE: u32 = 3;
        const MEASURE_SCOPE: u32 = 4;
        const SETUP_WORKER: u32 = 5;
        const RUN_WORKER: u32 = 6;
        const COOKIE_CHECK: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u32 {
            unsafe { (a as *const u8).read() as u32 }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u32 {
            unsafe { rd8(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
        }
        #[inline(always)]
        unsafe fn gw32(va: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(va) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn gw8(va: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(va) as *mut u8).write(v as u8) }
        }
        #[inline(always)]
        unsafe fn gw16(va: u32, v: u16) {
            unsafe {
                (lf_checker_rt::relocated(va) as *mut u16).write_unaligned(v)
            }
        }
        #[inline(always)]
        unsafe fn gwf(va: u32, v: f32) {
            unsafe { gw32(va, v.to_bits()) }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn cvtt(v: f32) -> i32 {
            if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
                i32::MIN
            } else {
                v as i32
            }
        }
        #[inline(always)]
        fn below_eq(a: f32, b: f32) -> bool {
            !(core::hint::black_box(a) > core::hint::black_box(b))
        }
        /// Signed divide by 40 through the original's multiply-and-shift.
        #[inline(always)]
        fn div40(v: u32) -> u32 {
            let q = (v as i32).wrapping_mul(0x66666667) >> 4;
            q.wrapping_add((q as u32 >> 31) as i32) as u32
        }
        #[inline(always)]
        unsafe fn cookie_check(cookie: u32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, cookie);
            }
        }

        let cookie = g32(COOKIE);
        let rad = f32::from_bits(radius);
        gw8(FLAG0, (flags >> 0) & 1);
        gw8(FLAG1, (flags >> 2) & 1);
        gw8(FLAG2, (flags >> 5) & 1);
        gw8(FLAG3, (flags >> 6) & 1);
        gw8(FLAG4, (flags >> 1) & 1);
        let raw = fmul((g32(RAW_RADIUS_SRC) as i32) as f32, gf(RADIUS_GAIN));
        let r = if below_eq(raw, rad) { raw } else { rad };
        let cx = f32::from_bits(rd32(center));
        let cy = f32::from_bits(rd32(center.wrapping_add(4)));
        let cz = f32::from_bits(rd32(center.wrapping_add(8)));
        // Five sample points: center, +/- x, +/- y.
        let mut pts = [[0u32; 3]; 5];
        pts[0] = [cx.to_bits(), cy.to_bits(), cz.to_bits()];
        pts[1] = [fsub(cx, r).to_bits(), cy.to_bits(), cz.to_bits()];
        pts[2] = [fadd(cx, r).to_bits(), cy.to_bits(), cz.to_bits()];
        pts[3] = [cx.to_bits(), fsub(cy, r).to_bits(), cz.to_bits()];
        pts[4] = [cx.to_bits(), fadd(cy, r).to_bits(), cz.to_bits()];
        let mut ids = [0u32; 5];
        let mut nids = 0usize;
        for p in pts.iter() {
            let id = lf_checker_rt::callee_cdecl!(
                RESOLVE_ID,
                u32,
                p.as_ptr() as u32
            );
            if !ids[..nids].contains(&id) {
                ids[nids] = id;
                nids += 1;
            }
        }
        let mut best = gf(F32_MAX_BITS);
        let mut stale_sel = 0u32;
        let mut sel = NO_ID12;
        let mut member = MISS;
        let mut accepted = false;
        for i in 0..nids {
            let scope = lf_checker_rt::callee_cdecl!(LOOKUP_SCOPE, u32, ids[i]);
            stale_sel = scope;
            if scope == 0 {
                continue;
            }
            let mut probe_out = [0u32; 3];
            let hit = lf_checker_rt::callee_thiscall!(
                PROBE_SCOPE,
                u32,
                scope,
                center,
                probe_out.as_mut_ptr() as u32,
                0x40800000u32
            );
            if hit == MISS {
                continue;
            }
            let slot = rd32(scope.wrapping_add(SCOPE_MEMBERS))
                .wrapping_add(hit.wrapping_mul(MEMBER_STRIDE));
            if slot == 0 {
                continue;
            }
            if g8(FLAG0) != 0 && (rd32(slot) >> 2) & 1 == 0 {
                continue;
            }
            if g8(FLAG1) != 0 && (rd8(slot.wrapping_add(MEMBER_BITS)) >> 7) & 1 != 0 {
                continue;
            }
            if g8(FLAG2) != 0 && (rd8(slot.wrapping_add(MEMBER_BITS)) >> 6) & 1 != 0 {
                continue;
            }
            if g8(FLAG3) != 0 && (rd32(slot) >> 7) & 1 != 0 {
                continue;
            }
            member = hit;
            accepted = true;
            break;
        }
        if !accepted {
            gw32(SAVED_FLAGS, flags);
            for i in 0..nids {
                let scope = lf_checker_rt::callee_cdecl!(LOOKUP_SCOPE, u32, ids[i]);
                if scope == 0 {
                    continue;
                }
                let mut m_out = [0u32; 3];
                let h = lf_checker_rt::callee_thiscall!(
                    MEASURE_SCOPE,
                    u32,
                    scope,
                    center,
                    radius,
                    m_out.as_mut_ptr() as u32,
                    lf_checker_rt::relocated(MEASURE_CB),
                    0,
                    flags,
                    0,
                    0
                );
                if h == 0 {
                    continue;
                }
                let ox = f32::from_bits(m_out[0]);
                let oy = f32::from_bits(m_out[1]);
                let oz = f32::from_bits(m_out[2]);
                let dx = fsub(ox, cx);
                let dy = fsub(oy, cy);
                let dz = fsub(oz, cz);
                let d = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
                if below_eq(best, d) {
                    continue;
                }
                best = d;
                // Stale by design: the last first-loop scope, not this one.
                sel = stale_sel;
                let base = rd32(scope.wrapping_add(SCOPE_MEMBERS));
                member = div40(h.wrapping_sub(base));
            }
            if member == MISS {
                cookie_check(cookie);
                return 0;
            }
        }
        let scope = lf_checker_rt::callee_cdecl!(LOOKUP_SCOPE, u32, sel);
        if scope == 0 {
            cookie_check(cookie);
            return 0;
        }
        let slot = rd32(scope.wrapping_add(SCOPE_MEMBERS))
            .wrapping_add(member.wrapping_mul(MEMBER_STRIDE));
        gw32(RESULT_SLOT, 0);
        gwf(QUERY_POS_X, cx);
        gwf(QUERY_POS_Y, cy);
        gwf(QUERY_POS_Z, cz);
        gwf(QUERY_POS_W, f32::from_bits(rd32(center.wrapping_add(0xc))));
        gw32(QUERY_FLAGS, flags);
        gw32(QUERY_AUX0, aux0);
        gw32(QUERY_AUX1, aux1);
        gwf(QUERY_RAD, rad);
        gwf(QUERY_RAD2, fmul(rad, rad));
        let gain = gf(BOUND_GAIN);
        gw16(BOUND_LO_X, cvtt(fmul(fsub(cx, rad), gain)) as u16);
        gw16(BOUND_LO_Y, cvtt(fmul(fsub(cy, rad), gain)) as u16);
        gw16(BOUND_LO_Z, cvtt(fmul(fsub(cz, rad), gain)) as u16);
        gw16(BOUND_HI_X, cvtt(fmul(fadd(cx, rad), gain)) as u16);
        gw16(BOUND_HI_Y, cvtt(fmul(fadd(cy, rad), gain)) as u16);
        gw16(BOUND_HI_Z, cvtt(fmul(fadd(cz, rad), gain)) as u16);
        gwf(BEST_DIST, f32::from_bits(0x7f7fffff));
        gw32(RESULT_ID12, NO_ID12);
        gw32(RESULT_IDX, MISS);
        lf_checker_rt::callee_thiscall!(
            SETUP_WORKER,
            u32,
            lf_checker_rt::relocated(SETUP_THIS)
        );
        let stamp = rd16(lf_checker_rt::relocated(STAMP));
        (slot.wrapping_add(0xa) as *mut u16).write_unaligned(stamp as u16);
        gw32(WORK_COUNT, 1);
        gw32(WORK_PTR, scope);
        gw32(WORK_SLOT, slot);
        lf_checker_rt::callee_cdecl!(RUN_WORKER, u32);
        if g32(RESULT_IDX) == MISS {
            cookie_check(cookie);
            return 0;
        }
        let rx = f32::from_bits(g32(RESULT_X));
        let ry = f32::from_bits(g32(RESULT_Y));
        let rz = f32::from_bits(g32(RESULT_Z));
        let rw = f32::from_bits(g32(RESULT_W));
        (out as *mut u32).write_unaligned(rx.to_bits());
        (out.wrapping_add(4) as *mut u32).write_unaligned(ry.to_bits());
        (out.wrapping_add(8) as *mut u32).write_unaligned(rz.to_bits());
        (out.wrapping_add(0xc) as *mut u32).write_unaligned(rw.to_bits());
        let rec = g32(RESULT_SLOT);
        let bit = (rd8(rec.wrapping_add(MEMBER_BITS)) >> 6) & 1;
        cookie_check(cookie);
        bit.wrapping_add(1)
    }
});
