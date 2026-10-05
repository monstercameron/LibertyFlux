// original: 0x00b30400 ped_task_match_and Adopt (proposed)

/// Scan the 32-entry global slot table for a record matching `kind`.
///
/// Each record is `RECORD_STRIDE` bytes starting at `TABLE_BASE`, headed by
/// a signed tag. Records with a non-positive tag are skipped. A record is a
/// candidate when its tag pairs with `kind`: kinds 3-11 pair with tags
/// 3-11, and pairs (0x1B, 0x1D), (0x18, 0x19), (0x11, 0x12) match
/// cross-wise; any other combination requires tag equality. The first
/// candidate runs one assimilation by kind and the function returns 1; with
/// no candidate it returns 0. Only the low result byte is significant.
///
/// Kinds 0x1B/0x18/0x11 blend `point` into the record's running average at
/// `+AVG_{X,Y,Z}` (weight `1/(count+1)`, `count` the u16 at `+COUNT`, which
/// 0x1B and 0x18 increment first) when the squared 2D distance clears a gate
/// (compared against `GATE_FAR`/`GATE_NEAR`, else against the square of the
/// radius at `+RADIUS`); a high count then promotes the tag (to 0x1D/0x19/
/// 0x12) with a radius bump. Kinds 0x1E/0x1A average with weight 0.5 after an
/// id check. Any other kind calls the settle callee when `key_a` or (nonzero)
/// `key_b` equals one of the record's ids at `+ID0`/`+ID1`, then claims
/// records tagged 3-11 below `kind` with a global sequence number. The
/// settle/finalise callee pair runs before every return of 1. Float gates
/// keep the original's unordered (NaN) behaviour; the slot at `+0x1C` is
/// filled from an uninitialised stack word, which the proof defines as zero.
///
/// Original: cdecl, five stack words; loop-carried blend registers are dead
/// after the blend (it always returns) but updated faithfully.
lf_checker_rt::export!(cdecl, rw_00b30400(kind: u32, stamp: u32, point: u32, key_a: u32, key_b: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x0166_1A60;
        const TABLE_END: u32 = 0x0166_2474;
        const RECORD_STRIDE: u32 = 0x50;
        const REC_OFFSET: u32 = 0x14;
        const AVG_X: u32 = 0x10;
        const AVG_Y: u32 = 0x14;
        const AVG_Z: u32 = 0x18;
        const AVG_W: u32 = 0x1C;
        const RADIUS: u32 = 0x20;
        const COUNT: u32 = 0x24;
        const SEEN: u32 = 0x27;
        const ID0: u32 = 0x28;
        const ID1: u32 = 0x2C;
        const SEQ_SLOT: u32 = 0x44;
        const GATE_FAR: u32 = 0x00FE_8C20; // 400.0
        const GATE_NEAR: u32 = 0x00FE_8BB0; // 100.0
        const ONE_AT: u32 = 0x00FE_88E8; // 1.0
        const BUMP_BIG_AT: u32 = 0x00FE_8B08; // 10.0
        const BUMP_SMALL_AT: u32 = 0x00FE_8AD8; // 5.0
        const HALF_AT: u32 = 0x00FE_8830; // 0.5
        const LIMIT_GLOBAL: u32 = 0x0166_19FC;
        const TICK_GLOBAL: u32 = 0x0117_35B4;
        const SEQ_GLOBAL: u32 = 0x0166_1404;
        const PROMOTE_AT: u16 = 10;
        const PROMOTE_AT_FAST: u16 = 3;
        const CALLEE_REFRESH: u32 = 1;
        const CALLEE_SETTLE: u32 = 2;
        const CALLEE_FINALISE: u32 = 3;
        // The original copies an uninitialised stack word into AVG_W; the
        // contract zeroes scratch (stack_fill 0), so it reads 0.
        const STALE: f32 = 0.0;

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
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { (a as *const f32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut f32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn settle(rec: u32, first: u32) {
            unsafe {
                let tick = rd32(lf_checker_rt::relocated(TICK_GLOBAL));
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_SETTLE,
                    u32,
                    lf_checker_rt::relocated(rec),
                    first,
                    tick
                );
            }
        }
        #[inline(always)]
        unsafe fn finalise(rec: u32) {
            unsafe {
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    CALLEE_FINALISE,
                    u32,
                    lf_checker_rt::relocated(rec)
                );
            }
        }

        let one = rdf(lf_checker_rt::relocated(ONE_AT));
        let mut gate_far = rdf(lf_checker_rt::relocated(GATE_FAR));
        let mut gate_near = rdf(lf_checker_rt::relocated(GATE_NEAR));
        let mut cursor = TABLE_BASE.wrapping_add(REC_OFFSET);
        loop {
            let rec = cursor.wrapping_sub(REC_OFFSET);
            let here = lf_checker_rt::relocated(rec);
            let tag = rd32(here) as i32;
            if tag <= 0 {
                cursor = cursor.wrapping_add(RECORD_STRIDE);
                if (cursor as i32) < (TABLE_END as i32) {
                    continue;
                }
                return 0;
            }
            let tag_u = tag as u32;
            let paired = if kind.wrapping_sub(3) <= 8 {
                tag >= 3 && tag <= 0xB
            } else if kind == 0x1B {
                tag_u == 0x1D
            } else if kind == 0x18 {
                tag_u == 0x19
            } else if kind == 0x11 {
                tag_u == 0x12
            } else {
                false
            };
            if !paired && tag_u != kind {
                cursor = cursor.wrapping_add(RECORD_STRIDE);
                if (cursor as i32) < (TABLE_END as i32) {
                    continue;
                }
                return 0;
            }
            // Squared 2D distance between the record point and `point`.
            let dx = sub(rdf(here.wrapping_add(AVG_X)), rdf(point));
            let dy = sub(
                rdf(here.wrapping_add(AVG_Y)),
                rdf(point.wrapping_add(4)),
            );
            let dist = add(mul(dy, dy), mul(dx, dx));
            if kind == 0x1B || kind == 0x18 || kind == 0x11 {
                let wide = kind == 0x1B;
                let gate = if wide { gate_far } else { gate_near };
                // Ordered strict (0x1B/0x18) or non-strict (0x11) pass.
                let mut passed = if wide || kind == 0x18 {
                    gate > dist
                } else {
                    gate >= dist
                };
                if !passed {
                    let r = rdf(here.wrapping_add(RADIUS));
                    passed = mul(r, r) > dist;
                }
                if !passed {
                    cursor = cursor.wrapping_add(RECORD_STRIDE);
                    if (cursor as i32) < (TABLE_END as i32) {
                        continue;
                    }
                    return 0;
                }
                let count = if kind == 0x11 {
                    (here.wrapping_add(COUNT) as *const u16).read_unaligned()
                } else {
                    let p = here.wrapping_add(COUNT) as *mut u16;
                    let c = p.read_unaligned().wrapping_add(1);
                    p.write_unaligned(c);
                    c
                };
                let n = ((count as u32).wrapping_add(1) as i32) as f32;
                let f = div(one, n);
                let g = sub(one, f);
                let nx = add(
                    mul(rdf(point), f),
                    mul(g, rdf(here.wrapping_add(AVG_X))),
                );
                let ny = add(
                    mul(rdf(point.wrapping_add(4)), f),
                    mul(g, rdf(here.wrapping_add(AVG_Y))),
                );
                let nz = add(
                    mul(rdf(point.wrapping_add(8)), f),
                    mul(g, rdf(here.wrapping_add(AVG_Z))),
                );
                wrf(here.wrapping_add(AVG_X), nx);
                wrf(here.wrapping_add(AVG_Y), ny);
                wrf(here.wrapping_add(AVG_Z), nz);
                wrf(here.wrapping_add(AVG_W), STALE);
                // Dead but faithful: the blend leaves these behind.
                gate_near = nz;
                gate_far = g;
                if kind == 0x1B {
                    if count <= PROMOTE_AT {
                        settle(rec, stamp);
                        finalise(rec);
                        return 1;
                    }
                    if tag_u == 0x1B {
                        let bump =
                            rdf(lf_checker_rt::relocated(BUMP_BIG_AT));
                        let r = add(rdf(here.wrapping_add(RADIUS)), bump);
                        wrf(here.wrapping_add(RADIUS), r);
                    }
                    let mut lim =
                        rd32(lf_checker_rt::relocated(LIMIT_GLOBAL));
                    if stamp > lim {
                        lim = stamp;
                    }
                    (here.wrapping_add(SEEN) as *mut u8).write(1);
                    (here as *mut u32).write_unaligned(0x1D);
                    settle(rec, lim);
                    finalise(rec);
                    return 1;
                }
                let need = if kind == 0x18 {
                    PROMOTE_AT_FAST
                } else {
                    PROMOTE_AT
                };
                if count > need
                    && key_a != rd32(here.wrapping_add(ID0))
                {
                    if (kind == 0x18 && tag_u == 0x18)
                        || (kind == 0x11 && tag_u == 0x11)
                    {
                        let at = if kind == 0x18 {
                            BUMP_BIG_AT
                        } else {
                            BUMP_SMALL_AT
                        };
                        let bump = rdf(lf_checker_rt::relocated(at));
                        let r = add(rdf(here.wrapping_add(RADIUS)), bump);
                        wrf(here.wrapping_add(RADIUS), r);
                    }
                    let next = if kind == 0x18 { 0x19 } else { 0x12 };
                    (here as *mut u32).write_unaligned(next);
                }
                let id0 = rd32(here.wrapping_add(ID0));
                if id0 != 0 && key_a != id0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CALLEE_REFRESH,
                        u32,
                        lf_checker_rt::relocated(rec)
                    );
                }
                let rr = rdf(here.wrapping_add(RADIUS));
                if dist > mul(rr, rr) {
                    wrf(here.wrapping_add(RADIUS), dist.sqrt());
                }
                settle(rec, stamp);
                finalise(rec);
                return 1;
            }
            if kind == 0x1E || kind == 0x1A {
                if key_a != 0 {
                    if key_a != rd32(here.wrapping_add(ID0)) {
                        cursor = cursor.wrapping_add(RECORD_STRIDE);
                        if (cursor as i32) < (TABLE_END as i32) {
                            continue;
                        }
                        return 0;
                    }
                } else {
                    if rd32(here.wrapping_add(ID0)) != 0 {
                        cursor = cursor.wrapping_add(RECORD_STRIDE);
                        if (cursor as i32) < (TABLE_END as i32) {
                            continue;
                        }
                        return 0;
                    }
                    if !(gate_near >= dist) {
                        cursor = cursor.wrapping_add(RECORD_STRIDE);
                        if (cursor as i32) < (TABLE_END as i32) {
                            continue;
                        }
                        return 0;
                    }
                }
                settle(rec, stamp);
                let half = rdf(lf_checker_rt::relocated(HALF_AT));
                let nx = mul(
                    add(rdf(here.wrapping_add(AVG_X)), rdf(point)),
                    half,
                );
                let ny = mul(
                    add(rdf(point.wrapping_add(4)), rdf(here.wrapping_add(AVG_Y))),
                    half,
                );
                let nz = mul(
                    add(rdf(point.wrapping_add(8)), rdf(here.wrapping_add(AVG_Z))),
                    half,
                );
                wrf(here.wrapping_add(AVG_X), nx);
                wrf(here.wrapping_add(AVG_Z), nz);
                wrf(here.wrapping_add(AVG_W), STALE);
                wrf(here.wrapping_add(AVG_Y), ny);
                gate_near = ny;
                gate_far = nx;
                let rr = rdf(here.wrapping_add(RADIUS));
                if dist > mul(rr, rr) {
                    wrf(here.wrapping_add(RADIUS), dist.sqrt());
                }
                finalise(rec);
                return 1;
            }
            let hit = key_a == rd32(here.wrapping_add(ID0))
                || key_a == rd32(here.wrapping_add(ID1))
                || (key_b != 0
                    && (key_b == rd32(here.wrapping_add(ID0))
                        || key_b == rd32(here.wrapping_add(ID1))));
            if !hit {
                cursor = cursor.wrapping_add(RECORD_STRIDE);
                if (cursor as i32) < (TABLE_END as i32) {
                    continue;
                }
                return 0;
            }
            settle(rec, stamp);
            if tag >= 3 && tag <= 0xB && (kind as i32) > tag {
                let seq =
                    rd32(lf_checker_rt::relocated(SEQ_GLOBAL)).wrapping_add(1);
                (lf_checker_rt::relocated(SEQ_GLOBAL) as *mut u32)
                    .write_unaligned(seq);
                (here.wrapping_add(SEQ_SLOT) as *mut u32).write_unaligned(seq);
                (here as *mut u32).write_unaligned(kind);
            }
            return 1;
        }
    }
});
