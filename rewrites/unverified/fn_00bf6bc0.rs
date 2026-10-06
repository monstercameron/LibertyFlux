//! Lane a-Q16 rewrite: full anim_pose_compose_emit (staged proof).
//!
//! Batch of four float-heavy thiscall functions sharing one shape: blend a
//! few floats, emit them through helper calls, then dispatch on a hashed
//! name id. Each rewrite below is an independent export.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

#[inline(always)]
unsafe fn rd8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr8(addr: u32, v: u8) {
    unsafe { (addr as *mut u8).write(v) }
}

#[inline(always)]
unsafe fn wr32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rf32(addr: u32) -> f32 {
    unsafe { (addr as *const f32).read_unaligned() }
}

// original: 0x00bf6bc0 anim_pose_compose_emit
/// Blend four pose lanes, solve and compose a matrix, emit, then dispatch
/// on a mode flag. The sink buffer holds a 4x4 row-major result whose last
/// column is never written (matching the input copies, which also skip the
/// last column); the twelve written words are fully modelled.
///
/// Behaviour: emits `lerp(+0x1c)` with key K74 when the mode byte is 0,
/// else `lerp(+0x1c)` with K7C plus `lerp(+0x28)` with K88; then
/// `lerp(+0x20)`/K90 and `lerp(+0x18)`/K94, where
/// `lerp(a, b) = (b - a) * t + a`. Copies the 4x4 matrix at `[aux+0x20]`
/// (last column skipped) into two frame layouts and runs a pre-pass (id
/// 2), fetches working vectors (ids 3, 4), negates `v` when the 4-dot
/// `(v1*w1 + v0*w0) + v2*w2 + v3*w3` is strictly negative, runs the
/// solver/setter pair (ids 5, 6), blends `p` toward `this+0x30..0x38`,
/// calls the sink (id 7), then dispatches on `(this+0x3c >> 4) & 3`:
/// 1 runs id 8 then id 9 `(dst, blend, 4, 6, -1)` and returns its answer;
/// 2 runs id 8 then id 10 `(dst, blend, 2.5, -1)`; otherwise returns the
/// sink answer with its low byte replaced by the flag value.
export!(thiscall, rw_bf6bc0(
    this_ptr: u32,
    dst: u32,
    aux: u32,
    src: u32,
    t_bits: u32,
    mode: u32,
) -> u32 {
    const EMIT: u32 = 1; // 0x607490 thiscall/2 (key, value)
    const PREPASS: u32 = 2; // 0x466280 thiscall/1 (copy2), ecx = copy1
    const FETCH: u32 = 3; // 0xBF7320 thiscall/2 (w_out, p_out), ecx = src
    const SOLVE_V: u32 = 4; // 0xDB0450 cdecl/2 (v_out, this+0x2c)
    const SOLVER: u32 = 5; // 0x6712E0 thiscall/3 (t, v, w), ecx = q_out
    const SETTER: u32 = 6; // 0x6708D0 thiscall/1 (q), ecx = e_out
    const SINK: u32 = 7; // 0x607410 thiscall/1 (comp_out), ecx = dst
    const MODESET: u32 = 8; // 0x607BF0 thiscall/0, ecx = dst
    const APPLY1: u32 = 9; // 0xC19220 cdecl/5 (dst, blend, 4, 6, -1)
    const APPLY2: u32 = 10; // 0xC19920 cdecl/4 (dst, blend, 2.5, -1)
    const K74: u32 = 0x00EBBB74;
    const K7C: u32 = 0x00EBBB7C;
    const K88: u32 = 0x00EBBB88;
    const K90: u32 = 0x00EBBB90;
    const K94: u32 = 0x00EBBB94;
    const G_SIGN: u32 = 0x00FE8FA0;
    const G_ONE: u32 = 0x00FE88E8;
    #[inline(always)]
    unsafe fn lane(base: u32, src: u32, t: f32) -> u32 {
        unsafe {
            let d = rf32(src) - rf32(base);
            let m = d * t;
            (m + rf32(base)).to_bits()
        }
    }
    unsafe {
        let t = f32::from_bits(t_bits);
        // Part 1: conditional emits. The original stashes `this` in a frame
        // slot and reloads it; model the slot explicitly.
        let s10 = this_ptr;
        let l1c = lane(s10 + 0x1c, src + 0x1c, t);
        if (mode as u8) == 0 {
            callee_thiscall!(EMIT, u32, dst, relocated(K74), l1c);
        } else {
            callee_thiscall!(EMIT, u32, dst, relocated(K7C), l1c);
            callee_thiscall!(
                EMIT,
                u32,
                dst,
                relocated(K88),
                lane(s10 + 0x28, src + 0x28, t)
            );
        }
        callee_thiscall!(
            EMIT,
            u32,
            dst,
            relocated(K90),
            lane(s10 + 0x20, src + 0x20, t)
        );
        callee_thiscall!(
            EMIT,
            u32,
            dst,
            relocated(K94),
            lane(s10 + 0x18, src + 0x18, t)
        );
        // Part 2: matrix copies (4x4 at [aux+0x20], last column skipped,
        // holes keep the 4x4 layout).
        let m = rd32(aux + 0x20);
        let mut c1 = [0u32; 16];
        let mut c2 = [0u32; 16];
        let m0 = rd32(m);
        let m1 = rd32(m + 4);
        let m2 = rd32(m + 8);
        let m4 = rd32(m + 0x10);
        let m5 = rd32(m + 0x14);
        let m6 = rd32(m + 0x18);
        let m8 = rd32(m + 0x20);
        let m9 = rd32(m + 0x24);
        let m10 = rd32(m + 0x28);
        let m12 = rd32(m + 0x30);
        let m13 = rd32(m + 0x34);
        let m14 = rd32(m + 0x38);
        c1[0] = m0;
        c1[1] = m1;
        c1[2] = m2;
        c1[4] = m4;
        c1[5] = m5;
        c1[6] = m6;
        c1[8] = m8;
        c1[9] = m9;
        c1[10] = m10;
        c1[12] = m12;
        c1[13] = m13;
        c1[14] = m14;
        c2[0] = m0;
        c2[1] = m1;
        c2[2] = m2;
        c2[4] = m4;
        c2[5] = m5;
        c2[6] = m6;
        c2[8] = m8;
        c2[9] = m9;
        c2[10] = m10;
        c2[12] = m12;
        c2[13] = m13;
        c2[14] = m14;
        callee_thiscall!(
            PREPASS,
            u32,
            c1.as_mut_ptr() as u32,
            c2.as_mut_ptr() as u32
        );
        // Part 3: working vectors, dot/negate, solver pair.
        let mut w = [0u32; 4];
        let mut p = [0u32; 4];
        callee_thiscall!(
            FETCH,
            u32,
            src,
            w.as_mut_ptr() as u32,
            p.as_mut_ptr() as u32
        );
        let mut v = [0u32; 4];
        let this2 = s10; // original reloads `this` from its frame slot here
        callee_cdecl!(
            SOLVE_V,
            u32,
            v.as_mut_ptr() as u32,
            rd32(this2 + 0x2c)
        );
        // Note the order: the v1*w1 term is accumulated first here.
        let dot = (f32::from_bits(v[1]) * f32::from_bits(w[1])
            + f32::from_bits(v[0]) * f32::from_bits(w[0]))
            + f32::from_bits(v[2]) * f32::from_bits(w[2])
            + f32::from_bits(v[3]) * f32::from_bits(w[3]);
        if dot < 0.0 {
            let mask = global::<u32>(G_SIGN).read();
            v[0] ^= mask;
            v[1] ^= mask;
            v[2] ^= mask;
            v[3] ^= mask;
        }
        let mut q = [0u32; 4];
        callee_thiscall!(
            SOLVER,
            u32,
            q.as_mut_ptr() as u32,
            t_bits,
            v.as_mut_ptr() as u32,
            w.as_mut_ptr() as u32
        );
        let mut e = [0u32; 11];
        callee_thiscall!(
            SETTER,
            u32,
            e.as_mut_ptr() as u32,
            q.as_mut_ptr() as u32
        );
        let mut fbuf = [0u32; 16];
        // ---- mechanically transcribed composition (entry blends + sink-buffer math) ----
        // Each line below is one scalar SSE instruction from the original, in
        // the same order with the same operand order. Vector registers are
        // modelled by their low lane only: every lane is first set by a
        // memory load (which clears the upper lanes) and no lane-crossing
        // operation appears, so upper lanes can never reach a result.
        let one = global::<f32>(G_ONE).read();
        let mut f10 = 0u32; let mut f14 = 0u32;
        let mut f18 = 0u32; let mut f20 = 0u32;
        let mut t8c = 0u32; let mut t90 = 0u32; let mut t94 = 0u32;
        let mut t98 = 0u32; let mut t9c = 0u32;
        let mut s_e0 = 0u32; let mut s_e4 = 0u32; let mut s_e8 = 0u32;
        let mut dead_ec = 0u32;
        let (mut x0, mut x1, mut x2, mut x3);
        let (mut x4, mut x5, mut x6, mut x7);
        x3 = t;
        x5 = one;
        x0 = f32::from_bits(p[0]);
        x1 = f32::from_bits(p[1]);
        x2 = f32::from_bits(p[2]);
        x5 = x5 - x3;
        x0 = x0 * x3;
        x1 = x1 * x3;
        x4 = x5;
        x4 = x4 * rf32(this2 + 0x30);
        x2 = x2 * x3;
        x4 = x4 + x0;
        x0 = f32::from_bits(q[3]);
        x3 = x5;
        x3 = x3 * rf32(this2 + 0x34);
        x5 = x5 * rf32(this2 + 0x38);
        dead_ec = x0.to_bits();
        x3 = x3 + x1;
        x1 = f32::from_bits(e[1]);
        x5 = x5 + x2;
        x2 = f32::from_bits(e[2]);
        f20 = x4.to_bits();
        s_e0 = x4.to_bits();
        x4 = x1;
        x4 = x4 * f32::from_bits(c1[5]);
        x1 = x1 * f32::from_bits(c1[6]);
        f10 = x3.to_bits();
        s_e4 = x3.to_bits();
        x3 = f32::from_bits(e[0]);
        x0 = x3;
        x0 = x0 * f32::from_bits(c1[1]);
        x7 = f32::from_bits(c1[0]);
        x6 = f32::from_bits(c1[8]);
        x4 = x4 + x0;
        x0 = x2;
        x0 = x0 * f32::from_bits(c1[9]);
        f18 = x5.to_bits();
        s_e8 = x5.to_bits();
        x5 = f32::from_bits(e[5]);
        x5 = x5 * f32::from_bits(c1[4]);
        x4 = x4 + x0;
        x0 = x3;
        x0 = x0 * f32::from_bits(c1[2]);
        t94 = x4.to_bits();
        x1 = x1 + x0;
        x0 = x2;
        x0 = x0 * f32::from_bits(c1[10]);
        x2 = f32::from_bits(e[4]);
        x1 = x1 + x0;
        x0 = x2;
        x0 = x0 * x7;
        t90 = x1.to_bits();
        x1 = f32::from_bits(e[6]);
        x5 = x5 + x0;
        x0 = x1;
        x0 = x0 * x6;
        x5 = x5 + x0;
        x0 = f32::from_bits(e[5]);
        t98 = x5.to_bits();
        x5 = x0;
        x5 = x5 * f32::from_bits(c1[5]);
        x0 = x2;
        x0 = x0 * f32::from_bits(c1[1]);
        x5 = x5 + x0;
        x0 = x1;
        x0 = x0 * f32::from_bits(c1[9]);
        x5 = x5 + x0;
        t9c = x5.to_bits();
        x2 = x2 * f32::from_bits(c1[2]);
        x0 = f32::from_bits(e[5]);
        x0 = x0 * f32::from_bits(c1[6]);
        x1 = x1 * f32::from_bits(c1[10]);
        x0 = x0 + x2;
        x2 = f32::from_bits(e[8]);
        x5 = f32::from_bits(e[9]);
        x4 = x5;
        x4 = x4 * f32::from_bits(c1[4]);
        x0 = x0 + x1;
        x1 = f32::from_bits(e[10]);
        x3 = x5;
        x3 = x3 * f32::from_bits(c1[5]);
        t8c = x0.to_bits();
        x5 = x5 * f32::from_bits(c1[6]);
        x0 = x2;
        x0 = x0 * x7;
        x4 = x4 + x0;
        x0 = x1;
        x0 = x0 * x6;
        x4 = x4 + x0;
        x0 = x2;
        x0 = x0 * f32::from_bits(c1[1]);
        x2 = x2 * f32::from_bits(c1[2]);
        x3 = x3 + x0;
        x0 = x1;
        x0 = x0 * f32::from_bits(c1[9]);
        x1 = x1 * f32::from_bits(c1[10]);
        x3 = x3 + x0;
        x5 = x5 + x2;
        x2 = f32::from_bits(c1[4]);
        x0 = x7;
        x0 = x0 * f32::from_bits(f20);
        x5 = x5 + x1;
        x1 = x2;
        x2 = f32::from_bits(f10);
        x1 = x1 * x2;
        x1 = x1 + x0;
        x0 = x6;
        x0 = x0 * f32::from_bits(f18);
        x1 = x1 + x0;
        x0 = f32::from_bits(c1[5]);
        x0 = x0 * x2;
        x1 = x1 + f32::from_bits(c1[12]);
        f14 = x0.to_bits();
        x0 = f32::from_bits(c1[1]);
        x0 = x0 * f32::from_bits(f20);
        x7 = f32::from_bits(f14);
        x7 = x7 + x0;
        x0 = f32::from_bits(c1[9]);
        x0 = x0 * f32::from_bits(f18);
        x7 = x7 + x0;
        x0 = x7;
        x0 = x0 + f32::from_bits(c1[13]);
        f14 = x0.to_bits();
        x0 = f32::from_bits(c1[6]);
        x0 = x0 * x2;
        f10 = x0.to_bits();
        x0 = f32::from_bits(c1[2]);
        x0 = x0 * f32::from_bits(f20);
        x2 = f32::from_bits(f10);
        x2 = x2 + x0;
        x0 = f32::from_bits(c1[10]);
        x0 = x0 * f32::from_bits(f18);
        x2 = x2 + x0;
        x0 = x2;
        x0 = x0 + f32::from_bits(c1[14]);
        f10 = x0.to_bits();
        x0 = f32::from_bits(e[1]);
        x0 = x0 * f32::from_bits(c1[4]);
        f18 = x0.to_bits();
        x0 = f32::from_bits(e[0]);
        x0 = x0 * f32::from_bits(c1[0]);
        x2 = f32::from_bits(f18);
        x2 = x2 + x0;
        x0 = f32::from_bits(e[2]);
        x0 = x0 * x6;
        x2 = x2 + x0;
        x0 = f32::from_bits(t94);
        fbuf[1] = x0.to_bits();
        fbuf[0] = x2.to_bits();
        x0 = f32::from_bits(t90);
        fbuf[2] = x0.to_bits();
        x0 = f32::from_bits(t98);
        fbuf[4] = x0.to_bits();
        x0 = f32::from_bits(t9c);
        fbuf[5] = x0.to_bits();
        x0 = f32::from_bits(t8c);
        fbuf[6] = x0.to_bits();
        x0 = f32::from_bits(f14);
        fbuf[13] = x0.to_bits();
        x0 = f32::from_bits(f10);
        fbuf[8] = x4.to_bits();
        fbuf[9] = x3.to_bits();
        fbuf[10] = x5.to_bits();
        fbuf[12] = x1.to_bits();
        fbuf[14] = x0.to_bits();
        let blend = [s_e0, s_e4, s_e8];
        let fans = callee_thiscall!(SINK, u32, dst, fbuf.as_mut_ptr() as u32);
        let flag = (rd8(this2 + 0x3c) >> 4) & 3;
        if flag == 1 {
            callee_thiscall!(MODESET, u32, dst);
            callee_cdecl!(
                APPLY1,
                u32,
                dst,
                blend.as_ptr() as u32,
                0x40800000,
                0x40c00000,
                0xbf800000
            )
        } else if flag == 2 {
            callee_thiscall!(MODESET, u32, dst);
            callee_cdecl!(
                APPLY2,
                u32,
                dst,
                blend.as_ptr() as u32,
                0x40200000,
                0xbf800000
            )
        } else {
            (fans & 0xffffff00) | (flag as u32)
        }
    }
});

