// original: 0x00d83d60 input_device_tick
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Input-device update tick (cdecl, 6 pointer/float arguments, returns a status
/// word in eax).
///
/// Behaviour. Resolves the sibling object, builds a direction pair (negated
/// sibling-matrix row, or sine/cosine of the heading angle when the sibling is
/// absent), normalises it unless its squared length is exactly zero (in which
/// case the normalised pair is (1, raw))), walks a device list accumulating a
/// budget and a cursor pair, reduces them through two scripted heading calls,
/// wraps two angle differences into [-PI, PI], and dispatches on four mode
/// tables. Two frame words computed along the way are handed to downstream
/// callees by pointer and compared by the proof: the direction/normalise word
/// (overwritten by the second angle wrap on the main path) and the cursor word
/// (overwritten by the budget/bound chain on the main path). Callee-returned
/// script values, fault parity on unmapped reads, and the exact float operand
/// order (including NaN healing through ordered comparisons) are part of the
/// contract.
export!(cdecl, rw_d83d60(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {

    #[inline(always)]
    fn fadd(a: f32, b: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(a) + core::hint::black_box(b))
    }
    #[inline(always)]
    fn fsub(a: f32, b: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(a) - core::hint::black_box(b))
    }
    #[inline(always)]
    fn fmul(a: f32, b: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(a) * core::hint::black_box(b))
    }
    #[inline(always)]
    fn fdiv(a: f32, b: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(a) / core::hint::black_box(b))
    }
    #[inline(always)]
    fn fsqrt(a: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(a).sqrt())
    }
    #[inline(always)]
    fn fneg(a: f32) -> f32 {
        core::hint::black_box(-core::hint::black_box(a))
    }
    #[inline(always)]
    unsafe fn fr(base: u32, off: u32) -> f32 {
        unsafe { ((base.wrapping_add(off)) as *const f32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn ur(base: u32, off: u32) -> u32 {
        unsafe { ((base.wrapping_add(off)) as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn vcall_0ec(obj: u32, arg: u32) -> u32 {
        unsafe {
            let vt = ur(obj, 0);
            let fptr = ur(vt, 0xEC);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(fptr as usize);
            f(obj, arg)
        }
    }

    #[inline(always)]
    unsafe fn br(base: u32, off: u32) -> u8 {
        unsafe { ((base.wrapping_add(off)) as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn wr(base: u32, off: u32) -> u16 {
        unsafe { ((base.wrapping_add(off)) as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn sxb(base: u32, off: u32) -> u32 {
        unsafe { ((base.wrapping_add(off)) as *const i8).read() as i32 as u32 }
    }
    #[inline(always)]
    unsafe fn sxw(base: u32, off: u32) -> u32 {
        unsafe { ((base.wrapping_add(off)) as *const i16).read_unaligned() as i32 as u32 }
    }
    #[inline(always)]
    unsafe fn uw(base: u32, off: u32, v: u32) {
        unsafe { ((base.wrapping_add(off)) as *mut u32).write_unaligned(v) }
    }
    #[inline(always)]
    unsafe fn fw(base: u32, off: u32, v: f32) {
        unsafe { ((base.wrapping_add(off)) as *mut f32).write_unaligned(v) }
    }
    #[inline(always)]
    unsafe fn bw(base: u32, off: u32, v: u8) {
        unsafe { ((base.wrapping_add(off)) as *mut u8).write(v) }
    }

    const PI_F: f32 = f32::from_bits(0x40490FDB);
    const FRAC_PI_2: f32 = f32::from_bits(0x3FC90FDB);
    const NEG_FRAC_PI_2: f32 = f32::from_bits(0xBFC90FDB);

    const C_1: f32 = f32::from_bits(0x3F800000);
    const C_HALF: f32 = f32::from_bits(0x3F000000);
    const C_02: f32 = f32::from_bits(0x3E4CCCCD);
    const C_11: f32 = f32::from_bits(0x41300000);
    const C_6: f32 = f32::from_bits(0x40C00000);
    const C_10: f32 = f32::from_bits(0x41200000);
    const C_5: f32 = f32::from_bits(0x40A00000);
    const C_8: f32 = f32::from_bits(0x41000000);
    const C_NEG1: f32 = f32::from_bits(0xBF800000);
    const C_NEG_PI: f32 = f32::from_bits(0xC0490FDB);
    const C_2PI: f32 = f32::from_bits(0x40C90FDB);
    const T1DEC: [u8; 7] = [0, 1, 2, 2, 1, 1, 1];
    const T2DEC: [u8; 7] = [0, 0, 1, 1, 1, 0, 0];
    const T3DEC: [u8; 7] = [0, 0, 1, 1, 0, 1, 0];
    const T4DEC: [u8; 7] = [0, 0, 1, 1, 1, 0, 0];
    #[inline(always)]
    unsafe fn exit_report(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, mut eaxv: u32) -> u32 {
        unsafe {
            let mode = br(a0, 0xE6E);
            eaxv = (eaxv & 0xFFFFFF00) | mode as u32;
            if mode != 4 && mode != 14 {
                return eaxv;
            }
            let e6f = br(a0, 0xE6F);
            eaxv = e6f as u32;
            let f6f = e6f as f32;
            let r = callee_cdecl!(27, u32, a0, 0, fr(a0, 0xE4C).to_bits(),
                fr(a0, 0xE50).to_bits(), a1, a2, a3, a4, a5, f6f.to_bits());
            eaxv = r;
            eaxv
        }
    }

    unsafe {
        let mut eaxv: u32;
        let mut dummy = 0u32;
        let dummy_ptr = (&mut dummy as *mut u32) as u32;
        let gate = callee_cdecl!(5, u32, a0);
        eaxv = gate;
        if gate & 0xFF != 0 {
            return eaxv;
        }
        let flag1 = br(a0, 0xE73) & 1 != 0;
        uw(a0, 0xEE8, *global::<u32>(0x11735B4));
        let sib0 = ur(a0, 0x20);
        let ang = fr(a0, 0x1C);
        let (dx, dy): (f32, f32);
        if flag1 {
            if sib0 != 0 {
                dx = fneg(fr(sib0, 0x10));
                dy = fneg(fr(sib0, 0x14));
            } else {
                let s = f32::from_bits(callee_cdecl!(6, u32, ang.to_bits()));
                let ns = fneg(s);
                let c = f32::from_bits(callee_cdecl!(7, u32, ang.to_bits()));
                dx = fneg(ns);
                dy = fneg(c);
            }
        } else if sib0 != 0 {
            dx = fr(sib0, 0x10);
            dy = fr(sib0, 0x14);
        } else {
            let s = f32::from_bits(callee_cdecl!(6, u32, ang.to_bits()));
            let ns = fneg(s);
            let c = f32::from_bits(callee_cdecl!(7, u32, ang.to_bits()));
            dx = ns;
            dy = c;
        }
        let mut s44 = dy;
        let len = fadd(fmul(dy, dy), fmul(dx, dx));
        let mut s40: f32;
        if len != 0.0 {
            let k = fdiv(C_1, fsqrt(len));
            s40 = fmul(k, dx);
            s44 = fmul(dy, k);
        } else {
            s40 = C_1;
        }
        let mut s30: u32 = 0;
        if br(a5, 0x26) == 8 {
            let f50 = global::<u32>(0x1797750);
            let fv = f50.read_unaligned();
            if fv & 1 == 0 {
                f50.write_unaligned(fv | 1);
            }
            let r1 = callee_thiscall!(1, u32, a5);
            eaxv = r1;
            if r1 != 0 {
                let r2 = callee_thiscall!(1, u32, a5);
                eaxv = r2;
                let q = ur(r2, 0x20);
                if q != 0 {
                    eaxv = q.wrapping_add(0x30);
                    *global::<f32>(0x1797740) = fr(q, 0x30);
                    *global::<f32>(0x1797744) = fr(q, 0x34);
                    *global::<f32>(0x1797748) = fr(q, 0x38);
                    *global::<f32>(0x179774C) = fr(q, 0x3C);
                } else {
                    eaxv = r2.wrapping_add(0x10);
                    *global::<f32>(0x1797740) = fr(r2, 0x10);
                    *global::<f32>(0x1797744) = fr(r2, 0x14);
                    *global::<f32>(0x1797748) = fr(r2, 0x18);
                    *global::<f32>(0x179774C) = fr(r2, 0x1C);
                }
                s30 = relocated(0x1797740);
            } else {
                let v4 = fr(a5, 4);
                let t = fadd(fadd(v4, fr(a5, 8)), fr(a5, 0xC));
                if t != 0.0 {
                    *global::<f32>(0x1797740) = v4;
                    *global::<f32>(0x1797744) = fr(a5, 8);
                    *global::<f32>(0x1797748) = fr(a5, 0xC);
                    s30 = relocated(0x1797740);
                }
            }
        }
        let mut c15: u8 = 0;
        let mut c16: u8 = 0;
        let mut c17: u8 = 0;
        let mut o20 = [0u32; 2];
        callee_cdecl!(8, u32, a0, ur(a0, 0xDDC), ur(a0, 0xDE0), ur(a0, 0xDE4),
            sxw(a0, 0xE10), sxw(a0, 0xE12), sxb(a0, 0xE2B), sxb(a0, 0xE2C),
            (&mut o20 as *mut u32) as u32);
        eaxv = 0;
        let mut o28 = [0u32; 2];
        callee_cdecl!(28, u32, a0, ur(a0, 0xDE0), ur(a0, 0xDE4), ur(a0, 0xDE8),
            sxw(a0, 0xE12), sxw(a0, 0xE14), sxb(a0, 0xE2C), sxb(a0, 0xE2D),
            (&mut o28 as *mut u32) as u32);
        eaxv = 0;
        let gret = callee_thiscall!(9, u32, dummy_ptr, ur(a0, 0x20).wrapping_add(0x30), 0);
        eaxv = gret;
        let mut s36 = ur(gret, 0);
        let mut s32 = ur(gret, 4);
        eaxv = s32;
        let f1 = vcall_0ec(a0, dummy_ptr);
        eaxv = f1;
        let f10 = fr(f1, 0);
        let f14 = fr(f1, 4);
        let m2 = fadd(fmul(f10, f10), fmul(f14, f14));
        let e6f = br(a0, 0xE6F);
        let t2 = fmul(fsqrt(m2), fsub(C_1, C_02));
        let t0 = e6f as f32;
        let wde0 = ur(a0, 0xDE0);
        let tptr = ur(relocated(0x1178284), (wde0 & 0xFFFF).wrapping_mul(4));
        eaxv = (wde0 >> 16) << 5;
        let tbit = br(tptr.wrapping_add(eaxv), 0x1E) & 0x30;
        let w2e = sxw(a0, 0x2E);
        eaxv = w2e;
        let uptr = ur(relocated(0x1295CD8), w2e.wrapping_mul(4));
        eaxv = uptr;
        let x1 = fr(uptr, 0xB8);
        let e = fadd(t2, fmul(t0, C_02));
        let mut e2 = fsub(e, C_11);
        if 0.0 > e2 {
            e2 = 0.0;
        }
        let res1: f32;
        if tbit != 0 {
            let t = fmul(x1, C_HALF);
            let u = fmul(e2, C_1);
            let t = fadd(t, C_1);
            let t = fmul(t, C_10);
            res1 = fadd(t, u);
        } else {
            let t = fmul(x1, C_HALF);
            let u = fmul(C_1, e2);
            let t = fadd(t, C_1);
            let t = fmul(t, C_6);
            res1 = fadd(t, u);
        }
        let mut f20 = fmul(fr(a0, 0xF04), res1);
        let mut f1c = f32::from_bits(s32);
        let mut f18 = f32::from_bits(s36);
        let mut cl: u8 = c16;
        eaxv = (eaxv & 0xFFFFFF00) | c16 as u32;
        let mut esi = 4u32;
        loop {
            let mut x5 = f1c;
            let mut x0 = f18;
            if eaxv & 0xFF != 0 {
                return exit_report(a0, a1, a2, a3, a4, a5, eaxv);
            }
            let o58 = f32::from_bits(o28[0]);
            let o5c = f32::from_bits(o28[1]);
            let o60 = f32::from_bits(o20[0]);
            let o64 = f32::from_bits(o20[1]);
            let mut x6 = o58;
            let mut x3 = o5c;
            let mut x2 = fsub(x6, o60);
            x6 = fsub(x6, x0);
            let mut x7 = x3;
            let mut x1 = x3;
            x1 = fsub(x1, o64);
            x7 = fsub(x7, x5);
            x3 = fmul(x7, x1);
            x0 = fmul(x6, x2);
            x1 = fmul(x1, x1);
            x2 = fmul(x2, x2);
            x3 = fadd(x3, x0);
            x1 = fadd(x1, x2);
            x0 = fsqrt(x1);
            x3 = fdiv(x3, x0);
            if x3 > 0.0 {
                if f20 > x3 {
                    s36 = o28[0];
                    s32 = o28[1];
                    f20 = fsub(f20, x3);
                    x0 = f32::from_bits(s32);
                    cl = c15;
                    f1c = x0;
                    x0 = f32::from_bits(s36);
                    f18 = x0;
                } else {
                    let mut q = fdiv(f20, x3);
                    cl = 1;
                    x0 = fmul(q, x6);
                    q = fmul(q, x7);
                    x5 = fadd(x5, q);
                    c15 = cl;
                    x0 = fadd(x0, f18);
                    f1c = x5;
                    f18 = x0;
                }
            }
            if esi == 4 {
                if C_5 > x3 {
                    callee_thiscall!(10, u32, a0.wrapping_add(0xDD4));
                    callee_cdecl!(11, u32, a0);
                    eaxv = 0;
                    let r12 = callee_thiscall!(12, u32, a5);
                    eaxv = r12;
                    if r12 & 0xFF != 0 {
                        callee_thiscall!(13, u32, a0.wrapping_add(0xDD4));
                    }
                    let lw = wr(a0, 0xDE8) as u32;
                    eaxv = lw;
                    if lw != 0xFFFF {
                        eaxv = (eaxv & 0xFFFFFF00) | c16 as u32;
                    } else {
                        let r14 = callee_cdecl!(14, u32, a0, 0, s30);
                        eaxv = r14;
                        c16 = (r14 & 0xFF) as u8;
                        c17 = 1;
                    }
                    cl = c15;
                }
            }
            if cl != 0 {
                if eaxv & 0xFF != 0 {
                    return exit_report(a0, a1, a2, a3, a4, a5, eaxv);
                }
                break;
            }
            if eaxv & 0xFF != 0 {
                continue;
            }
            esi = esi.wrapping_add(1);
            if esi >= 13 {
                break;
            }
            let base4 = esi.wrapping_mul(4).wrapping_add(0xDD4);
            let lw = wr(a0, base4) as u32;
            eaxv = lw;
            if lw == 0xFFFF {
                let r14 = callee_cdecl!(30, u32, a0, 0, s30);
                eaxv = r14;
                cl = c15;
                c16 = (r14 & 0xFF) as u8;
                c17 = 1;
                if r14 & 0xFF != 0 {
                    continue;
                }
            }
            let dw = ur(a0, base4);
            eaxv = dw & 0xFFFF;
            if eaxv == 0xFFFF {
                break;
            }
            let tv = ur(relocated(0x1178284), eaxv.wrapping_mul(4));
            if tv == 0 {
                break;
            }
            o20 = o28;
            let mut no28 = [0u32; 2];
            let bse = a0.wrapping_add(esi);
            let wse = a0.wrapping_add(esi.wrapping_mul(2));
            eaxv = sxb(bse, 0xE27);
            let a6 = eaxv;
            eaxv = sxb(bse, 0xE28);
            let a7 = eaxv;
            eaxv = sxw(wse, 0xE0C);
            let a5w = eaxv;
            eaxv = sxw(wse, 0xE0A);
            let a4w = eaxv;
            callee_cdecl!(29, u32, a0, ur(a0, base4.wrapping_sub(4)), ur(a0, base4),
                ur(a0, base4.wrapping_add(4)), a4w, a5w, a6, a7,
                (&mut no28 as *mut u32) as u32);
            eaxv = 0;
            o28 = no28;
            eaxv = (eaxv & 0xFFFFFF00) | c16 as u32;
            cl = c15;
        }
        if wr(a0, 0xDF8) as u32 == 0xFFFF && c17 == 0 {
            let r14 = callee_cdecl!(31, u32, a0, 0, s30);
            eaxv = r14;
        }
        let sib = ur(a0, 0x20);
        let hx0 = fsub(f1c, fr(sib, 0x34));
        let hx2 = fsub(f18, fr(sib, 0x30));
        let h1 = callee_cdecl!(3, f32, hx2.to_bits(), hx0.to_bits());
        eaxv = h1.to_bits();
        let mut s84 = h1;
        let mut s68 = h1;
        let h2 = callee_cdecl!(3, f32, s40.to_bits(), s44.to_bits());
        eaxv = h2.to_bits();
        let mut d1 = 0u32;
        let mut d2 = 0u32;
        let r15 = callee_cdecl!(15, u32, a0, h1.to_bits(), h2.to_bits(),
            (&mut d2 as *mut u32) as u32, (&mut d1 as *mut u32) as u32);
        eaxv = r15;
        // E-88 mirror: the loop-cursor value below is what the early path hands
        // to the second downstream call; the main path overwrites the slot.
        let mut e88v = f1c;
        if r15 & 0xFF == 0 {
            let e70 = br(a0, 0xE70);
            eaxv = e70 as i8 as i32 as u32;
            let ts = e70.wrapping_sub(1);
            eaxv = ts as u32;
            let t1c = if ts > 6 {
                2
            } else {
                eaxv = T1DEC[ts as usize] as u32;
                T1DEC[ts as usize]
            };
            let (c5, c6, c7) = match t1c {
                0 => (0, 0, 0),
                1 => (2, 1, 1),
                _ => (1, 0, 0),
            };
            let r16 = callee_cdecl!(16, f32, a0, 0, s68.to_bits(), h2.to_bits(),
                C_1.to_bits(), c5, c6, c7, a5);
            eaxv = r16.to_bits();
            let x3_54 = fsub(s68, h2);
            let mut d = r16;
            while C_NEG_PI > d {
                d = fadd(d, C_2PI);
            }
            while d > PI_F {
                d = fsub(d, C_2PI);
            }
            let gt0 = if 0.0 > x3_54 { 1u32 } else { 0u32 };
            eaxv = gt0;
            let r17 = callee_cdecl!(17, f32, a0, gt0, h2.to_bits(), d.to_bits());
            eaxv = r17.to_bits();
            // E-44 mirror: wrap (r17 - h2) into the slot (same loop shape as d).
            let mut d2w = fsub(r17, h2);
            while C_NEG_PI > d2w {
                d2w = fadd(d2w, C_2PI);
            }
            while d2w > PI_F {
                d2w = fsub(d2w, C_2PI);
            }
            s44 = d2w;
            let r18a = callee_cdecl!(18, f32, (e6f as f32).to_bits(), fr(a0, 0xEF0).to_bits());
            eaxv = r18a.to_bits();
            s84 = r18a;
            let in_t2a = if br(a0, 0xE6E) == 1 {
                eaxv = e70 as i8 as i32 as u32;
                if e70 <= 6 {
                    eaxv = T2DEC[e70 as usize] as u32;
                    T2DEC[e70 as usize] == 0
                } else {
                    false
                }
            } else {
                false
            };
            if in_t2a {
                let r19 = callee_thiscall!(19, u32, a0.wrapping_add(0xDD4));
                eaxv = r19;
                if r19 & 0xFF != 0 {
                    s68 = s84;
                    if s84 > 0.0 {
                        let r20 = callee_cdecl!(20, f32, ur(a0, 0xF50));
                        eaxv = r20.to_bits();
                        s68 = fadd(r20, s84);
                    }
                } else {
                    s68 = s84;
                }
            } else {
                s68 = s84;
            }
            eaxv = e70 as i8 as i32 as u32;
            let in_t3a = if e70 <= 6 {
                eaxv = T3DEC[e70 as usize] as u32;
                T3DEC[e70 as usize] == 0
            } else {
                false
            };
            let mut s88: f32;
            if in_t3a {
                if s68 > 0.0 {
                    let r21 = callee_cdecl!(21, f32, a0, a0.wrapping_add(0xE48), a5, s68.to_bits());
                    eaxv = r21.to_bits();
                    s40 = r21;
                    s88 = fdiv(r21, s68);
                } else {
                    s88 = 0.0;
                }
            } else {
                s88 = C_1;
            }
            eaxv = e70 as i8 as i32 as u32;
            let in_t4a = if e70 <= 6 {
                eaxv = T4DEC[e70 as usize] as u32;
                T4DEC[e70 as usize] == 0
            } else {
                false
            };
            if in_t4a {
                if br(a0, 0xED4) == 2 {
                    let r22 = callee_cdecl!(22, u32, a0);
                    eaxv = r22;
                    s88 = 0.0;
                } else {
                    let mut w36 = 0u32;
                    let r23 = callee_cdecl!(23, u32, a0, (&mut w36 as *mut u32) as u32);
                    eaxv = r23;
                    if r23 & 0xFF != 0 {
                        let w = f32::from_bits(w36);
                        if !(w > s88) {
                            s88 = w;
                        }
                    }
                }
            }
            let r18b = callee_cdecl!(18, f32, (e6f as f32).to_bits(), fr(a0, 0xEF0).to_bits());
            eaxv = r18b.to_bits();
            let r24 = callee_cdecl!(24, f32, a0, h2.to_bits(), r18b.to_bits());
            eaxv = r24.to_bits();
            s40 = r24;
            let m1 = if r24 > C_1 { C_1 } else { r24 };
            let m2 = if s88 > m1 { m1 } else { s88 };
            s40 = fmul(m2, s68);
            e88v = s88;
        }
        // Hand the mirrored slot values; the stubs overwrite w44/w84 with the
        // scripted answers, and the snaps compare these pre-call words.
        let mut w44 = s44.to_bits();
        let mut w88 = 0u32;
        let mut w84 = 0u32;
        let r4 = callee_cdecl!(4, u32, a0, (&mut w44 as *mut u32) as u32,
            (&mut w88 as *mut u32) as u32, (&mut w84 as *mut u32) as u32);
        eaxv = r4;
        let f2 = vcall_0ec(a0, dummy_ptr);
        eaxv = f2;
        let sib2 = ur(a0, 0x20);
        let m10 = fr(sib2, 0x10);
        let m14 = fr(sib2, 0x14);
        let m18 = fr(sib2, 0x18);
        let vm0 = fadd(fadd(fmul(fr(f2, 4), m14), fmul(fr(f2, 0), m10)), fmul(fr(f2, 8), m18));
        s68 = vm0;
        let mut vm = vm0;
        if flag1 {
            vm = fneg(vm0);
            s68 = vm;
        }
        let f84 = f32::from_bits(w84);
        let f40 = s40;
        let mut w88b = e88v.to_bits();
        let r25 = callee_cdecl!(25, u32, f40.to_bits(), vm.to_bits(), a3, a2, a0,
            (&mut w88b as *mut u32) as u32, f84.to_bits());
        eaxv = r25;
        eaxv = a1;
        fw(a1, 0, f32::from_bits(w44));
        eaxv = a4;
        bw(a4, 0, 0);
        let r26 = callee_cdecl!(26, u32, a0, s68.to_bits(), f32::from_bits(w44).to_bits(), a4);
        eaxv = r26;
        if br(a0, 0xE6E) == 4 && br(a0, 0xE72) == 0 {
            let e = ur(a0, 0x20);
            eaxv = e;
            let x3 = fr(a0, 0xE4C);
            let x2 = fr(e, 0x34);
            let x4 = fr(a0, 0xE50);
            let x1 = fr(e, 0x30);
            let x0 = fr(e, 0x38);
            let x5 = fr(a0, 0xE54);
            let dd2 = fsub(x2, x4);
            let dd1 = fsub(x1, x3);
            let dd0 = fsub(x0, x5);
            let q2 = fadd(fadd(fmul(dd2, dd2), fmul(dd1, dd1)), fmul(dd0, dd0));
            let sq = fsqrt(q2);
            if C_8 > sq {
                let s3 = fsub(x3, fr(e, 0x30));
                let s4 = fsub(x4, fr(e, 0x34));
                let s5 = fsub(x5, fr(e, 0x38));
                let dp = fadd(fadd(fmul(fr(e, 0x10), s3), fmul(fr(e, 0x14), s4)), fmul(fr(e, 0x18), s5));
                let mut dpn = dp;
                if 0.0 > dpn {
                    dpn = fneg(dpn);
                }
                let dq = fdiv(dpn, sq);
                if C_02 > dq {
                    bw(a5, 0x2A, 3);
                    let g = *global::<u32>(0x11735B4);
                    eaxv = g;
                    let g2 = g.wrapping_add(0x7D0);
                    eaxv = g2;
                    uw(a5, 0x10, g2);
                }
            }
        }
        if !flag1 {
            return eaxv;
        }
        eaxv = a1;
        let o1 = fr(a1, 0);
        fw(a1, 0, fmul(o1, C_NEG1));
        let o2 = fr(a2, 0);
        fw(a2, 0, fmul(o2, C_NEG1));
        eaxv
    }
});
