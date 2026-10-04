// original: 0x00B5FE20 Bullet_Trajectory
/// Exact emulation of x86 `cvttss2si`: truncate toward zero; NaN,
/// infinities and out-of-range values yield 0x80000000 (Rust's `as`
/// saturates instead, so it cannot be used directly).
fn cvttss2si(x: f32) -> u32 {
    if x.is_nan() {
        return 0x80000000;
    }
    if x >= 2147483648.0 || x < -2147483648.0 {
        return 0x80000000;
    }
    (x as i32) as u32
}
#[inline(always)]
fn is_nan_bits(b: u32) -> bool {
    (b & 0x7f800000) == 0x7f800000 && (b & 0x007fffff) != 0
}

/// Exact x86 `addss`: first NaN operand (quieted) wins; otherwise one plain
/// add whose operand order no longer matters bit-wise.
#[inline(always)]
fn ss_add(a: f32, b: f32) -> f32 {
    let ab = a.to_bits();
    let bb = b.to_bits();
    if is_nan_bits(ab) {
        return f32::from_bits(ab | 0x00400000);
    }
    if is_nan_bits(bb) {
        return f32::from_bits(bb | 0x00400000);
    }
    a + b
}

/// Exact x86 `subss`.
#[inline(always)]
fn ss_sub(a: f32, b: f32) -> f32 {
    let ab = a.to_bits();
    let bb = b.to_bits();
    if is_nan_bits(ab) {
        return f32::from_bits(ab | 0x00400000);
    }
    if is_nan_bits(bb) {
        return f32::from_bits(bb | 0x00400000);
    }
    a - b
}

/// Exact x86 `mulss`.
#[inline(always)]
fn ss_mul(a: f32, b: f32) -> f32 {
    let ab = a.to_bits();
    let bb = b.to_bits();
    if is_nan_bits(ab) {
        return f32::from_bits(ab | 0x00400000);
    }
    if is_nan_bits(bb) {
        return f32::from_bits(bb | 0x00400000);
    }
    a * b
}

