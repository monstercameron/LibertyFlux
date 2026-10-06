// original: 0x00ce6110 update_blend_from_nm (proposed)
#![allow(non_snake_case)]

//! See the export's doc comment for the full specification.

use lf_checker_rt::{callee_cdecl, callee_thiscall};

const G_TIMESTEP: u32 = 0x11735BC;
const G_STAMP: u32 = 0x11735B4;
const G_NMOBJ: u32 = 0x12B9C78;
const G_DIVISOR: u32 = 0x103B694;
const G_NAME0: u32 = 0x1051CC0;
const G_NAME1: u32 = 0x1051CC8;
const G_NAME2: u32 = 0x1051CE8;
const G_NAME3: u32 = 0x1051CE0;
const G_IDENT: u32 = 0x1110090;
const G_VEC_A: u32 = 0x1B4B320;
const G_VEC_B: u32 = 0x1B4B2A0;

const K_1000: f32 = f32::from_bits(0x447A0000);
const K_PI_2: f32 = f32::from_bits(0x3FC90FDB);
const K_1_2: f32 = f32::from_bits(0x3F99999A);
const K_1_0: f32 = 1.0;
const K_0_2: f32 = f32::from_bits(0x3E4CCCCD);
const K_0_35: f32 = f32::from_bits(0x3EB33333);
const K_0_25: f32 = 0.25;
const K_0_8: f32 = f32::from_bits(0x3F4CCCCD);
const K_0_5: f32 = 0.5;
const K_PI_4: f32 = f32::from_bits(0x3F490FDB);
const K_2PI: f32 = f32::from_bits(0x40C90FDB);
const K_1_3: f32 = f32::from_bits(0x3FA66666);
const K_0_1: f32 = f32::from_bits(0x3DCCCCCD);
const K_60: f32 = f32::from_bits(0x42700000);
const K_50: f32 = f32::from_bits(0x42480000);
const K_0_75: f32 = f32::from_bits(0x3F400000);
const K_1_1: f32 = f32::from_bits(0x3F8CCCCD);
const K_MSG_TARGET: u32 = 0xCDBE40;

