// original: 0x00D45FB0 SURPRISED (symbols)

/// React with surprise: gather candidate triggers, pick one, and respond.
///
/// `this` is the reaction task (`+0x50` base mood, `+0x5c` done flag,
/// `+0x34`/`+0x3c` result slots); `ped` is the ped; the third word's low byte
/// chooses best-pick over random-pick. Returns 1 (low byte) on every handled
/// path and 0 only when nothing was gathered; upper bytes keep stale eax.
///
/// Behaviour: done tasks return at once. Otherwise a fast path queries the
/// ped's slot-52 helper five times: when every answer validates, one
/// synthetic entry (score 3.0) stands for the gathering. Failing that, two
/// tables of sixteen pointers off the ped are scored through the scorer
/// helper; scores strictly above `3.0 - mood` are kept with their pointers
/// (at most 32), tracking the running total and the best index. Best-pick
/// takes the best; random-pick draws a threshold of
/// `float(rand) * K * total` and takes the first entry whose running sum
/// exceeds it (or the last). The picked pointer is published to `+0x3c`,
/// announced through the link helper, and `+0x34` receives a scaled random
/// intensity. A style code other than 0 or 2 issues response one and returns.
/// The fallback verifies the pick (kind bits, anchor within 36 units) and
/// then either consults the task's slot-16 helper plus model bits (responses
/// two or three) or the two models' kind rows (response four, response five,
/// or a quiet return). Float operation order is the original's.
///
/// Original: 0x00D45FB0 (thiscall, two stack words, returns u32, al significant).
lf_checker_rt::export!(thiscall, rw_00D45FB0(this: u32, ped: u32, mode: u32) -> u32 {
    unsafe {
        const MOOD: u32 = 0x50;
        const DONE: u32 = 0x5c;
        const OUT_INTENSITY: u32 = 0x34;
        const OUT_PICK: u32 = 0x3c;
        const PED_VT: u32 = 0xd0;
        const PED_ARRAYS: u32 = 0x224;
        const ARR1_OFF: u32 = 0x168;
        const ARR2_OFF: u32 = 0x10c;
        const PED_DIST: u32 = 0x20;
        const PED_212: u32 = 0x21c;
        const PED_MODEL: u32 = 0x2e;
        const PED_REQ: u32 = 0x570;
        const KIND_TABLE: u32 = 0x0129_5CD8;
        const MAX_CANDS: usize = 32;
        const BASE_MOOD: f32 = f32::from_bits(0x4040_0000);
        const SYN_SCORE: f32 = f32::from_bits(0x4040_0000);
        const PICK_K: f32 = f32::from_bits(0x3800_0100);
        const HALF: f32 = f32::from_bits(0x3F00_0000);
        const ONE: f32 = f32::from_bits(0x3F80_0000);
        const GROW: f32 = f32::from_bits(0x3FCC_CCCD);
        const BIAS: f32 = f32::from_bits(0x3F66_6666);
        const MOOD_K: f32 = f32::from_bits(0x3F4C_CCCD);
        const NEAR2: f32 = f32::from_bits(0x4210_0000);
        const STR1: u32 = 0x00EE_3F98;
        const STR2: u32 = 0x00EE_3FA4;
        const STR2B: u32 = 0x00EE_3FAC;
        const STR3: u32 = 0x00EE_3FBC;
        const STR3B: u32 = 0x00EE_3FC8;
        const STR4: u32 = 0x00EE_3FD8;
        const STR5: u32 = 0x00EE_3FE0;
        const VT_CALLEE: u32 = 1;
        const LINKA_CALLEE: u32 = 2;
        const STYLE_CALLEE: u32 = 3;
        const SCORE_CALLEE: u32 = 4;
        const RAND_CALLEE: u32 = 5;
        const LINKB_CALLEE: u32 = 6;
        const RESP_CALLEE: u32 = 7;
        const LOOKUP_CALLEE: u32 = 8;
        const TASKQ_CALLEE: u32 = 9;
        const COOKIE_CALLEE: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        unsafe fn cookie() {
            unsafe { lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,) };
        }
        #[inline(always)]
        unsafe fn ped_slot(ped: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(ped).wrapping_add(PED_VT));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(ped)
            }
        }
        #[inline(always)]
        unsafe fn respond(req_this: u32, style: u32, a5: u32, a6: u32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    RESP_CALLEE, u32, req_this, style, 0u32, 0u32, 0u32,
                    0xFFFF_FFFFu32, a5, a6, 0x3F80_0000u32, 0u32, 0u32
                );
            }
        }
        #[inline(always)]
        unsafe fn kind_row(obj: u32) -> u32 {
            unsafe {
                rd32(
                    (lf_checker_rt::global::<u32>(KIND_TABLE) as u32).wrapping_add(
                        (rd16(obj.wrapping_add(PED_MODEL)) as i16 as i32 as u32)
                            .wrapping_mul(4),
                    ),
                )
            }
        }

        if rd8(this.wrapping_add(DONE)) & 2 != 0 {
            cookie();
            return 1;
        }
        let limit = sub(BASE_MOOD, rdf(this.wrapping_add(MOOD)));
        let mut scores = [0.0f32; MAX_CANDS];
        let mut ptrs = [0u32; MAX_CANDS];
        let mut total = 0.0f32;
        let mut best = 0u32;
        let mut n = 0u32;
        let mut style = 0u32;
        // Fast path: five slot queries must all validate.
        let v1 = ped_slot(ped);
        let mut gathered = false;
        if v1 != 0 {
            let v2 = ped_slot(ped);
            if rd8(v2.wrapping_add(0x17c)) != 0 {
                let c1 = lf_checker_rt::callee_thiscall!(LINKA_CALLEE, u32, ped_slot(ped));
                if c1 != 0 {
                    let c2 = lf_checker_rt::callee_thiscall!(LINKA_CALLEE, u32, ped_slot(ped));
                    ptrs[0] = c2;
                    total = SYN_SCORE;
                    scores[0] = SYN_SCORE;
                    style = lf_checker_rt::callee_thiscall!(STYLE_CALLEE, u32, ped_slot(ped));
                    n = 1;
                    gathered = true;
                }
            }
        }
        if !gathered {
            best = 0;
            let base = rd32(ped.wrapping_add(PED_ARRAYS));
            for pass in 0..2u32 {
                for i in 0..16u32 {
                    if n >= MAX_CANDS as u32 {
                        break;
                    }
                    let arrb = base.wrapping_add(if pass == 0 { ARR1_OFF } else { ARR2_OFF });
                    let p = rd32(arrb.wrapping_add(i.wrapping_mul(4)));
                    if p == 0 {
                        continue;
                    }
                    let s: f32 =
                        lf_checker_rt::callee_cdecl!(SCORE_CALLEE, f32, ped, p);
                    if !(s > limit) {
                        continue;
                    }
                    scores[n as usize] = s;
                    if s > scores[best as usize] {
                        best = n;
                    }
                    ptrs[n as usize] = p;
                    total = add(s, total);
                    n += 1;
                }
                if n >= MAX_CANDS as u32 {
                    break;
                }
            }
        }
        if (n as i32) <= 0 {
            cookie();
            return 0;
        }
        let idx = if (mode as u8) != 0 {
            best
        } else {
            let rand = lf_checker_rt::callee_cdecl!(RAND_CALLEE, u32,);
            let threshold = mul(mul((rand as i32) as f32, PICK_K), total);
            let mut running = 0.0f32;
            let mut pick = 0u32;
            let mut i = 0u32;
            while i < n {
                let c = add(scores[i as usize], running);
                running = c;
                if c > threshold || i + 1 >= n {
                    pick = i;
                    break;
                }
                i += 1;
            }
            pick
        };
        let pick_ptr = ptrs[idx as usize];
        let out = this.wrapping_add(OUT_PICK);
        wr32(out, pick_ptr);
        lf_checker_rt::callee_thiscall!(LINKB_CALLEE, u32, pick_ptr, out);
        let grown = mul(scores[idx as usize], HALF);
        let scale = if ONE > grown { ONE } else { grown };
        let rand2 = lf_checker_rt::callee_cdecl!(RAND_CALLEE, u32,);
        let intensity = mul(
            mul(
                add(mul(mul((rand2 as i32) as f32, PICK_K), GROW), BIAS),
                MOOD_K,
            ),
            scale,
        );
        wrf(this.wrapping_add(OUT_INTENSITY), intensity);
        if style != 0 && style != 2 {
            respond(ped.wrapping_add(PED_REQ), lf_checker_rt::relocated(STR1), 0, 0);
            cookie();
            return 1;
        }
        // Fallback on the published pick.
        let cand = rd32(this.wrapping_add(OUT_PICK));
        if rd32(cand.wrapping_add(0x28)) & 0x3c0 != 0xc0 {
            cookie();
            return 1;
        }
        let at = rd32(cand.wrapping_add(0x20));
        let anchor = if at != 0 { at.wrapping_add(0x30) } else { cand.wrapping_add(0x10) };
        let dist = rd32(ped.wrapping_add(PED_DIST));
        let dx = sub(rdf(dist.wrapping_add(0x30)), rdf(anchor));
        let dy = sub(rdf(dist.wrapping_add(0x34)), rdf(anchor.wrapping_add(4)));
        let dz = sub(rdf(dist.wrapping_add(0x38)), rdf(anchor.wrapping_add(8)));
        let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        if !(NEAR2 > dist2) {
            cookie();
            return 1;
        }
        let e212 = rd32(rd32(ped.wrapping_add(PED_212)).wrapping_add(0x12c));
        let s212 = rd32(rd32(cand.wrapping_add(PED_212)).wrapping_add(0x12c));
        if e212 == 2 && s212 != 2 {
            let slot = rd32(rd32(this).wrapping_add(0x40));
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            if f(this, 0x390) != 0 {
                if rd32(kind_row(cand).wrapping_add(0x124)) & 0x10 != 0 {
                    respond(
                        ped.wrapping_add(PED_REQ),
                        lf_checker_rt::relocated(STR2),
                        0,
                        0,
                    );
                    cookie();
                    return 1;
                }
                let look = lf_checker_rt::callee_cdecl!(
                    LOOKUP_CALLEE, u32,
                    lf_checker_rt::relocated(STR2B),
                    0u32
                );
                respond(
                    ped.wrapping_add(PED_REQ),
                    lf_checker_rt::relocated(STR3),
                    cand,
                    look,
                );
                cookie();
                return 1;
            }
        }
        let erow = kind_row(ped);
        if rd32(erow.wrapping_add(0x120)) & 2 != 0 {
            let srow = kind_row(cand);
            if rd32(srow.wrapping_add(0x120)) & 2 == 0
                && !(rd8(erow.wrapping_add(0xed)) > rd8(srow.wrapping_add(0xed)))
            {
                let look = lf_checker_rt::callee_cdecl!(
                    LOOKUP_CALLEE, u32,
                    lf_checker_rt::relocated(STR3B),
                    0u32
                );
                respond(
                    ped.wrapping_add(PED_REQ),
                    lf_checker_rt::relocated(STR4),
                    cand,
                    look,
                );
                cookie();
                return 1;
            }
        }
        if rd32(erow.wrapping_add(0x124)) & 0x10 != 0 {
            cookie();
            return 1;
        }
        if rd32(kind_row(cand).wrapping_add(0x124)) & 0x10 == 0 {
            cookie();
            return 1;
        }
        respond(ped.wrapping_add(PED_REQ), lf_checker_rt::relocated(STR5), 0, 0);
        cookie();
        1
    }
});
