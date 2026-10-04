// original: 0x00d7fff0 aim_assist_update

/// Update the aim-assist state from a fresh angle sample.
///
/// Wraps the angle delta into range, then either follows the latched-mode
/// path (modes 0x1d-0x20) or reads the sensor vector through vtable slot
/// 0xec and runs the length, integrator and alignment gates. Passing trials
/// read the sensor once more plus two scalar hooks (slots 0x60/0x64). When
/// the frame timer has advanced more than 500 ticks since the stamp, the
/// long-range raycast block runs (solver pair, range-word scatter, two range
/// gates), followed by the crossing check, the latch countdown and the
/// rotate-or-keep decision. The tail writes the clamped magnitude to
/// `out_mag` and the sign flag to `out_sign`. Returns 1 when the outputs
/// were written, 0 on any early exit.
export!(cdecl, rw_00d7fff0(this: u32, f1_bits: u32, f2_bits: u32, out_sign: u32, out_mag: u32) -> u32 {
    unsafe {
        let neg_pi = lf_checker_rt::global::<f32>(0x00fe_8dc4).read();
        let tau = lf_checker_rt::global::<f32>(0x00fe_8aec).read();
        let pi = lf_checker_rt::global::<f32>(0x00fe_8aa0).read();
        let mut ang = f32::from_bits(f1_bits) - f32::from_bits(f2_bits);
        while neg_pi > ang {
            ang += tau;
        }
        while ang > pi {
            ang -= tau;
        }
        let abs_mask = lf_checker_rt::global::<u32>(0x00fe_8f80).read();
        let abs_ang = f32::from_bits(ang.to_bits() & abs_mask);
        let entry_mode = ((this + 0xe6e) as *const u8).read();
        let vtable = (this as *const u32).read();
        let sense: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(((vtable + 0xec) as *const u32).read() as usize);
        let mut slot = [0u32; 4];
        let mode = ((this + 0xe9a) as *const u8).read();
        if mode < 0x1d || mode > 0x20 {
            ((this + 0xefc) as *mut u8).write(0);
            let v = sense(this, slot.as_mut_ptr() as u32);
            let x = (v as *const f32).read();
            let y = ((v + 4) as *const f32).read();
            let len = (x * x + y * y).sqrt();
            let one = lf_checker_rt::global::<f32>(0x00fe_88e8).read();
            if len > one {
                ((this + 0xee0) as *mut u32).write(0);
                ((this + 0xefc) as *mut u8).write(0);
                return 0;
            }
            let e70 = ((this + 0xe70) as *const u8).read();
            let small = lf_checker_rt::global::<f32>(0x00fe_881c).read();
            let gated = e70 == 0 || e70 == 1 || e70 == 4 || e70 == 5;
            if !gated && small > len {
                let proceed: u8 = lf_checker_rt::callee_thiscall!(2, u8, this.wrapping_add(0xdd4));
                if proceed == 0 {
                    let saturated = ((this + 0x1304) as *const u32).read() == 2;
                    let limit = if saturated { 0x1388u32 } else { 0x4b0u32 };
                    let tick = lf_checker_rt::global::<f32>(0x0117_35bc).read();
                    let per_sec = lf_checker_rt::global::<f32>(0x00fe_8c58).read();
                    let trunc = trunc_f32_to_i64(tick * per_sec) as u32;
                    let acc = ((this + 0xee0) as *const u32).read().wrapping_add(trunc);
                    ((this + 0xee0) as *mut u32).write(acc);
                    if acc > limit {
                        ((this + 0xefc) as *mut u8).write(2);
                        let up = if !(0.0 > ang) { 1u32 } else { 0 };
                        ((this + 0xe6e) as *mut u8).write((up * 2 + 0x1e) as u8);
                        let now = lf_checker_rt::global::<u32>(0x0117_35b4).read();
                        ((this + 0xedc) as *mut u32).write(now);
                        return sense_tail(this, out_sign, out_mag, vtable, entry_mode);
                    }
                    // Small sum: EE0 keeps the accumulator, no zeroing.
                } else {
                    ((this + 0xee0) as *mut u32).write(0);
                }
            } else {
                ((this + 0xee0) as *mut u32).write(0);
            }
            let half_pi = lf_checker_rt::global::<f32>(0x00fe_8978).read();
            if !(abs_ang > half_pi) {
                return 0;
            }
            let w = sense(this, slot.as_mut_ptr() as u32);
            let x0 = (w as *const f32).read();
            let y0 = ((w + 4) as *const f32).read();
            let z0 = ((w + 8) as *const f32).read();
            let sumsq = (x0 * x0 + y0 * y0) + z0 * z0;
            if !(one > sumsq) {
                return 0;
            }
            let e73 = ((this + 0xe73) as *const u8).read();
            let m = if ang > 0.0 {
                ((!e73) & 1) | 0x1e
            } else if e73 & 1 == 0 {
                0x1d
            } else {
                0x20
            };
            ((this + 0xe6e) as *mut u8).write(m);
            let now = lf_checker_rt::global::<u32>(0x0117_35b4).read();
            ((this + 0xedc) as *mut u32).write(now);
            return sense_tail(this, out_sign, out_mag, vtable, entry_mode);
        }
        let latch = lf_checker_rt::global::<f32>(0x00ee_c90c).read();
        if latch > abs_ang {
            if ((this + 0xefc) as *const u8).read() == 0 {
                return 0;
            }
        }
        ((this + 0xee0) as *mut u32).write(0);
        ((this + 0xe6e) as *mut u8).write(mode);
        sense_tail(this, out_sign, out_mag, vtable, entry_mode)
    }
});

