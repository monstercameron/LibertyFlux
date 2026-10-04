// original: 0x009277f0 input_ui_proximity_scan (proposed)

/// Scan the entity array for records near a target point and refresh the
/// proximity tables.
///
/// With no arguments and no result, everything flows through globals. The
/// target point comes from a self-indexed table (`P1`): its first word is
/// an index selecting one of the following words as the record holding
/// three scaled floats plus three offsets. Each entity record (0x80 bytes
/// at `ENT`, `BOUND` of them) whose flag bits 1-2 are set is either
/// cleared (when the validity call answers 0 and bit 8 is set) or measured:
/// the squared distance to the target, square-rooted, minus a threshold,
/// clamped at zero from below. The best record feeds a lookup call whose
/// answer selects a table row; eight (distance, row, tag) triples are
/// insertion-sorted through a rank call, and a final pass rewrites the
/// display rows. Two counters advance (one wrapping modulo 8), and the
/// security-cookie check runs on every exit. A successful lookup skips the
/// counter and the table-state blocks and jumps straight to the final pass.
///
/// Float order is the original's: lane products before sums, destination
/// operand first, unordered comparisons falling through to the complement
/// branch. All pure moves copy bits, never converting. The divisor call's
/// answer is never zero in the proof (a zero divisor raises #DE in the
/// original but aborts in the rewrite, and the checker requires the same
/// fault code).
///
/// Original: 0x009277f0 (cdecl, no stack words; the inventory size 2418 is
/// 9 bytes short and cuts the last block mid-instruction, the true size is
/// 2427).
lf_checker_rt::export!(cdecl, rw_009277f0() -> u32 {
    unsafe {
        const CALLEE_GATE: u32 = 1;
        const CALLEE_PICK: u32 = 2;
        const CALLEE_DIV: u32 = 3;
        const CALLEE_VALID: u32 = 4;
        const CALLEE_RANK: u32 = 5;
        const CALLEE_LOOKUP: u32 = 6;
        const CALLEE_COOKIE: u32 = 7;

        const G_FLAG: u32 = 0x01036780;
        const G_SCALE: u32 = 0x01036784;
        const G_B1: u32 = 0x0105c884;
        const G_B1B: u32 = 0x0105c888;
        const G_B0: u32 = 0x0105c880;
        const G_B0B: u32 = 0x0105c87c;
        const G_ENT: u32 = 0x0103eed8;
        const G_P1: u32 = 0x0118d818;
        const G_CNT: u32 = 0x0119cfe0;
        const G_F1A: u32 = 0x0119d008;
        const G_F1B: u32 = 0x0119d00c;
        const G_NEG: u32 = 0x0119d08c;
        const G_DL: u32 = 0x0119d011;
        const G_CYC: u32 = 0x0119d098;
        const G_SEL: u32 = 0x0119d030;
        const G_BOUND: u32 = 0x0150e240;
        const G_THRESH: u32 = 0x00fe8ab8;
        const G_K0: u32 = 0x00e863a0;
        const G_C6: u32 = 0x00e863c0;
        const G_W: u32 = 0x00e863e0;

        const BLK: u32 = 0x0119f1c0;
        const TAGA: u32 = 0x0119f1f8;
        const TAGB: u32 = 0x0119f1f4;
        const TAGF: u32 = 0x0119f1ec;
        const TAGC: u32 = 0x0119f1fc;
        const TAGD: u32 = 0x0119f200;
        const TAGV: u32 = 0x0119f1f0;
        const SLOTB0: u32 = 0x0119f1ed;
        const SLOT0: u32 = 0x0119f1f8;
        const SLOT_END: u32 = 0x0119fa78;
        const CLR0: u32 = 0x0119f2d8;
        const CLR_END: u32 = 0x0119fa48;
        const TAB_BEC: u32 = 0x011a0bec;
        const TAB_BF0: u32 = 0x011a0bf0;
        const TAB_BF4: u32 = 0x011a0bf4;
        const TAB_BFC: u32 = 0x011a0bfc;
        const TAB_LOOP: u32 = 0x011a0cf4;
        const TAB_LOOP_END: u32 = 0x011a14f4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn g(va: u32) -> u32 {
            unsafe { lf_checker_rt::relocated(va) }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn shufps(a: [u32; 4], b: [u32; 4], imm: u8) -> [u32; 4] {
            [
                a[(imm & 3) as usize],
                a[((imm >> 2) & 3) as usize],
                b[((imm >> 4) & 3) as usize],
                b[((imm >> 6) & 3) as usize],
            ]
        }
        #[inline(always)]
        fn unpckhps(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
            [a[2], b[2], a[3], b[3]]
        }
        #[inline(always)]
        unsafe fn read4(va: u32) -> [u32; 4] {
            unsafe {
                let p = lf_checker_rt::global::<u32>(va);
                [
                    p.add(0).read_unaligned(),
                    p.add(1).read_unaligned(),
                    p.add(2).read_unaligned(),
                    p.add(3).read_unaligned(),
                ]
            }
        }
        #[inline(always)]
        unsafe fn cookie() {
            unsafe { lf_checker_rt::callee_cdecl!(CALLEE_COOKIE, u32,) };
        }
        /// Copy one entity record's display fields into the output block.
        #[inline(always)]
        unsafe fn emit_block(e: u32) {
            unsafe {
                wr32(g(BLK), rd32(e + 0x20));
                wr32(g(BLK + 4), rd32(e + 0x24));
                wr32(g(BLK + 8), rd32(e + 0x28));
                wr32(g(BLK + 0x0c), rd32(e + 0x2c));
                wr32(g(BLK + 0x10), rd32(e));
                wr32(g(BLK + 0x14), rd32(e + 4));
                wr32(g(BLK + 0x18), rd32(e + 8));
                wr32(g(BLK + 0x1c), rd32(e + 0x0c));
                wr32(g(BLK + 0x20), rd32(e + 0x54));
            }
        }

        if rd8(g(G_FLAG)) == 0 {
            cookie();
            return 0;
        }
        let gate: u32 = lf_checker_rt::callee_cdecl!(CALLEE_GATE, u32,);
        if gate & 0xff == 0 {
            for k in 0..8u32 {
                wr32(g(SLOT0 + k * 0x110), 0);
                wr8(g(SLOT0 - 0x0b + k * 0x110), 0);
            }
            cookie();
            return 0;
        }

        let pick1: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PICK, u32,);
        let v1 = if pick1 & 0xff != 0 { rd32(g(G_B1B)) } else { rd32(g(G_B1)) };
        wrf(g(G_F1A), (v1 as i32) as f32);
        let pick0: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PICK, u32,);
        let v0 = if pick0 & 0xff != 0 { rd32(g(G_B0B)) } else { rd32(g(G_B0)) };
        let p1b = g(G_P1);
        let p2 = rd32(p1b.wrapping_add(rd32(p1b).wrapping_mul(4)));
        wrf(g(G_F1B), (v0 as i32) as f32);

        let s = rdf(g(G_SCALE));
        let l30 = fadd(rdf(p2 + 0x80), fmul(rdf(p2 + 0x70), s));
        let l4c = fadd(rdf(p2 + 0x84), fmul(rdf(p2 + 0x74), s));
        let l50 = fadd(rdf(p2 + 0x88), fmul(rdf(p2 + 0x78), s));

        let cnt = rd32(g(G_CNT)).wrapping_add(1);
        wr32(g(G_CNT), cnt);
        wr32(g(G_NEG), 0xffff_ffff);
        let cdiv: u32 = lf_checker_rt::callee_cdecl!(CALLEE_DIV, u32,);
        if cnt % cdiv != 0 {
            cookie();
            return 0;
        }

        let k = read4(G_K0);
        let c6 = read4(G_C6);
        let w = read4(G_W);
        let x4 = shufps(c6, c6, 0xe5);
        let x5 = unpckhps(x4, x4);
        let x7 = unpckhps(x5, x5);
        // The original initializes only the first 7 entries of each row;
        // the 8th reads uninitialized stack (zero under the proof's fill).
        let mut s5 = [f32::from_bits(0xbf80_0000); 8];
        s5[7] = 0.0;
        let mut s7 = [0xffff_ffffu32; 8];
        s7[7] = 0;
        let mut s9 = [0u32; 8];

        let mut tbase = TAB_LOOP;
        while (tbase as i32) < TAB_LOOP_END as i32 {
            let a = rd32(g(tbase.wrapping_sub(0x108)));
            let b = rd32(g(tbase + 0xf8));
            let c = rd32(g(tbase.wrapping_sub(8)));
            let d = rd32(g(tbase + 0x1f8));
            // The three unpacks interleave ([A,B] with [C,D]) lane by lane,
            // so the summed lanes run [A,C,B,D], not [A,B,C,D].
            let v = [
                a.wrapping_add(k[0]),
                c.wrapping_add(k[1]),
                b.wrapping_add(k[2]),
                d.wrapping_add(k[3]),
            ];
            wr32(g(tbase + 0x200), w[3]);
            wr32(g(tbase + 0x204), w[3]);
            wr32(g(tbase.wrapping_sub(0x108)), v[0]);
            wr32(g(tbase.wrapping_sub(0xf4)), 0);
            wr32(g(tbase.wrapping_sub(8)), v[1]);
            wr32(g(tbase + 0x0c), 0);
            wr32(g(tbase + 0x10c), 0);
            wr32(g(tbase + 0xf8), v[2]);
            wr32(g(tbase + 0x20c), 0);
            wr32(g(tbase.wrapping_sub(0x100)), w[0]);
            wr32(g(tbase), w[1]);
            wr32(g(tbase + 0x100), w[2]);
            wr32(g(tbase.wrapping_sub(0xfc)), w[0]);
            wr32(g(tbase + 4), w[1]);
            wr32(g(tbase + 0x104), w[2]);
            wr32(g(tbase + 0x1f8), v[3]);
            wr32(g(tbase.wrapping_sub(0x114)), c6[0]);
            wr32(g(tbase.wrapping_sub(0x14)), x4[0]);
            wr32(g(tbase + 0xec), x5[0]);
            wr32(g(tbase + 0x1ec), x7[0]);
            tbase = tbase.wrapping_add(0x400);
        }

        let bound = rd32(g(G_BOUND)) as i32;
        let ebase = rd32(g(G_ENT));
        let mut best_idx = 0xffff_ffffu32;
        let mut best_val = 0.0f32;
        let mut l20 = 0.0f32;
        let mut l24 = 0u32;
        if bound > 0 {
            let mut esi = 0u32;
            let mut edx = 0u32;
            loop {
                let ent = ebase.wrapping_add(esi);
                let flags = rd32(ent + 0x48);
                if flags & 6 != 0 {
                    let f2 = flags & 2;
                    let (b12, b13) = if f2 == 0 {
                        (1u8, 0u8)
                    } else if flags & 4 != 0 {
                        (0u8, 0u8)
                    } else {
                        (0u8, 1u8)
                    };
                    let dr: u32 = lf_checker_rt::callee_cdecl!(CALLEE_VALID, u32,);
                    if dr & 0xff == 0 && rd32(ent + 0x48) & 0x100 != 0 {
                        let zslot = rd32(ent + 0x64)
                            .wrapping_shl(8)
                            .wrapping_add(TAB_BF0);
                        wr32(g(zslot), 0);
                        let tag = rd32(ent + 0x60);
                        let mut a = SLOT0;
                        while a < SLOT_END {
                            if rd32(g(a)) == tag {
                                let kf = (a - SLOT0) / 0x110;
                                wr32(g(SLOT0 + kf * 0x110), 0);
                                wr8(g(SLOT0 - 0x0b + kf * 0x110), 0);
                                break;
                            }
                            a += 0x110;
                        }
                    } else {
                        let dx = fsub(l30, rdf(ent + 0x20));
                        let dy = fsub(l4c, rdf(ent + 0x24));
                        let dz = fsub(l50, rdf(ent + 0x28));
                        let dy2 = fmul(dy, dy);
                        let dx2 = fmul(dx, dx);
                        let dz2 = fmul(dz, dz);
                        let d2 = fadd(fadd(dy2, dx2), dz2);
                        let dist = core::hint::black_box(d2).sqrt();
                        l20 = dist;
                        let mut x0 = fsub(dist, rdf(g(G_THRESH)));
                        if 0.0 > x0 {
                            x0 = 0.0;
                        }
                        let ent64 = rd32(ent + 0x64);
                        if b12 == 0 && ent64 != 0xffff_ffff {
                            let slot =
                                ent64.wrapping_shl(8).wrapping_add(TAB_BF4);
                            wr32(g(slot), 1);
                            wr32(g(slot + 4), edx);
                            wrf(g(slot - 0x14), x0);
                            wr32(g(slot + 0x0c), 0);
                        }
                        // 0 = next record, 1 = rank loop, 2 = best update.
                        let action = if b13 == 1 {
                            if ent64 == 0xffff_ffff { 2 } else { 0 }
                        } else if b12 == 1 {
                            1
                        } else if ent64 == 0xffff_ffff {
                            2
                        } else {
                            1
                        };
                        if action == 1 {
                            let mut chain = flags;
                            l24 = edx;
                            for kk in 0..8usize {
                                let slotv = s9[kk];
                                let r: u32 =
                                    lf_checker_rt::callee_cdecl!(CALLEE_RANK, u32, chain, slotv);
                                if r == 0 || (r == 1 && s5[kk] > l20) {
                                    let t0 = s5[kk];
                                    let t1 = s7[kk];
                                    s9[kk] = chain;
                                    chain = slotv;
                                    s5[kk] = l20;
                                    s7[kk] = l24;
                                    l20 = t0;
                                    l24 = t1;
                                }
                            }
                        } else if action == 2 {
                            if best_idx == 0xffff_ffff || best_val > x0 {
                                best_val = x0;
                                best_idx = edx;
                            }
                        }
                    }
                }
                edx += 1;
                esi = esi.wrapping_add(0x80);
                if !((edx as i32) < bound) {
                    break;
                }
            }
        }

        let dl = rd8(g(G_DL));
        // A successful lookup jumps straight to the final pass, skipping
        // the counter and the table-state blocks.
        let mut skip_mid = false;
        if dl == 0 && best_idx != 0xffff_ffff {
            let fr: u32 = lf_checker_rt::callee_cdecl!(CALLEE_LOOKUP, u32, best_val.to_bits());
            if fr != 0xffff_ffff {
                let e = ebase.wrapping_add(best_idx.wrapping_shl(7));
                emit_block(e);
                wr32(g(TAGV), fr);
                let t = fr.wrapping_shl(8).wrapping_add(TAB_BEC);
                wr32(g(TAGA), rd32(e + 0x60));
                wr32(g(TAGB), 0xffff_ffff);
                unsafe { (g(TAGF) as *mut u16).write_unaligned(0x0100) };
                wr32(g(TAGC), 3);
                wr32(g(TAGD), 0);
                wr32(g(t), 0);
                wr32(g(t + 4), rd32(e + 0x60));
                wr32(g(t + 8), 3);
                wr32(g(t + 0x0c), 0xffff_ffff);
                wr32(g(t + 0x10), 0xffff_ffff);
                wr32(g(t + 0x14), 0);
                skip_mid = true;
            }
        }

        if !skip_mid {
            let c0 = rd32(g(G_CYC)).wrapping_add(1);
            let mut c = c0 & 0x8000_0007;
            if (c as i32) < 0 {
                c = c.wrapping_sub(1) | 0xffff_fff8;
                c = c.wrapping_add(1);
            }
            wr32(g(G_CYC), c);

            let mut sel = c;
            if dl != 0 {
                sel = rd32(g(G_SEL));
                wr32(g(sel.wrapping_shl(8).wrapping_add(TAB_BFC)), 0xffff_ffff);
            }
            let t2 = sel.wrapping_shl(8).wrapping_add(TAB_BF4);
            if rd32(g(t2)) == 1 {
                let ei = rd32(g(t2 + 4));
                let e = ebase.wrapping_add(ei.wrapping_shl(7));
                emit_block(e);
                wr32(g(TAGA), rd32(e + 0x60));
                let prev = rd32(g(t2 + 8));
                wr32(g(TAGV), sel);
                wr32(g(TAGB), prev);
                unsafe { (g(TAGF) as *mut u16).write_unaligned(0x0100) };
                wr32(g(TAGC), 3);
                wr32(g(TAGD), 0);
                wr32(g(t2), 2);
                wr32(g(t2 + 0x0c), 0);
            } else {
                wr8(g(SLOTB0), 0);
            }
        }

        let mut fp = 0usize;
        let mut s = CLR0;
        while (s as i32) < CLR_END as i32 {
            let fv = s7[fp];
            if fv != 0xffff_ffff {
                let dr: u32 = lf_checker_rt::callee_cdecl!(CALLEE_VALID, u32,);
                let eb = rd32(g(G_ENT));
                let e = eb.wrapping_add(fv.wrapping_shl(7));
                if dr & 0xff != 0 || rd32(e + 0x48) & 0x100 == 0 {
                    wr32(g(s + 0x28), rd32(e + 0x64));
                    let dl2 =
                        if (rd32(e + 0x6c) as i32) > 0 && rd32(e + 0x68) != 0xffff_ffff {
                            1u8
                        } else {
                            0u8
                        };
                    wr32(g(s.wrapping_sub(8)), rd32(e + 0x20));
                    wr32(g(s.wrapping_sub(4)), rd32(e + 0x24));
                    wr32(g(s), rd32(e + 0x28));
                    wr32(g(s + 4), rd32(e + 0x2c));
                    wr32(g(s + 8), rd32(e));
                    wr32(g(s + 0x0c), rd32(e + 4));
                    wr32(g(s + 0x10), rd32(e + 8));
                    wr32(g(s + 0x14), rd32(e + 0x0c));
                    wr32(g(s + 0x18), rd32(e + 0x54));
                    wr32(g(s + 0x30), rd32(e + 0x60));
                    wr8(g(s + 0x25), 1);
                    wr8(g(s + 0x24), dl2);
                    wr32(g(s + 0x34), 4);
                    wr32(g(s + 0x38), 0);
                } else {
                    wr32(e + 0x64, 0xffff_ffff);
                    wr32(g(s + 0x28), 0xffff_ffff);
                    wr8(g(s + 0x25), 0);
                }
            } else {
                wr8(g(s + 0x25), 0);
            }
            s += 0x110;
            fp += 1;
        }
        cookie();
        0
    }
});
