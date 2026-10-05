// original: 0x00B3DAF0 bvh_query_emit_sample (proposed)

/// Recursive spatial query over a node tree that emits one blended sample
/// into a 16-slot ring buffer when a member passes every gate.
///
/// Arguments (cdecl, four stack words): `node` is the query root (member
/// array at `+0x6c`, index table at `+0x60`), `list` the candidate list
/// (candidate block at `+0x2c`, four child pointers at `+0x30`), `param` an
/// integer parameter and `point` a pointer to the query floats. Returns 1
/// when the query is accepted (or the ring still has room) and 0 when the
/// ring filled up or a recursive child rejected it.
///
/// With a non-null candidate block the function scans at most `count`
/// entries (16-bit at block `+0x0c`), starting from a random rotation. An
/// entry is taken when its flag word (bit 13 clear), its level (unsigned
/// threshold kept in the globals) and its position (resolved through a
/// callee, then inside the stored radius, inside one of two stored bands
/// and on the kept side of six stored planes) all pass, and when a second
/// callee accepts the position. The accepted entry's sub-range is walked
/// through a third callee, a mixer callee builds a token from it, and an
/// id-mapping callee is polled until its answer changes; two table rows
/// selected by the two ids are blended with a fresh random weight and
/// stored into the next ring slot together with one flag bit, and the ring
/// count is advanced. The return is whether the count stayed below 16; the
/// slot is written even when it did not.
///
/// With a null candidate block the four child indices are shuffled with
/// three random draws (Fisher-Yates over the shrinking tail) and each child
/// whose bounds contain the query point is visited recursively; the first
/// child returning 0 ends the scan with 0, otherwise the result is 1. The
/// first swap of the shuffle lands one word below the index array and
/// overwrites the home slot of `param` with a small index (0 to 3), so any
/// child that passes the first two bound checks then reads through that
/// small integer as a pointer and faults; the rewrite reproduces the store
/// and the fault. The recursive site therefore never fires on a completed
/// trial and is exempt from the call coverage.
///
/// Float order is the original's throughout (see the helpers). The fourth
/// component stored into the ring slot is whatever the frame scratch held
/// (the slot is never written before); the contract pins the stack fill to
/// zero so it reads 0.0. The random callee's answers are constrained to the
/// real callee's range (0 to 32767); larger values would index past the
/// shuffle array in the original. The stack cookie is checked through the
/// intercepted checker callee on every exit; the cookie value passed on
/// cannot reproduce the original's stack-derived mix and is left uncompared.
///
/// Original: 0x00B3DAF0 (cdecl, four stack words; true end one shared
/// epilogue past the inventoried size).
lf_checker_rt::export!(cdecl, rw_00B3DAF0(node: u32, list: u32, param: u32, point: u32) -> u32 {
    unsafe {
        // Global cells (file addresses; read through the relocated image).
        const COOKIE: u32 = 0x1057FB4;
        const LEVEL_LIMIT: u32 = 0x16B7C94;
        const CENTER_X: u32 = 0x16C85A0;
        const CENTER_Y: u32 = 0x16C85A4;
        const CENTER_Z: u32 = 0x16C85A8;
        const BAND_HI: u32 = 0x16B7C88;
        const BAND_LO: u32 = 0x16B7C8C;
        const BAND2_HI: u32 = 0x16B7C90;
        const BAND2_LO: u32 = 0x16B7C84;
        const PLANE_BIAS: u32 = 0x16B7C5C;
        const PLANE_NORMALS: u32 = 0x16B8FD8;
        const PLANE_END: u32 = 0x16B9038;
        const PLANE_STRIDE: u32 = 0x10;
        const EXTRA_TEST_ON: u32 = 0x16B8F74;
        const EXTRA_X: u32 = 0x16C85B0;
        const EXTRA_Y: u32 = 0x16C85B4;
        const EXTRA_Z: u32 = 0x16C85B8;
        const EXTRA_BOUND: u32 = 0x16B8F70;
        const MIX_TABLE: u32 = 0x16C6730;
        const BLEND_TABLE: u32 = 0x16C3CD0;
        const RING_COUNT: u32 = 0x16B7C98;
        const RING_BASE: u32 = 0x16B9DC0;
        const RING_FLAGS: u32 = 0x16B7C9C;
        const RING_CAP: i32 = 16;
        // Read-only float constants.
        const RAND_SCALE: u32 = 0xFE8680;
        const BLEND_SCALE: u32 = 0xFE8684;
        const BLEND_ONE: u32 = 0xFE88E8;
        const PLANE_LIMIT: u32 = 0xFE8DB0;
        const UINT_ADJUST: u32 = 0xFE8F50;
        // Node/member layout.
        const CHILD_BLOCK: u32 = 0x2C;
        const CHILD_LEVEL: u32 = 2;
        const CHILD_COUNT: u32 = 0x0C;
        const CHILD_INDEX: u32 = 4;
        const NODE_MEMBERS: u32 = 0x6C;
        const NODE_SUBS: u32 = 0x60;
        const MEMBER_FLAGS: u32 = 0;
        const MEMBER_LEVEL: u32 = 4;
        const MEMBER_BITS: u32 = 0x1C;
        const MEMBER_STRIDE: u32 = 40;
        const CHILD_PTRS: u32 = 0x30;
        // Intercepted callees.
        const RNG: u32 = 1;
        const RESOLVE_POS: u32 = 2;
        const ACCEPT_POS: u32 = 3;
        const WALK_SUB: u32 = 4;
        const MIX_TOKEN: u32 = 5;
        const MAP_ID: u32 = 6;
        const COOKIE_CHECK: u32 = 7;
        const RECURSE: u32 = 8;

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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u32) {
            unsafe { (a as *mut u8).write(v as u8) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
        }
        #[inline(always)]
        unsafe fn gf64(va: u32) -> f64 {
            unsafe {
                f64::from_bits(
                    (lf_checker_rt::relocated(va) as *const u64).read_unaligned(),
                )
            }
        }
        #[inline(always)]
        unsafe fn gw32(va: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(va), v) }
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
        /// Truncate toward zero with x86 `cvttss2si` semantics: NaN and
        /// out-of-range results give `i32::MIN` instead of saturating.
        #[inline(always)]
        fn cvtt(v: f32) -> i32 {
            if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
                i32::MIN
            } else {
                v as i32
            }
        }
        /// Unsigned integer to float exactly as the original's
        /// `cvtdq2pd`/`addsd`/`cvtpd2ps` sequence (the sign bit selects the
        /// 0.0 or 2^32 table entry).
        #[inline(always)]
        unsafe fn u32_to_f32(v: u32) -> f32 {
            unsafe {
                let adj = gf64(UINT_ADJUST.wrapping_add((v >> 31).wrapping_mul(8)));
                ((v as i32 as f64) + adj) as f32
            }
        }
        // `comiss` outcomes: `above` is strict and false for NaN, `below`
        // and `below_eq` are true for NaN.
        #[inline(always)]
        fn above(a: f32, b: f32) -> bool {
            core::hint::black_box(a) > core::hint::black_box(b)
        }
        #[inline(always)]
        fn below(a: f32, b: f32) -> bool {
            !(core::hint::black_box(a) >= core::hint::black_box(b))
        }
        #[inline(always)]
        fn below_eq(a: f32, b: f32) -> bool {
            !(core::hint::black_box(a) > core::hint::black_box(b))
        }
        #[inline(always)]
        unsafe fn cookie_check(cookie: u32) {
            unsafe {
                // The original passes the cookie mixed with its stack
                // pointer, which the rewrite cannot reproduce; entry ECX is
                // left uncompared and only the call itself is observed.
                lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, cookie);
            }
        }

        let cookie = g32(COOKIE);
        let child = rd32(list.wrapping_add(CHILD_BLOCK));
        if child == 0 {
            // Leaf: shuffle the four child indices, then visit in order.
            // The random answers stay within the real callee's range
            // (0 to 32767), which keeps every swap index inside the array.
            let mut param_home = param;
            let mut idx = [0u32, 1u32, 2u32, 3u32];
            let mut tail = 4i32;
            let mut i = 0usize;
            while i < 4 {
                let r = lf_checker_rt::callee_cdecl!(RNG, u32) & 0xffff;
                let f = fmul(fmul(r as f32, gf(RAND_SCALE)), tail as f32);
                tail -= 1;
                let j = i.wrapping_add(cvtt(f) as usize);
                let a = idx[j];
                // The first swap lands one word below the array, on the home
                // slot of `param`; later ones rotate the array itself.
                if i == 0 {
                    param_home = a;
                } else {
                    idx[i - 1] = a;
                }
                idx[j] = idx[i];
                i += 1;
            }
            let mut k = 0usize;
            while k < 4 {
                let e = rd32(
                    list.wrapping_add(idx[k].wrapping_mul(4).wrapping_add(CHILD_PTRS)),
                );
                let px = f32::from_bits(rd32(point));
                if below(px, f32::from_bits(rd32(e))) {
                    k += 1;
                    continue;
                }
                let py = f32::from_bits(rd32(point.wrapping_add(4)));
                if below(py, f32::from_bits(rd32(e.wrapping_add(4)))) {
                    k += 1;
                    continue;
                }
                // `param_home` now holds a small shuffle index, read back as
                // the query pointer the original passes on; the read faults
                // exactly when the original's does.
                let q = param_home;
                let fx = f32::from_bits(rd32(e.wrapping_add(0x10)));
                if below(fx, f32::from_bits(rd32(q))) {
                    k += 1;
                    continue;
                }
                let fy = f32::from_bits(rd32(e.wrapping_add(0x14)));
                if below(fy, f32::from_bits(rd32(q.wrapping_add(4)))) {
                    k += 1;
                    continue;
                }
                let r = lf_checker_rt::callee_cdecl!(RECURSE, u32, node, e, q, point);
                if r & 0xff == 0 {
                    cookie_check(cookie);
                    return 0;
                }
                k += 1;
            }
            cookie_check(cookie);
            return 1;
        }

        let count = rd16(child.wrapping_add(CHILD_COUNT));
        if count == 0 {
            cookie_check(cookie);
            return 1;
        }
        if rd8(child.wrapping_add(CHILD_LEVEL)) < g32(LEVEL_LIMIT) {
            cookie_check(cookie);
            return 1;
        }
        let r = lf_checker_rt::callee_cdecl!(RNG, u32) & 0xffff;
        let mut pick = cvtt(fmul(fmul(r as f32, gf(RAND_SCALE)), count as f32));
        let mut left = count;
        // Countdown scan over the candidate entries.
        loop {
            let cur = left;
            left = left.wrapping_sub(1);
            if cur == 0 {
                cookie_check(cookie);
                return 1;
            }
            pick = pick.wrapping_add(1);
            let blk = rd32(list.wrapping_add(CHILD_BLOCK));
            let tab = rd32(blk.wrapping_add(CHILD_INDEX));
            let di = rd16(
                tab.wrapping_add((pick as u32).wrapping_mul(2).wrapping_sub(2)),
            );
            if (pick as u32) >= rd16(blk.wrapping_add(CHILD_COUNT)) {
                pick = 0;
            }
            let members = rd32(node.wrapping_add(NODE_MEMBERS));
            let member = members.wrapping_add(di.wrapping_mul(MEMBER_STRIDE));
            if (rd32(member.wrapping_add(MEMBER_FLAGS)) >> 13) & 1 != 0 {
                continue;
            }
            let level = rd32(member.wrapping_add(MEMBER_LEVEL)) >> 29;
            if above(u32_to_f32(g32(LEVEL_LIMIT)), level as f32) {
                continue;
            }
            let mut pos = [0u32; 3];
            lf_checker_rt::callee_stdcall!(
                RESOLVE_POS,
                u32,
                member,
                pos.as_mut_ptr() as u32
            );
            let px = f32::from_bits(pos[0]);
            let py = f32::from_bits(pos[1]);
            let pz = f32::from_bits(pos[2]);
            let dx = fsub(px, gf(CENTER_X));
            let dy = fsub(py, gf(CENTER_Y));
            let dz = fsub(pz, gf(CENTER_Z));
            let dist = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
            let band_hi = gf(BAND_HI);
            if above(dist, band_hi) {
                continue;
            }
            let in_band = !below(dist, gf(BAND_LO)) && !below(gf(BAND2_HI), dist);
            let in_band2 = !below(dist, gf(BAND2_LO)) && !below(band_hi, dist);
            if !in_band && !in_band2 {
                continue;
            }
            let mut planes_ok = 1u32;
            let mut va = PLANE_NORMALS;
            let mut pa = PLANE_BIAS;
            loop {
                let nx = gf(va.wrapping_sub(8));
                let ny = gf(va.wrapping_sub(4));
                let nz = gf(va);
                let mut d = fadd(fmul(ny, py), fmul(nx, px));
                d = fadd(d, fmul(nz, pz));
                d = fadd(d, gf(pa));
                if above(gf(PLANE_LIMIT), d) {
                    planes_ok = 0;
                    break;
                }
                va = va.wrapping_add(PLANE_STRIDE);
                pa = pa.wrapping_add(4);
                if va >= PLANE_END {
                    break;
                }
            }
            let proceed = if in_band2 {
                planes_ok != 0 || in_band
            } else {
                in_band && planes_ok == 0
            };
            if !proceed {
                continue;
            }
            if rd8(EXTRA_TEST_ON) != 0 {
                let ex = fsub(gf(EXTRA_X), px);
                let ey = fsub(gf(EXTRA_Y), py);
                let ez = fsub(gf(EXTRA_Z), pz);
                let edist =
                    fadd(fadd(fmul(ey, ey), fmul(ex, ex)), fmul(ez, ez));
                if below_eq(gf(EXTRA_BOUND), edist) {
                    continue;
                }
            }
            let ok =
                lf_checker_rt::callee_cdecl!(ACCEPT_POS, u32, pos.as_mut_ptr() as u32);
            if ok & 0xff != 0 {
                continue;
            }
            // Accepted: walk the sub-range, mix a token, blend a sample.
            let sub_count = (rd32(member.wrapping_add(MEMBER_FLAGS)) >> 0x15) & 0xf;
            let mut ti = 0u32;
            while ti < sub_count {
                let tp = lf_checker_rt::relocated(MIX_TABLE)
                    .wrapping_add(ti.wrapping_mul(PLANE_STRIDE));
                let c = (rd32(member.wrapping_add(MEMBER_LEVEL)) & 0x1ffff)
                    .wrapping_add(ti);
                let subs = rd32(node.wrapping_add(NODE_SUBS));
                let w = rd16(subs.wrapping_add(c.wrapping_mul(2)));
                lf_checker_rt::callee_thiscall!(WALK_SUB, u32, node, w, tp);
                ti += 1;
            }
            let token = lf_checker_rt::callee_cdecl!(
                MIX_TOKEN,
                u32,
                member,
                lf_checker_rt::relocated(MIX_TABLE),
                0,
                0,
                1,
                lf_checker_rt::relocated(BLEND_TABLE)
            );
            let first = lf_checker_rt::callee_cdecl!(MAP_ID, u32, 0, token);
            let mut retry = 10u32;
            let second = loop {
                let v = lf_checker_rt::callee_cdecl!(MAP_ID, u32, 0, token);
                if v != first {
                    break v;
                }
                let c = retry;
                retry = retry.wrapping_sub(1);
                if c == 0 {
                    break v;
                }
            };
            let rb = lf_checker_rt::callee_cdecl!(RNG, u32);
            let wf = fmul((rb as i32) as f32, gf(BLEND_SCALE));
            let t = fsub(gf(BLEND_ONE), wf);
            let srow =
                lf_checker_rt::relocated(BLEND_TABLE).wrapping_add(second.wrapping_shl(4));
            let s0 = f32::from_bits(rd32(srow));
            let s1 = f32::from_bits(rd32(srow.wrapping_add(4)));
            let s2 = f32::from_bits(rd32(srow.wrapping_add(8)));
            let frow =
                lf_checker_rt::relocated(BLEND_TABLE).wrapping_add(first.wrapping_shl(4));
            let f0 = f32::from_bits(rd32(frow));
            let f1 = f32::from_bits(rd32(frow.wrapping_add(4)));
            let f2 = f32::from_bits(rd32(frow.wrapping_add(8)));
            let b0 = fadd(fmul(f0, wf), fmul(t, s0));
            let b1 = fadd(fmul(f1, wf), fmul(s1, t));
            let b2 = fadd(fmul(f2, wf), fmul(s2, t));
            // The stored fourth component is never written before this read;
            // the contract pins the stack fill to zero so it reads 0.0.
            let slot_n = g32(RING_COUNT);
            let slot = lf_checker_rt::relocated(RING_BASE)
                .wrapping_add(slot_n.wrapping_shl(4));
            wr32(slot.wrapping_add(0xc), 0);
            wr32(slot, b0.to_bits());
            wr32(slot.wrapping_add(4), b1.to_bits());
            wr32(slot.wrapping_add(8), b2.to_bits());
            let flag = (rd8(member.wrapping_add(MEMBER_BITS)) >> 6) & 1;
            wr8(lf_checker_rt::relocated(RING_FLAGS).wrapping_add(slot_n), flag);
            let next = slot_n.wrapping_add(1);
            gw32(RING_COUNT, next);
            cookie_check(cookie);
            return ((next as i32) < RING_CAP) as u32;
        }
    }
});
