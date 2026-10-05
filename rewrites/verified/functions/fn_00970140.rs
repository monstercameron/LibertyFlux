// original: 0x00970140 audio_voice_filter_update (proposed)

/// Refresh one voice's filter row and resynthesize its 32 smoother taps
/// (thiscall, two stack words, returns `this`).
///
/// `this` is a voice object, `arg0` a nullable command block, `arg1` an
/// opaque cookie forwarded to the smoother. Callee 0 revalidates the voice.
/// The row block reads the row-list pointer at `+0x5a0` (skipped when null)
/// and the row index at `+0x5a4`, stride 51 bytes: the scale seed is
/// 1.0 minus row[0x21], and the flag byte at `+0x31dc` is 1 unless the
/// floats at row+0x36 and row+0x3a compare equal (NaN counts as unequal).
///
/// Dispatch: a null command, or a row tag of exactly 1, takes the smoother
/// path (workspace scaled by the seed, 32 smoother calls, epilogue blend).
/// Otherwise the full orchestration runs: a TLS slot selects one of three
/// tuning triples; a signed command word at command+0x2e (negative values
/// index backwards, so the table base must stay live) picks a handler
/// through the table at 0xe95cd8; each handler fans out to the row
/// validators, the tap rebuild (callees 5-9), the four-tap picker (callee
/// 11, nine frame-pointer arguments, snapshots observe the contents), the
/// voice scan (callee 12, four rounds) and, when enabled, a five-distance
/// sqrt blend that rewrites the epilogue's scale slot. Two fills run inside
/// the live frame: a 0x01-byte mask over [0x174..] whose last byte can land
/// past the frame into the incoming-args area (write-only, never read back),
/// and a one-fill over [0x1c..].
///
/// Integer compares are signed (`jle` on the picker answer against 0 and
/// 0x3f, `js` on the handler count, signed `min`/`max` spans); the voice
/// gate is an unsigned `& 0x3c0 == 0x100`. All float selects are `comiss` +
/// `ja`/`jbe` (NaN loses), operand order pinned via black_box. The frame is
/// modelled as a byte array the callee stubs write into directly.
/// Callee 2 is the security-cookie check, stubbed preserving all registers.

use lf_checker_rt as rt;

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
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
unsafe fn wrf(a: u32, v: f32) {
    unsafe { wr32(a, v.to_bits()) }
}

#[inline(always)]
fn add_mem_first(m: f32, d: f32) -> f32 {
    core::hint::black_box(m) + core::hint::black_box(d)
}

#[inline(always)]
fn sub_new_old(n: f32, o: f32) -> f32 {
    core::hint::black_box(n) - core::hint::black_box(o)
}

#[inline(always)]
fn mul_mem_sign(m: f32, s: f32) -> f32 {
    core::hint::black_box(m) * core::hint::black_box(s)
}

#[inline(always)]
fn div_ordered(n: f32, d: f32) -> f32 {
    core::hint::black_box(n) / core::hint::black_box(d)
}

#[inline(always)]
unsafe fn g1() -> f32 {
    unsafe { f32::from_bits(rd32(rt::relocated(0x00fe88e8))) }
}

const FN1_INIT: u32 = 0;
const FN1_SMOOTH: u32 = 1;
const FN1_COOKIE: u32 = 2;

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}

#[inline(always)]
unsafe fn rd16s(a: u32) -> i32 {
    unsafe { ((a as *const u16).read_unaligned() as i16) as i32 }
}

/// `comiss a, b` + `ja` min-select: b wins only when a is strictly above, so
/// a NaN in a keeps a while a NaN in b keeps a too (unordered takes else).
#[inline(always)]
fn pick_min(a: f32, b: f32) -> f32 {
    if core::hint::black_box(a) > core::hint::black_box(b) {
        b
    } else {
        a
    }
}

#[inline(always)]
unsafe fn gblend() -> f32 {
    unsafe { f32::from_bits(rd32(rt::relocated(0x00fe87e4))) }
}

