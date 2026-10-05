// original: 0x008af600 audio_voice_update

/// Per-frame update of one audio voice (channel) inside the audio manager.
///
/// `this` (ECX) is the audio manager object, `voice` the voice object and
/// `arg1` a small integer parameter (selector). The routine transforms the
/// voice's direction vector through two scripted helper calls, derives a
/// magnitude (returning `0x7f800000` early when it is NaN or infinite),
/// combines distance attenuation, directional gains and two lazily
/// initialised global parameter blocks, accumulates the result into the
/// voice's mixer slots, and finishes with an optional second-stage
/// spatialisation and accumulator fold-down. Returns the voice's auxiliary
/// object pointer (`voice+0xac`).
///
/// Layout read (voice offsets): direction floats at `+0x70`/`+0x74`, mixer
/// accumulators `+0x30`..`+0x5c`, saved slots `+0x60`..`+0x6c`, position
/// history `+0x80`..`+0x98`, running level `+0xa8`, aux pointer `+0xac`,
/// sample counter `+0xb0`, output gain `+0xbc`, mode `+0xc0`, scale `+0xc4`,
/// seed word `+0xc8`, table index word `+0xd0`, flag bytes `+0xd2`/`+0xd4`
/// (enable) /`+0xd5` (second stage)/`+0xd6`/`+0xd7` (shape). Manager
/// offsets: spatial tables near `+0x30` and `+0x530`, gain tables near
/// `+0x1724`, `+0x1730`, `+0x174c`..`+0x1770`, `+0x17ac`..`+0x17cc`,
/// `+0x17d0`/`+0x17d4`, flag bytes `+0x17e0`/`+0x17f4`, level selector
/// `+0x17ec`. Globals: pointer table at `0x115f814`, parameter blocks at
/// `0x115fd60`/`0x115fd84` with init flags in `0x115fd80` (self
/// initialising on first use), selector globals at `0x1030088`/`0x1030a1c`,
/// TLS slot index at `0x17aba14` (pinned to the fabricated slot by the
/// contract); the thread block word at `+0x70` indexes the manager tables.
///
/// Callees (all intercepted and scripted by the contract): two vector
/// transforms writing a float quad to a frame buffer, one float->double
/// helper, three more float->double helpers, the main voice worker, two
/// envelope shapers writing the voice head words, one spatialisation
/// helper writing the accumulator words, and a final gain helper. Float
/// operation order is the original's, pinned through `black_box` helpers;
/// every `comiss` branch is written with NaN-exact semantics, and the two
/// x87 truncate-and-convert (`fistp`) sequences are reproduced bit-exactly
/// including the invalid-operation result.
///
/// Original: 0x008af600 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_008af600(this: u32, voice: u32, arg1: u32) -> u32 {
    unsafe {
        const AUX_PTR: u32 = 0xac;
        const EXP_MASK: u32 = 0x7f80_0000;
        const EXP_INF: u32 = 0x7f80_0000;
        const ONE_BITS: u32 = 0x3f80_0000;
        const TLS_SLOT: usize = 5;
        const TWICE_POW63_F: f32 = 9.223372036854776e18; // 2^63 exactly

        const G_TABLE: u32 = 0x0115_f814;
        const G_P1_BASE: u32 = 0x0115_fd60;
        const G_P1_FLAG: u32 = 0x0115_fd80;
        const G_P2_BASE: u32 = 0x0115_fd84;
        const G_SEL: u32 = 0x0103_0088;
        const G_KMIX: u32 = 0x0103_0a1c;
        const G_TLSIDX: u32 = 0x017a_ba14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
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
        unsafe fn grdf(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        fn gvar(va: u32) -> u32 {
            lf_checker_rt::relocated(va)
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
        /// What an x87 `fistp qword` stores for `x` with the control word
        /// set to truncate: truncation toward zero, and `i64::MIN` on
        /// invalid (NaN or magnitude at/above 2^63, the `x == -2^63` case
        /// yielding the same value through the valid path).
        #[inline(always)]
        fn fistp_i64(x: f32) -> i64 {
            if x.is_nan() {
                return i64::MIN;
            }
            if x >= TWICE_POW63_F || x <= -TWICE_POW63_F {
                return i64::MIN;
            }
            x.trunc() as i64
        }

        // --- vector transforms and magnitude ---
        let voice_dir = voice.wrapping_add(0x70);
        let mut quad = [0u32; 4];
        let scratch = [0u32; 4];
        lf_checker_rt::callee_thiscall!(1, u32, this, quad.as_mut_ptr() as u32, voice_dir, arg1);
        lf_checker_rt::callee_thiscall!(2, u32, this, scratch.as_ptr() as u32, voice_dir, arg1);
        let qx = f32::from_bits(quad[0]);
        let qy = f32::from_bits(quad[1]);
        let qz = f32::from_bits(quad[2]);
        let mag = add(add(mul(qy, qy), mul(qx, qx)), mul(qz, qz)).sqrt();
        if (mag.to_bits() & EXP_MASK) == EXP_INF {
            return EXP_INF;
        }

        let zero = 0.0f32;
        wrf(voice.wrapping_add(0xbc), 1.0);
        // Threaded through the disabled-voice join below: (blend4, blend6).
        let (mut blend4, mut blend6);
        if rd8(voice.wrapping_add(0xd4)) != 0 {
            // --- directional gain ---
            let tidx = rd16(voice.wrapping_add(0xd0)) as u16 as i16 as i32;
            let tab = rd32(gvar(G_TABLE));
            let entry = tab
                .wrapping_add((tidx.wrapping_mul(9) as u32).wrapping_mul(4))
                .wrapping_add(4);
            let mut dir_gain = mul(rdf(entry), rdf(voice.wrapping_add(0xc4)));
            let aux = rd32(voice.wrapping_add(AUX_PTR));
            if aux != 0 {
                dir_gain = mul(rdf(aux.wrapping_add(0x18)), dir_gain);
            }
            core::hint::black_box(dir_gain);

            // --- thread-local spatial offset and distance ---
            // The original indexes the TLS array by the runtime slot index
            // (pinned to TLS_SLOT by the contract); the transport reads the
            // fabricated slot value directly.
            let tls_entry = lf_checker_rt::tls_slot(TLS_SLOT);
            let tls70 = rd32(tls_entry.wrapping_add(0x70));
            let off = tls70.wrapping_shl(6);
            let dx = sub(
                rdf(voice.wrapping_add(0x74)),
                rdf(this.wrapping_add(off).wrapping_add(0x534)),
            );
            let dy = sub(
                rdf(voice.wrapping_add(0x70)),
                rdf(this.wrapping_add(off).wrapping_add(0x530)),
            );
            let dist2 = add(mul(dx, dx), mul(dy, dy));
            let att: f64 = lf_checker_rt::callee_cdecl!(3, f64, dist2.to_bits());
            let att_f = att as f32;

            // --- attenuation polynomial with clamped partial sums ---
            let k1 = grdf(0x00fe88e8);
            let mut x5 = mul(dy, att_f);
            let mut x6 = mul(dx, att_f);
            let mut x3 = mul(x5, zero);
            let mut x7 = mul(mul(att_f, zero), zero);
            let mut x2 = add(add(x6, x3), x7);
            if zero > x2 {
                x2 = zero;
            }
            let mut x0 = mul(x6, zero);
            let mut x4 = add(x5, x0);
            x2 = mul(mul(x2, x2), rdf(this.wrapping_add(0x1764)));
            x4 = add(x4, x7);
            if zero > x4 {
                x4 = zero;
            }
            x4 = add(mul(mul(x4, x4), rdf(this.wrapping_add(0x1768))), x2);
            x6 = mul(x6, k1);
            x3 = add(sub(x3, x6), x7);
            if zero > x3 {
                x3 = zero;
            }
            x5 = mul(x5, k1);
            x3 = mul(mul(x3, x3), rdf(this.wrapping_add(0x176c)));
            x0 = add(sub(x0, x5), x7);
            x3 = add(x3, x4);
            if zero > x0 {
                x0 = zero;
            }
            x0 = add(mul(mul(x0, x0), rdf(this.wrapping_add(0x1770))), x3);
            let mut root = mul(dist2.sqrt(), grdf(0x00fe8704));
            if !(k1 > root) {
                root = k1;
            }
            x4 = sub(k1, root);
            root = mul(root, x0);
            let k3 = grdf(0x00fe8a24);
            x4 = add(mul(x4, rdf(this.wrapping_add(0x174c))), root);

            // --- first parameter block (self initialising) ---
            let flag = rd32(gvar(G_P1_FLAG));
            let (p5, p6, p7, p3);
            if (flag & 1) == 0 {
                wr32(gvar(G_P1_FLAG), flag | 1);
                wrf(gvar(G_P1_BASE).wrapping_add(0x00), zero);
                wrf(gvar(G_P1_BASE).wrapping_add(0x04), k1);
                let k4 = grdf(0x00fe8830);
                wrf(gvar(G_P1_BASE).wrapping_add(0x0c), k4);
                wrf(gvar(G_P1_BASE).wrapping_add(0x10), k1);
                wrf(gvar(G_P1_BASE).wrapping_add(0x18), k1);
                wrf(gvar(G_P1_BASE).wrapping_add(0x1c), k1);
                wrf(gvar(G_P1_BASE).wrapping_add(0x08), k3);
                wrf(gvar(G_P1_BASE).wrapping_add(0x14), k3);
                p5 = k1;
                p6 = k4;
                p7 = k1;
                p3 = k1;
            } else {
                p5 = rdf(gvar(G_P1_BASE).wrapping_add(0x10));
                p6 = rdf(gvar(G_P1_BASE).wrapping_add(0x0c));
                p7 = rdf(gvar(G_P1_BASE).wrapping_add(0x04));
                p3 = rdf(gvar(G_P1_BASE).wrapping_add(0x1c));
            }
            let t50 = sub(x4, p6);
            let mut g6 = mul(mul(t50, rdf(gvar(G_P1_BASE).wrapping_add(0x14))), sub(p3, p5));
            g6 = add(g6, p5);
            let mut g3 = sub(x4, rdf(gvar(G_P1_BASE)));
            if !(g3 >= zero) {
                g3 = p7;
            } else {
                g3 = add(mul(mul(g3, rdf(gvar(G_P1_BASE).wrapping_add(0x08))), sub(p5, p7)), p7);
            }
            if t50 >= zero {
                g3 = g6;
            }
            x4 = sub(x4, rdf(gvar(G_P1_BASE).wrapping_add(0x18)));
            if x4 >= zero {
                g3 = rdf(gvar(G_P1_BASE).wrapping_add(0x1c));
            }
            let t1750 = rdf(this.wrapping_add(0x1750));
            let c4a3 = add(sub(k1, t1750), mul(t1750, g3));

            // --- level helpers ---
            let d4: f64 = lf_checker_rt::callee_thiscall!(
                4, f64, this,
                rd32(voice.wrapping_add(0xc8)),
                dir_gain.to_bits(),
                mag.to_bits(),
                c4a3.to_bits()
            );
            let f4 = d4 as f32;
            let mut t164 = 1.0f32;
            let d5: f64 = lf_checker_rt::callee_thiscall!(
                5, f64, this,
                scratch.as_ptr() as u32,
                (&mut t164 as *mut f32) as u32
            );
            let f5 = d5 as f32;

            // --- second parameter block (self initialising) ---
            let flag2 = rd32(gvar(G_P1_FLAG));
            let (s28, s2c, s30, o, x4b, x5b, x7b);
            if (flag2 & 2) == 0 {
                let q1 = grdf(0x00e7cb90);
                wrf(gvar(G_P2_BASE).wrapping_add(0x18), k3);
                wrf(gvar(G_P2_BASE).wrapping_add(0x08), q1);
                let q1b = grdf(0x00e7cb94);
                let q5 = grdf(0x00fe8914);
                wr32(gvar(G_P1_FLAG), flag2 | 2);
                wrf(gvar(G_P2_BASE).wrapping_add(0x00), zero);
                wrf(gvar(G_P2_BASE).wrapping_add(0x04), zero);
                wrf(gvar(G_P2_BASE).wrapping_add(0x0c), q5);
                wrf(gvar(G_P2_BASE).wrapping_add(0x10), zero);
                wrf(gvar(G_P2_BASE).wrapping_add(0x1c), k1);
                wrf(gvar(G_P2_BASE).wrapping_add(0x14), q1b);
                s28 = q1b;
                s2c = q1;
                s30 = k3;
                o = k1;
                x4b = zero;
                x5b = q5;
                x7b = zero;
            } else {
                s30 = rdf(gvar(G_P2_BASE).wrapping_add(0x18));
                x4b = rdf(gvar(G_P2_BASE).wrapping_add(0x10));
                x5b = rdf(gvar(G_P2_BASE).wrapping_add(0x0c));
                x7b = rdf(gvar(G_P2_BASE).wrapping_add(0x04));
                s28 = rdf(gvar(G_P2_BASE).wrapping_add(0x14));
                s2c = rdf(gvar(G_P2_BASE).wrapping_add(0x08));
                o = rdf(gvar(G_P2_BASE).wrapping_add(0x1c));
            }
            let p1orig = sub(mag, x5b);
            let o2 = sub(o, x4b);
            let mut p5b = mul(mul(p1orig, s28), o2);
            p5b = add(p5b, x4b);
            let mut p1 = sub(mag, rdf(gvar(G_P2_BASE)));
            if !(p1 >= zero) {
                p1 = x7b;
            } else {
                p1 = add(mul(mul(p1, s2c), sub(x4b, x7b)), x7b);
            }
            if p1orig >= zero {
                p1 = p5b;
            }
            if sub(mag, s30) >= zero {
                p1 = rdf(gvar(G_P2_BASE).wrapping_add(0x1c));
            }

            // --- envelope combine ---
            let q0 = sub(g3, t164);
            let mut q4 = mul(p1, f5);
            let mut q5 = sub(g3, mul(q0, p1));
            let b6;
            if aux == 0 {
                b6 = f4;
            } else {
                let a20 = rdf(aux.wrapping_add(0x20));
                q4 = mul(q4, a20);
                let r0 = sub(g3, q5);
                b6 = mul(a20, f4);
                q5 = sub(g3, mul(r0, a20));
            }
            if (rd32(this.wrapping_add(0x17ec)) as i32) < 3
                && rd8(voice.wrapping_add(0xd6)) == 0
            {
                let s3 = mul(
                    sub(g3, q5),
                    grdf(0x00e7cba4),
                );
                let s5 = mul(q5, grdf(0x00e7cba8));
                let conv = fistp_i64(add(s3, s5));
                let b0 = rd32(voice.wrapping_add(0xb0));
                wr32(voice.wrapping_add(0xb0), b0.min(conv as u32));
            }
            blend4 = q4;
            blend6 = b6;
        } else {
            blend4 = zero;
            blend6 = zero;
        }

        // --- counter clamp from the manager table ---
        if rd8(this.wrapping_add(0x17e0)) != 0 || rd8(voice.wrapping_add(0xd7)) == 3 {
            let conv = fistp_i64(rdf(this.wrapping_add(0x1730)));
            let b0 = rd32(voice.wrapping_add(0xb0));
            wr32(voice.wrapping_add(0xb0), b0.min(conv as u32));
        }
        if (rd32(this.wrapping_add(0x17ec)) as i32) >= 3 {
            blend4 = zero;
        }
        let enabled = rd8(voice.wrapping_add(0xd4)) != 0;
        let acc = add(add(blend4, blend6), rdf(voice.wrapping_add(0xa8)));
        wrf(voice.wrapping_add(0xa8), acc);
        if enabled {
            lf_checker_rt::callee_thiscall!(6, u32, this, voice, arg1);
        } else {
            wr32(voice.wrapping_add(0x60), 0);
            wr32(voice.wrapping_add(0x64), 0);
            wr32(voice.wrapping_add(0x68), 0);
            wr32(voice.wrapping_add(0x6c), ONE_BITS);
            wr32(voice.wrapping_add(0x30), 0);
            wr32(voice.wrapping_add(0x34), 0);
            wr32(voice.wrapping_add(0x38), 0);
            wr32(voice.wrapping_add(0x3c), 0);
            wr32(voice.wrapping_add(0x40), 0);
            wr32(voice.wrapping_add(0x44), 0);
            wr32(voice.wrapping_add(0x48), 0);
            wr32(voice.wrapping_add(0x4c), 0);
            wr32(voice.wrapping_add(0x50), 0);
            wr32(voice.wrapping_add(0x54), 0);
            wr32(voice.wrapping_add(0x58), 0);
            wr32(voice.wrapping_add(0x5c), 0);
        }

        // --- envelope shaper selection ---
        if rd8(voice.wrapping_add(0xd2)) != 0 {
            lf_checker_rt::callee_thiscall!(7, u32, this, voice, rd32(gvar(G_SEL)));
            wrf(voice.wrapping_add(0xbc), 1.0);
            let d7 = rd8(voice.wrapping_add(0xd7));
            if d7 != 1 && d7 != 2 {
                for i in 0..4u32 {
                    let f = if i == 0 || i == 2 {
                        rdf(this.wrapping_add(0x1724))
                    } else {
                        rdf(this.wrapping_add(0x1728))
                    };
                    let v = rdf(voice.wrapping_add(i.wrapping_mul(4)));
                    wrf(voice.wrapping_add(i.wrapping_mul(4)), mul(v, f));
                }
            }
        } else if (rd32(voice.wrapping_add(0xc0)) as i32) >= 0 {
            lf_checker_rt::callee_thiscall!(8, u32, this, voice, rd32(gvar(G_SEL)));
            wrf(voice.wrapping_add(0xbc), 1.0);
        } else {
            let saved = rd32(gvar(G_SEL));
            lf_checker_rt::callee_thiscall!(9, u32, this, scratch.as_ptr() as u32, voice);
            for i in 0..6u32 {
                if i == 0 || i == 4 {
                    let v = rdf(voice.wrapping_add(i.wrapping_mul(4)));
                    wrf(
                        voice.wrapping_add(i.wrapping_mul(4)),
                        mul(v, rdf(this.wrapping_add(0x1724))),
                    );
                } else if i == 1 || i == 5 {
                    let v = rdf(voice.wrapping_add(i.wrapping_mul(4)));
                    wrf(
                        voice.wrapping_add(i.wrapping_mul(4)),
                        mul(v, rdf(this.wrapping_add(0x1728))),
                    );
                } else {
                    wr32(voice.wrapping_add(i.wrapping_mul(4)), 0);
                }
            }
            if saved == 2 {
                let k4 = grdf(0x00fe8830);
                let v10 = rdf(voice.wrapping_add(0x10));
                let v0 = rdf(voice);
                wr32(voice.wrapping_add(0x10), 0);
                let n0 = mul(add(mul(v10, v10), mul(v0, v0)).sqrt(), k4);
                let v14 = rdf(voice.wrapping_add(0x14));
                wrf(voice, n0);
                let v4 = rdf(voice.wrapping_add(4));
                wr32(voice.wrapping_add(0x14), 0);
                let n4 = mul(add(mul(v4, v4), mul(v14, v14)).sqrt(), k4);
                wrf(voice.wrapping_add(4), n4);
            }
        }

        // --- save accumulators across the spatialisation call ---
        let mut copies = [0.0f32; 12];
        for j in 0..12u32 {
            copies[j as usize] = rdf(voice.wrapping_add(0x30).wrapping_add(j.wrapping_mul(4)));
        }
        lf_checker_rt::callee_thiscall!(10, u32, this, voice);

        // --- mixer loop ---
        if rd8(voice.wrapping_add(0xd4)) != 0 {
            let kmix = rdf(gvar(G_KMIX));
            let mut p = this.wrapping_add(0x17ac);
            let mut dp = voice.wrapping_add(0x3c);
            for i in 0..3u32 {
                for k in 0..4u32 {
                    let f3 = rdf(this.wrapping_add(0x17d0));
                    let tidx = rd16(voice.wrapping_add(0xd0)) as u16 as i16 as i32;
                    let tab = rd32(gvar(G_TABLE));
                    let entry = tab
                        .wrapping_add((tidx.wrapping_mul(9) as u32).wrapping_mul(4))
                        .wrapping_add(0x10);
                    let off = match k {
                        0 => 0u32.wrapping_sub(0x0c),
                        1 => 0,
                        2 => 0x0c,
                        _ => 0x18,
                    };
                    let mut x1 = mul(f3, rdf(dp.wrapping_add(off)));
                    let t = mul(rdf(entry), rdf(this.wrapping_add(0x17d4)));
                    let x0t = mul(t, rdf(p.wrapping_add(off)));
                    if !(x1 > x0t) {
                        x1 = x0t;
                    }
                    let xa = copies[(3u32.wrapping_mul(k).wrapping_add(i)) as usize];
                    let mut x0 = mul(mul(xa, f3), kmix);
                    if !(x0 > x1) {
                        x0 = x1;
                    }
                    wrf(dp.wrapping_add(off), x0);
                }
                p = p.wrapping_add(4);
                dp = dp.wrapping_add(4);
            }
        }

        // --- second spatialisation stage ---
        if rd8(voice.wrapping_add(0xd5)) != 0 && rd8(voice.wrapping_add(0xd4)) != 0 {
            let d0 = sub(rdf(voice.wrapping_add(0x90)), rdf(voice.wrapping_add(0x80)));
            let tls_entry2 = lf_checker_rt::tls_slot(TLS_SLOT);
            let t70 = rd32(tls_entry2.wrapping_add(0x70));
            let v94 = rdf(voice.wrapping_add(0x94));
            let v98 = rdf(voice.wrapping_add(0x98));
            let vb8 = rdf(voice.wrapping_add(0xb8));
            let d1 = sub(v94, rdf(voice.wrapping_add(0x84)));
            let v90 = rdf(voice.wrapping_add(0x90));
            let ea = t70.wrapping_add(arg1.wrapping_add(0x51).wrapping_mul(4));
            let d2 = sub(v98, rdf(voice.wrapping_add(0x88)));
            let ec = t70
                .wrapping_add(arg1.wrapping_mul(4))
                .wrapping_shl(6);
            let ea2 = ea.wrapping_add(ea);
            let ea8 = ea2.wrapping_mul(8);
            let x3 = sub(
                rdf(this.wrapping_add(ec).wrapping_add(0x30)),
                rdf(this.wrapping_add(ea8)),
            );
            let v90b = sub(v90, rdf(this.wrapping_add(ec).wrapping_add(0x30)));
            let x4 = sub(
                rdf(this.wrapping_add(ec).wrapping_add(0x34)),
                rdf(this.wrapping_add(ea8).wrapping_add(4)),
            );
            let x5 = sub(
                rdf(this.wrapping_add(ec).wrapping_add(0x38)),
                rdf(this.wrapping_add(ea8).wrapping_add(8)),
            );
            let v98b = sub(v98, rdf(this.wrapping_add(ec).wrapping_add(0x38)));
            let r0 = sub(d0, x3);
            let r3 = sub(d1, x4);
            let r7 = sub(v94, rdf(this.wrapping_add(ec).wrapping_add(0x34)));
            let r2 = sub(d2, x5);
            let u7 = add(add(mul(r7, r7), mul(v90b, v90b)), mul(v98b, v98b));
            let u3 = add(add(mul(r3, r3), mul(r0, r0)), mul(r2, r2));
            let c11a0 = sub(u7.sqrt(), u3.sqrt());
            // Stack at the call: arg0 = c11a0, arg1 = 0x1e, arg2 = [voice+0xb8]
            // (both `(an instruction of the original)` are mere reservation, overwritten in place).
            let d11: f64 = lf_checker_rt::callee_thiscall!(
                11, f64, this,
                c11a0.to_bits(),
                0x1e,
                vb8.to_bits()
            );
            wrf(voice.wrapping_add(0xbc), d11 as f32);
        }

        // --- tail: aux teardown and accumulator fold-down ---
        let out = rd32(voice.wrapping_add(AUX_PTR));
        if out != 0 && rd8(out.wrapping_add(0x24)) != 0 {
            wr32(voice.wrapping_add(0x18), 0);
            wr32(voice.wrapping_add(0x1c), 0);
            wr32(voice.wrapping_add(0x20), 0);
            wr32(voice.wrapping_add(0x24), 0);
            wr32(voice.wrapping_add(0x28), 0);
            wr32(voice.wrapping_add(0x2c), 0);
        }
        if rd8(this.wrapping_add(0x17f4)) != 0 {
            let k7 = grdf(0x00fe87e4);
            let mut z0 = rdf(voice.wrapping_add(0x3c));
            z0 = add(z0, rdf(voice.wrapping_add(0x30)));
            wr32(voice.wrapping_add(0x3c), 0);
            z0 = add(z0, rdf(voice.wrapping_add(0x48)));
            wr32(voice.wrapping_add(0x48), 0);
            z0 = add(z0, rdf(voice.wrapping_add(0x54)));
            wr32(voice.wrapping_add(0x54), 0);
            wrf(voice.wrapping_add(0x30), mul(z0, k7));
            let mut z1 = rdf(voice.wrapping_add(0x40));
            z1 = add(z1, rdf(voice.wrapping_add(0x34)));
            wr32(voice.wrapping_add(0x40), 0);
            z1 = add(z1, rdf(voice.wrapping_add(0x4c)));
            wr32(voice.wrapping_add(0x4c), 0);
            z1 = add(z1, rdf(voice.wrapping_add(0x58)));
            wr32(voice.wrapping_add(0x58), 0);
            wrf(voice.wrapping_add(0x34), mul(z1, k7));
            let mut z2 = rdf(voice.wrapping_add(0x38));
            z2 = add(z2, rdf(voice.wrapping_add(0x44)));
            wr32(voice.wrapping_add(0x44), 0);
            z2 = add(z2, rdf(voice.wrapping_add(0x50)));
            wr32(voice.wrapping_add(0x50), 0);
            z2 = add(z2, rdf(voice.wrapping_add(0x5c)));
            wr32(voice.wrapping_add(0x5c), 0);
            wrf(voice.wrapping_add(0x38), mul(z2, k7));
        }
        out
    }
});
