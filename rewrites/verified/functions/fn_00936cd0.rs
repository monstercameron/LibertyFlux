// original: 0x00936CD0 net_tick_big (proposed)

/// Drive one network tick: refresh the object's deadline, resolve the
/// session, and scan its channel list for work.
///
/// With no arguments, `this` is the whole input. The deadline word is
/// raised to at least now+0x1388 when the gate byte allows; an expired
/// deadline, a missing session, or session states 2/3/4 end the tick
/// early. Otherwise a probe triple is fetched, the readiness check runs,
/// and a true answer, a mode word of 1, or a level at or above threshold
/// each finish the tick through one fixed notify call (kinds 8, 6 and 7).
///
/// The main loop then visits channels `0..count-1`. Channel 0 only extends
/// the running distance total. Later channels classify the total, compare
/// their flag nibble, and may emit a resync notify (kind 9); then follow
/// two transform calls, two projection calls and two dot products whose
/// branch chain either continues the loop, returns through a kind-5
/// notify, or enters the inner scan. The inner scan walks the remaining
/// channels pairwise through the same transform pair and ends with one
/// computed notify call, one classify call and a log call. Every path
/// returns the same status word.
///
/// One stack slot is read without ever being written (the final notify
/// kind); it holds the contract's 1.0 fill, hard-coded here. The probe
/// buffers likewise start at the fill. The float operation order is the
/// original's throughout.
///
/// Original: 0x00936CD0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00936CD0(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0xD60;
        const MODEW: u32 = 0xD64;
        const LAST_SEQ: u32 = 0xD88;
        const SYNC_SEQ: u32 = 0xD98;
        const LEVEL: u32 = 0xD9C;
        const DLN: u32 = 0xDA0;
        const FLAGS: u32 = 0xC80;
        const STATUS: u32 = 0x011A_4E20;
        const GATE_OBJ: u32 = 0x0128_4A60;
        const G_TIME: u32 = 0x0117_35B4;
        const G_ED4: u32 = 0x0103_6ED4;
        const G_ED0: u32 = 0x0103_6ED0;
        const G_EDC: u32 = 0x0103_6EDC;
        const G_EE4: u32 = 0x0103_6EE4;
        const LOG2: u32 = 1;
        const LOG3: u32 = 2;
        const CAL_GATE: u32 = 3;
        const CAL_SESSION: u32 = 4;
        const CAL_TRIPLE: u32 = 5;
        const CAL_READY: u32 = 6;
        const CAL_NOTIFY: u32 = 7;
        const CAL_CLASSIFY: u32 = 8;
        const CAL_XFORM: u32 = 9;
        const CAL_PAIR: u32 = 10;
        const CAL_PROJ: u32 = 11;
        const SIGN: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }
        #[inline(always)]
        fn absf(a: f32) -> f32 {
            f32::from_bits(a.to_bits() & !SIGN)
        }
        #[inline(always)]
        fn fistp_low32(x: f64) -> u32 {
            const TWO63: f64 = 9.223372036854776e18;
            if x.is_nan() || x >= TWO63 || x < -TWO63 {
                0
            } else {
                (x as i64) as u32
            }
        }

        let g = lf_checker_rt::relocated;
        let status = g(STATUS);
        // Fixed notify helper: (kind, -1.0, a, 0/-1, -1.0, -1).
        macro_rules! notify_const {
            ($kind:expr) => {{
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_NOTIFY,
                    u32,
                    this,
                    $kind,
                    0xBF80_0000u32,
                    0xFFFF_FFFFu32,
                    0u32,
                    0xBF80_0000u32,
                    0xFFFF_FFFFu32
                );
            }};
        }

        let _: u32 = lf_checker_rt::callee_cdecl!(LOG2, u32, status, g(0x00E8_790C));
        let gate: u32 = lf_checker_rt::callee_thiscall!(CAL_GATE, u32, g(GATE_OBJ),);
        if (gate as u8) != 0 {
            let floor = rd32(g(G_TIME)).wrapping_add(0x1388);
            let d = rd32(this.wrapping_add(DLN));
            wr32(this.wrapping_add(DLN), if d > floor { d } else { floor });
        }
        if rd32(this.wrapping_add(DLN)) > rd32(g(G_TIME)) {
            return status;
        }
        let sess: u32 = lf_checker_rt::callee_cdecl!(CAL_SESSION, u32, 0u32);
        if sess == 0 {
            return status;
        }
        let st = rd32(sess.wrapping_add(0x1304));
        if st == 2 || st == 4 {
            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_READY, u32, this,);
            return status;
        }
        if st == 3 {
            return status;
        }
        // Probe triple into a 1.0-filled buffer (the stack fill).
        let mut tri0 = [1.0f32; 3];
        let _: u32 = lf_checker_rt::callee_cdecl!(CAL_TRIPLE, u32, tri0.as_mut_ptr() as u32);
        let ready: u32 = lf_checker_rt::callee_thiscall!(CAL_READY, u32, this,);
        if (ready as u8) != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(LOG2, u32, status, g(0x00E8_7910));
            notify_const!(8u32);
            return status;
        }
        if rd32(this.wrapping_add(MODEW)) == 1 {
            let _: u32 = lf_checker_rt::callee_cdecl!(LOG2, u32, status, g(0x00E8_7924));
            notify_const!(6u32);
            return status;
        }
        if !(rdf(this.wrapping_add(LEVEL)) < rdf(g(G_ED4))) {
            let _: u32 = lf_checker_rt::callee_cdecl!(LOG2, u32, status, g(0x00E8_793C));
            notify_const!(7u32);
            return status;
        }
        let count = rd32(this.wrapping_add(COUNT));
        if (count as i32) <= 1 {
            return status;
        }
        // Main channel loop. Fill slots (never written by the original).
        const FILLF: f32 = 1.0;
        const FILLW: u32 = 0x3F80_0000;
        let mut total = 0.0f32;
        let mut cl: u8 = 0;
        let mut ebx: u32 = 0;
        // Second triple buffer (ebx == 0 path), also 1.0-filled.
        let mut triB = [1.0f32; 3];
        let mut pair = [0.0f32; 2];
        // Pair buffer: filled once, then keeps each call's scripted words
        // for the next iteration's pre-call snapshot, like the original's.
        let mut vA0 = [FILLF; 2];
        loop {
            let flag = rd8(this.wrapping_add(FLAGS).wrapping_add(ebx));
            let nib = (flag & 0xF) as u32;
            let hi = flag >> 7;
            let saved_hi = hi;
            if (ebx as i32) > 0 {
                if cl == 0 {
                    if hi != 0 {
                        let code: u32 = lf_checker_rt::callee_stdcall!(
                            CAL_CLASSIFY,
                            u32,
                            total.to_bits(),
                            0u32
                        );
                        let _: u32 = lf_checker_rt::callee_cdecl!(
                            LOG3,
                            u32,
                            status,
                            g(0x00E8_7968),
                            code
                        );
                    }
                } else if hi == 0 {
                    let code: u32 = lf_checker_rt::callee_stdcall!(
                        CAL_CLASSIFY,
                        u32,
                        total.to_bits(),
                        0u32
                    );
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        LOG3,
                        u32,
                        status,
                        g(0x00E8_7984),
                        code
                    );
                }
                if nib > 2 {
                    let last = rd32(this.wrapping_add(LAST_SEQ));
                    if (ebx as i32) > (last as i32)
                        && last != rd32(this.wrapping_add(SYNC_SEQ))
                    {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            CAL_NOTIFY,
                            u32,
                            this,
                            9u32,
                            0xBF80_0000u32,
                            last,
                            0u32,
                            0xBF80_0000u32,
                            0xFFFF_FFFFu32
                        );
                    }
                    let e = ebx.wrapping_mul(2);
                    // Transform pair over channel deltas.
                    let d0 = sub(
                        rdf(this.wrapping_add(e.wrapping_mul(8))),
                        rdf(
                            this.wrapping_add(e.wrapping_mul(8)).wrapping_sub(0x10),
                        ),
                    );
                    let d1 = sub(
                        rdf(this.wrapping_add(e.wrapping_mul(8).wrapping_add(4))),
                        rdf(
                            this.wrapping_add(e.wrapping_mul(8)).wrapping_sub(0x0C),
                        ),
                    );
                    let d2 = sub(
                        rdf(this.wrapping_add(e.wrapping_mul(8).wrapping_add(0x10))),
                        rdf(this.wrapping_add(e.wrapping_mul(8))),
                    );
                    let d3 = sub(
                        rdf(this.wrapping_add(e.wrapping_mul(8).wrapping_add(0x14))),
                        rdf(this.wrapping_add(e.wrapping_mul(8).wrapping_add(4))),
                    );
                    let mut v30 = [d0, d1, 0.0f32];
                    let mut v80 = [d2, d3, 0.0f32];
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_XFORM,
                        u32,
                        v30.as_mut_ptr() as u32,
                    );
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_XFORM,
                        u32,
                        v80.as_mut_ptr() as u32,
                    );
                    // Cross and dot of the transformed pair. The push
                    // for the next call shifts the frame, so these land
                    // in [esp+0x54] and [esp+0x2c], where the branch
                    // chain and the inner scan read them back.
                    let cross = sub(mul(v80[1], v30[0]), mul(v80[0], v30[1]));
                    let outer = add(
                        add(mul(v80[1], v30[1]), mul(v80[0], v30[0])),
                        mul(v80[2], v30[2]),
                    );
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        CAL_PAIR,
                        u32,
                        vA0.as_mut_ptr() as u32
                    );
                    let px = sub(rdf(this.wrapping_add(e.wrapping_mul(8))), tri0[0]);
                    let py = sub(
                        rdf(this.wrapping_add(e.wrapping_mul(8).wrapping_add(4))),
                        tri0[1],
                    );
                    let ng = neg(tri0[2]);
                    let mut v60 = [px, py, ng];
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_PROJ,
                        u32,
                        v60.as_mut_ptr() as u32,
                    );
                    let qx = sub(rdf(this.wrapping_add(0x10)), tri0[0]);
                    let qy = sub(rdf(this.wrapping_add(0x14)), tri0[1]);
                    let mut v40 = [qx, qy, ng];
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_PROJ,
                        u32,
                        v40.as_mut_ptr() as u32,
                    );
                    let dot = add(
                        add(mul(vA0[0], v60[0]), mul(vA0[1], v60[1])),
                        mul(v60[2], 0.0f32),
                    );
                    let over = sub(total, rdf(g(G_ED0)));
                    let capped = if over > 0.0 { over } else { 0.0f32 };
                    let sess2: u32 = lf_checker_rt::callee_cdecl!(CAL_SESSION, u32, 0u32);
                    if sess2 != 0 {
                        let obj = rd32(sess2.wrapping_add(0x20));
                        let f0 = rdf(obj.wrapping_add(0x10));
                        let f1 = rdf(obj.wrapping_add(0x14));
                        let f2 = rdf(obj.wrapping_add(0x18));
                        let dot1 = add(add(mul(f1, v40[1]), mul(f0, v40[0])), mul(f2, v40[2]));
                        if rdf(g(G_EDC)) > dot1 {
                            let dot2 = add(
                                add(mul(f0, v60[0]), mul(f1, v60[1])),
                                mul(f2, v60[2]),
                            );
                            if rdf(g(G_EDC)) > dot2 {
                                let _: u32 = lf_checker_rt::callee_cdecl!(
                                    LOG2,
                                    u32,
                                    status,
                                    g(0x00E8_79A0)
                                );
                                notify_const!(5u32);
                                return status;
                            }
                        }
                    }
                    let mut slot5c = 100000u32;
                    if absf(dot) > rdf(g(0x00FE_879C)) {
                        let scaled = div(mul(capped, rdf(g(0x00FE_8C58))), dot);
                        slot5c = fistp_low32(scaled as f64);
                    }
                    // Scratch-pair chain over the cross and outer dot
                    // (kept in [esp+0x54] and [esp+0x2c]). Mirrored
                    // exactly, including the fall-through into the
                    // second half when 0.0 > s54 but not 0.0 > s2c.
                    let s54 = cross;
                    let s2c = outer;
                    let mut take_inner = false;
                    if rdf(g(0x00FE_8D7C)) > s54 {
                        take_inner = true;
                    } else {
                        let mut at_tail = true;
                        if 0.0f32 > s54 {
                            if 0.0f32 > s2c {
                                take_inner = true;
                                at_tail = false;
                            }
                        }
                        if at_tail {
                            if s54 > rdf(g(0x00FE_8830)) {
                                take_inner = true;
                            } else if s54 > 0.0f32 && 0.0f32 > s2c {
                                take_inner = true;
                            }
                        }
                    }
                    if take_inner {
                        pair[0] = rdf(this.wrapping_add(e.wrapping_mul(8)));
                        pair[1] =
                            rdf(this.wrapping_add(e.wrapping_mul(8).wrapping_add(4)));
                        // Inner scan over the remaining channels.
                        let bound =
                            (rd32(this.wrapping_add(COUNT)) as i32).wrapping_sub(1);
                        let mut ecx = ebx.wrapping_add(1);
                        let mut esi: u32 = 0xFFFF_FFFF;
                        if (ecx as i32) < bound {
                            let mut ptr = this
                                .wrapping_add(ecx.wrapping_mul(16))
                                .wrapping_add(0x18);
                            loop {
                                let i0 = sub(rdf(ptr.wrapping_sub(0x18)), rdf(ptr.wrapping_sub(0x28)));
                                let i1 = sub(rdf(ptr.wrapping_sub(0x14)), rdf(ptr.wrapping_sub(0x24)));
                                let i2 = sub(rdf(ptr.wrapping_sub(8)), rdf(ptr.wrapping_sub(0x18)));
                                let i3 = sub(rdf(ptr.wrapping_sub(4)), rdf(ptr.wrapping_sub(0x14)));
                                let mut w30 = [i0, i1, 0.0f32];
                                let mut w40 = [i2, i3, 0.0f32];
                                let _: u32 = lf_checker_rt::callee_thiscall!(
                                    CAL_XFORM,
                                    u32,
                                    w30.as_mut_ptr() as u32,
                                );
                                let _: u32 = lf_checker_rt::callee_thiscall!(
                                    CAL_XFORM,
                                    u32,
                                    w40.as_mut_ptr() as u32,
                                );
                                let j0 = sub(rdf(ptr.wrapping_sub(4)), pair[1]);
                                let j1 = sub(rdf(ptr.wrapping_sub(8)), pair[0]);
                                let j2 = sub(
                                    rdf(ptr),
                                    rdf(this.wrapping_add(e.wrapping_mul(8)).wrapping_add(8)),
                                );
                                let cr = sub(mul(w40[1], w30[0]), mul(w40[0], w30[1]));
                                let dd = add(add(mul(j0, j0), mul(j1, j1)), mul(j2, j2));
                                let dist = dd.sqrt();
                                if !(rdf(g(G_EE4)) > dist) {
                                    break;
                                }
                                if rdf(g(0x00FE_8D7C)) > cr {
                                    esi = 1;
                                    break;
                                }
                                if 0.0f32 > cr {
                                    let t = add(
                                        add(mul(w40[1], w30[1]), mul(w40[0], w30[0])),
                                        mul(w40[2], w30[2]),
                                    );
                                    if 0.0f32 > t {
                                        esi = 1;
                                        break;
                                    }
                                }
                                if cross > rdf(g(0x00FE_8830)) {
                                    esi = 0;
                                    break;
                                }
                                if !(cross > 0.0f32) {
                                    // fall through to next channel
                                } else if 0.0f32 > outer {
                                    esi = 0;
                                    break;
                                }
                                ecx = ecx.wrapping_add(1);
                                ptr = ptr.wrapping_add(0x10);
                                if !((ecx as i32) < bound) {
                                    break;
                                }
                            }
                        }
                        let kind = FILLW;
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            CAL_NOTIFY,
                            u32,
                            this,
                            kind,
                            capped.to_bits(),
                            ebx,
                            slot5c,
                            dot.to_bits(),
                            esi
                        );
                        let code: u32 = lf_checker_rt::callee_stdcall!(
                            CAL_CLASSIFY,
                            u32,
                            capped.to_bits(),
                            0u32
                        );
                        let tag = if esi == 0xFFFF_FFFF {
                            0x00E8_79A8u32
                        } else {
                            0x00E8_79B8u32
                        };
                        let _: u32 =
                            lf_checker_rt::callee_cdecl!(LOG3, u32, status, g(tag), code);
                        return status;
                    }
                }
            }
            // Loop bottom: extend the distance total.
            let e = ebx.wrapping_mul(2);
            let (mut ax, mut ay);
            if ebx == 0 {
                let _: u32 =
                    lf_checker_rt::callee_cdecl!(CAL_TRIPLE, u32, triB.as_mut_ptr() as u32);
                ax = triB[1];
                ay = triB[0];
            } else {
                ax = rdf(this.wrapping_add(e.wrapping_mul(8).wrapping_add(4)));
                ay = rdf(this.wrapping_add(e.wrapping_mul(8)));
            }
            ax = sub(ax, rdf(this.wrapping_add(e.wrapping_mul(8)).wrapping_add(0x14)));
            ay = sub(ay, rdf(this.wrapping_add(e.wrapping_mul(8)).wrapping_add(0x10)));
            cl = saved_hi;
            ebx = ebx.wrapping_add(1);
            let step = add(mul(ax, ax), mul(ay, ay)).sqrt();
            total = add(step, total);
            let lim = (rd32(this.wrapping_add(COUNT)) as i32).wrapping_sub(1);
            if !((ebx as i32) < lim) {
                return status;
            }
        }
    }
});