const FN1_A: u32 = 3;
const FN1_B: u32 = 4;
const FN1_C: u32 = 5;
const FN1_D: u32 = 6;
const FN1_E: u32 = 7;
const FN1_F: u32 = 8;
const FN1_G: u32 = 9;
const FN1_H: u32 = 10;
const FN1_I: u32 = 11;
const FN1_J: u32 = 12;

/// Full orchestration path (arg0 set, row tag not 1): drives the voice
/// through the outer table loop, rebuilding the tap cells per iteration.
/// The original spills its frame across these calls (one fill even lands
/// inside live slots), so the frame is modelled as a byte array that the
/// callee stubs write into directly.
unsafe fn full_path(
    this: u32,
    arg0: u32,
    arg1: u32,
    seed: f32,
    v2d: f32,
    v32: f32,
    v36: f32,
    v3a: f32,
    t20: f32,
    t64: f32,
    t48: f32,
) -> u32 {
    unsafe {
        let one = g1();
        let blend = gblend();
        let mut fr = [0u8; 0x190];
        let fb = fr.as_mut_ptr() as u32;
        wr32(fb + 0x38, this);
        wr32(fb + 0x44, arg0);
        wrf(fb + 0x54, one);
        wrf(fb + 0x58, seed);
        wrf(fb + 0x68, v2d);
        wrf(fb + 0x6c, v32);
        wrf(fb + 0x40, v36);
        wrf(fb + 0x3c, v3a);
        wrf(fb + 0x20, t20);
        wrf(fb + 0xa0, t20);
        wrf(fb + 0x64, t64);
        wrf(fb + 0xa4, t64);
        wrf(fb + 0x48, t48);
        wrf(fb + 0xa8, t48);
        for i in 0..32u32 {
            wrf(fb + 0xf0 + i * 4, 1.0);
        }
        // Table lookup on the signed command word.
        let w = rd16s(arg0 + 0x2e);
        let entry = rd32(
            rt::relocated(0x01295cd8)
                .wrapping_add((w as u32).wrapping_mul(4)),
        );
        if entry != 0 {
            let e70 = rd32(entry + 0x70);
            if e70 != 0 {
                wr32(fb + 0x78, e70);
                let ans: u32 = rt::callee_thiscall!(FN1_A, u32, rd32(fb + 0x44), 0);
                let mut edi = ans.wrapping_sub(1) as i32;
                wr32(fb + 0x60, edi as u32);
                if edi >= 0 {
                    loop {
                        let e70cur = rd32(fb + 0x78);
                        let b: u32 =
                            rt::callee_thiscall!(FN1_B, u32, e70cur, 0, edi as u32);
                        let mut next = false;
                        if rd32(b + 0x40) != 0 && rd32(b + 0x44) != 0 {
                            next = true;
                        }
                        if !next {
                            let al = rd8(b + 0x4c);
                            if al & 4 != 0 {
                                next = true;
                            } else {
                                let g = rd8(rt::relocated(0x0121f62c));
                                if g != 0 && al & 2 != 0 {
                                    next = true;
                                } else {
                                    if al & 2 != 0 {
                                        let f44 = rd32(fb + 0x44);
                                        let a: u32 = rt::callee_thiscall!(
                                            FN1_C, u32, f44, 0, edi as u32
                                        );
                                        if a != 0 {
                                            wr32(fb + 0x18, 0);
                                            wr32(fb + 0x14, 0xFFFFFFFF);
                                            let _: u32 = rt::callee_thiscall!(
                                                FN1_D, u32, a, f44, fb + 0x18, fb + 0x14
                                            );
                                            let ecx2 = rd32(fb + 0x18);
                                            if ecx2 != 0 {
                                                let a2: u32 = rt::callee_thiscall!(
                                                    FN1_E, u32, ecx2, 0, rd32(fb + 0x14)
                                                );
                                                // Signed: the original uses jle (take iff
                                                // a2 > 0 and a2 != 0x3f).
                                                let take = (a2 as i32) > 0 && a2 != 0x3f;
                                                if take {
                                                    wr8(fb + 0x13, 0);
                                                    let a3: u32 = rt::callee_thiscall!(
                                                        FN1_F, u32, this, ecx2, a2, fb + 0x13
                                                    );
                                                    let b13 = rd8(fb + 0x13);
                                                    let _: u32 = rt::callee_thiscall!(
                                                        FN1_G, u32, this, a3, b13 as u32
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    if !next {
                                        let f44 = rd32(fb + 0x44);
                                        let _: u32 = rt::callee_thiscall!(
                                            FN1_H, u32, f44, 0, edi as u32, fb + 0xb0
                                        );
                                        wr32(fb + 0x7c, 0);
                                        wr32(fb + 0x9c, 0);
                                        let mut edip = fb + 0xb0;
                                        for k in 0..4u32 {
                                            let s = 2 * k;
                                            let th = rd32(fb + 0x38);
                                            let _: u32 = rt::callee_thiscall!(
                                                FN1_I, u32, th, fb + 0xa0, edip,
                                                fb + 0x7c, fb + 0x184 + s, fb + 0x185 + s,
                                                fb + 0x70, fb + 0x17c + s,
                                                fb + 0x17d + s, fb + 0x9c
                                            );
                                            edip += 0x10;
                                        }
                                        // Byte min/max chains over the four tag bytes.
                                        let dl = rd8(fb + 0x180);
                                        let dh = rd8(fb + 0x182);
                                        let ch = rd8(fb + 0x17e);
                                        let cl = rd8(fb + 0x17c);
                                        let min1 = if dl < dh { dl } else { dh };
                                        let min2 = if cl < ch { cl } else { ch };
                                        wr32(fb + 0x34, min1 as u32);
                                        wr32(fb + 0x14, min2 as u32);
                                        let max1 = if dl > dh { dl } else { dh };
                                        let max2 = if cl > ch { cl } else { ch };
                                        wr32(fb + 0x18, max1 as u32);
                                        wr32(fb + 0x5c, max2 as u32);
                                        let lo = if min2 < min1 { min2 } else { min1 };
                                        let hi = if dl > max1 { dl } else { max1 };
                                        // Float blends over the sixteen tap words.
                                        let mut x0 = add_mem_first(rdf(fb + 0xb0), rdf(fb + 0xc0));
                                        let mut x1 = rdf(fb + 0xb4);
                                        let mut x2 = add_mem_first(rdf(fb + 0xb8), rdf(fb + 0xc8));
                                        x1 = add_mem_first(x1, rdf(fb + 0xc4));
                                        let mut x3 = rdf(fb + 0xd0);
                                        x2 = add_mem_first(x2, rdf(fb + 0xd8));
                                        x2 = add_mem_first(x2, rdf(fb + 0xe8));
                                        x3 = add_mem_first(x3, x0);
                                        x0 = rdf(fb + 0xd4);
                                        x3 = add_mem_first(x3, rdf(fb + 0xe0));
                                        x0 = add_mem_first(x0, x1);
                                        x0 = add_mem_first(x0, rdf(fb + 0xe4));
                                        x3 = mul_mem_sign(x3, blend);
                                        x0 = mul_mem_sign(x0, blend);
                                        x2 = mul_mem_sign(x2, blend);
                                        wrf(fb + 0x80, x3);
                                        wrf(fb + 0x98, x0);
                                        wrf(fb + 0x70, x2);
                                        // One-fill over [0x1c + lo .. 0x1c + hi].
                                        wr32(fb + 0x1c, 0);
                                        let (loi, hii) = (lo as i32, hi as i32);
                                        if loi <= hii {
                                            let n = (hii - loi + 1) as u32;
                                            for i in 0..n {
                                                wr8(fb + 0x1c + lo as u32 + i, 1);
                                            }
                                        }
                                        // Mask rows from the tag bytes.
                                        wr32(fb + 0x174, 0);
                                        wr32(fb + 0x178, 0);
                                        for outer in 0..3u32 {
                                            let dhh = rd8(fb + 0x184 + outer * 2);
                                            wr32(fb + 0x34, 0);
                                            wr32(fb + 0x5c, 0);
                                            let mut esi = 1 + outer;
                                            while esi < 4 {
                                                let dll = rd8(fb + 0x184 + esi * 2);
                                                wr8(fb + 0x13, dhh);
                                                let mn = if dhh < dll { dhh } else { dll };
                                                let mx = if dhh > dll { dhh } else { dll };
                                                wr32(fb + 0x5c, mn as u32);
                                                wr32(fb + 0x34, mx as u32);
                                                let (mni, mxi) = (mn as i32, mx as i32);
                                                let span = mxi - mni;
                                                if span < 4 {
                                                    if mni <= mxi {
                                                        let n = (span + 1) as u32;
                                                        for i in 0..n {
                                                            wr8(fb + 0x174 + mn as u32 + i, 1);
                                                        }
                                                    }
                                                } else if span == 4 {
                                                    wr32(fb + 0x174, 0x01010101);
                                                    wr32(fb + 0x178, 0x01010101);
                                                } else {
                                                    let top = mni + 8;
                                                    if mxi <= top {
                                                        let mut c = mxi;
                                                        while c <= top {
                                                            wr8(fb + 0x174 + (c % 8) as u32, 1);
                                                            c += 1;
                                                        }
                                                    }
                                                }
                                                esi += 1;
                                            }
                                        }
                                        // Voice scan: four taps through the picker.
                                        wrf(fb + 0x14, rdf(fb + 0x68));
                                        let mut x3 = 0.0f32;
                                        let f44j = rd32(fb + 0x44);
                                        let edij = rd32(fb + 0x60);
                                        for k in 0..4u32 {
                                            let ans: u32 = rt::callee_thiscall!(
                                                FN1_J, u32, f44j, 0, edij, k
                                            );
                                            if ans != 0 {
                                                if rd32(ans + 0x28) & 0x3c0 == 0x100 {
                                                    if rd8(ans + 0x22b) == 1 {
                                                        let al = rd8(ans + 0x248);
                                                        if al != 0 {
                                                            let ecx = (al as u32)
                                                                .wrapping_mul(32);
                                                            let base = rd32(
                                                                rt::relocated(0x01040100),
                                                            );
                                                            let ptr = base
                                                                .wrapping_sub(0x20)
                                                                .wrapping_add(ecx);
                                                            if ptr != 0 {
                                                                let v = rdf(ptr + 0x10);
                                                                let vabs =
                                                                    f32::from_bits(
                                                                        v.to_bits()
                                                                            & 0x7fffffff,
                                                                    );
                                                                let th = f32::from_bits(rd32(
                                                                    rt::relocated(0x01038830),
                                                                ));
                                                                if vabs > th {
                                                                    wrf(fb + 0x14, 0.0);
                                                                } else {
                                                                    let f14 = rdf(fb + 0x14);
                                                                    let f6c = rdf(fb + 0x6c);
                                                                    if f14 > f6c {
                                                                        wrf(fb + 0x14, f6c);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    // 0x872: only when [0x28] passes (xmm0 is 0
                                                    // on every path reaching it); ans==0 and
                                                    // [0x28]-fail go straight to 0x886.
                                                    if rd8(ans + 0x210) & 0x10 != 0 {
                                                        x3 = 0.0;
                                                        wrf(fb + 0x14, 0.0);
                                                    } else {
                                                        x3 = rdf(fb + 0x14);
                                                    }
                                                } else {
                                                    x3 = rdf(fb + 0x14);
                                                }
                                            } else {
                                                x3 = rdf(fb + 0x14);
                                            }
                                        }
                                        // Distance blend toward the focus tap.
                                        let gate = rd8(rt::relocated(0x0103909c));
                                        if gate != 0 && rd8(this + 0x31dc) != 0 {
                                            let x67 = rdf(fb + 0x20);
                                            let x65 = rdf(fb + 0x64);
                                            let f48 = rdf(fb + 0x48);
                                            let dist = |ax: f32, ay: f32, az: f32| {
                                                let dx = sub_new_old(ax, x67);
                                                let dy = sub_new_old(ay, x65);
                                                let dz = sub_new_old(az, f48);
                                                let dx2 = mul_mem_sign(dx, dx);
                                                let dy2 = mul_mem_sign(dy, dy);
                                                let dz2 = mul_mem_sign(dz, dz);
                                                add_mem_first(
                                                    add_mem_first(dy2, dx2),
                                                    dz2,
                                                )
                                                .sqrt()
                                            };
                                            let d0 = dist(
                                                rdf(fb + 0xb0),
                                                rdf(fb + 0xb4),
                                                rdf(fb + 0xb8),
                                            );
                                            let d1 = dist(
                                                rdf(fb + 0xc0),
                                                rdf(fb + 0xc4),
                                                rdf(fb + 0xc8),
                                            );
                                            let d2 = dist(
                                                rdf(fb + 0xd0),
                                                rdf(fb + 0xd4),
                                                rdf(fb + 0xd8),
                                            );
                                            let d3 = {
                                                let dy =
                                                    sub_new_old(rdf(fb + 0xe0), x67);
                                                let dz =
                                                    sub_new_old(rdf(fb + 0xe8), f48);
                                                let dy2 = mul_mem_sign(dy, dy);
                                                let dz2 = mul_mem_sign(dz, dz);
                                                let ey = sub_new_old(
                                                    rdf(fb + 0xe4),
                                                    rdf(fb + 0x64),
                                                );
                                                let ey2 = mul_mem_sign(ey, ey);
                                                add_mem_first(
                                                    add_mem_first(ey2, dy2),
                                                    dz2,
                                                )
                                                .sqrt()
                                            };
                                            let d4 = {
                                                let dz =
                                                    sub_new_old(rdf(fb + 0x70), f48);
                                                let dy = sub_new_old(
                                                    rdf(fb + 0x98),
                                                    rdf(fb + 0x64),
                                                );
                                                let dx = sub_new_old(
                                                    rdf(fb + 0x80),
                                                    rdf(fb + 0x20),
                                                );
                                                let dz2 = mul_mem_sign(dz, dz);
                                                let dy2 = mul_mem_sign(dy, dy);
                                                let dx2 = mul_mem_sign(dx, dx);
                                                add_mem_first(
                                                    add_mem_first(dy2, dx2),
                                                    dz2,
                                                )
                                                .sqrt()
                                            };
                                            let m1 = pick_min(d3, d2);
                                            let m0 = pick_min(d1, d0);
                                            let mut m = pick_min(m1, m0);
                                            m = pick_min(d4, m);
                                            let f3c = rdf(fb + 0x3c);
                                            let f40 = rdf(fb + 0x40);
                                            let mut mp = div_ordered(
                                                sub_new_old(m, f40),
                                                sub_new_old(f3c, f40),
                                            );
                                            if 0.0 > mp {
                                                mp = 0.0;
                                            }
                                            if mp > 1.0 {
                                                mp = 1.0;
                                            }
                                            let x1 = sub_new_old(one, mp);
                                            let x0 = sub_new_old(one, x1);
                                            let mut x2 = mul_mem_sign(x1, x3);
                                            x2 = add_mem_first(x2, x0);
                                            let f54 = rdf(fb + 0x54);
                                            if !(x2 > f54) {
                                                wrf(fb + 0x54, x2);
                                            }
                                        }
                                        // Cell min pass under both masks.
                                        let dh = rd8(fb + 0x17b);
                                        let dlm = rd8(fb + 0x17a);
                                        for c2 in 0..4u32 {
                                            let ea = fb + 0xf4 + c2 * 0x20;
                                            let m2 = rd8(fb + 0x1c + c2);
                                            if m2 != 0 {
                                                let cells = [
                                                    (rd8(fb + 0x174), ea - 4),
                                                    (rd8(fb + 0x175), ea),
                                                    (rd8(fb + 0x176), ea + 4),
                                                    (rd8(fb + 0x177), ea + 8),
                                                    (rd8(fb + 0x178), ea + 0xc),
                                                    (rd8(fb + 0x179), ea + 0x10),
                                                    (dh, ea + 0x14),
                                                    (dlm, ea + 0x18),
                                                ];
                                                for (m1, cell) in cells {
                                                    if m1 != 0 {
                                                        let cv = rdf(cell);
                                                        if cv > x3 {
                                                            wrf(cell, x3);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        edi -= 1;
                        wr32(fb + 0x60, edi as u32);
                        if edi < 0 {
                            break;
                        }
                    }
                }
            }
        }
        // Tail: scale the cells, smooth each tap, blend the epilogue.
        let s58 = rdf(fb + 0x58);
        for k in 0..32u32 {
            let c = rdf(fb + 0xf0 + k * 4);
            wrf(fb + 0xf0 + k * 4, mul_mem_sign(c, s58));
        }
        let mut store = this + 0x9b0;
        let mut obj = this + 0xa30;
        for k in 0..32u32 {
            let ans: f32 = rt::callee_thiscall!(
                FN1_SMOOTH,
                f32,
                obj,
                rdf(fb + 0xf0 + k * 4).to_bits(),
                arg1
            );
            wrf(store, ans);
            store += 4;
            obj += 0x1c;
        }
        let c = f32::from_bits(rd32(rt::relocated(0x01037a58)));
        let mut x = sub_new_old(one, c);
        x = mul_mem_sign(x, rdf(fb + 0x58));
        x = mul_mem_sign(x, rdf(fb + 0x54));
        x = add_mem_first(x, c);
        wrf(this + 0x2a0c, x);
        let _: u32 = rt::callee_cdecl!(FN1_COOKIE, u32,);
        rd32(fb + 0x38)
    }
}

unsafe fn body1(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let one = g1();
        let _: u32 = rt::callee_thiscall!(FN1_INIT, u32, this);
        // Row block.
        let row_base = rd32(this + 0x5a0);
        let row_idx = rd32(this + 0x5a4);
        let mut seed = 0.0f32;
        let mut flag: u8 = 0;
        let mut dl: u8 = 0;
        let (mut v2d, mut v32, mut v36, mut v3a) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
        if row_base != 0 {
            let r = row_base.wrapping_add(row_idx.wrapping_mul(0x33));
            let v21 = rdf(r + 0x21);
            v2d = rdf(r + 0x2d);
            v32 = rdf(r + 0x32);
            v36 = rdf(r + 0x36);
            v3a = rdf(r + 0x3a);
            seed = sub_new_old(one, v21);
            let eq = v36 == v3a;
            flag = if eq { 0 } else { 1 };
            dl = rd8(r + 0x31);
        }
        wr8(this + 0x31dc, flag);
        // Workspace base: 0.0 on the null path, 1.0 past the TLS block.
        let base: f32;
        if arg0 != 0 {
            let slot = rd32(rt::relocated(0x017aba14));
            let tls_ptr = rt::tls_slot(slot as usize);
            let tidx = rd32(tls_ptr + 0x70);
            let tbase = 0x0115e420u32.wrapping_add(tidx.wrapping_mul(64));
            let t20 = rdf(rt::relocated(tbase));
            let t64 = rdf(rt::relocated(tbase + 4));
            let t48 = rdf(rt::relocated(tbase + 8));
            if dl != 1 {
                return full_path(
                    this, arg0, arg1, seed, v2d, v32, v36, v3a, t20, t64, t48,
                );
            }
            base = 1.0;
        } else {
            base = 0.0;
        }
        let s = mul_mem_sign(base, seed);
        for k in 0..32u32 {
            let ans: f32 = rt::callee_thiscall!(
                FN1_SMOOTH,
                f32,
                this + 0xa30 + k * 0x1c,
                s.to_bits(),
                arg1
            );
            wrf(this + 0x9b0 + k * 4, ans);
        }
        // Epilogue blend: two pops shift esp, so the reads hit the seed slots
        // ([F+0x58] = seed, [F+0x54] = 1.0), not uninitialized memory.
        let c = f32::from_bits(rd32(rt::relocated(0x01037a58)));
        let mut x = sub_new_old(one, c);
        x = mul_mem_sign(x, seed);
        x = mul_mem_sign(x, one);
        x = add_mem_first(x, c);
        wrf(this + 0x2a0c, x);
        let _: u32 = rt::callee_cdecl!(FN1_COOKIE, u32,);
        this
    }
}

rt::export!(thiscall, rw_00970140(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe { body1(this, arg0, arg1) }
});