/// Truncate a float toward zero exactly like x87 `fistp` with the
/// round-control bits forced to truncate: out-of-range and NaN inputs yield
/// `i64::MIN`, everything else truncates (matching Rust `as` in range).
fn trunc_f32_to_i64(x: f32) -> i64 {
    const TWO63: f32 = 9.223372036854776e18;
    if x.is_nan() || x >= TWO63 || x < -TWO63 {
        i64::MIN
    } else {
        x as i64
    }
}

/// Raycast block A (modes 0x1d/0x1f): positive-projection solver.
///
/// Builds the scaled direction triple, runs the solver twice through the
/// shared finish, and reports whether either range gate hit. Returns 0
/// without calling when the projection is negative.
unsafe fn raycast_a(this: u32, m: u32, proj: f32, first: f32, second: f32, mode: u8) -> u8 {
    unsafe {
        let rf = |p: u32| (p as *const f32).read();
        if proj < 0.0 {
            return 0;
        }
        let half = lf_checker_rt::global::<f32>(0x00fe_8830).read();
        let four = lf_checker_rt::global::<f32>(0x00fe_8ab8).read();
        let s = proj * half;
        let mut k = if s > four { four } else { s };
        let unit = lf_checker_rt::global::<f32>(0x00fe_87d0).read();
        let r1 = rf(m + 4) * unit;
        let r2 = rf(m + 8) * unit;
        let one = lf_checker_rt::global::<f32>(0x00fe_88e8).read();
        k += one;
        let (dir, side, fwd) = if mode == 0x1d {
            let t = rf(m) * unit;
            (rf(m + 0x14) + r1, rf(m + 0x18) + r2, t + rf(m + 0x10))
        } else {
            let t = rf(m) * unit;
            (rf(m + 0x14) - r1, rf(m + 0x18) - r2, rf(m + 0x10) - t)
        };
        let d0 = dir * k;
        let ck = fwd * k;
        let bk = side * k;
        let mut abuf = [0u32; 12];
        abuf[0] = first.to_bits();
        abuf[1] = second.to_bits();
        abuf[2] = 0x3e99_999a;
        abuf[4] = ck.to_bits();
        abuf[10] = bk.to_bits();
        let mut dummy = [0u32; 4];
        let e = lf_checker_rt::callee_cdecl!(5, u32, dummy.as_mut_ptr() as u32, m, abuf.as_mut_ptr() as u32);
        let r0 = rf(e);
        let r1 = rf(e + 4);
        let r2 = rf(e + 8);
        let r3 = rf(e + 12);
        let mut bbuf = [0u32; 12];
        bbuf[0] = (-first).to_bits();
        bbuf[1] = second.to_bits();
        bbuf[2] = 0x3e99_999a;
        let f = lf_checker_rt::callee_cdecl!(5, u32, dummy.as_mut_ptr() as u32, m, bbuf.as_mut_ptr() as u32);
        let w = f32::from_bits(bbuf[7]);
        raycast_finish(this, m, r0, r1, r2, r3, rf(f), rf(f + 4), rf(f + 8), rf(f + 12), d0, ck, bk, w)
    }
}

