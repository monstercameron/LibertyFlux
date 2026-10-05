// original: 0x00acce10 audio_mixer_render (proposed)
/// Mixer render: copy the source matrix block into the out buffer, run a
/// bounded voice loop, scale per-voice records, run four chained 6-word
/// struct fills, reduce their outputs, run one 5-word combine call, run a
/// per-voice second loop, and finish with an 8-word emit call whose answer
/// is returned.
///
/// `cdecl` with nine stack words: `a0` source object, `a1`/`a2` small
/// parameter blocks, `a3` out buffer, `a4` voice array (stride `0x170`),
/// `a5` voice bound (signed: `<= 0` skips all three voice loops), `a6` a
/// float, `a7` an opaque word, `a8` a float. Returns the emit call's answer.
///
/// The opening ~60 float ops (a matrix-style chain over `a0`/`a1`/`a2`)
/// are dead: every result lands in a frame slot that is never loaded again
/// (verified by reading all loads), so the rewrite omits them. What the
/// function observably does, in order:
/// 1. Call the double callee with `(f64)(a8 / global)`, convert the answer
///    to float, take `max(answer, 1.0)` (NaN-preserving) and truncate to
///    the repeat `count`. `count <= 0` (only NaN) skips the whole middle.
///    `xmm6 = a8 / (float)count`.
/// 2. Copy the `a0` matrix block into `a3` (sparse words `0..0x38` plus the
///    `0x40`/`0x50` groups).
/// 3. Voice loop (`a5` iterations): store `voice+0x88` into the out words
///    at `a3+0x60`, call the per-voice reset (`thiscall/0`, zeroes
///    `voice+0xF0..0x108`, never read back), zero three of every four
///    words at `a3+0x80`.
/// 4. Outer loop, `count` iterations (1 or 2 in the contract): subtract
///    `xmm6` from every `voice+0x148` with a NaN-preserving clamp at 0
///    (unrolled x4 for bounds `>= 4`, scalar tail); run fills A..D
///    (`cdecl/6`, each taking the previous fill's 56-word out struct plus
///    shared in-structs); reduce the four out structs with the 2.0/0.1667
///    weights; run the 5-word combine call; fold the results into
///    `a3+0x40..0x5C` (the `+0x4C`/`+0x5C` words are the never-written
///    frame word, i.e. `0.0` under the zero stack fill); run the second
///    loop blending struct rows with the out words.
///    The only register carried across outer iterations is `xmm2` (the
///    accumulating `B+0x28` slot); `xmm3` is reloaded to 1.0 after the
///    second loop on every iteration, so each iteration divides 1.0.
/// 5. Emit call (`cdecl/8`) over `a0`/`a3`/`a4`, the bound, the two floats,
///    the opaque word and `a3+0x40`; return its answer.
///
/// Float order is pinned with the explicit-operand-order helpers: the
/// signalling-NaN-first, else destination-quiet-NaN-first rule is
/// implemented in bits because LLVM reorders operands at opt-level 3 and
/// two-NaN payload forwarding is positional (measured on the worker).
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};
#[inline(always)]
fn is_nan_bits(b: u32) -> bool {
    (b & 0x7F80_0000) == 0x7F80_0000 && (b & 0x007F_FFFF) != 0
}
#[inline(always)]
fn is_snan_bits(b: u32) -> bool {
    (b & 0x7F80_0000) == 0x7F80_0000
        && (b & 0x007F_FFFF) != 0
        && (b & 0x0040_0000) == 0
}
#[inline(always)]
fn quiet(b: u32) -> u32 {
    b | 0x0040_0000
}
#[inline(always)]
// Scalar-SSE NaN rule, measured on the stock worker (two trials of
// 0x00acce10 pin it from opposite sides): a signalling NaN wins over
// everything (destination sNaN, then source sNaN), else a quiet NaN wins
// destination-first, else the hardware op. Trial 22: addss of canonical
// destination NaN and payload source sNaN forwards the source quieted.
// Trial 0: an upstream op of two quiet NaNs forwards the destination.
fn add_ss(d: f32, s: f32) -> f32 {
    let (db, sb) = (d.to_bits(), s.to_bits());
    if is_snan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_snan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    if is_nan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    d + s
}
#[inline(always)]
fn sub_ss(d: f32, s: f32) -> f32 {
    let (db, sb) = (d.to_bits(), s.to_bits());
    if is_snan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_snan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    if is_nan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    d - s
}
#[inline(always)]
fn mul_ss(d: f32, s: f32) -> f32 {
    let (db, sb) = (d.to_bits(), s.to_bits());
    if is_snan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_snan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    if is_nan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    d * s
}
#[inline(always)]
fn div_ss(d: f32, s: f32) -> f32 {
    let (db, sb) = (d.to_bits(), s.to_bits());
    if is_snan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_snan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    if is_nan_bits(db) {
        return f32::from_bits(quiet(db));
    }
    if is_nan_bits(sb) {
        return f32::from_bits(quiet(sb));
    }
    d / s
}
// ==== fn 0x00acce10 below (out-file extraction starts at FN1DOC) ====
#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn rd_f(a: u32) -> f32 {
    unsafe { f32::from_bits(rd32(a)) }
}
#[inline(always)]
unsafe fn wr_f(a: u32, v: f32) {
    unsafe { wr32(a, v.to_bits()) }
}
// x86 cvttss2si: truncate toward zero; NaN, infinity and out-of-range all
// yield 0x80000000 (Rust `as i32` saturates instead, so open-code it).
#[inline(always)]
fn cvtt_ss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}
unsafe fn render1(
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: i32,
    a6: f32,
    a7: u32,
    a8: f32,
    store44: bool,
) -> u32 {
    unsafe {
        // NOTE: the a0/a1/a2 matrix block is dead (see doc); only the
        // argument spills below are live, as plain variables.
        let _ = (a1, a2);
        // Double call.
        let divv = *global::<f32>(0x0103F360);
        let q = div_ss(a8, divv);
        let qb = (q as f64).to_bits();
        let measured: f64 =
            callee_cdecl!(1, f64, (qb & 0xFFFF_FFFF) as u32, (qb >> 32) as u32);
        let mut f = measured as f32;
        if 1.0 > f {
            f = 1.0;
        }
        let count = cvtt_ss2si(f);
        let cntf = count as f32;
        let x6 = div_ss(a8, cntf);
        // Block copy a0 -> a3.
        wr32(a3, rd32(a0.wrapping_add(0xD0)));
        wr32(a3.wrapping_add(4), rd32(a0.wrapping_add(0xD4)));
        wr32(a3.wrapping_add(8), rd32(a0.wrapping_add(0xD8)));
        wr32(a3.wrapping_add(0x10), rd32(a0.wrapping_add(0xE0)));
        wr32(a3.wrapping_add(0x14), rd32(a0.wrapping_add(0xE4)));
        wr32(a3.wrapping_add(0x18), rd32(a0.wrapping_add(0xE8)));
        wr32(a3.wrapping_add(0x20), rd32(a0.wrapping_add(0xF0)));
        wr32(a3.wrapping_add(0x24), rd32(a0.wrapping_add(0xF4)));
        wr32(a3.wrapping_add(0x28), rd32(a0.wrapping_add(0xF8)));
        wr32(a3.wrapping_add(0x30), rd32(a0.wrapping_add(0x100)));
        wr32(a3.wrapping_add(0x34), rd32(a0.wrapping_add(0x104)));
        wr32(a3.wrapping_add(0x38), rd32(a0.wrapping_add(0x108)));
        let a3p40 = a3.wrapping_add(0x40);
        wr32(a3p40, rd32(a0.wrapping_add(0x110)));
        wr32(a3p40.wrapping_add(4), rd32(a0.wrapping_add(0x114)));
        wr32(a3p40.wrapping_add(8), rd32(a0.wrapping_add(0x118)));
        wr32(a3p40.wrapping_add(0xC), rd32(a0.wrapping_add(0x11C)));
        wr32(a3.wrapping_add(0x50), rd32(a0.wrapping_add(0x120)));
        wr32(a3.wrapping_add(0x54), rd32(a0.wrapping_add(0x124)));
        wr32(a3.wrapping_add(0x58), rd32(a0.wrapping_add(0x128)));
        wr32(a3.wrapping_add(0x5C), rd32(a0.wrapping_add(0x12C)));
        // Shared fill in-structs.
        let mut s94 = [0u32; 4];
        s94[0] = *global::<u32>(0x0103F390);
        s94[1] = *global::<u32>(0x0103F394);
        s94[2] = *global::<u32>(0x0103F398);
        s94[3] = *global::<u32>(0x0103F39C);
        let r78 = a8;
        let r7c = div_ss(1.0, a8);
        let r80 = x6;
        let r84 = div_ss(1.0, x6);
        // Voice loop.
        if a5 > 0 {
            let mut dp = a3.wrapping_add(0x60);
            let mut vp = a4;
            let mut zp = a3.wrapping_add(0x84);
            let mut n = a5;
            loop {
                wr32(dp, rd32(vp.wrapping_add(0x88)));
                callee_thiscall!(2, u32, vp);
                dp = dp.wrapping_add(4);
                vp = vp.wrapping_add(0x170);
                n -= 1;
                wr32(zp.wrapping_add(4), 0);
                wr32(zp, 0);
                wr32(zp.wrapping_sub(4), 0);
                zp = zp.wrapping_add(0x10);
                if n == 0 {
                    break;
                }
            }
        }
        let k_half = *global::<f32>(0x00FE8830);
        let k_2x = *global::<f32>(0x00FE8A24);
        let k_1d6 = *global::<f32>(0x00FE87C4);
        let k_1 = *global::<f32>(0x00FE88E8);
        if count > 0 {
            let r14 = mul_ss(x6, k_half);
            let mut out_a = [0u32; 56];
            let mut out_b = [0u32; 56];
            let mut out_c = [0u32; 56];
            let mut out_d = [0u32; 56];
            let mut s78 = [0u32; 6];
            s78[0] = r78.to_bits();
            s78[1] = r7c.to_bits();
            s78[2] = r80.to_bits();
            s78[3] = r84.to_bits();
            let mut x2r = x6;
            let mut b28 = x6;
            let mut ctr: i32 = 0;
            loop {
                // Scale loop over voice+0x148.
                let f = |p: u32| unsafe {
                    let t = sub_ss(rd_f(p), x6);
                    wr_f(p, if !(0.0 > t) { t } else { 0.0 });
                };
                let mut done: i32 = 0;
                if a5 >= 4 {
                    let groups = (((a5 - 4) as u32) >> 2) + 1;
                    let mut g: u32 = 0;
                    while g < groups {
                        let mut k: u32 = 0;
                        while k < 4 {
                            f(a4.wrapping_add((g * 4 + k).wrapping_mul(0x170)).wrapping_add(0x148));
                            k += 1;
                        }
                        g += 1;
                    }
                    done = groups.wrapping_mul(4) as i32;
                }
                {
                    let mut i = done;
                    while i < a5 {
                        f(a4.wrapping_add((i as u32).wrapping_mul(0x170)).wrapping_add(0x148));
                        i += 1;
                    }
                }
                // Fill argument block (carried xmm2; xmm3 is 1.0 every time).
                let r88 = x2r;
                let r8c = div_ss(k_1, x2r);
                s78[4] = r88.to_bits();
                s78[5] = r8c.to_bits();
                let b60 = ctr as f32;
                // (max(1.0, counter) feeds only a dead frame store: skipped.)
                callee_cdecl!(3, u32, out_a.as_mut_ptr() as u32, a3, s78.as_mut_ptr() as u32, 0, 0, s94.as_mut_ptr() as u32);
                callee_cdecl!(4, u32, out_b.as_mut_ptr() as u32, a3, s78.as_mut_ptr() as u32, r14.to_bits(), out_a.as_mut_ptr() as u32, s94.as_mut_ptr() as u32);
                callee_cdecl!(5, u32, out_c.as_mut_ptr() as u32, a3, s78.as_mut_ptr() as u32, r14.to_bits(), out_b.as_mut_ptr() as u32, s94.as_mut_ptr() as u32);
                callee_cdecl!(6, u32, out_d.as_mut_ptr() as u32, a3, s78.as_mut_ptr() as u32, x6.to_bits(), out_c.as_mut_ptr() as u32, s94.as_mut_ptr() as u32);
                // Reduction over the four out structs.
                let af = |o: &[u32; 56], i: usize| f32::from_bits(o[i]);
                let (mut x0, mut x1, mut x2, mut x3, mut x4);
                x1 = add_ss(af(&out_b, 0), af(&out_c, 0));
                x2 = add_ss(af(&out_b, 1), af(&out_c, 1));
                x3 = add_ss(af(&out_b, 2), af(&out_c, 2));
                x0 = af(&out_d, 0);
                x1 = mul_ss(x1, k_2x);
                x2 = mul_ss(x2, k_2x);
                x3 = mul_ss(x3, k_2x);
                x1 = add_ss(x1, af(&out_a, 0));
                x2 = add_ss(x2, af(&out_a, 1));
                x3 = add_ss(x3, af(&out_a, 2));
                x0 = add_ss(x0, x1);
                x1 = add_ss(af(&out_d, 1), x2);
                x2 = add_ss(af(&out_d, 2), x3);
                x0 = mul_ss(x0, k_1d6);
                x1 = mul_ss(x1, k_1d6);
                let r34 = x0;
                let r38 = x1;
                x1 = add_ss(af(&out_b, 4), af(&out_c, 4));
                x3 = add_ss(af(&out_b, 6), af(&out_c, 6));
                x2 = mul_ss(x2, k_1d6);
                let r3c = x2;
                x0 = af(&out_d, 4);
                x1 = mul_ss(x1, k_2x);
                x2 = add_ss(af(&out_b, 5), af(&out_c, 5));
                x1 = add_ss(x1, af(&out_a, 4));
                x3 = mul_ss(x3, k_2x);
                x4 = af(&out_d, 8);
                x2 = mul_ss(x2, k_2x);
                x3 = add_ss(x3, af(&out_a, 6));
                x0 = add_ss(x0, x1);
                x2 = add_ss(x2, af(&out_a, 5));
                x1 = af(&out_d, 5);
                x0 = mul_ss(x0, k_1d6);
                x1 = add_ss(x1, x2);
                x2 = add_ss(af(&out_d, 6), x3);
                let r_d4 = x0;
                x0 = add_ss(af(&out_b, 8), af(&out_c, 8));
                x3 = af(&out_d, 9);
                x2 = mul_ss(x2, k_1d6);
                x1 = mul_ss(x1, k_1d6);
                let r_dc = x2;
                x2 = add_ss(af(&out_b, 10), af(&out_c, 10));
                x0 = mul_ss(x0, k_2x);
                let r_d8 = x1;
                x1 = add_ss(af(&out_b, 9), af(&out_c, 9));
                x0 = add_ss(x0, af(&out_a, 8));
                x2 = mul_ss(x2, k_2x);
                x4 = add_ss(x4, x0);
                x2 = add_ss(x2, af(&out_a, 10));
                x0 = af(&out_d, 10);
                x1 = mul_ss(x1, k_2x);
                x4 = mul_ss(x4, k_1d6);
                x1 = add_ss(x1, af(&out_a, 9));
                x0 = add_ss(x0, x2);
                x2 = add_ss(af(&out_b, 14), af(&out_c, 14));
                let r04 = x4;
                x4 = af(&out_d, 12);
                x0 = mul_ss(x0, k_1d6);
                x3 = add_ss(x3, x1);
                x1 = add_ss(af(&out_b, 13), af(&out_c, 13));
                let r54 = x0;
                x0 = add_ss(af(&out_b, 12), af(&out_c, 12));
                x2 = mul_ss(x2, k_2x);
                x3 = mul_ss(x3, k_1d6);
                x2 = add_ss(x2, af(&out_a, 14));
                x0 = mul_ss(x0, k_2x);
                x1 = mul_ss(x1, k_2x);
                x0 = add_ss(x0, af(&out_a, 12));
                let r5c = x3;
                x1 = add_ss(x1, af(&out_a, 13));
                x3 = af(&out_d, 13);
                x4 = add_ss(x4, x0);
                x0 = af(&out_d, 14);
                x0 = add_ss(x0, x2);
                x3 = add_ss(x3, x1);
                x4 = mul_ss(x4, k_1d6);
                x0 = mul_ss(x0, k_1d6);
                x3 = mul_ss(x3, k_1d6);
                let r6c = x0;
                let r44 = x4;
                let r74 = x3;
                // Combine call.
                let s34 = [r34.to_bits(), r38.to_bits(), r3c.to_bits()];
                let s_d4 = [r_d4.to_bits(), r_d8.to_bits(), r_dc.to_bits()];
                callee_cdecl!(
                    7, u32, a3, a3, s34.as_ptr() as u32,
                    s_d4.as_ptr() as u32, x6.to_bits()
                );
                // Fold into a3+0x40..0x5C.
                x0 = mul_ss(r54, x6);
                x2 = mul_ss(r04, x6);
                x0 = add_ss(x0, rd_f(a3.wrapping_add(0x48)));
                x1 = mul_ss(r5c, x6);
                x2 = add_ss(x2, rd_f(a3.wrapping_add(0x40)));
                let x7 = b60;
                x1 = add_ss(x1, rd_f(a3.wrapping_add(0x44)));
                let ctr1 = ctr + 1;
                wr_f(a3.wrapping_add(0x40), x2);
                x2 = r44;
                wr_f(a3.wrapping_add(0x48), x0);
                x0 = 0.0; // frame word B+0x40, never written: zero fill.
                wr_f(a3.wrapping_add(0x4C), x0);
                x0 = r6c;
                if store44 {
                    wr_f(a3.wrapping_add(0x44), x1);
                }
                x1 = r74;
                x0 = mul_ss(x0, x6);
                x2 = mul_ss(x2, x6);
                x0 = add_ss(x0, rd_f(a3.wrapping_add(0x58)));
                x1 = mul_ss(x1, x6);
                x2 = add_ss(x2, rd_f(a3.wrapping_add(0x50)));
                x1 = add_ss(x1, rd_f(a3.wrapping_add(0x54)));
                wr_f(a3.wrapping_add(0x50), x2);
                wr_f(a3.wrapping_add(0x58), x0);
                x0 = 0.0; // B+0x40 again.
                wr_f(a3.wrapping_add(0x5C), x0);
                let ctrf = ctr1 as f32;
                let x7 = div_ss(x7, ctrf);
                wr_f(a3.wrapping_add(0x54), x1);
                // Second loop.
                if a5 > 0 {
                    let xm4 = sub_ss(k_1, x7);
                    let mut esi = a3.wrapping_add(0x60);
                    let mut ecx = a3.wrapping_add(0x88);
                    let mut i: i32 = 0;
                    x4 = xm4;
                    while i < a5 {
                        let u = i as u32;
                        x0 = add_ss(f32::from_bits(out_c[(16 + u) as usize]), f32::from_bits(out_b[(16 + u) as usize]));
                        x1 = add_ss(f32::from_bits(out_c[(24 + u * 4) as usize]), f32::from_bits(out_b[(24 + u * 4) as usize]));
                        x2 = add_ss(f32::from_bits(out_c[(25 + u * 4) as usize]), f32::from_bits(out_b[(25 + u * 4) as usize]));
                        x0 = mul_ss(x0, k_2x);
                        x3 = add_ss(f32::from_bits(out_c[(26 + u * 4) as usize]), f32::from_bits(out_b[(26 + u * 4) as usize]));
                        x0 = add_ss(x0, f32::from_bits(out_a[(16 + u) as usize]));
                        x1 = mul_ss(x1, k_2x);
                        x2 = mul_ss(x2, k_2x);
                        x0 = add_ss(x0, f32::from_bits(out_d[(16 + u) as usize]));
                        x3 = mul_ss(x3, k_2x);
                        let mut x5 = xm4;
                        x0 = mul_ss(x0, k_1d6);
                        x0 = mul_ss(x0, x6);
                        esi = esi.wrapping_add(4);
                        ecx = ecx.wrapping_add(0x10);
                        x0 = add_ss(x0, rd_f(esi.wrapping_sub(4)));
                        wr_f(esi.wrapping_sub(4), x0);
                        x0 = add_ss(f32::from_bits(out_a[(24 + u * 4) as usize]), x1);
                        x1 = add_ss(f32::from_bits(out_a[(25 + u * 4) as usize]), x2);
                        x2 = add_ss(f32::from_bits(out_a[(26 + u * 4) as usize]), x3);
                        x3 = add_ss(f32::from_bits(out_d[(24 + u * 4) as usize]), x0);
                        x0 = add_ss(f32::from_bits(out_d[(25 + u * 4) as usize]), x1);
                        x1 = add_ss(f32::from_bits(out_d[(26 + u * 4) as usize]), x2);
                        x3 = mul_ss(x3, k_1d6);
                        x0 = mul_ss(x0, k_1d6);
                        x1 = mul_ss(x1, k_1d6);
                        x2 = rd_f(ecx.wrapping_sub(0x18));
                        x5 = mul_ss(x5, x3);
                        x3 = xm4;
                        x4 = mul_ss(x4, x0);
                        x3 = mul_ss(x3, x1);
                        x1 = rd_f(ecx.wrapping_sub(0x10));
                        x0 = mul_ss(x7, rd_f(ecx.wrapping_sub(0x14)));
                        x2 = mul_ss(x2, x7);
                        x1 = mul_ss(x1, x7);
                        x4 = add_ss(x4, x0);
                        x0 = 0.0; // B+0x40 again.
                        x5 = add_ss(x5, x2);
                        x3 = add_ss(x3, x1);
                        wr_f(ecx.wrapping_sub(0x18), x5);
                        wr_f(ecx.wrapping_sub(0x14), x4);
                        x4 = xm4;
                        wr_f(ecx.wrapping_sub(0x10), x3);
                        wr_f(ecx.wrapping_sub(0x0C), x0);
                        i += 1;
                    }
                }
                // Accumulate B+0x28; carry xmm2.
                b28 = add_ss(b28, x6);
                x2r = b28;
                ctr = ctr1;
                if !(ctr < count) {
                    break;
                }
            }
        }
        // Emit call.
        let ans: u32 = callee_cdecl!(
            8, u32, a0, a3, a4, a5 as u32, a6.to_bits(), a7,
            a8.to_bits(), a3p40
        );
        ans
    }
}

export!(cdecl, rw_00acce10(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: i32, a6: f32, a7: u32, a8: f32) -> u32 {
    unsafe { render1(a0, a1, a2, a3, a4, a5, a6, a7, a8, true) }
});
