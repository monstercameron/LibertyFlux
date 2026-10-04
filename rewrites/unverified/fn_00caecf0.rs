// original: 0x00caecf0 ped_route_accept (proposed)

/// Decide whether the pedestrian accepts its current route point (1) or
/// keeps moving (0), from a mode poll, a planar-distance test and a
/// look-ahead projection.
///
/// `this` is the task object, `a0` the route point (three floats), `ped` the
/// pedestrian (position block at `+POS`, aux block at `+AUX`, mode flag at
/// `+AUXFLAG`). Only the low byte of the result is defined.
///
/// Behaviour. Let `settled` be true when the aux block shows no activity
/// (`[AUX+AUXBUSY] & AUXBIT == 0` and `[AUX+AUXSEQ] == -1`):
/// - Poll the mode (id 1) with `(ped, a0, &slot)` where `slot` starts as
///   bits 11-14 of `+MODE`; the callee answers true/false and stores the
///   live mode back through the pointer (only the pointed-to word is
///   compared). Merge the answered mode back into `+MODE` (other bits kept)
///   and, when the poll answered true with bit `INHIBIT` of the old mode
///   clear and `settled`, return 1.
/// - Planar test. With `d = a0 - P` (`+GOAL` vs the position block),
///   `z0 = |P.z - a0.z|` and `s = dy*dy + dx*dx` (in that order), fetch the
///   probe value (id 2, single-precision result). When
///   `(goal_bias + probe)^2 > s` (strict, ordered) with `INHIBIT` clear in
///   the merged mode, the clamped limit (`+LIMIT`, raised to at least
///   `MINLIM` while `[ped+CLAMPAT] & CLAMPBIT` holds) above `z0`, and either
///   bit `DIVERGE` clear or the alignment `v = (a0.y-mid.y)*dy +
///   (a0.x-mid.x)*dx` negative: if `settled`, return 1. The midpoint words
///   are refreshed from the position block (`+MID = P.x`, `+MID2 = P.z+4`)
///   before the limit is read.
/// - Otherwise, when `DIVERGE` is set, return 0. Fetch the steering vector
///   (id 3) with a slot holding the global tick float, project
///   `px = P.x + z0*sx`, `py = P.y + z0*sy`, and form
///   `q = (a0.y-py)*dy + (a0.x-px)*dx` (in that order). When `0 < q`
///   (strict, ordered) or `INHIBIT` is set, return 0; return 1 iff
///   `settled`.
///
/// Original: 0x00caecf0 (thiscall, two stack words). Single-precision SSE in
/// the original's operand order (pinned through `black_box`); the absolute
/// value is a sign-bit clear. The merge mask keeps every mode bit except
/// 11-14.
fn decide_00caecf0<const EARLY_ZERO: bool>(this: u32, a0: u32, ped: u32) -> u32 {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read_unaligned() }
    }
    #[inline(always)]
    fn fsub(x: f32, y: f32) -> f32 {
        core::hint::black_box(x) - core::hint::black_box(y)
    }
    #[inline(always)]
    fn fmul(x: f32, y: f32) -> f32 {
        core::hint::black_box(x) * core::hint::black_box(y)
    }
    #[inline(always)]
    fn fadd(x: f32, y: f32) -> f32 {
        core::hint::black_box(x) + core::hint::black_box(y)
    }
    fn fabs_bits(x: f32) -> f32 {
        f32::from_bits(x.to_bits() & 0x7fff_ffff)
    }
    unsafe {
        const POS: u32 = 0x20;
        const PX: u32 = 0x30;
        const AUX: u32 = 0xa80;
        const AUXSEQ: u32 = 0x48;
        const AUXBUSY: u32 = 0x50;
        const AUXBIT: u8 = 0x20;
        const SEQ_EMPTY: u32 = 0xffff_ffff;
        const MID: u32 = 0x50;
        const MID2: u32 = 0x5c;
        const BIAS: u32 = 0xa0;
        const LIMIT: u32 = 0xa4;
        const MODE: u32 = 0xc4;
        const INHIBIT: u32 = 0x40;
        const DIVERGE: u32 = 2;
        const MODE_SHIFT: u32 = 0xb;
        const MODE_BITS: u32 = 0x7800;
        const CLAMPAT: u32 = 0x29c;
        const CLAMPBIT: u8 = 4;
        const MINLIM_GVA: u32 = 0x00fe_8ab8;
        const TICKF_GVA: u32 = 0x0117_35bc;
        let q = rd32(ped + AUX);
        let settled = rd8(q + AUXBUSY) & AUXBIT == 0 && rd32(q + AUXSEQ) == SEQ_EMPTY;
        let c4 = rd32(this + MODE);
        let mut slot = (c4 >> MODE_SHIFT) & 0x0f;
        let slot_ptr = (&mut slot as *mut u32) as u32;
        let ok = lf_checker_rt::callee_thiscall!(1, u32, this, ped, a0, slot_ptr) as u8;
        if ok != 0 && c4 & INHIBIT == 0 && settled {
            return if EARLY_ZERO { 0 } else { 1 };
        }
        let c4b = (((slot << MODE_SHIFT) ^ c4) & MODE_BITS) ^ c4;
        wr32(this + MODE, c4b);
        let p = rd32(ped + POS);
        let x = fsub(
            ((a0) as *const f32).read_unaligned(),
            ((p + PX) as *const f32).read_unaligned(),
        );
        let y = fsub(
            ((a0 + 4) as *const f32).read_unaligned(),
            ((p + PX + 4) as *const f32).read_unaligned(),
        );
        let z0 = fabs_bits(fsub(
            ((p + PX + 8) as *const f32).read_unaligned(),
            ((a0 + 8) as *const f32).read_unaligned(),
        ));
        let s = fadd(fmul(y, y), fmul(x, x));
        let vecp = p.wrapping_add(PX);
        let h = rd32(ped + AUX);
        let hv = rd32(h);
        let tgt = rd32(hv + 0x34);
        let probe: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(tgt as usize);
        let h2 = probe(h);
        let t = fadd(((this + BIAS) as *const f32).read_unaligned(), h2);
        let u = fmul(t, t);
        let w = fmul(
            fsub(
                ((a0) as *const f32).read_unaligned(),
                ((this + MID) as *const f32).read_unaligned(),
            ),
            x,
        );
        let v = fadd(
            fmul(
                fsub(
                    ((a0 + 4) as *const f32).read_unaligned(),
                    ((this + MID + 4) as *const f32).read_unaligned(),
                ),
                y,
            ),
            w,
        );
        let above = u > s;
        let neg = 0.0f32 > v;
        wr32(this + MID, rd32(vecp));
        wr32(this + MID2, rd32(vecp + 0x0c));
        let mut lim = ((this + LIMIT) as *const f32).read_unaligned();
        if rd8(ped + CLAMPAT) & CLAMPBIT != 0 {
            let minlim = lf_checker_rt::global::<f32>(MINLIM_GVA).read_unaligned();
            if !(lim > minlim) {
                lim = minlim;
            }
        }
        if above && c4b & INHIBIT == 0 && lim > z0 && (c4b & DIVERGE == 0 || neg) && settled {
            return 1;
        }
        if c4b & DIVERGE != 0 {
            return 0;
        }
        let mut buf: u32 = lf_checker_rt::global::<u32>(TICKF_GVA).read_unaligned();
        let buf_ptr = (&mut buf as *mut u32) as u32;
        let pv = rd32(ped);
        let ft = rd32(pv + 0xec);
        let fetch: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(ft as usize);
        let r2 = fetch(ped, buf_ptr);
        let r2x = ((r2) as *const f32).read_unaligned();
        let r2y = ((r2 + 4) as *const f32).read_unaligned();
        let px = fadd(((vecp) as *const f32).read_unaligned(), fmul(z0, r2x));
        let py = fadd(((vecp + 4) as *const f32).read_unaligned(), fmul(r2y, z0));
        let qx = fmul(fsub(((a0) as *const f32).read_unaligned(), px), x);
        let qy = fmul(fsub(((a0 + 4) as *const f32).read_unaligned(), py), y);
        let qq = fadd(qy, qx);
        if 0.0f32 < qq {
            return 0;
        }
        if c4b & INHIBIT != 0 {
            return 0;
        }
        if settled {
            1
        } else {
            0
        }
    }
}

lf_checker_rt::export!(thiscall, rw_00caecf0(this: u32, a0: u32, ped: u32) -> u32 {
    decide_00caecf0::<false>(this, a0, ped)
});