/// Raycast block B (modes 0x1e/0x20): negative-projection solver.
///
/// Mirror of block A with negated offsets. Returns 0 without calling when
/// the projection is positive.
unsafe fn raycast_b(this: u32, m: u32, proj: f32, first: f32, second: f32, mode: u8) -> u8 {
    unsafe {
        let rf = |p: u32| (p as *const f32).read();
        if proj > 0.0 {
            return 0;
        }
        let nhalf = lf_checker_rt::global::<f32>(0x00fe_8d7c).read();
        let four = lf_checker_rt::global::<f32>(0x00fe_8ab8).read();
        let s = proj * nhalf;
        let mut k = if s > four { four } else { s };
        let unit = lf_checker_rt::global::<f32>(0x00fe_87d0).read();
        let t3 = rf(m) * unit;
        let t1 = rf(m + 4) * unit;
        let t2 = rf(m + 8) * unit;
        let one = lf_checker_rt::global::<f32>(0x00fe_88e8).read();
        k += one;
        let h = -rf(m + 0x10);
        let p5 = -rf(m + 0x14);
        let p6 = -rf(m + 0x18);
        let (d, e, f) = if mode == 0x1e {
            (h - t3, p5 - t1, p6 - t2)
        } else {
            (h + t3, p5 + t1, p6 + t2)
        };
        let dk = d * k;
        let ek = e * k;
        let fk = f * k;
        let mut abuf = [0u32; 12];
        abuf[0] = first.to_bits();
        abuf[1] = (-second).to_bits();
        abuf[2] = 0x3e99_999a;
        let mut dummy = [0u32; 4];
        let e = lf_checker_rt::callee_cdecl!(5, u32, dummy.as_mut_ptr() as u32, m, abuf.as_mut_ptr() as u32);
        let r0 = rf(e);
        let r1 = rf(e + 4);
        let r2 = rf(e + 8);
        let r3 = rf(e + 12);
        let mut bbuf = [0u32; 12];
        bbuf[0] = (-first).to_bits();
        bbuf[1] = (-second).to_bits();
        bbuf[2] = 0x3e99_999a;
        let f = lf_checker_rt::callee_cdecl!(5, u32, dummy.as_mut_ptr() as u32, m, bbuf.as_mut_ptr() as u32);
        let w = f32::from_bits(bbuf[11]);
        raycast_finish(this, m, r0, r1, r2, r3, rf(f), rf(f + 4), rf(f + 8), rf(f + 12), ek, dk, fk, w)
    }
}

/// Shared raycast finish: solver-output scatter, flag, globals and gates.
///
/// `d0/ck/bk` are the scaled direction locals, `w` the solver's trailing
/// word. Runs the finalizer hook, publishes the sixteen range words, then
/// the two range gates; returns 1 when either gate hit.
#[allow(clippy::too_many_arguments)]
unsafe fn raycast_finish(
    this: u32,
    m: u32,
    r0: f32,
    r1: f32,
    r2: f32,
    r3: f32,
    s0: f32,
    s1: f32,
    s2: f32,
    s3: f32,
    d0: f32,
    ck: f32,
    bk: f32,
    w: f32,
) -> u8 {
    unsafe {
        let _ = m;
        lf_checker_rt::callee_cdecl!(6, u32,);
        let flag = lf_checker_rt::global::<u32>(0x0179_7730);
        flag.write(flag.read() | 1);
        let g = |off: u32| lf_checker_rt::global::<f32>(0x0179_76f0 + off);
        g(0x00).write(r0);
        g(0x04).write(r1);
        g(0x08).write(r2);
        g(0x10).write(r0 + ck);
        g(0x1c).write(w);
        g(0x14).write(r1 + d0);
        g(0x18).write(r2 + bk);
        g(0x20).write(s0);
        g(0x24).write(s1);
        g(0x28).write(s2);
        g(0x30).write(s0 + ck);
        g(0x0c).write(r3);
        g(0x2c).write(s3);
        g(0x34).write(s1 + d0);
        g(0x38).write(s2 + bk);
        g(0x3c).write(w);
        let e38 = ((this + 0x38) as *const u32).read();
        let mut dummy = [0u32; 4];
        let g0 = lf_checker_rt::relocated(0x0179_76f0);
        let hit1: u32 = lf_checker_rt::callee_stdcall!(
            7, u32, g0, dummy.as_mut_ptr() as u32, e38, 0xae, 0xffff_ffff, 7, 1, 0
        );
        if hit1 != 0 {
            return 1;
        }
        let g1 = lf_checker_rt::relocated(0x0179_7710);
        let hit2: u32 = lf_checker_rt::callee_stdcall!(
            7, u32, g1, dummy.as_mut_ptr() as u32, e38, 0xae, 0xffff_ffff, 7, 1, 0
        );
        if hit2 != 0 { 1 } else { 0 }
    }
}