#[inline(always)]
fn g32(va: u32) -> u32 {
    unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned() }
}
#[inline(always)]
fn gf(va: u32) -> f32 {
    f32::from_bits(g32(va))
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
#[inline(always)]
fn div(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}
#[inline(always)]
fn neg(a: f32) -> f32 {
    f32::from_bits(a.to_bits() ^ 0x8000_0000)
}

/// Emulate `fistp qword` of `x` under chop (truncate) control: NaN and
/// out-of-range values store the indefinite 0x8000... (low dword 0).
#[inline(always)]
fn fistp_qword(x: f32) -> (u32, u32) {
    const TWO63: f32 = 9.223372e18; // 2^63 exactly
    let q: i64 = if x.is_nan() || x >= TWO63 || x <= -TWO63 {
        i64::MIN
    } else {
        x as i64
    };
    (q as u32, (q >> 32) as u32)
}

macro_rules! fr {
    ($fr:ident, $off:expr) => {
        $fr[$off / 4]
    };
}
macro_rules! frf {
    ($fr:ident, $off:expr) => {
        f32::from_bits($fr[$off / 4])
    };
}
macro_rules! setf {
    ($fr:ident, $off:expr, $v:expr) => {
        $fr[$off / 4] = ($v).to_bits()
    };
}

#[inline(always)]
fn fp(fr: &[u32], off: usize) -> u32 {
    (&fr[off / 4] as *const u32) as u32
}
#[inline(always)]
fn rd(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
fn wr(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}
#[inline(always)]
fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}
#[inline(always)]
fn rd16(a: u32) -> u16 {
    unsafe { (a as *const u16).read_unaligned() }
}
#[inline(always)]
fn rdf(a: u32) -> f32 {
    f32::from_bits(rd(a))
}
#[inline(always)]
fn wrf(a: u32, v: f32) {
    wr(a, v.to_bits())
}

/// Refresh a NaturalMotion-driven blend task from the NM engine state.
///
/// `this` is the task (`+0x18` cooldown accumulator, `+0x1c` second timer,
/// `+0x20`/`+0x24`/`+0x28`/`+0x2c`/`+0x30`/`+0x34`/`+0x38` mode fields,
/// `+0x40` active channel handle, `+0x50` bypass flag) and `ped` is the
/// driven ped (`+0x219` NM-enabled byte, `+0x7b4` NM state object whose
/// `+0x10..+0x48` blend vector is read and rewritten, `+0x78` channel
/// owner, `+0x224`/`+0xa60`/`+0x118`/`+0xf4` behavior inputs).
///
/// Behaviour: accumulate scaled time into `+0x18` (x87 truncation of
/// timestep times 1000); while it stays at or below the unsigned limit
/// (2000, or 300 when the ped's NM byte is set) only restamp
/// `ped+0x7bc` and return the stamp. Otherwise zero the accumulator and
/// run the blend: fetch the ped's group and behaviour state through
/// helper calls, iterate an angle accumulator from 0 to 2*pi in pi/4
/// steps while polling two helpers, copy the NM blend vector to the
/// frame, issue the ped-vtable blend call, convert two frame floats to
/// doubles across the double maths helper, push the results through two
/// float setters, mirror the frame vector back into the NM object, poll
/// the group helpers, then either send two NM messages (bool + float)
/// and return void through the mid epilogue, or run the channel phase
/// (resolve/refresh the `+0x40` channel, tune its `+0x54` float by mode,
/// optionally rebuild its owner data, open or reuse a channel by the
/// `+0x28`/`+0x2c` or `+0x30`/`+0x34` selectors) and finish through the
/// tail epilogue. Return value is incidental (last helper answer, the
/// stamp on the gate path, or the ped pointer on one early path).
///
/// All compared counters and limits are UNSIGNED (the gate `jbe`, the
/// `+0x1c`/`+0x20` checks); float compares are ordered `>` with the
/// unordered case falling to the `jbe` side, matching `comiss`+`jbe`.
/// The two stale `(an instruction of the original)) of the two
/// `0x734d50` calls (thiscall/2). The security cookie and stack probe
/// run natively on the original side only.
///
/// Original: 0x00ce6110 (thiscall, one stack word).
/// Shared body. `signed_gate` is always false here; it selected the
/// deliberately-wrong signed gate of the mutant run, which is not shipped.
fn run_inner(this: u32, ped: u32, signed_gate: bool) -> u32 {

    unsafe {
    let mut fr = vec![0u32; 0x700].into_boxed_slice();
    // x87 control-word residue the original leaves via fnstcw/fldcw
    // (trampoline cw is 0x37F: high byte survives at F+0x10, cw|0xC00 at F+0x18).
    fr[0x10 / 4] = 0x0000_0300;
    fr[0x18 / 4] = 0x0000_0F7F;

    // Prologue: cooldown accumulation with x87 truncation.
    let step = mul(gf(G_TIMESTEP), K_1000);
    let (tick_lo, tick_hi) = fistp_qword(step);
    fr[0x98 / 4] = tick_lo;
    fr[0x9C / 4] = tick_hi;
    wr(this.wrapping_add(0x18), rd(this.wrapping_add(0x18)).wrapping_add(tick_lo));
    let nm_on = rd8(ped.wrapping_add(0x219)) != 0;
    let limit: u32 = if nm_on { 0x12c } else { 0x7d0 };
    // Gate (unsigned jbe): restamp and leave when still cooling down.
    if gate_exits(rd(this.wrapping_add(0x18)), limit, signed_gate) {
        let stamp = g32(G_STAMP);
        wr(ped.wrapping_add(0x7bc), stamp);
        return stamp;
    }
    let mut eax: u32;
    fr[0xC / 4] = ped;
    // Fetch behaviour state into F+0x198.
    wr(this.wrapping_add(0x18), 0);
    eax = callee_thiscall!(16, u32, ped, fp(&fr, 0x198), 0);
    // Identity-ish matrix from globals to F+0x58.
    fr[0x58 / 4] = g32(G_IDENT);
    fr[0x5C / 4] = g32(G_IDENT.wrapping_add(4));
    fr[0x60 / 4] = g32(G_IDENT.wrapping_add(8));
    fr[0x68 / 4] = g32(G_IDENT.wrapping_add(0x10));
    fr[0x6C / 4] = g32(G_IDENT.wrapping_add(0x14));
    fr[0x70 / 4] = g32(G_IDENT.wrapping_add(0x18));
    fr[0x78 / 4] = g32(G_IDENT.wrapping_add(0x20));
    fr[0x7C / 4] = g32(G_IDENT.wrapping_add(0x24));
    fr[0x80 / 4] = g32(G_IDENT.wrapping_add(0x28));
    // Copy fetched vector F+0x1C8 to F+0x88 (before the call: snapped).
    fr[0x88 / 4] = fr[0x1C8 / 4];
    fr[0x8C / 4] = fr[0x1CC / 4];
    fr[0x90 / 4] = fr[0x1D0 / 4];
    fr[0x94 / 4] = fr[0x1D4 / 4];
    // Gate helper on (ped, F+0x58).
    let g1: u32 = callee_thiscall!(40, u32, this, ped, fp(&fr, 0x58));
    eax = g1;
    fr[0x10 / 4] = (fr[0x10 / 4] & !0xff) | ((g1 & 0xff));
    setf!(fr, 0x38, 0.0f32);
    let mut x3: f32;
    if (g1 & 0xff) == 0 {
        // Angle seed through the double helper and two float helpers.
        let d1 = frf!(fr, 0x1AC) as f64;
        let d0 = neg(frf!(fr, 0x1A8)) as f64;
        let b0 = d0.to_bits();
        let b1 = d1.to_bits();
        let r43: u64 = callee_cdecl!(43, u64, b0 as u32, (b0 >> 32) as u32, b1 as u32, (b1 >> 32) as u32);
        eax = r43 as u32;
        let a0 = sub(f64::from_bits(r43) as f32, K_PI_2);
        setf!(fr, 0x18, a0);
        let h1b: u32 = callee_cdecl!(41, u32, a0.to_bits());
        let h1 = f32::from_bits(h1b);
        eax = h1.to_bits();
        setf!(fr, 0x54, h1);
        let h0b: u32 = callee_cdecl!(42, u32, frf!(fr, 0x18).to_bits());
        let h0 = f32::from_bits(h0b);
        eax = h0.to_bits();
        setf!(fr, 0x5C, h0);
        setf!(fr, 0x58, h1);
        fr[0x60 / 4] = 0;
        setf!(fr, 0x68, neg(h0));
        setf!(fr, 0x6C, h1);
        fr[0x70 / 4] = 0;
        fr[0x78 / 4] = 0;
        fr[0x7C / 4] = 0;
        fr[0x80 / 4] = 0x3f800000;
        x3 = frf!(fr, 0x1D0);
    } else {
        x3 = frf!(fr, 0x90);
    }
    x3 = sub(x3, K_1_2);
    // Probe descriptor at F+0x138 from global vector A, then the probe call.
    let vx = gf(G_VEC_A);
    let vy = gf(G_VEC_A.wrapping_add(4));
    let vz = gf(G_VEC_A.wrapping_add(8));
    fr[0x180 / 4] = 0;
    fr[0x18A / 4] = (fr[0x18A / 4] & !(0xffff << 16)) | (0 << 16);
    fr[0x138 / 4] = 0;
    setf!(fr, 0x148, vx);
    setf!(fr, 0x14C, vy);
    setf!(fr, 0x150, vz);
    setf!(fr, 0x158, vx);
    setf!(fr, 0x15C, vy);
    setf!(fr, 0x160, vz);
    setf!(fr, 0x168, vx);
    setf!(fr, 0x16C, vy);
    setf!(fr, 0x170, vz);
    fr[0x178 / 4] = 0;
    fr[0x17C / 4] = 0;
    fr[0x184 / 4] = 0xffff;
    fr[0x188 / 4] = (fr[0x188 / 4] & !0xff) | 0;
    // cdecl6(F+0x88, x3bits-over-edi-slot, ped, F+0x138, 0x8e, 4).
    let a25: u32 = callee_cdecl!(25, u32, fp(&fr, 0x88), x3.to_bits(), ped, fp(&fr, 0x138), 0x8e, 4);
    eax = a25;
    if (a25 & 0xff) != 0 {
        let p2 = add(frf!(fr, 0x150), K_1_0);
        let p0 = frf!(fr, 0x148);
        let p1 = frf!(fr, 0x14C);
        setf!(fr, 0x88, p0);
        let q = frf!(fr, 0x124);
        setf!(fr, 0x8C, p1);
        setf!(fr, 0x90, p2);
        setf!(fr, 0x94, q);
    }
    // Mode reset when the gate flag is set.
    if (fr[0x10 / 4] & 0xff) != 0 {
        let m28 = rd(this.wrapping_add(0x28));
        if m28 == 0xf || m28 == 0x62 {
            wr(this.wrapping_add(0x28), 0xffffffff);
        }
    }
    if rd(this.wrapping_add(0x50)) != 0 {
        // Bypass: skip straight to the NM-vector phase.
        // (implemented by the phase-3 block below via bypass flag)
        return blend_phase3(this, ped, &mut fr, eax);
    }
    // Reachability probe on (F+0xC, F+0x58).
    let a26a: u32 = callee_cdecl!(26, u32, fp(&fr, 0x58), fp(&fr, 0xC), 1, 0xae, 0xffffffff);
    eax = a26a;
    if (a26a & 0xff) == 0 {
        return blend_l2mid(this, ped, &mut fr, eax);
    }
    // Group chain: ped group, its info, NM byte.
    fr[0x10 / 4] = (fr[0x10 / 4] & !0xff) | (rd8(ped.wrapping_add(0x219)) as u32);
    setf!(fr, 0x34, K_0_2);
    let grp0: u32 = callee_thiscall!(17, u32, ped);
    eax = grp0;
    let mut flag_set = false;
    if grp0 != 0 {
        let grp1: u32 = callee_thiscall!(17, u32, ped);
        eax = grp1;
        let info1: u32 = callee_thiscall!(6, u32, grp1.wrapping_add(8));
        eax = info1;
        if info1 != 0 {
            let grp2: u32 = callee_thiscall!(17, u32, ped);
            eax = grp2;
            let info2: u32 = callee_thiscall!(6, u32, grp2.wrapping_add(8));
            eax = info2;
            if rd8(info2.wrapping_add(0x219)) != 0 {
                flag_set = true;
            }
        }
    }
    if !flag_set {
        // Local-player fallback chain.
        let mut fb = (fr[0x10 / 4] & 0xff) != 0;
        if rd8(ped.wrapping_add(0xa60)) == 2 {
            let lp0: u32 = callee_cdecl!(14, u32,);
            eax = lp0;
            if lp0 != 0 {
                let aux = rd(ped.wrapping_add(0x224));
                let lp1: u32 = callee_cdecl!(14, u32,);
                eax = lp1;
                let a27: u32 = callee_thiscall!(27, u32, aux, lp1);
                eax = a27;
                if (a27 & 0xff) != 0 {
                    fb = true;
                }
            }
        }
        if fb {
            fr[0x10 / 4] = (fr[0x10 / 4] & !0xff) | 1;
            flag_set = true;
        }
    } else {
        fr[0x10 / 4] = (fr[0x10 / 4] & !0xff) | 1;
    }
    if flag_set {
        let c: f32 = if rd(this.wrapping_add(0x1c)) > 0 { K_0_35 } else { K_0_25 };
        setf!(fr, 0x34, c);
    } else if rd(this.wrapping_add(0x28)) == 0x60 {
        let c: f32 = if rd(this.wrapping_add(0x1c)) > 0 { K_0_8 } else { K_0_5 };
        setf!(fr, 0x34, c);
    }
    // Frame-pair helper (encrypted callee 1).
    eax = callee_thiscall!(1, u32, fp(&fr, 0x1D8), fp(&fr, 0x58));
    // x1 = F+0x34 * 0.0; main angle loop.
    let mut x0 = 0.0f32;
    setf!(fr, 0x54, mul(frf!(fr, 0x34), 0.0));
    let mut to_mid = false;
    loop {
        let r42itb: u32 = callee_cdecl!(42, u32, x0.to_bits());
        let r42it = f32::from_bits(r42itb);
        eax = r42it.to_bits();
        x0 = neg(r42it);
        setf!(fr, 0x18, x0);
        let acc0 = frf!(fr, 0x38);
        let hb: u32 = callee_cdecl!(41, u32, acc0.to_bits());
        let h = f32::from_bits(hb);
        eax = h.to_bits();
        let mut x2 = mul(frf!(fr, 0x18), frf!(fr, 0x34));
        let mut x1 = h;
        x0 = frf!(fr, 0x54);
        x1 = mul(x1, frf!(fr, 0x34));
        x0 = add(x0, frf!(fr, 0x90));
        x2 = add(x2, frf!(fr, 0x88));
        x1 = add(x1, frf!(fr, 0x8C));
        setf!(fr, 0x210, x0);
        let t124 = frf!(fr, 0x124);
        let b26: u32 = callee_cdecl!(52, u32, fp(&fr, 0x1D8), fp(&fr, 0xC), 1, 0xae, 0x20);
        eax = b26;
        setf!(fr, 0x208, x2);
        setf!(fr, 0x20C, x1);
        setf!(fr, 0x214, t124);
        if (b26 & 0xff) == 0 {
            let b24: u32 = callee_cdecl!(24, u32, fp(&fr, 0x1C8), fp(&fr, 0x208), 0, 0, 0x86, 0xffffffff, 4);
            eax = b24;
            if b24 == 0 {
                if !(K_2PI > frf!(fr, 0x38)) {
                    break; // to L2 top
                }
                eax = callee_thiscall!(1, u32, fp(&fr, 0x58), fp(&fr, 0x1D8));
                to_mid = true;
                break; // to L2 mid
            }
        }
        let acc = add(frf!(fr, 0x38), K_PI_4);
        setf!(fr, 0x38, acc);
        x0 = acc; // 0x667F reloads the accumulator into xmm0
        if !(K_2PI > acc) {
            break; // to L2 top
        }
    }
    if to_mid {
        return blend_l2mid(this, ped, &mut fr, eax);
    }
    // L2 top.
    wr(this.wrapping_add(0x1c), rd(this.wrapping_add(0x1c)).wrapping_add(0x7d0));
    let a2: u32 = callee_cdecl!(2, u32,);
    eax = a2;
    let mut esi: u32 = 0x1770;
    if (a2 & 0xff) != 0 {
        esi = 0xbb8;
    }
    if (fr[0x10 / 4] & 0xff) == 0 {
        return blend_phase2(this, ped, &mut fr, eax);
    }
    if rd(this.wrapping_add(0x1c)) <= esi {
        return blend_phase2(this, ped, &mut fr, eax);
    }
    // Ray pair through the cdecl7/cdecl3 helpers.
    setf!(fr, 0x98, frf!(fr, 0x88));
    setf!(fr, 0x9C, frf!(fr, 0x8C));
    setf!(fr, 0xA0, frf!(fr, 0x90));
    let pa = ped.wrapping_add(0x80);
    fr[0x18 / 4] = pa;
    let c4: u32 = callee_thiscall!(4, u32, pa);
    eax = c4;
    let mut esi2: u32 = 0x18e;
    if (c4 & 0xff) == 0 {
        let c5: u32 = callee_thiscall!(5, u32, fr[0x18 / 4]);
        eax = c5;
        if (c5 & 0xff) == 0 {
            esi2 = 0x1ae;
        }
    }
    if rd8(ped.wrapping_add(0x118)) & 1 == 0 {
        esi2 |= 0x40;
    }
    let b33: u32 = callee_cdecl!(33, u32, fp(&fr, 0x88), fp(&fr, 0x98), 0x42480000, esi2, 0, 0, 0);
    eax = b33;
    if b33 != 0 {
        // copy F+0x98.. to F+0x88
        fr[0x88 / 4] = fr[0x98 / 4];
        fr[0x8C / 4] = fr[0x9C / 4];
        fr[0x90 / 4] = fr[0xA0 / 4];
        fr[0x94 / 4] = fr[0xA4 / 4];
    } else {
        let b32: u32 = callee_cdecl!(32, u32, fp(&fr, 0x88), fp(&fr, 0x98), 0x42480000);
        eax = b32;
        if (b32 & 0xff) != 0 {
            fr[0x88 / 4] = fr[0x98 / 4];
            fr[0x8C / 4] = fr[0x9C / 4];
            fr[0x90 / 4] = fr[0xA0 / 4];
            fr[0x94 / 4] = fr[0xA4 / 4];
        }
    }
    wr8(ped.wrapping_add(0xbe), 1);
    return blend_l2mid(this, ped, &mut fr, eax);
    }
}

#[inline(always)]
fn gate_exits(counter: u32, limit: u32, signed: bool) -> bool {
    if signed {
        (counter as i32) <= (limit as i32)
    } else {
        counter <= limit
    }
}

lf_checker_rt::export!(thiscall, rw_00ce6110(this: u32, ped: u32) -> u32 {
    run_inner(this, ped, false)
});

// (The deliberately-wrong signed-gate version used for the mutant run is not shipped.)

/// L2 mid (0x67b1): frame helper, second probe, NM-vector phase.
fn blend_l2mid(this: u32, ped: u32, fr: &mut [u32], mut eax: u32) -> u32 {
    unsafe {
        if rd(this.wrapping_add(0x50)) == 0 {
            eax = callee_thiscall!(3, u32, fp(fr, 0x1D8));
            // Second probe on (F+0x88, F+0x98, F+0xA0) with pre-stores.
            let t88 = frf!(fr, 0x88);
            setf!(fr, 0x98, t88);
            let t8c = frf!(fr, 0x8C);
            setf!(fr, 0x9C, t8c);
            let t90 = sub(frf!(fr, 0x90), K_1_3);
            setf!(fr, 0xA0, t90);
            let heap2c4 = rd(ped.wrapping_add(0x2c4));
            // cdecl9(F+0x88, F+0x98, 0.1, heap2c4, F+0x1D8, 0x19e, 0x20, 1, 0).
            let a23: u32 = callee_cdecl!(
                23, u32, fp(fr, 0x88), fp(fr, 0x98), 0x3DCCCCCD, heap2c4,
                fp(fr, 0x1D8), 0x19e, 0x20, 1, 0
            );
            eax = a23;
            if (a23 & 0xff) != 0 {
                let v = add(frf!(fr, 0x1F0), K_1_0);
                setf!(fr, 0x90, v);
                wr(ped.wrapping_add(0x26c), rd(ped.wrapping_add(0x26c)) | 1);
                wr(ped.wrapping_add(0x26c), rd(ped.wrapping_add(0x26c)) | 2);
                wr(ped.wrapping_add(0xb14), 0xbf800000);
            }
        }
        blend_phase3(this, ped, fr, eax)
    }
}

/// NM-vector phase (0x6884): copy blend vector, vtable calls, setters.
fn blend_phase3(this: u32, ped: u32, fr: &mut [u32], mut eax: u32) -> u32 {
    unsafe {
        let nm = rd(ped.wrapping_add(0x7b4));
        let skip_b = (rd8(ped.wrapping_add(0xf4)) & 0x20) != 0;
        // Copy NM blend vector +0x10..+0x48 to scattered frame slots.
        setf!(fr, 0x38, rdf(nm.wrapping_add(0x10)));
        setf!(fr, 0x34, rdf(nm.wrapping_add(0x14)));
        setf!(fr, 0x10, rdf(nm.wrapping_add(0x18)));
        setf!(fr, 0xB8, rdf(nm.wrapping_add(0x20)));
        setf!(fr, 0xFC, rdf(nm.wrapping_add(0x24)));
        setf!(fr, 0xF0, rdf(nm.wrapping_add(0x28)));
        setf!(fr, 0x108, rdf(nm.wrapping_add(0x30)));
        setf!(fr, 0xD4, rdf(nm.wrapping_add(0x34)));
        setf!(fr, 0x100, rdf(nm.wrapping_add(0x38)));
        setf!(fr, 0xD8, rdf(nm.wrapping_add(0x40)));
        setf!(fr, 0x54, rdf(nm.wrapping_add(0x44)));
        setf!(fr, 0x18, rdf(nm.wrapping_add(0x48)));
        if !skip_b {
            eax = callee_thiscall!(30, u32, ped);
        }
        // Ped vtable slot +4 on (F+0x58, 1, 0).
        {
            let vt: u32 = rd(ped);
            let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd(vt.wrapping_add(4)) as usize);
            eax = f(ped, fp(fr, 0x58), 1, 0);
        }
        // Two frame floats through the double helper, then two setters.
        let e1 = frf!(fr, 0x6C) as f64;
        let e0 = neg(frf!(fr, 0x68)) as f64;
        let c0 = e0.to_bits();
        let c1 = e1.to_bits();
        let r43: u64 =
            callee_cdecl!(43, u64, c0 as u32, (c0 >> 32) as u32, c1 as u32, (c1 >> 32) as u32);
        eax = r43 as u32;
        let s0 = f64::from_bits(r43) as f32;
        setf!(fr, 0xB4, s0);
        eax = callee_thiscall!(18, u32, ped, s0.to_bits());
        eax = callee_thiscall!(19, u32, ped, fr[0xB4 / 4]);
        // Mirror frame vector back into the NM object.
        let nm2 = rd(ped.wrapping_add(0x7b4));
        wrf(nm2.wrapping_add(0x10), frf!(fr, 0x58));
        wrf(nm2.wrapping_add(0x14), frf!(fr, 0x5C));
        wrf(nm2.wrapping_add(0x18), frf!(fr, 0x60));
        wrf(nm2.wrapping_add(0x20), frf!(fr, 0x68));
        wrf(nm2.wrapping_add(0x24), frf!(fr, 0x6C));
        wrf(nm2.wrapping_add(0x28), frf!(fr, 0x70));
        wrf(nm2.wrapping_add(0x30), frf!(fr, 0x78));
        wrf(nm2.wrapping_add(0x34), frf!(fr, 0x7C));
        wrf(nm2.wrapping_add(0x38), frf!(fr, 0x80));
        wrf(nm2.wrapping_add(0x40), frf!(fr, 0x88));
        wrf(nm2.wrapping_add(0x44), frf!(fr, 0x8C));
        wrf(nm2.wrapping_add(0x48), frf!(fr, 0x90));
        // Poll trio + ped-vtable chain (first site group).
        let gobj = g32(G_NMOBJ);
        let nmw = rd16(rd(ped.wrapping_add(0x7b4)).wrapping_add(8)) as u32;
        eax = callee_thiscall!(8, u32, gobj, nmw, 0);
        eax = callee_thiscall!(28, u32, ped);
        eax = callee_thiscall!(29, u32, ped, 1);
        eax = callee_thiscall!(20, u32, ped, 1);
        let vt: u32 = rd(rd(ped).wrapping_add(0xa0));
        let fa: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(vt as usize);
        let r1 = fa(ped);
        eax = r1;
        if r1 == 0 {
            return blend_chain_tail(this, ped, fr, eax, rd(ped.wrapping_add(0x100)));
        }
        let r2 = fa(ped);
        eax = r2;
        let chvt: u32 = rd(rd(r2).wrapping_add(0xe0));
        let fb: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(chvt as usize);
        eax = fb(r2);
        blend_chain_tail(this, ped, fr, eax, eax)
    }
}

