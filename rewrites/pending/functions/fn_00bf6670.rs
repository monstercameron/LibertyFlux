// original: 0x00bf6670 quat_blend_emit
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rf32(addr: u32) -> f32 {
    unsafe { (addr as *const f32).read_unaligned() }
}

// original: 0x00bf6670 quat_blend_emit (proposed)
/// Solve a quaternion blend for this object and emit two blended lanes.
///
/// Behaviour: fetches two working vectors through helper calls (ids 1
/// and 2 fill `w`/`p` and `v`), takes their 4-dot, negates `v` when the
/// dot is strictly negative, runs the pair through a solver (id 3) and a
/// setter (id 4), blends `p` toward this object's `+0x18..0x20` by `t`
/// (the blend feeds the real sink call's struct argument, id 5), then
/// emits `lerp(+0x28)` and `lerp(+0x2c)` of the source vector toward this
/// object with sink keys K0/K1 (id 6) and stores `lerp(+0x30)` directly
/// at `dst+0x1a0`, where `lerp(a, b) = (b - a) * t + a`. Returns the
/// source pointer.
export!(thiscall, rw_bf6670(
    this_ptr: u32,
    dst: u32,
    src: u32,
    t_bits: u32,
) -> u32 {
    const FETCH: u32 = 1; // callee/2 (w_out, p_out), ecx = src
    const SOLVE_V: u32 = 2; // cdecl/2 (v_out, this+0x24)
    const SOLVER: u32 = 3; // thiscall/3 (t, v, w), ecx = q_out
    const SETTER: u32 = 4; // thiscall/1 (q), ecx = tail
    const SINK: u32 = 5; // thiscall/1 (tail), ecx = dst
    const EMIT: u32 = 6; // thiscall/2 (key, value), ecx = dst
    const K0: u32 = 0x00EBBE40;
    const K1: u32 = 0x00EBBE4C;
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
        // Frame model: the exact regions the original threads through calls.
        let mut v = [0u32; 4];
        let mut w = [0u32; 4];
        let mut p = [0u32; 4];
        let mut q = [0u32; 4];
        let mut tail = [0u32; 8];
        let mut blend = [0u32; 4];
        callee_thiscall!(
            FETCH,
            u32,
            src,
            w.as_mut_ptr() as u32,
            p.as_mut_ptr() as u32
        );
        callee_cdecl!(
            SOLVE_V,
            u32,
            v.as_mut_ptr() as u32,
            rd32(this_ptr + 0x24)
        );
        // 4-dot in the original's accumulation order.
        let dot = (f32::from_bits(v[0]) * f32::from_bits(w[0])
            + f32::from_bits(v[1]) * f32::from_bits(w[1]))
            + f32::from_bits(v[2]) * f32::from_bits(w[2])
            + f32::from_bits(v[3]) * f32::from_bits(w[3]);
        if dot < 0.0 {
            let mask = global::<u32>(G_SIGN).read();
            v[0] ^= mask;
            v[1] ^= mask;
            v[2] ^= mask;
            v[3] ^= mask;
        }
        let t = f32::from_bits(t_bits);
        callee_thiscall!(
            SOLVER,
            u32,
            q.as_mut_ptr() as u32,
            t_bits,
            v.as_mut_ptr() as u32,
            w.as_mut_ptr() as u32
        );
        callee_thiscall!(
            SETTER,
            u32,
            tail.as_mut_ptr() as u32,
            q.as_mut_ptr() as u32
        );
        // Blend p toward this+0x18 by t (kept in the frame model like the
        // original; the real sink reads it through its struct argument).
        let one = global::<f32>(G_ONE).read();
        let omt = one - t;
        let b0 = omt * rf32(this_ptr + 0x18) + f32::from_bits(p[0]) * t;
        let b1 = omt * rf32(this_ptr + 0x1c) + f32::from_bits(p[1]) * t;
        let b2 = omt * rf32(this_ptr + 0x20) + f32::from_bits(p[2]) * t;
        blend[0] = b0.to_bits();
        blend[1] = b1.to_bits();
        blend[2] = b2.to_bits();
        blend[3] = q[3];
        core::hint::black_box(&blend);
        callee_thiscall!(SINK, u32, dst, tail.as_mut_ptr() as u32);
        callee_thiscall!(
            EMIT,
            u32,
            dst,
            relocated(K0),
            lane(this_ptr + 0x28, src + 0x28, t)
        );
        callee_thiscall!(
            EMIT,
            u32,
            dst,
            relocated(K1),
            lane(this_ptr + 0x2c, src + 0x2c, t)
        );
        wr32(dst + 0x1a0, lane(this_ptr + 0x30, src + 0x30, t));
        src
    }
});
