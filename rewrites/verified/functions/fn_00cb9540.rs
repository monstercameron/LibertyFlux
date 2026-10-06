// original: 0x00cb9540 CTaskSimpleMoveGoToPointOnRoute::vf20
/// Advance a route-following move task one tick and refresh its steering result.
///
/// `this` is the task (flag dword at `+0xc4`, goal speed at `+0x24`, source
/// point at `+0x40`, aux triples at `+0x70`/`+0x80`, timer words at
/// `+0xa8`/`+0xac`/`+0xb4`/`+0xb8`, state bytes at `+0xb0`/`+0xbc`/`+0xbd`,
/// callee answers at `+0xc0`/`+0xc1`, result triple at `+0x90`, output at
/// `+0x18`); `arg0` is the mover (position struct at `+0x20`, hook holder at
/// `+0x224`, hook holder at `+0xa80`, flag words at `+0x274`/`+0x29c`).
/// Returns 1 on the early-exit path, else 0; only `al` is significant.
///
/// Behaviour: after an entry copy gated by flag bit 3 and an early exit on
/// flag bit 4 or a veto hook, the task measures the planar distance to the
/// mover, scales it by a unit tunable over its root (zero scale and an
/// infinite factor when the squared distance is exactly zero, NaN throughout
/// when it is NaN) and may set a hook-holder bit when that factor is below a
/// threshold. Two refresh
/// blocks, each guarded by a config byte, a state byte and a SIGNED timer
/// comparison (`base+span <= counter`, wrapping add, signed `<=`), invoke a
/// route-scan hook, a normaliser hook and a second scan hook. Two answer
/// bytes then select a projection path (a distance hook, a second
/// normaliser call, fused results), a direct path, or a join that keeps the
/// entry values. A pair of virtual hooks (slots `0x58`/`0x5c`) always runs,
/// then an optional aim block (config byte, holder bit 1 clear, goal above
/// a threshold): an angle hook, an absolute value against an angle limit, a
/// virtual aim hook (slot `0x54`) selecting a clamped correction through a
/// shaping hook, or a goal copy. A final virtual hook (slot unspecified in
/// the callee table) may clear the output; flag bit 8 runs a mid hook, an
/// unconditional hook always runs, and flag bit 3 runs a virtual slot-8
/// hook whose answer feeds a follow-up hook. One frame word is read before
/// any write on paths that skip the first normaliser call; the worker fills
/// it with a defined value on both sides. All float comparisons are ordered
/// (NaN takes the "not greater" side); the two zero tests after the length
/// computations are exact-equality tests (`-0.0` counts as zero). The state
/// byte at `+0xb1` is always zero at its test, so its reset block is dead.
/// Calling convention is thiscall; only `al` is significant on return.
///
/// Original: 0x00cb9540 (merged symbol CTaskSimpleMoveGoToPointOnRoute::vf20).
#[inline(always)]
unsafe fn v2_rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn v2_wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn v2_rb(a: u32) -> u8 {
    unsafe { (a as *const u8).read_unaligned() }
}
#[inline(always)]
unsafe fn v2_wb(a: u32, v: u8) {
    unsafe { (a as *mut u8).write_unaligned(v) }
}
#[inline(always)]
unsafe fn v2_rdf(a: u32) -> f32 {
    unsafe { f32::from_bits(v2_rd32(a)) }
}
#[inline(always)]
unsafe fn v2_wrf(a: u32, v: f32) {
    unsafe { v2_wr32(a, v.to_bits()) }
}
#[inline(always)]
unsafe fn v2_g32(va: u32) -> u32 {
    unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
}
#[inline(always)]
unsafe fn v2_gf(va: u32) -> f32 {
    unsafe { f32::from_bits(v2_g32(va)) }
}
#[inline(always)]
fn v2_add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}
#[inline(always)]
fn v2_sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}
#[inline(always)]
fn v2_mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}
#[inline(always)]
fn v2_div(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

/// Shared body for the rewrite and its wrong version (`flip_exit` inverts
/// the flag-bit exit gate in the mutant only).
unsafe fn vf20_inner(this: u32, arg0: u32, flip_exit: bool) -> u32 {
    unsafe {
        const T_OUT: u32 = 0x18;
        const T_GOAL: u32 = 0x24;
        const T_SRC: u32 = 0x40;
        const T_AUX: u32 = 0x70;
        const T_AUX2: u32 = 0x80;
        const T_RES: u32 = 0x90;
        const T_T0: u32 = 0xA8;
        const T_T1: u32 = 0xAC;
        const T_B0: u32 = 0xB0;
        const T_T2: u32 = 0xB4;
        const T_T3: u32 = 0xB8;
        const T_BC: u32 = 0xBC;
        const T_BD: u32 = 0xBD;
        const T_C0: u32 = 0xC0;
        const T_C1: u32 = 0xC1;
        const T_FLAGS: u32 = 0xC4;
        const F_COPY: u32 = 0x8;
        const F_EXIT: u32 = 0x10;
        const F_DONE: u32 = 0x20;
        const F_TMP: u32 = 0x40;
        const F_SEEN: u32 = 0x80;
        const F_MID: u32 = 0x100;
        const A_POS: u32 = 0x20;
        const A_78: u32 = 0x78;
        const A_VT8H: u32 = 0x224;
        const A_F274: u32 = 0x274;
        const A_M290: u32 = 0x290;
        const A_F29C: u32 = 0x29C;
        const A_V54H: u32 = 0xA80;
        const A_AA0: u32 = 0xAA0;
        const A_AA4: u32 = 0xAA4;
        const A_BE0: u32 = 0xBE0;
        const G_CNT: u32 = 0x1173_5B4;
        const G_F4: u32 = 0x171B_BF4;
        const G_F8: u32 = 0x171B_BF8;
        const G_CFG: u32 = 0x1050_E28;
        const G_ANG: u32 = 0x1050_E2C;
        const G_S40: u32 = 0x1050_E40;
        const G_S44: u32 = 0x1050_E44;
        const G_GATE: u32 = 0x1050_E5C;
        const G_LIM: u32 = 0x1050_E74;
        const G_TWO: u32 = 0x1051_458;
        const G_ONE: u32 = 0xFE88E8;
        const G_HALF: u32 = 0xFE8830;
        const G_R9: u32 = 0xFE8A24;
        const G_PI: u32 = 0xFE8AA0;
        let mut c4 = v2_rd32(this + T_FLAGS);
        if c4 & F_COPY != 0 {
            let pos = v2_rd32(arg0 + A_POS);
            v2_wr32(this + 0x30, v2_rd32(pos + 0x30));
            v2_wrf(this + 0x34, v2_rdf(pos + 0x34));
            v2_wrf(this + 0x38, v2_rdf(pos + 0x38));
            v2_wr32(this + 0x3C, v2_rd32(pos + 0x3C));
        }
        c4 |= F_SEEN;
        v2_wr32(this + T_FLAGS, c4);
        v2_wr32(arg0 + A_BE0, v2_rd32(arg0 + A_BE0) | 2);
        if (c4 & F_EXIT != 0) ^ flip_exit {
            c4 |= F_DONE;
            v2_wr32(this + T_FLAGS, c4);
            lf_checker_rt::callee_thiscall!(11, u32, this, arg0);
            return 1;
        }
        let r1 = lf_checker_rt::callee_thiscall!(1, u32, this, this + T_SRC, arg0);
        if (r1 & 0xFF) != 0 {
            c4 |= F_DONE;
            v2_wr32(this + T_FLAGS, c4);
            lf_checker_rt::callee_thiscall!(11, u32, this, arg0);
            return 1;
        }
        let s24 = v2_rdf(this + T_GOAL);
        v2_wrf(this + T_OUT, s24);
        v2_wr32(this + 0x90, v2_rd32(this + 0x40));
        let s40 = v2_rdf(this + 0x40);
        let s44 = v2_rdf(this + 0x44);
        v2_wrf(this + 0x98, v2_rdf(this + 0x48));
        v2_wrf(this + 0x94, s44);
        v2_wr32(this + 0x9C, v2_rd32(this + 0x4C));
        let pos = v2_rd32(arg0 + A_POS);
        let posx = v2_rdf(pos + 0x30);
        let posy = v2_rdf(pos + 0x34);
        let posz = v2_rdf(pos + 0x38);
        let dx = v2_sub(s40, posx);
        let dy = v2_sub(s44, posy);
        let sum = v2_add(v2_mul(dy, dy), v2_mul(dx, dx));
        let r6 = v2_gf(G_ONE);
        let (x2s, x3s, x5s, x1) = if sum != 0.0 {
            let sq = sum.sqrt();
            let x4 = v2_div(r6, sq);
            (v2_mul(dx, x4), v2_mul(dy, x4), v2_mul(x4, 0.0), v2_div(r6, x4))
        } else {
            (v2_mul(dx, 0.0), v2_mul(dy, 0.0), v2_mul(0.0, 0.0), v2_div(r6, 0.0))
        };
        if v2_gf(G_TWO) > x1 {
            let h = v2_rd32(arg0 + A_V54H);
            v2_wr32(h + 0x50, v2_rd32(h + 0x50) | 0x80000);
        }
        let e0c = x3s;
        let mut e10 = x2s;
        let mut e20 = x5s;
        let mut buf = [0.0f32; 4];
        let cnt = v2_g32(G_CNT);
        if v2_rb(this + T_B0) != 0 {
            let a8 = v2_rd32(this + T_T0);
            let a1b = v2_rd32(this + T_T1);
            if (a8.wrapping_add(a1b) as i32) <= (cnt as i32) {
                if v2_rd32(arg0 + A_F274) & 0x80 != 0 {
                    v2_wr32(this + T_T1, v2_g32(G_F4));
                    buf[0] = x2s;
                    buf[1] = x3s;
                    buf[2] = x5s;
                    v2_wr32(this + T_T0, cnt);
                    v2_wb(this + T_B0, 1);
                    let r2 = lf_checker_rt::callee_stdcall!(2, u32, arg0,
                        buf.as_mut_ptr() as u32, this + T_AUX);
                    v2_wb(this + T_C0, (r2 & 0xFF) as u8);
                }
            }
        }
        let mut id3fired = false;
        let mut w6c = 0.0f32;
        if ((v2_g32(G_CFG) >> 8) & 0xFF) != 0 {
            if v2_rb(this + T_BC) != 0 {
                if v2_rb(this + T_BD) != 0 {
                    v2_wr32(this + T_T2, cnt);
                    v2_wb(this + T_BD, 0);
                }
                let b8 = v2_rd32(this + T_T3);
                let b4 = v2_rd32(this + T_T2);
                if (b8.wrapping_add(b4) as i32) <= (cnt as i32) {
                    if v2_rd32(arg0 + A_F274) & 0x100 != 0 {
                        v2_wr32(this + T_T2, cnt);
                        v2_wr32(this + T_T3, v2_g32(G_F8));
                        v2_wb(this + T_BC, 1);
                        buf[0] = v2_add(x2s, v2_rdf(this + 0x70));
                        buf[1] = v2_add(v2_rdf(this + 0x74), x3s);
                        buf[2] = v2_add(v2_rdf(this + 0x78), x5s);
                        buf[3] = 0.0;
                        let id3: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(
                                lf_checker_rt::callee_addr(3) as usize);
                        id3(buf.as_mut_ptr() as u32);
                        id3fired = true;
                        w6c = buf[3];
                        let r4 = lf_checker_rt::callee_thiscall!(4, u32, this, arg0,
                            buf.as_mut_ptr() as u32, this + T_AUX2);
                        v2_wb(this + T_C1, (r4 & 0xFF) as u8);
                    }
                }
            }
        }
        let s9c = if id3fired { w6c } else { 0.0 };
        if v2_rb(this + T_C0) == 0 && v2_rb(this + T_C1) == 0 {
            if ((v2_g32(G_CFG) >> 16) & 0xFF) != 0
                && v2_rd32(arg0 + A_F29C) & 0x800 != 0
            {
                let s30 = v2_rdf(this + 0x30);
                let s34 = v2_rdf(this + 0x34);
                let mut b50 = [s30, s34, 0.0f32];
                let mut b40 = [v2_sub(s40, s30), v2_sub(s44, s34), 0.0f32];
                let mut b60 = [posx, posy, 0.0f32];
                let id5: extern "cdecl" fn(u32, u32, u32) -> f32 =
                    core::mem::transmute(lf_checker_rt::callee_addr(5) as usize);
                let r = id5(b50.as_mut_ptr() as u32, b40.as_mut_ptr() as u32,
                    b60.as_mut_ptr() as u32);
                let t34 = v2_sub(v2_add(v2_mul(b40[1], r), s34), posy);
                let t30 = v2_sub(v2_add(v2_mul(b40[0], r), s30), posx);
                let t38 = v2_sub(v2_add(v2_mul(0.0, r), 0.0), 0.0);
                if v2_add(v2_mul(t34, t34), v2_mul(t30, t30)) > v2_gf(G_GATE) {
                    let mut b32 = [t30, t34, t38, 0.0f32];
                    let id3b: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(
                            lf_checker_rt::callee_addr(3) as usize);
                    id3b(b32.as_mut_ptr() as u32);
                    let s40g = v2_gf(G_S40);
                    let m290 = v2_rdf(arg0 + A_M290);
                    let x6 = v2_add(v2_add(v2_mul(e10, s40g), posx),
                        v2_mul(b32[0], m290));
                    let x0 = v2_add(v2_add(posz, v2_mul(s40g, e20)),
                        v2_mul(b32[2], m290));
                    let x1b = v2_add(v2_add(posy, v2_mul(e0c, s40g)),
                        v2_mul(b32[1], m290));
                    v2_wrf(this + 0x90, x6);
                    v2_wrf(this + 0x98, x0);
                    v2_wrf(this + 0x94, x1b);
                    v2_wrf(this + 0x9C, s9c);
                }
            }
        } else {
            let s40g = v2_gf(G_S40);
            let x1m = v2_mul(e0c, s40g);
            let x0m = v2_mul(e10, s40g);
            let x3m = v2_mul(s40g, e20);
            let x6a = v2_add(v2_rdf(this + 0x84), v2_rdf(this + 0x74));
            let x5a = v2_add(v2_rdf(this + 0x70), v2_rdf(this + 0x80));
            let x4a = v2_add(v2_rdf(this + 0x88), v2_rdf(this + 0x78));
            let x6 = v2_add(v2_add(v2_mul(x6a, x1m), v2_mul(x5a, x0m)),
                v2_mul(x4a, x3m));
            v2_wrf(this + 0x90, v2_add(v2_add(posx, x0m), x5a));
            v2_wrf(this + 0x98, v2_add(v2_add(posz, x3m), x4a));
            v2_wrf(this + 0x94, v2_add(v2_add(posy, x1m), x6a));
            v2_wrf(this + 0x9C, s9c);
            if 0.0 > x6 {
                let x6n = f32::from_bits(x6.to_bits() ^ 0x8000_0000);
                let s44 = v2_gf(G_S44);
                let x6c = v2_mul(v2_sub(x6n, s44), v2_div(r6, s44));
                let m = if x6c < 0.0 { 0.0 } else { x6c };
                let d = v2_sub(r6, m);
                let r8 = v2_gf(G_HALF);
                let fac = if d > r8 { d } else { r8 };
                v2_wrf(this + T_OUT, v2_mul(fac, s24));
            }
        }
        let vt = v2_rd32(this);
        for slot in [0x58u32, 0x5Cu32] {
            let tgt = v2_rd32(vt + slot);
            let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(this, arg0, this + T_RES);
        }
        if (v2_g32(G_CFG) & 0xFF) != 0 {
            let h = v2_rd32(arg0 + A_V54H);
            if ((v2_rd32(h + 0x50) >> 1) & 1) == 0 {
                let r9 = v2_gf(G_R9);
                if s24 >= r9 {
                    let slim = v2_gf(G_LIM);
                    e20 = v2_sub(s24, slim);
                    let aa = v2_sub(v2_rdf(arg0 + A_AA4), v2_rdf(arg0 + A_AA0));
                    let id7: extern "cdecl" fn(u32) -> f32 =
                        core::mem::transmute(
                            lf_checker_rt::callee_addr(7) as usize);
                    let r7 = id7(aa.to_bits());
                    let ar7 = f32::from_bits(r7.to_bits() & 0x7FFF_FFFF);
                    e10 = ar7;
                    if ar7 > v2_gf(G_ANG) {
                        if v2_rdf(this + T_OUT) > slim {
                            let vt8t = v2_rd32(h);
                            let tgt = v2_rd32(vt8t + 0x54);
                            let f8: extern "thiscall" fn(u32, u32) -> u32 =
                                core::mem::transmute(tgt as usize);
                            if f8(h, v2_rd32(arg0 + A_78)) == 0 {
                                let t = v2_sub(v2_gf(G_PI), v2_gf(G_ANG));
                                let q = v2_div(e10, t);
                                let cc = if q < 0.0 {
                                    0.0
                                } else if q > r6 {
                                    r6
                                } else {
                                    q
                                };
                                e10 = v2_mul(v2_mul(cc, e20), r9);
                                let mut b9 = [v2_sub(s40, posx),
                                    v2_sub(s44, posy), 0.0f32];
                                let id9: extern "cdecl" fn(u32) -> f32 =
                                    core::mem::transmute(
                                        lf_checker_rt::callee_addr(9) as usize);
                                let r9a = id9(b9.as_mut_ptr() as u32);
                                let x = if r9 > r9a {
                                    v2_add(v2_sub(r6, v2_mul(r9a, v2_gf(G_HALF))), e10)
                                } else {
                                    e10
                                };
                                let d18 = v2_sub(s24, x);
                                v2_wrf(this + T_OUT,
                                    if d18 > slim { d18 } else { slim });
                            } else {
                                v2_wr32(this + T_OUT, v2_rd32(this + T_GOAL));
                            }
                        }
                    }
                }
            }
        }
        let r10 = lf_checker_rt::callee_thiscall!(10, u32, this, arg0, this + T_RES);
        if (r10 & 0xFF) == 0 {
            v2_wr32(this + T_OUT, 0);
        }
        if c4 & F_MID != 0 {
            lf_checker_rt::callee_thiscall!(11, u32, this, arg0);
            c4 &= !F_MID;
            v2_wr32(this + T_FLAGS, c4);
        }
        lf_checker_rt::callee_thiscall!(12, u32, this, arg0);
        c4 &= !F_TMP;
        v2_wr32(this + T_FLAGS, c4);
        if c4 & F_COPY != 0 {
            let h = v2_rd32(arg0 + A_VT8H);
            let tgt = v2_rd32(v2_rd32(h) + 8);
            let f13: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            let id14: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(14) as usize);
            id14(f13(h, arg0, 0));
            c4 &= !F_COPY;
            v2_wr32(this + T_FLAGS, c4);
        }
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00cb9540(this: u32, arg0: u32) -> u32 {
    unsafe { vf20_inner(this, arg0, false) }
});