/// After the ped-vtable chain (0x6bbf): channel owner probe + row select.
fn blend_chain_tail(this: u32, ped: u32, fr: &mut [u32], mut eax: u32, chain: u32) -> u32 {
    unsafe {
        let owner = rd(ped.wrapping_add(0x78));
        eax = callee_thiscall!(37, u32, owner, chain, 0x3f800000, 0xc0800000);
        // Row defaults from global vector B, refreshed from the row helper.
        setf!(fr, 0x118, gf(G_VEC_B));
        setf!(fr, 0x11C, gf(G_VEC_B.wrapping_add(4)));
        setf!(fr, 0x120, gf(G_VEC_B.wrapping_add(8)));
        let r0: u32 = callee_thiscall!(22, u32, ped);
        eax = r0;
        if r0 != 0 {
            let r1: u32 = callee_thiscall!(22, u32, ped);
            eax = r1;
            setf!(fr, 0x118, rdf(r1.wrapping_add(0x110)));
            setf!(fr, 0x11C, rdf(r1.wrapping_add(0x114)));
            setf!(fr, 0x120, rdf(r1.wrapping_add(0x118)));
        }
        // Mirror scattered slots back, call slot +0x104, mirror F+0x58 set.
        let nm = rd(ped.wrapping_add(0x7b4));
        wrf(nm.wrapping_add(0x10), frf!(fr, 0x38));
        wrf(nm.wrapping_add(0x14), frf!(fr, 0x34));
        wrf(nm.wrapping_add(0x18), frf!(fr, 0x10));
        wrf(nm.wrapping_add(0x20), frf!(fr, 0xB8));
        wrf(nm.wrapping_add(0x24), frf!(fr, 0xFC));
        wrf(nm.wrapping_add(0x28), frf!(fr, 0xF0));
        wrf(nm.wrapping_add(0x30), frf!(fr, 0x108));
        wrf(nm.wrapping_add(0x34), frf!(fr, 0xD4));
        wrf(nm.wrapping_add(0x38), frf!(fr, 0x100));
        wrf(nm.wrapping_add(0x40), frf!(fr, 0xD8));
        wrf(nm.wrapping_add(0x44), frf!(fr, 0x54));
        wrf(nm.wrapping_add(0x48), frf!(fr, 0x18));
        {
            let vt: u32 = rd(rd(nm).wrapping_add(0x104));
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(vt as usize);
            eax = f(nm);
        }
        let nm = rd(ped.wrapping_add(0x7b4));
        wrf(nm.wrapping_add(0x10), frf!(fr, 0x58));
        wrf(nm.wrapping_add(0x14), frf!(fr, 0x5C));
        wrf(nm.wrapping_add(0x18), frf!(fr, 0x60));
        wrf(nm.wrapping_add(0x20), frf!(fr, 0x68));
        wrf(nm.wrapping_add(0x24), frf!(fr, 0x6C));
        wrf(nm.wrapping_add(0x28), frf!(fr, 0x70));
        wrf(nm.wrapping_add(0x30), frf!(fr, 0x78));
        wrf(nm.wrapping_add(0x34), frf!(fr, 0x7C));
        wrf(nm.wrapping_add(0x38), frf!(fr, 0x80));
        wrf(nm.wrapping_add(0x40), frf!(fr, 0x88));
        wrf(nm.wrapping_add(0x44), frf!(fr, 0x8C));
        wrf(nm.wrapping_add(0x48), frf!(fr, 0x90));
        eax = callee_thiscall!(28, u32, ped);
        // Normaliser from divisor global + combiner arithmetic.
        let divisor = (g32(G_DIVISOR) as i32) as f32;
        let mut x4 = mul(frf!(fr, 0x118), frf!(fr, 0x68));
        let q = div(gf(G_TIMESTEP), divisor);
        setf!(fr, 0x98, q);
        let x1 = frf!(fr, 0x11C);
        x4 = add(x4, mul(x1, frf!(fr, 0x6C)));
        x4 = add(x4, mul(frf!(fr, 0x70), frf!(fr, 0x120)));
        let t = mul(frf!(fr, 0x58), frf!(fr, 0x118));
        setf!(fr, 0xB4, x4);
        x4 = mul(frf!(fr, 0x5C), x1);
        x4 = add(x4, t);
        x4 = add(x4, mul(frf!(fr, 0x60), frf!(fr, 0x120)));
        setf!(fr, 0x134, x4);
        // Second ped-vtable chain.
        let vt2: u32 = rd(rd(ped).wrapping_add(0xa0));
        let fa2: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(vt2 as usize);
        let s1 = fa2(ped);
        eax = s1;
        let chain2: u32;
        if s1 == 0 {
            chain2 = rd(ped.wrapping_add(0x100));
            eax = chain2;
        } else {
            let s2 = fa2(ped);
            eax = s2;
            let cvt: u32 = rd(rd(s2).wrapping_add(0xe0));
            let fb2: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(cvt as usize);
            eax = fb2(s2);
            chain2 = eax;
        }
        let owner2 = rd(ped.wrapping_add(0x78));
        eax = callee_thiscall!(
            38, u32, owner2, chain2, fr[0x98 / 4], fr[0x134 / 4], fr[0xB4 / 4]
        );
        // Mirror scattered slots, call slot +0x70, mirror F+0x58 set.
        let nm = rd(ped.wrapping_add(0x7b4));
        wrf(nm.wrapping_add(0x10), frf!(fr, 0x38));
        wrf(nm.wrapping_add(0x14), frf!(fr, 0x34));
        wrf(nm.wrapping_add(0x18), frf!(fr, 0x10));
        wrf(nm.wrapping_add(0x20), frf!(fr, 0xB8));
        wrf(nm.wrapping_add(0x24), frf!(fr, 0xFC));
        wrf(nm.wrapping_add(0x28), frf!(fr, 0xF0));
        wrf(nm.wrapping_add(0x30), frf!(fr, 0x108));
        wrf(nm.wrapping_add(0x34), frf!(fr, 0xD4));
        wrf(nm.wrapping_add(0x38), frf!(fr, 0x100));
        wrf(nm.wrapping_add(0x40), frf!(fr, 0xD8));
        wrf(nm.wrapping_add(0x44), frf!(fr, 0x54));
        wrf(nm.wrapping_add(0x48), frf!(fr, 0x18));
        {
            let vt: u32 = rd(rd(nm).wrapping_add(0x70));
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(vt as usize);
            eax = f(nm);
        }
        let nm = rd(ped.wrapping_add(0x7b4));
        wrf(nm.wrapping_add(0x10), frf!(fr, 0x58));
        wrf(nm.wrapping_add(0x14), frf!(fr, 0x5C));
        wrf(nm.wrapping_add(0x18), frf!(fr, 0x60));
        wrf(nm.wrapping_add(0x20), frf!(fr, 0x68));
        wrf(nm.wrapping_add(0x24), frf!(fr, 0x6C));
        wrf(nm.wrapping_add(0x28), frf!(fr, 0x70));
        wrf(nm.wrapping_add(0x30), frf!(fr, 0x78));
        wrf(nm.wrapping_add(0x34), frf!(fr, 0x7C));
        wrf(nm.wrapping_add(0x38), frf!(fr, 0x80));
        wrf(nm.wrapping_add(0x40), frf!(fr, 0x88));
        wrf(nm.wrapping_add(0x44), frf!(fr, 0x8C));
        wrf(nm.wrapping_add(0x48), frf!(fr, 0x90));
        blend_channel(this, ped, fr, eax)
    }
}

