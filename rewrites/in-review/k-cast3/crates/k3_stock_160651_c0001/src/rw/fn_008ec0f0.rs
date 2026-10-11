// original: 0x008ec0f0 cached_proximity_search_emit (proposed)

/// Proximity search over packed point records with a cached-query fast path,
/// keeping the best hits in a global table and emitting them one per call.
///
/// `handle` owns record-array bases at `+0x804` and SIGNED counts at `+0xb04`
/// (64 slots). The query is `*query` (four floats, xyz used), `radius` and an
/// id in `player`. When every cached-state check passes (id and key globals
/// match, stored xyz/radius equal the query bit-for-bit with -0 equal to +0,
/// the table holds un-emitted rows, and the 32-byte signature at `handle +
/// 0x1bc4` matches the stored one through callee 1), the scan is skipped and
/// the next table row is emitted at once.
///
/// Otherwise the query is stored to the globals, two cluster calls (callee 2,
/// stdcall of player, flag, out-vector; only the first answer matters, and
/// only with bit 2 of `flags`: it replaces the scan centre) seed the scan,
/// and every record of every live slot is tested: flag byte `& 0xf0 == 0x80`,
/// a signature byte selected by another record byte must be nonzero, and the
/// 2-D distance from the centre must sit strictly below `radius`. Survivors
/// are scored by callee 3 (stdcall of three frame vectors, the query, id,
/// flags and the clamped float; f32 answer on x87) and appended as
/// (id, score) rows while the table holds fewer than 64; past that a nonzero
/// `flags` word replaces the weakest row and re-scans the minimum, while a
/// zero word does a random replacement through the rand callee (callee 5).
/// A zero `flags` word then shuffles the table 128 times, a nonzero word
/// sorts it through callee 4 and shuffles within score clusters. The emit
/// tail writes the decoded row point (three scaled words) to `*out_vec` and
/// its id word to `*out_id`, advancing the emit cursor, and returns 1; with
/// nothing left to emit it stores -1 to `*out_id` and returns 0.
///
/// The float argument is clamped to a minimum of 1.0 in the caller's own
/// stack slot (NaN clamps too), which the rewrite cannot write back, so the
/// stack comparison stays off and the clamped value is observed as callee
/// 3's float argument instead.
///
/// Original: 0x008ec0f0 (thiscall, seven stack words, u8 return; true size
/// 1885 bytes, the inventory lists 1865 and cuts the return-0 epilogue).
lf_checker_rt::export!(thiscall, rw_008ec0f0(
    handle: u32,
    query: u32,
    radius: u32,
    out_vec: u32,
    out_id: u32,
    player: u32,
    flags: u32,
    flt: u32,
) -> u8 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16i(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn imgf(va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(va).read() }
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
        const SLOTS: u32 = 0x804;
        const N_SLOTS: u32 = 0x40;
        const SIG_OFF: u32 = 0x1bc4;
        const TABLE_N: i32 = 0x40;
        const CALLEE_MEMCMP: u32 = 1;
        const CALLEE_CLUSTER: u32 = 2;
        const CALLEE_SCORE: u32 = 3;
        const CALLEE_SORT: u32 = 4;
        const CALLEE_RAND: u32 = 5;
        const G_KEY: u32 = 0x01173604;
        const G_DB0: u32 = 0x01176DB0;
        const G_DB4: u32 = 0x01176DB4;
        const G_FILL: u32 = 0x01176E4C;
        const G_CUR: u32 = 0x01176E50;
        const G_TRIED: u32 = 0x01176E54;
        const G_GRAD: u32 = 0x01176E58;
        const G_GFLG: u32 = 0x01176E5C;
        const G_SIG: u32 = 0x01176E60;
        const G_QX: u32 = 0x0117E6B0;
        const G_MINIDX: u32 = 0x010330C8;
        const G_MINVAL: u32 = 0x010330CC;
        const G_TAB: u32 = 0x01179690;
        const MINVAL_INIT: u32 = 0xc479c000; // -999.0f, an immediate, not a global read

        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        unsafe fn wg32(va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(va).write(v) }
        }
        #[inline(always)]
        unsafe fn wg8(va: u32, v: u8) {
            unsafe { lf_checker_rt::global::<u8>(va).write(v) }
        }
        #[inline(always)]
        unsafe fn tab_id(tab: u32, i: u32) -> u32 {
            unsafe { rd32(tab.wrapping_add(i.wrapping_mul(8))) }
        }
        #[inline(always)]
        unsafe fn tab_sc(tab: u32, i: u32) -> f32 {
            unsafe { f32::from_bits(rd32(tab.wrapping_add(i.wrapping_mul(8)).wrapping_add(4))) }
        }
        #[inline(always)]
        unsafe fn wtab(tab: u32, i: u32, id: u32, sc_bits: u32) {
            unsafe {
                wr32(tab.wrapping_add(i.wrapping_mul(8)), id);
                wr32(tab.wrapping_add(i.wrapping_mul(8)).wrapping_add(4), sc_bits);
            }
        }

        let xy = imgf(0x00FE87A4);
        let zsc = imgf(0x00FE8720);
        let k64 = imgf(0x00FE8B88);
        let kidx = imgf(0x00FE8680);
        let krnd = imgf(0x00FE8684);
        let neg64 = imgf(0x00E833E0);
        let eps = imgf(0x00FE870C);
        let one = imgf(0x00FE88E8);
        let tab = lf_checker_rt::relocated(G_TAB);
        let sig = lf_checker_rt::relocated(G_SIG);

        let qx = f32::from_bits(rd32(query));
        let qy = f32::from_bits(rd32(query.wrapping_add(4)));
        let qz = f32::from_bits(rd32(query.wrapping_add(8)));
        let qw = f32::from_bits(rd32(query.wrapping_add(12)));
        let radf = f32::from_bits(radius);
        let f = f32::from_bits(flt);
        // Clamp, kept iff strictly above 1.0 (ordered); the original also
        // writes this back over its incoming stack slot, which a Rust
        // rewrite cannot do, so the stack check is off for this function.
        let clamped = if f > one { f } else { one };

        // Cached-query fast gate.
        let mut edi = g32(G_FILL);
        let mut cur = g32(G_CUR);
        let mut fast = false;
        if g32(G_DB4) == player
            && g32(G_DB0) == g32(G_KEY)
            && (edi as i32) > (cur as i32)
            && qx == f32::from_bits(g32(G_QX))
            && qy == f32::from_bits(g32(G_QX.wrapping_add(4)))
            && qz == f32::from_bits(g32(G_QX.wrapping_add(8)))
            && radf == f32::from_bits(g32(G_GRAD))
            && g32(G_GFLG) == 0
        {
            let m: u32 = lf_checker_rt::callee_cdecl!(CALLEE_MEMCMP, u32, sig, handle.wrapping_add(SIG_OFF));
            if (m as u8) != 0 {
                fast = true;
            }
        }

        if !fast {
            wg32(G_QX, qx.to_bits());
            wg32(G_QX.wrapping_add(4), qy.to_bits());
            wg32(G_QX.wrapping_add(8), qz.to_bits());
            wg32(G_QX.wrapping_add(12), qw.to_bits());
            let mut centerx = qx;
            let mut centery = qy;
            let mut out1 = [0u32; 4];
            let mut out2 = [0u32; 4];
            wg32(G_DB0, g32(G_KEY));
            wg32(G_DB4, player);
            wg32(G_MINIDX, 0xffff_ffff);
            wg32(G_MINVAL, MINVAL_INIT);
            let a1: u32 = lf_checker_rt::callee_stdcall!(CALLEE_CLUSTER, u32, player, 1, out1.as_mut_ptr() as u32);
            if (a1 as u8) != 0 && (flags as u8) & 4 != 0 {
                centerx = f32::from_bits(out1[0]);
                centery = f32::from_bits(out1[1]);
            }
            let _: u32 = lf_checker_rt::callee_stdcall!(CALLEE_CLUSTER, u32, player, 0, out2.as_mut_ptr() as u32);
            edi = 0;
            wg32(G_GRAD, radius);
            wg32(G_TRIED, 0);
            wg32(G_CUR, 0);
            wg32(G_FILL, 0);
            wg32(G_GFLG, 0);
            {
                let mut j = 0u32;
                while j < 0x20 {
                    (sig.wrapping_add(j) as *mut u8).write(rd8(handle.wrapping_add(SIG_OFF).wrapping_add(j)));
                    j += 1;
                }
            }
            let mut fill = 0u32;
            let mut slotptr = handle.wrapping_add(0xb04);
            let mut rem = N_SLOTS;
            while rem != 0 {
                let arec = rd32(slotptr.wrapping_sub(0x300));
                if arec != 0 {
                    let count = rd32(slotptr) as i32;
                    if count > 0 {
                        let mut k = 0i32;
                        while k < count {
                            let rec = arec.wrapping_add((k as u32).wrapping_mul(0x20));
                            let mut pass = true;
                            if rd8(rec.wrapping_add(0x1c)) & 0xf0 != 0x80 {
                                pass = false;
                            }
                            if pass && rd8(sig.wrapping_add(rd8(rec.wrapping_add(0x1a)) as u32)) == 0 {
                                pass = false;
                            }
                            if pass {
                                let x = mul(rd16i(rec.wrapping_add(0x14)) as f32, xy);
                                let y = mul(rd16i(rec.wrapping_add(0x16)) as f32, xy);
                                let z = mul(rd16i(rec.wrapping_add(0x18)) as f32, zsc);
                                let v0 = [x.to_bits(), y.to_bits(), z.to_bits(), 0u32];
                                let dy = sub(y, centery);
                                let dx = sub(x, centerx);
                                let dist = core::hint::black_box(add(mul(dy, dy), mul(dx, dx))).sqrt();
                                if radf > dist {
                                    let recid = rd32(rec.wrapping_add(8));
                                    let score: f32 = lf_checker_rt::callee_stdcall!(
                                        CALLEE_SCORE, f32,
                                        v0.as_ptr() as u32,
                                        query,
                                        out2.as_ptr() as u32,
                                        out1.as_ptr() as u32,
                                        player,
                                        flags,
                                        clamped.to_bits()
                                    );
                                    edi = fill;
                                    if (edi as i32) >= TABLE_N {
                                        if flags == 0 {
                                            let r1: u32 = lf_checker_rt::callee_cdecl!(CALLEE_RAND, u32,);
                                            let t = g32(G_TRIED) as i32 as f32;
                                            let lhs = div(k64, t);
                                            let rhs = mul((r1 as i32) as f32, krnd);
                                            if lhs > rhs {
                                                let r2: u32 = lf_checker_rt::callee_cdecl!(CALLEE_RAND, u32,);
                                                let idx = mul(mul(((r2 & 0xffff) as f32), kidx), neg64) as i32;
                                                let addr = tab.wrapping_sub((idx as u32).wrapping_mul(8));
                                                wr32(addr, recid);
                                            }
                                            edi = fill;
                                        } else {
                                            let minval = f32::from_bits(g32(G_MINVAL));
                                            if score > minval {
                                                let mi = g32(G_MINIDX);
                                                wtab(tab, mi, recid, score.to_bits());
                                                wg32(G_MINVAL, score.to_bits());
                                                let mut c = 2u32;
                                                let mut slot_off = 0xcu32;
                                                loop {
                                                    let s0 = f32::from_bits(rd32(tab.wrapping_add(slot_off).wrapping_sub(8)));
                                                    if score > s0 {
                                                        wg32(G_MINIDX, c.wrapping_sub(2));
                                                        wg32(G_MINVAL, score.to_bits());
                                                    }
                                                    let s1 = f32::from_bits(rd32(tab.wrapping_add(slot_off)));
                                                    if score > s1 {
                                                        wg32(G_MINIDX, c.wrapping_sub(1));
                                                        wg32(G_MINVAL, score.to_bits());
                                                    }
                                                    let s2 = f32::from_bits(rd32(tab.wrapping_add(slot_off).wrapping_add(8)));
                                                    if score > s2 {
                                                        wg32(G_MINIDX, c);
                                                        wg32(G_MINVAL, score.to_bits());
                                                    }
                                                    let s3 = f32::from_bits(rd32(tab.wrapping_add(slot_off).wrapping_add(16)));
                                                    if score > s3 {
                                                        wg32(G_MINIDX, c.wrapping_add(1));
                                                        wg32(G_MINVAL, score.to_bits());
                                                    }
                                                    c = c.wrapping_add(4);
                                                    slot_off = slot_off.wrapping_add(0x20);
                                                    if !((c.wrapping_sub(2) as i32) < TABLE_N) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        wtab(tab, fill, recid, score.to_bits());
                                        if flags != 0 {
                                            if g32(G_MINIDX) == 0xffff_ffff {
                                                wg32(G_MINIDX, fill);
                                                wg32(G_MINVAL, score.to_bits());
                                            } else {
                                                let minval = f32::from_bits(g32(G_MINVAL));
                                                if minval > score {
                                                    wg32(G_MINIDX, fill);
                                                    wg32(G_MINVAL, score.to_bits());
                                                }
                                            }
                                        }
                                        fill = fill.wrapping_add(1);
                                        wg32(G_FILL, fill);
                                        edi = fill;
                                    }
                                    wg32(G_TRIED, g32(G_TRIED).wrapping_add(1));
                                }
                            }
                            k += 1;
                        }
                    }
                }
                slotptr = slotptr.wrapping_add(4);
                rem -= 1;
            }

            if (edi as i32) > 0 {
                if flags == 0 {
                    let mut ctr = 0x80u32;
                    while ctr != 0 {
                        let r1: u32 = lf_checker_rt::callee_cdecl!(CALLEE_RAND, u32,);
                        let fillg = g32(G_FILL);
                        let i1 = mul(mul(((r1 & 0xffff) as f32), kidx), (edi as i32) as f32) as i32 as u32;
                        let r2: u32 = lf_checker_rt::callee_cdecl!(CALLEE_RAND, u32,);
                        ctr -= 1;
                        let e1id = tab_id(tab, i1);
                        let e1sc = rd32(tab.wrapping_add(i1.wrapping_mul(8)).wrapping_add(4));
                        let i2 = mul(mul(((r2 & 0xffff) as f32), kidx), (fillg as i32) as f32) as i32 as u32;
                        wtab(tab, i1, tab_id(tab, i2), rd32(tab.wrapping_add(i2.wrapping_mul(8)).wrapping_add(4)));
                        edi = fillg;
                        wtab(tab, i2, e1id, e1sc);
                        if ctr == 0 {
                            break;
                        }
                    }
                } else {
                    let _: u32 = lf_checker_rt::callee_cdecl!(CALLEE_SORT, u32, tab, tab.wrapping_add(edi.wrapping_mul(8)), 0);
                    edi = g32(G_FILL);
                    let mut cc = 1i32;
                    let mut ss = 0i32;
                    let mut base0 = 0u32;
                    if (edi as i32) > cc {
                        loop {
                            let d = sub(tab_sc(tab, cc as u32), tab_sc(tab, ss as u32));
                            let ad = if d < 0.0 { -d } else { d };
                            let mut run_inner = false;
                            if ad > eps {
                                run_inner = true;
                            } else if cc == (edi as i32).wrapping_sub(1) {
                                run_inner = true;
                            }
                            if run_inner {
                                if (cc.wrapping_sub(1) > ss) && (ss < cc) {
                                    let n = (cc.wrapping_sub(ss)) as u32;
                                    let nf = (n as i32) as f32;
                                    let mut nn = n;
                                    while nn != 0 {
                                        let r1: u32 = lf_checker_rt::callee_cdecl!(CALLEE_RAND, u32,);
                                        let i1 = ((mul(mul(((r1 & 0xffff) as f32), kidx), nf) as i32) as u32).wrapping_add(base0);
                                        let r2: u32 = lf_checker_rt::callee_cdecl!(CALLEE_RAND, u32,);
                                        let i2 = ((mul(mul(((r2 & 0xffff) as f32), kidx), nf) as i32) as u32).wrapping_add(base0);
                                        let e1id = tab_id(tab, i1);
                                        let e1sc = rd32(tab.wrapping_add(i1.wrapping_mul(8)).wrapping_add(4));
                                        wtab(tab, i1, tab_id(tab, i2), rd32(tab.wrapping_add(i2.wrapping_mul(8)).wrapping_add(4)));
                                        wtab(tab, i2, e1id, e1sc);
                                        nn -= 1;
                                    }
                                    edi = g32(G_FILL);
                                }
                                ss = cc;
                                base0 = cc as u32;
                            }
                            cc += 1;
                            if !(cc < (edi as i32)) {
                                break;
                            }
                        }
                    }
                }
            }
            cur = g32(G_CUR);
        }

        if (cur as i32) >= (edi as i32) {
            wr32(out_id, 0xffff_ffff);
            return 0;
        }
        let eid = tab_id(tab, cur);
        let slot = (eid & 0xffff) as u32;
        let idx = (eid >> 16) as u32;
        let sub = rd32(handle.wrapping_add(SLOTS).wrapping_add(slot.wrapping_mul(4))).wrapping_add(idx.wrapping_mul(32));
        wr32(out_vec, mul(rd16i(sub.wrapping_add(0x14)) as f32, xy).to_bits());
        wr32(out_vec.wrapping_add(4), mul(rd16i(sub.wrapping_add(0x16)) as f32, xy).to_bits());
        wr32(out_vec.wrapping_add(8), mul(rd16i(sub.wrapping_add(0x18)) as f32, zsc).to_bits());
        wr32(out_id, rd32(sub.wrapping_add(8)));
        wg32(G_CUR, cur.wrapping_add(1));
        1
    }
});

