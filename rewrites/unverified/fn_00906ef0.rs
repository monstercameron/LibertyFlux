// original: 0x00906EF0 blip_polyline_render (proposed)

/// Render one polyline as a chain of vertex batches, one segment per
/// source point after the first.
///
/// Arguments are `count` points at `pts` (16 bytes apart, two floats each),
/// a color word, two base floats `f1`/`f2`, a flag byte, and two focus
/// floats `f3`/`f4`. The function returns nothing.
///
/// The prologue runs a chain of gate checks, each able to exit early: with
/// the mode byte clear, a gate call plus a pair of state calls (whose second
/// answer points at words compared against zero, the second **signed**
/// greater-than-zero, then a flag byte); a second flag byte gating a query
/// call; a third flag byte when the mode byte is set; and the point count
/// (exits unless signed-greater than 1). With the mode byte set, a further
/// query call either keeps the color or scrambles it with a fixed bit
/// shuffle. A limit slot is then chosen as the smaller of a
/// resolution-derived product and a constant (each side picked by the mode
/// byte), and two anchors are formed by adding the base floats to it.
///
/// The main loop walks segments 1..count. The first segment, when the flag
/// byte is set, projects the running start point onto the segment: the
/// projection factor is clamped at zero and at the segment length, and the
/// start point moves to the projected spot. The start and end points are
/// then ordered (swapping when the start's second coordinate exceeds the
/// end's) and a remember bit records whether the end's first coordinate was
/// already ahead, flipped by the swap.
///
/// Two vertex batches follow per segment: a six-vertex batch, then a
/// four-vertex batch. Each batch fills a vertex buffer from the ordered
/// points, the anchors, the limit slot and the remember bit (two wirings
/// chosen by the bit; zero words come from a frame slot the original never
/// writes, so they read the zero fill), then runs each vertex through the
/// viewport transform with per-vertex resolution scaling. Each batch then
/// either (thread object ready) allocates and constructs draw items,
/// calling each item's virtual slot twice and folding the answers into the
/// item with the same 16-byte alignment arithmetic as the list updater, or
/// (fallback) emits the vertices through the direct nine-word draw call.
/// The loop advances the start point along the array.
///
/// Every exit path, early or late, runs the cookie check call before
/// returning. A failed item allocation faults on a null read, exactly as
/// the original does.
///
/// Original: 0x00906EF0 (cdecl, eight stack words, no return value).
lf_checker_rt::export!(
    cdecl,
    rw_00906EF0(
        count: u32,
        pts: u32,
        color0: u32,
        f1w: u32,
        f2w: u32,
        flagw: u32,
        f3w: u32,
        f4w: u32
    ) -> u32 {
        unsafe {
            const FLAG_G: u32 = 0x11609f6;
            const FLAG2_G: u32 = 0x11db23b;
            const FLAG3_G: u32 = 0x11a2ea0;
            const FLAG4_G: u32 = 0x11e622f;
            const RESF_A: u32 = 0x11609cc;
            const RESF_B: u32 = 0x118f4b0;
            const RC_A: u32 = 0xe84cd0;
            const RC_B: u32 = 0xe84cd4;
            const LIM_A: u32 = 0xfe8b0c;
            const LIM_B: u32 = 0xfe8b38;
            const TLS_INDEX_G: u32 = 0x17aba14;
            const TLS_READY_OFF: u32 = 0x8cc;
            const RES_A: u32 = 0x105c884;
            const RES_B: u32 = 0x105c888;
            const RES_C: u32 = 0x105c880;
            const RES_D: u32 = 0x105c87c;
            const C0_G: u32 = 0xfe88e8;
            const ADJ_C: u32 = 0xfe8830;
            const HANDLE_CTR_G: u32 = 0x10327a0;
            const VT_FIRST: u32 = 0xe7e048;
            const VT_FINAL: u32 = 0xe84c78;
            const ITEM_KIND: u32 = 0x59d8b0;
            const VT_SLOT: u32 = 8;
            const VP_CONST: u32 = 0x1190e70;
            const NEG_ONE: u32 = 0xbf800000;
            const ONE_BITS: u32 = 0x3f800000;

            const A_GATE: u32 = 1;
            const B_GX: u32 = 2;
            const C_Q: u32 = 3;
            const D_Q0: u32 = 4;
            const E_XFORM: u32 = 5;
            const F_QZ: u32 = 6;
            const G_ALLOC: u32 = 7;
            const H_CTOR: u32 = 8;
            const J_LEG: u32 = 10;
            const K_AB: u32 = 11;
            const L_SETUP: u32 = 12;
            const M_DRAW: u32 = 13;
            const N_END: u32 = 14;
            const O_REL: u32 = 15;
            const P_COOKIE: u32 = 16;

            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
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

            /// Fixed bit shuffle of the color word.
            #[inline(always)]
            fn munge(color: u32) -> u32 {
                let mut edx = (color >> 16) & 0xff;
                edx &= 0xfffffffe;
                let c = (color >> 8) & 0xff;
                edx = (edx << 8) | c;
                edx &= 0xfffffffe;
                let a = color & 0xff;
                edx |= 0xfffe0000;
                edx = edx.wrapping_shl(7);
                edx | (a >> 1)
            }

            /// Build a 16-byte item tagged 6 (same layout as the list
            /// updater's items).
            #[inline(always)]
            unsafe fn construct10(obj: u32) {
                unsafe {
                    let saved = rd32(obj + 4);
                    wr32(obj, lf_checker_rt::relocated(VT_FIRST));
                    let ctr = rd32(lf_checker_rt::relocated(HANDLE_CTR_G));
                    let mix = (saved ^ ctr) & 0x3fff;
                    wr32(obj + 4, saved ^ mix);
                    wr32(
                        lf_checker_rt::relocated(HANDLE_CTR_G),
                        ctr.wrapping_add(1),
                    );
                    wr32(obj, lf_checker_rt::relocated(VT_FINAL));
                    wr32(obj + 8, lf_checker_rt::relocated(ITEM_KIND));
                    wr32(obj + 0xc, 6);
                }
            }

            /// Item alignment fixup: two virtual-slot calls folded into the
            /// buffer word. Reads the table pointer unconditionally, so a
            /// null item faults on address 0 exactly like the original.
            #[inline(always)]
            unsafe fn align_fixup(obj: u32) {
                unsafe {
                    let load_hook = |o: u32| unsafe {
                        let slot: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(
                                rd32(rd32(o).wrapping_add(VT_SLOT)) as usize,
                            );
                        slot
                    };
                    let first = load_hook(obj)(obj);
                    let rem = (first as i32) % 16;
                    let padded = 16i32 - rem;
                    let pad = (padded % 16) as u32;
                    let second = load_hook(obj)(obj);
                    let total = second.wrapping_add(pad);
                    let scaled = ((total as i32) / 16) as u32;
                    let folded = scaled.wrapping_shl(14);
                    let buf = rd32(obj + 4);
                    let tmp = (buf ^ folded) & 0x01ffc000;
                    wr32(obj + 4, buf ^ tmp);
                }
            }

            /// One frame word of the draw-call area by absolute offset:
            /// output constants, the zero gap word, or batch outputs.
            #[inline(always)]
            fn wslot(off: u32, out8: &[u32; 8], vout: &[u32; 14]) -> u32 {
                if (0x12c..0x14c).contains(&off) {
                    out8[((off - 0x12c) / 4) as usize]
                } else if off == 0x14c {
                    0
                } else if (0x150..0x188).contains(&off) {
                    vout[((off - 0x150) / 4) as usize]
                } else {
                    0
                }
            }

            let f1 = f32::from_bits(f1w);
            let f2 = f32::from_bits(f2w);
            let f3 = f32::from_bits(f3w);
            let f4 = f32::from_bits(f4w);
            let flagbyte = (flagw & 0xff) as u8;
            let mut color = color0;

            let run_main = 'gates: {
                let mut al = rd8(lf_checker_rt::relocated(FLAG_G));
                if al == 0 {
                    let r = lf_checker_rt::callee_cdecl!(A_GATE, u32, 0);
                    if r == 0 {
                        let r2 = lf_checker_rt::callee_cdecl!(B_GX, u32,);
                        if r2 != 0 {
                            let r3 = lf_checker_rt::callee_cdecl!(B_GX, u32,);
                            let p = r3.wrapping_add(0x80);
                            if rd32(p.wrapping_add(0x30)) != 0 {
                                let w38 = rd32(p.wrapping_add(0x38));
                                let over = (w38 as i32) > 0;
                                if over {
                                    break 'gates None;
                                }
                            }
                            if rd8(lf_checker_rt::relocated(FLAG2_G)) != 0 {
                                break 'gates None;
                            }
                        }
                    }
                    al = rd8(lf_checker_rt::relocated(FLAG_G));
                }
                if rd8(lf_checker_rt::relocated(FLAG3_G)) != 0 {
                    let q = lf_checker_rt::callee_cdecl!(C_Q, u32,) & 0xff;
                    if q == 0 {
                        break 'gates None;
                    }
                    al = rd8(lf_checker_rt::relocated(FLAG_G));
                }
                if al != 0 && rd8(lf_checker_rt::relocated(FLAG4_G)) != 0 {
                    break 'gates None;
                }
                if (count as i32) <= 1 {
                    break 'gates None;
                }
                let slot: f32;
                if al == 0 {
                    let x = mul(
                        rdf(lf_checker_rt::relocated(RESF_B)),
                        rdf(lf_checker_rt::relocated(RC_B)),
                    );
                    let y = rdf(lf_checker_rt::relocated(LIM_B));
                    slot = if y > x { x } else { y };
                } else {
                    let r = lf_checker_rt::callee_cdecl!(D_Q0, u32, 0) & 0xff;
                    if r == 0 {
                        color = munge(color);
                    }
                    if rd8(lf_checker_rt::relocated(FLAG_G)) == 0 {
                        let x = mul(
                            rdf(lf_checker_rt::relocated(RESF_B)),
                            rdf(lf_checker_rt::relocated(RC_B)),
                        );
                        let y = rdf(lf_checker_rt::relocated(LIM_B));
                        slot = if y > x { x } else { y };
                    } else {
                        let x = mul(
                            rdf(lf_checker_rt::relocated(RESF_A)),
                            rdf(lf_checker_rt::relocated(RC_A)),
                        );
                        let y = rdf(lf_checker_rt::relocated(LIM_A));
                        slot = if y > x { x } else { y };
                    }
                }
                Some(slot)
            };
            if run_main.is_none() {
                lf_checker_rt::callee_cdecl!(P_COOKIE, u32,);
                return 0;
            }
            let slot = run_main.unwrap();
            // Dead recheck from the original (count is unchanged since the
            // gate above); kept so every exit funnels identically.
            if (count as i32) <= 1 {
                lf_checker_rt::callee_cdecl!(P_COOKIE, u32,);
                return 0;
            }

            let slot_idx = rd32(lf_checker_rt::relocated(TLS_INDEX_G));
            let tlsval = lf_checker_rt::tls_slot(slot_idx as usize);
            let a0 = add(slot, f2);
            let a1 = add(slot, f1);
            // Per-iteration output constants (the prologue's frame words,
            // never rewritten in the loop).
            let out8: [u32; 8] = [0, ONE_BITS, 0, 0, ONE_BITS, ONE_BITS, ONE_BITS, 0];
            let mut x0 = rdf(pts);
            let mut y0 = rdf(pts.wrapping_add(4));
            let mut it = 1u32;
            let c0 = rdf(lf_checker_rt::relocated(C0_G));
            let adj_c = rdf(lf_checker_rt::relocated(ADJ_C));
            while (it as i32) < (count as i32) {
                let off = it.wrapping_mul(16);
                let px = rdf(pts.wrapping_add(off));
                let py = rdf(pts.wrapping_add(off + 4));
                let mut cx0 = x0;
                let mut cy0 = y0;
                if it == 1 && flagbyte != 0 {
                    let dx = sub(x0, px);
                    let dy = sub(y0, py);
                    let s = add(mul(dy, dy), mul(dx, dx));
                    let len = s.sqrt();
                    let t = div(c0, len);
                    let sub0 = sub(f4, py);
                    let sub1 = sub(f3, px);
                    let mut v = add(mul(dy, sub0), mul(dx, sub1));
                    v = mul(v, t);
                    if 0.0 > v {
                        v = 0.0;
                    }
                    if len > v {
                        let w = mul(t, v);
                        let nx = add(mul(w, dx), px);
                        let ny = add(mul(w, dy), py);
                        cx0 = nx;
                        cy0 = ny;
                        x0 = nx;
                        y0 = ny;
                    }
                }
                let mut cl: u8 = if px > cx0 { 1 } else { 0 };
                let mut sx = cx0;
                let mut sy = cy0;
                let mut qx = px;
                let mut qy = py;
                if cy0 > py {
                    sx = px;
                    sy = py;
                    qx = cx0;
                    qy = cy0;
                    cl = if cl == 0 { 1 } else { 0 };
                }
                // Vertex fill A: 24 words plus the struct word and w60.
                let t0 = add(a0, qy);
                let t1b = add(sub(qy, slot), f2);
                let t2 = add(a0, sy);
                let e58 = t0;
                let mut vbuf = [0u32; 28];
                vbuf[1] = t1b.to_bits();
                vbuf[5] = t0.to_bits();
                vbuf[9] = t2.to_bits();
                vbuf[21] = t1b.to_bits();
                let v20: f32;
                let w60: f32;
                if cl != 0 {
                    let u0 = add(sub(qx, slot), f1);
                    v20 = u0;
                    vbuf[0] = u0.to_bits();
                    vbuf[4] = u0.to_bits();
                    let u1 = add(sub(sx, slot), f1);
                    vbuf[8] = u1.to_bits();
                    vbuf[12] = u1.to_bits();
                    vbuf[16] = add(a1, sx).to_bits();
                    w60 = add(a1, qx);
                } else {
                    let w1p = add(a1, qx);
                    let w0p = add(a1, sx);
                    w60 = w1p;
                    vbuf[0] = w1p.to_bits();
                    vbuf[4] = w1p.to_bits();
                    vbuf[8] = w0p.to_bits();
                    vbuf[12] = w0p.to_bits();
                    vbuf[16] = add(sub(sx, slot), f1).to_bits();
                    v20 = add(sub(qx, slot), f1);
                }
                let y0c = add(sub(sy, slot), f2);
                vbuf[20] = (if cl != 0 { w60 } else { v20 }).to_bits();
                vbuf[17] = y0c.to_bits();
                // Vertex loop A: six vertices through the transform.
                let estruct = [
                    0u32,
                    0u32,
                    e58.to_bits(),
                    a1.to_bits(),
                    w60.to_bits(),
                    0u32,
                ];
                let mut vout = [0u32; 14];
                for edi in 0..6u32 {
                    let bp = vbuf.as_ptr() as u32 + edi.wrapping_mul(16);
                    lf_checker_rt::callee_cdecl!(
                        E_XFORM,
                        u32,
                        bp,
                        estruct.as_ptr() as u32,
                        lf_checker_rt::relocated(VP_CONST),
                        1
                    );
                    vout[(edi.wrapping_mul(2)) as usize] = 0;
                    vout[(edi.wrapping_mul(2) + 3) as usize] = w60.to_bits();
                    let fa =
                        lf_checker_rt::callee_cdecl!(F_QZ, u32,) & 0xff;
                    let esi = if fa != 0 {
                        rd32(lf_checker_rt::relocated(RES_B))
                    } else {
                        rd32(lf_checker_rt::relocated(RES_A))
                    };
                    let fb =
                        lf_checker_rt::callee_cdecl!(F_QZ, u32,) & 0xff;
                    let ecx = if fb != 0 {
                        rd32(lf_checker_rt::relocated(RES_D))
                    } else {
                        rd32(lf_checker_rt::relocated(RES_C))
                    };
                    let r = div((esi as i32) as f32, (ecx as i32) as f32);
                    if c0 > r && rd8(lf_checker_rt::relocated(FLAG_G)) != 0 {
                        let old =
                            f32::from_bits(vout[(edi.wrapping_mul(2)) as usize]);
                        let v =
                            add(sub(adj_c, mul(r, adj_c)), mul(old, r));
                        vout[(edi.wrapping_mul(2)) as usize] = v.to_bits();
                    }
                }
                if rd32(tlsval.wrapping_add(TLS_READY_OFF)) != 0 {
                    let o1 = lf_checker_rt::callee_cdecl!(G_ALLOC, u32, 0x10, 0);
                    if o1 != 0 {
                        construct10(o1);
                    }
                    align_fixup(o1);
                    let o2 = lf_checker_rt::callee_cdecl!(G_ALLOC, u32, 0x78, 0);
                    if o2 != 0 {
                        let h0 = [
                            0u32, vout[0], vout[1], vout[2], vout[3], vout[4],
                            vout[5], vout[6],
                        ];
                        let built = lf_checker_rt::callee_thiscall!(
                            H_CTOR,
                            u32,
                            o2,
                            h0.as_ptr() as u32,
                            out8.as_ptr() as u32,
                            0,
                            color,
                            5,
                            6
                        );
                        align_fixup(built);
                    } else {
                        align_fixup(0);
                    }
                } else {
                    lf_checker_rt::callee_cdecl!(J_LEG, u32, 2, 6);
                    lf_checker_rt::callee_cdecl!(K_AB, u32,);
                    lf_checker_rt::callee_cdecl!(L_SETUP, u32, 5, 6);
                    for k in 0..6u32 {
                        let es = k.wrapping_mul(8);
                        lf_checker_rt::callee_cdecl!(
                            M_DRAW,
                            u32,
                            wslot(0x14c + es, &out8, &vout),
                            wslot(0x138 + es, &out8, &vout),
                            0,
                            0,
                            0,
                            NEG_ONE,
                            color,
                            wslot(0x12c + es, &out8, &vout),
                            wslot(0x130 + es, &out8, &vout)
                        );
                    }
                    lf_checker_rt::callee_cdecl!(N_END, u32,);
                    lf_checker_rt::callee_cdecl!(O_REL, u32,);
                }
                // Vertex fill B rewires the first 16 buffer words.
                vbuf[0] = v20.to_bits();
                vbuf[1] = t0.to_bits();
                vbuf[4] = w60.to_bits();
                vbuf[5] = t0.to_bits();
                vbuf[8] = w60.to_bits();
                vbuf[9] = t1b.to_bits();
                vbuf[12] = v20.to_bits();
                vbuf[13] = t1b.to_bits();
                for edi in 0..4u32 {
                    let bp = vbuf.as_ptr() as u32 + edi.wrapping_mul(16);
                    lf_checker_rt::callee_cdecl!(
                        E_XFORM,
                        u32,
                        bp,
                        estruct.as_ptr() as u32,
                        lf_checker_rt::relocated(VP_CONST),
                        1
                    );
                    vout[(edi.wrapping_mul(2)) as usize] = 0;
                    vout[(edi.wrapping_mul(2) + 3) as usize] = w60.to_bits();
                    let fa =
                        lf_checker_rt::callee_cdecl!(F_QZ, u32,) & 0xff;
                    let esi = if fa != 0 {
                        rd32(lf_checker_rt::relocated(RES_B))
                    } else {
                        rd32(lf_checker_rt::relocated(RES_A))
                    };
                    let fb =
                        lf_checker_rt::callee_cdecl!(F_QZ, u32,) & 0xff;
                    let ecx = if fb != 0 {
                        rd32(lf_checker_rt::relocated(RES_D))
                    } else {
                        rd32(lf_checker_rt::relocated(RES_C))
                    };
                    let r = div((esi as i32) as f32, (ecx as i32) as f32);
                    if c0 > r && rd8(lf_checker_rt::relocated(FLAG_G)) != 0 {
                        let old =
                            f32::from_bits(vout[(edi.wrapping_mul(2)) as usize]);
                        let v =
                            add(sub(adj_c, mul(r, adj_c)), mul(r, old));
                        vout[(edi.wrapping_mul(2)) as usize] = v.to_bits();
                    }
                }
                if rd32(tlsval.wrapping_add(TLS_READY_OFF)) != 0 {
                    let o2 = lf_checker_rt::callee_cdecl!(G_ALLOC, u32, 0x78, 0);
                    if o2 != 0 {
                        let h0 = [
                            0u32, vout[0], vout[1], vout[2], vout[3], vout[4],
                            vout[5], vout[6],
                        ];
                        let built = lf_checker_rt::callee_thiscall!(
                            H_CTOR,
                            u32,
                            o2,
                            h0.as_ptr() as u32,
                            out8.as_ptr() as u32,
                            0,
                            color,
                            5,
                            4
                        );
                        align_fixup(built);
                    } else {
                        align_fixup(0);
                    }
                } else {
                    lf_checker_rt::callee_cdecl!(K_AB, u32,);
                    lf_checker_rt::callee_cdecl!(L_SETUP, u32, 5, 4);
                    for k in 0..4u32 {
                        let es = k.wrapping_mul(8);
                        lf_checker_rt::callee_cdecl!(
                            M_DRAW,
                            u32,
                            wslot(0x14c + es, &out8, &vout),
                            wslot(0x138 + es, &out8, &vout),
                            0,
                            0,
                            0,
                            NEG_ONE,
                            color,
                            wslot(0x12c + es, &out8, &vout),
                            wslot(0x130 + es, &out8, &vout)
                        );
                    }
                    lf_checker_rt::callee_cdecl!(N_END, u32,);
                    lf_checker_rt::callee_cdecl!(O_REL, u32,);
                }
                x0 = rdf(pts.wrapping_add(off));
                y0 = rdf(pts.wrapping_add(off + 4));
                it = it.wrapping_add(1);
            }
            lf_checker_rt::callee_cdecl!(P_COOKIE, u32,);
            0
        }
    }
);