/// Message phase (0x6ad3): restamp, NM poll, two messages, mid return.
fn blend_phase2(this: u32, ped: u32, fr: &mut [u32], mut eax: u32) -> u32 {
    unsafe {
        eax = ped; // 0x6AD3 reloads eax before the stamp write
        wr(ped.wrapping_add(0x7bc), g32(G_STAMP));
        if rd8(this.wrapping_add(0x16)) != 0 {
            return eax; // tail epilogue
        }
        let nm = rd(ped.wrapping_add(0x7b4));
        let vt: u32 = rd(rd(nm).wrapping_add(0x50));
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(vt as usize);
        eax = f(nm);
        if eax == 0xffffffff {
            return eax; // tail epilogue
        }
        // First message buffer at F+0xF08.
        eax = callee_thiscall!(10, u32, fp(fr, 0xF08));
        let nm = rd(ped.wrapping_add(0x7b4));
        eax = callee_thiscall!(9, u32, nm, g32(G_NAME0), fp(fr, 0xF08));
        // Second message: bool + float.
        eax = callee_thiscall!(10, u32, fp(fr, 0x240));
        eax = callee_thiscall!(13, u32, fp(fr, 0x240), g32(G_NAME1), 1);
        eax = callee_thiscall!(12, u32, fp(fr, 0x240), g32(G_NAME2), K_60.to_bits());
        let nm = rd(ped.wrapping_add(0x7b4));
        eax = callee_thiscall!(9, u32, nm, g32(G_NAME3), fp(fr, 0x240));
        wr8(this.wrapping_add(0x16), 1);
        eax = callee_thiscall!(11, u32, fp(fr, 0x240));
        eax = callee_thiscall!(11, u32, fp(fr, 0xF08));
        eax // mid return (cookie check native on the original side)
    }
}