/// Bullet_Trajectory (original 0x00B5FE20; merged symbol name, low confidence): transforms the listener
/// frame, dispatches one of four mix helpers through a jump table, then runs
/// a counter block that schedules the next update. Returns the mix helper's
/// status byte, or 0 on any early exit.
export!(thiscall, rw_00b5fe20(this: u32, p8: u32, p12: u32, p16: u32, p20: u32, p24: u32, p28: u32, p32: u32, p36: u32, p40: u32) -> u8 {
    unsafe {
        let ld = |b: u32, w: usize| unsafe {
            f32::from_bits((b as *const u32).add(w).read_unaligned())
        };
        let af: u32 = callee_cdecl!(1, u32, (this as *const u32).add(0x18 / 4).read_unaligned());
        let edi = p8;
        // Orig: the three je's fall into the s1c gate, but the jne on [af+8]
        // jumps past the gate straight to the main body.
        let mut past_gate = false;
        if edi != 0 {
            let g = (edi as *const u32).add(0x6c / 4).read_unaligned();
            if g != 0 && (g as *const u8).add(0x0e).read_unaligned() != 0 {
                if (af as *const u32).add(8 / 4).read_unaligned() == 3 {
                    callee_thiscall!(2, u32, this, edi, p12, 0, p20, 0);
                    return 0;
                }
                past_gate = true;
            }
        }
        if !past_gate {
            let s1c = (this as *const u32).add(0x1c / 4).read_unaligned();
            if s1c != 0 && ((p24 & 0xff) == 0 || s1c != 1) {
                return 0;
            }
        }
        // Input vector: direct block or reference offset.
        let mut inv = [0.0f32; 4];
        if p16 != 0 {
            inv[0] = ld(p16, 0);
            inv[1] = ld(p16, 1);
            inv[2] = ld(p16, 2);
            inv[3] = ld(p16, 3);
        } else {
            inv[0] = ld(p12, 0x30 / 4);
            inv[1] = ld(p12, 0x34 / 4);
            inv[2] = ld(p12, 0x38 / 4);
            inv[3] = ld(p12, 0x3c / 4);
        }
        // Output vector: direct block or scaled reference plus input.
        let mut out = [0.0f32; 4];
        if p20 != 0 {
            out[0] = ld(p20, 0);
            out[1] = ld(p20, 1);
            out[2] = ld(p20, 2);
            out[3] = ld(p20, 3);
        } else {
            let s: f32 = callee_thiscall!(3, f32, this, edi);
            out[0] = ss_add(ss_mul(ld(p12, 0x10 / 4), s), inv[0]);
            out[1] = ss_add(ss_mul(ld(p12, 0x14 / 4), s), inv[1]);
            out[2] = ss_add(ss_mul(ld(p12, 0x18 / 4), s), inv[2]);
            out[3] = inv[3];
        }
        if edi != 0 {
            let raw = (af as *const u32).add(0x60 / 4).read_unaligned();
            let t = f32::from_bits(raw & 0x7fffffff);
            if t > *global::<f32>(0x00fe86b4) {
                if (edi as *const u32).add(0x20 / 4).read_unaligned() == 0 {
                    callee_thiscall!(4, u32, edi);
                    let z = (edi as *const u32).add(0x20 / 4).read_unaligned();
                    callee_thiscall!(5, u32, edi.wrapping_add(0x10), z);
                }
                let m = (edi as *const u32).add(0x20 / 4).read_unaligned();
                let dx = ss_sub(out[0], ld(m, 0x30 / 4));
                let dy = ss_sub(out[1], ld(m, 0x34 / 4));
                let dz = ss_sub(out[2], ld(m, 0x38 / 4));
                let r0 = ss_add(ss_add(ss_mul(ld(m, 1), dy), ss_mul(dx, ld(m, 0))), ss_mul(ld(m, 2), dz));
                let r1 = ss_add(ss_add(ss_mul(ld(m, 0x14 / 4), dy), ss_mul(dx, ld(m, 0x10 / 4))), ss_mul(ld(m, 0x18 / 4), dz));
                let r2 = ss_add(ss_add(ss_mul(ld(m, 0x24 / 4), dy), ss_mul(dx, ld(m, 0x20 / 4))), ss_mul(ld(m, 0x28 / 4), dz));
                // Filter pair: the intercepted callees preserve xmm0, so the
                // loaded gain is reused after both calls, as on the original.
                let t60 = ld(af, 0x60 / 4);
                callee_cdecl!(6, u32,);
                callee_cdecl!(7, u32,);
                let g1 = ss_sub(ss_mul(r1, t60), ss_mul(r2, t60));
                let g2 = ss_add(ss_mul(r1, t60), ss_mul(r2, t60));
                out[0] = r0;
                out[1] = g1;
                out[2] = g2;
                if m == 0 {
                    // Defensive refill that cannot fire once the first refill
                    // landed (kept faithful to the original's check).
                    callee_thiscall!(4, u32, edi);
                    let z2 = (edi as *const u32).add(0x20 / 4).read_unaligned();
                    callee_thiscall!(5, u32, edi.wrapping_add(0x10), z2);
                }
                let m2 = (edi as *const u32).add(0x20 / 4).read_unaligned();
                let o0 = ss_add(ss_add(ss_add(ss_mul(ld(m2, 0x10 / 4), g1), ss_mul(ld(m2, 0), r0)), ss_mul(ld(m2, 0x20 / 4), g2)), ld(m2, 0x30 / 4));
                let o1 = ss_add(ss_add(ss_add(ss_mul(ld(m2, 0x14 / 4), g1), ss_mul(ld(m2, 1), r0)), ss_mul(ld(m2, 0x24 / 4), g2)), ld(m2, 0x34 / 4));
                let o2 = ss_add(ss_add(ss_add(ss_mul(ld(m2, 0x18 / 4), g1), ss_mul(ld(m2, 2), r0)), ss_mul(ld(m2, 0x28 / 4), g2)), ld(m2, 0x38 / 4));
                out[0] = o0;
                out[1] = o1;
                out[2] = o2;
                // The block also refreshes the w component from the input.
                out[3] = inv[3];
            }
        }
        callee_cdecl!(8, u32, inv.as_ptr() as u32, out.as_ptr() as u32);
        // Jump-table dispatch on the mode word (1,2,3,6 call helpers;
        // anything else returns 0).
        let idx = (af as *const u32).add(2).read_unaligned();
        let case_al: u8 = match idx.wrapping_sub(1) {
            0 => callee_thiscall!(9, u8, this, edi, p12, inv.as_ptr() as u32,
                out.as_ptr() as u32, p28, p36, p40),
            1 => callee_thiscall!(10, u8, this, edi, p12, inv.as_ptr() as u32,
                out.as_ptr() as u32, p28),
            2 => callee_thiscall!(11, u8, this, edi, p12, inv.as_ptr() as u32,
                out.as_ptr() as u32, p28, p32),
            5 => callee_thiscall!(12, u8, this, edi, p12, inv.as_ptr() as u32,
                out.as_ptr() as u32, p28),
            _ => return 0,
        };
        if edi != 0 {
            let g = (edi as *const u32).add(0x6c / 4).read_unaligned();
            if g != 0 && (g as *const u8).add(0x0e).read_unaligned() != 0 {
                return case_al;
            }
        }
        if case_al == 0 {
            return 0;
        }
        // Counter block: two saturating counters select the schedule path.
        let mut cx = (this as *const i32).add(0x5c / 4).read_unaligned();
        let mut dx = (this as *const u32).add(0x60 / 4).read_unaligned();
        if cx > 0 {
            cx -= 1;
        }
        if dx.wrapping_sub(1) <= 0x61a6 {
            dx = dx.wrapping_sub(1);
        }
        let gnow = *global::<u32>(0x011735b4);
        let acc: u32;
        if cx > 0 {
            (this as *mut u32).add(0x1c / 4).write_unaligned(1);
            acc = (af as *const u32).add(0x8c / 4).read_unaligned().wrapping_add(gnow);
        } else if (dx as i32) > 0 {
            (this as *mut u32).add(0x1c / 4).write_unaligned(3);
            let mut w20 = (af as *const u32).add(0x94 / 4).read_unaligned();
            let sel = (((edi as *const u32).add(0x28 / 4).read_unaligned() >> 6) & 0xf) as u32;
            if sel == 3 {
                let c1: u8 = callee_thiscall!(13, u8, edi);
                if c1 != 0 {
                    w20 = (af as *const u32).add(0x9c / 4).read_unaligned();
                } else {
                    w20 = (af as *const u32).add(0x94 / 4).read_unaligned();
                    let c2: u8 = callee_thiscall!(14, u8, edi);
                    if c2 != 0 {
                        let c3: u32 = callee_cdecl!(15, u32, 0xc8,
                            (af as *const u32).add(0x28 / 4).read_unaligned());
                        if c3 != 0 {
                            w20 = (af as *const u32).add(0x98 / 4).read_unaligned();
                        }
                    }
                }
                if *global::<u32>(0x011d6fd4) == 2 {
                    let c4: u8 = callee_cdecl!(16, u8,);
                    if c4 != 0 {
                        let vt = (edi as *const u32).read_unaligned();
                        let tg = ((vt as *const u8).add(0x128) as *const u32).read_unaligned();
                        let vf: extern "thiscall" fn(u32) -> u8 =
                            core::mem::transmute(tg as usize);
                        if vf(edi) != 0 {
                            let e = (edi as *const u32).add(0x228 / 4).read_unaligned();
                            if e != 0 {
                                let s1 = ld(e, 0x5ac / 4);
                                if s1 > *global::<f32>(0x00fe8874) {
                                    w20 = cvttss2si(ss_mul((w20 as i32) as f32, s1));
                                }
                            }
                        }
                    }
                }
                acc = gnow.wrapping_add(w20);
            } else if sel == 2 {
                let edi2 = (edi as *const u32).add(0xf50 / 4).read_unaligned();
                if *global::<u32>(0x011d6fd4) == 2 {
                    let c4: u8 = callee_cdecl!(16, u8,);
                    if c4 != 0 && edi2 != 0 {
                        let vt = (edi2 as *const u32).read_unaligned();
                        let tg = ((vt as *const u8).add(0x128) as *const u32).read_unaligned();
                        let vf: extern "thiscall" fn(u32) -> u8 =
                            core::mem::transmute(tg as usize);
                        if vf(edi2) != 0 {
                            let e = (edi2 as *const u32).add(0x228 / 4).read_unaligned();
                            if e != 0 {
                                let s1 = ld(e, 0x5ac / 4);
                                w20 = cvttss2si(ss_mul((w20 as i32) as f32, s1));
                            }
                        }
                    }
                }
                acc = gnow.wrapping_add(w20);
            } else {
                acc = gnow.wrapping_add(w20);
            }
        } else {
            (this as *mut u32).add(0x1c / 4).write_unaligned(4);
            acc = gnow;
        }
        (this as *mut u32).add(0x20 / 4).write_unaligned(acc);
        callee_thiscall!(19, u32, this, dx);
        callee_thiscall!(20, u32, this, cx as u32);
        case_al
    }
});