/// Sensor re-read, hook pair, long-range block and output tail (stage 2).
unsafe fn sense_tail(
    this: u32,
    out_sign: u32,
    out_mag: u32,
    vtable: u32,
    entry_mode: u8,
) -> u32 {
    unsafe {
        let sense: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(((vtable + 0xec) as *const u32).read() as usize);
        let mut slot = [0u32; 4];
        let v = sense(this, slot.as_mut_ptr() as u32);
        let m = ((this + 0x20) as *const u32).read();
        let rf = |p: u32| (p as *const f32).read();
        let v0 = rf(v);
        let v1 = rf(v + 4);
        let v2 = rf(v + 8);
        let dot = (v1 * rf(m + 0x14) + v0 * rf(m + 0x10)) + v2 * rf(m + 0x18);
        let hook0: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vtable + 0x60) as *const u32).read() as usize);
        let a0 = hook0(this);
        let first = rf(a0);
        let hook1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vtable + 0x64) as *const u32).read() as usize);
        let a1 = hook1(this);
        let second = rf(a1 + 4);
        let now = lf_checker_rt::global::<u32>(0x0117_35b4).read();
        let stamped = ((this + 0xedc) as *const u32).read();
        if now.wrapping_sub(stamped) > 0x1f4 {
            let mode = ((this + 0xe6e) as *const u8).read();
            let case = (mode as i8 as i32 - 0x1d) as u32;
            // Raycast table: cases 0/2 run block A, cases 1/3 block B.
            let hit = if case == 0 || case == 2 {
                raycast_a(this, m, dot, first, second, mode)
            } else if case == 1 || case == 3 {
                raycast_b(this, m, dot, first, second, mode)
            } else {
                0
            };
            // Crossing check with a fresh sensor read.
            let mut dl: u8 = 0;
            if now > stamped.wrapping_add(0x64) {
                let w = sense(this, slot.as_mut_ptr() as u32);
                let x = rf(w);
                let y = rf(w + 4);
                let z = rf(w + 8);
                let cross = (rf(m + 0x14) * y + rf(m + 0x10) * x) + rf(m + 0x18) * z;
                let t = (mode as i8 as i32 - 0x1d) as u32;
                if t <= 3 {
                    let edge = rf(this + 0xf00);
                    let tol = lf_checker_rt::global::<f32>(0x00fe_86b4).read();
                    if t == 0 || t == 2 {
                        dl = if !(edge + tol < cross) { 1 } else { 0 };
                    } else {
                        dl = if !(cross < edge - tol) { 1 } else { 0 };
                    }
                }
            }
            // Latch countdown, then the rotate-or-keep decision.
            let mut al: u8 = 0;
            let latch = ((this + 0xefc) as *const u8).read();
            if latch != 0 && now > stamped.wrapping_add(0x5dc) {
                let dec = latch.wrapping_sub(1);
                ((this + 0xefc) as *mut u8).write(dec);
                if dec != 0 {
                    al = 1;
                } else {
                    ((this + 0xe6e) as *mut u8).write(entry_mode);
                    ((this + 0xee0) as *mut u32).write(0);
                    return 0;
                }
            }
            if hit != 0 || dl != 0 || al != 0 {
                let t = (((this + 0xe6e) as *const u8).read() as i8 as i32 - 0x1d) as u32;
                if t <= 3 {
                    ((this + 0xe6e) as *mut u8).write([0x1e, 0x1d, 0x20, 0x1f][t as usize]);
                }
                ((this + 0xedc) as *mut u32).write(now);
            }
        }
        let dir = ((this + 0xe6e) as *const u8).read() as i8 as i32;
        let case = (dir - 0x1d) as u32;
        if case > 3 {
            return 1;
        }
        let mag = ((this + 0xe6f) as *const u8).read() as f32;
        let hi = lf_checker_rt::global::<f32>(0x00fe_8afc).read();
        let lo = lf_checker_rt::global::<f32>(0x00fe_8dd8).read();
        match case {
            0 | 2 => {
                let r = if mag > hi { hi } else { mag };
                (out_mag as *mut f32).write(r);
            }
            _ => {
                let n = -mag;
                let r = if lo > n { lo } else { n };
                (out_mag as *mut f32).write(r);
            }
        }
        let edge = lf_checker_rt::global::<f32>(0x00fe_8d5c).read();
        // Tail table: cases 0/1 use the inverted gate, 2/3 the normal one.
        let plus = !(dot < edge);
        let neg_out = (case < 2) == plus;
        (out_sign as *mut u32).write(if neg_out { 0xbf80_0000 } else { 0x3f80_0000 });
        1
    }
}