/// Channel phase (0x7016): poll, channel open/refresh, tail work.
fn blend_channel(this: u32, ped: u32, fr: &mut [u32], mut eax: u32) -> u32 {
    unsafe {
        let gobj = g32(G_NMOBJ);
        let nmw = rd16(rd(ped.wrapping_add(0x7b4)).wrapping_add(8)) as u32;
        eax = callee_thiscall!(8, u32, gobj, nmw, 0);
        eax = callee_thiscall!(28, u32, ped);
        let d28 = rd(this.wrapping_add(0x28));
        let d2c = rd(this.wrapping_add(0x2c));
        if d28 != 0xffffffff && d2c != 0xffffffff {
            let owner = rd(ped.wrapping_add(0x78));
            // thiscall4(owner, d2c, d28, 1000.0, -1).
            eax = callee_thiscall!(35, u32, owner, d2c, d28, K_1000.to_bits(), 0xffffffff);
            wr(this.wrapping_add(0x40), eax);
            let chan = eax;
            // thiscall3(chan, 1, target, this).
            eax = callee_thiscall!(39, u32, chan, 1, lf_checker_rt::relocated(K_MSG_TARGET), this);
            let c = rd(this.wrapping_add(0x2c));
            let a = rd(this.wrapping_add(0x28));
            let set3d = c == 4 || (c == 3 && a != 0x61 && a != 0x60);
            if set3d {
                wr8(this.wrapping_add(0x3d), 1);
            }
            // Mode dispatch on [this+0x38].
            let m = rd(this.wrapping_add(0x38)).wrapping_sub(0x64);
            if m == 0 {
                let a2: u32 = callee_cdecl!(2, u32,);
                eax = a2;
                let ch = rd(this.wrapping_add(0x40));
                if (a2 & 0xff) != 0 && rd8(ped.wrapping_add(0x219)) != 0 {
                    wr(ch.wrapping_add(0x54), K_1_2.to_bits());
                } else {
                    wr(ch.wrapping_add(0x54), K_1_1.to_bits());
                }
            } else if m.wrapping_sub(1) == 0 {
                let ch = rd(this.wrapping_add(0x40));
                wr(ch.wrapping_add(0x54), K_0_5.to_bits());
            } else if m == 3 {
                let ch = rd(this.wrapping_add(0x40));
                wr(ch.wrapping_add(0x54), K_0_75.to_bits());
            }
            if rd(this.wrapping_add(0x20)) != 0 {
                let ch = rd(this.wrapping_add(0x40));
                let mut bits = rd(ch.wrapping_add(4));
                if ((bits >> 4) & 1) != 0 {
                    bits &= !0x10;
                    wr(ch.wrapping_add(4), bits);
                }
                let o1 = rd(ped.wrapping_add(0x78));
                eax = callee_thiscall!(36, u32, o1);
                wr(eax.wrapping_add(0x4c), 0);
                let o2 = rd(ped.wrapping_add(0x78));
                let r: u32 = callee_thiscall!(36, u32, o2);
                eax = r;
                eax = callee_thiscall!(7, u32, r.wrapping_add(0x24), 0);
                wr(r.wrapping_add(0x54), 0);
                wr(r.wrapping_add(0x50), 0);
            }
        } else {
            let e30 = rd(this.wrapping_add(0x30));
            let e34 = rd(this.wrapping_add(0x34));
            if e30 != 0xffffffff && e34 != 0 {
                let flag50 = rd(this.wrapping_add(0x50));
                let k: u32 = if flag50 == 0 { 0xc000 } else { 0x4000 };
                let owner = rd(ped.wrapping_add(0x78));
                // thiscall5(owner, e30, e34, k, 3, 1000.0).
                eax = callee_thiscall!(34, u32, owner, e30, e34, k, 3, K_1000.to_bits());
                wr(this.wrapping_add(0x40), eax);
                let chan = eax;
                eax = callee_thiscall!(39, u32, chan, 1, lf_checker_rt::relocated(K_MSG_TARGET), this);
            } else {
                wr8(this.wrapping_add(0x3c), 1);
            }
            wr(this.wrapping_add(0x20), 0);
        }
        if rd(this.wrapping_add(0x24)) == 5 {
            wr8(this.wrapping_add(0x3d), 0);
        }
        eax = callee_thiscall!(21, u32, ped, 1, 1, 0);
        eax = callee_thiscall!(31, u32, ped, fp(fr, 0x118));
        let tail = rd(this.wrapping_add(0x50));
        if tail != 0 {
            // thiscall6(ped, tail, 0, 0x105, this+0x60, 0, 0).
            eax = callee_thiscall!(15, u32, ped, tail, 0, 0x105, this.wrapping_add(0x60), 0, 0);
        }
        eax // tail epilogue
    }
}
