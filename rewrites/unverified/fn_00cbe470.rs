// original: 0x00cbe470 ped_move_blend_update (proposed)

/// Advance the on-foot move blender one step and report success.
///
/// `this` is the blender object with its state block at `+0x24`; `a0` is a
/// parameter block, `a1` selects the animation channel (zero scans channels
/// `0x15`, `0x16`, `0x13`, `0x14` for the first live one, otherwise `a1`
/// itself is the channel tag), `a2` is a fallback float, and the low byte
/// of `a3` is a done flag. Returns 1 in the low byte (the original leaves
/// the upper bytes as register leftovers, which the contract does not
/// compare).
///
/// Behaviour: the blender stores `0xFFFFFFFF` at `this+0x40`, scales a
/// callee-provided rate by `1/11` (clamped up to a global floor when the
/// state flag at `+0x219` is set), and takes an early exit when the done
/// flag is set, a state disable bit is set, a global enable byte is clear,
/// the parameter block's mode bit is clear, or the state mode selects the
/// still pose (mode field `>= 2` with a zero blend value). The early exit
/// scales `this+0x28` by a global factor, a start-up global and the raw
/// rate, then issues a default blend call.
///
/// Otherwise an angle from a third callee (exact zero takes a second tail)
/// is range-checked against a global threshold: above it the main path
/// runs, below its negation the mirrored path runs, and between them a
/// middle tail runs. The main and mirrored paths each resolve a target
/// through a polled getter (or a blend call when the getter answers null),
/// normalise a phase against `(1 - target.phase) * (pi/2)` (positive
/// remainder on the main path, negative on the mirrored one), divide by a
/// callee-provided total, set or clear the target's latched flag from the
/// channel tag, store the scaled rate, and finish by scaling `this+0x68`
/// with the start-up global into `this+0x30`. A null blend answer faults
/// on the flag access exactly like the original.
///
/// The original reuses its dead incoming-argument slots as scratch; the
/// rewrite uses locals, and the contract switches the stack comparison off
/// for this reason.
///
/// Original: 0x00cbe470 (thiscall, four stack words). Float arithmetic is
/// in the original's operand order; ordered comparisons reproduce the
/// original's NaN behaviour (`jbe` is taken for NaN, `ja` is not).
lf_checker_rt::export!(thiscall, rw_00cbe470(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0x24;
        const INNER_OFF: u32 = 0x78;
        const TAG_OFF: u32 = 0x40;
        const FLAGS_OFF: u32 = 0x50;
        const DISABLE_BIT: u32 = 0x200;
        const ACTIVE_BIT: u32 = 0x100000;
        const MODE_BYTE: u32 = 0x1e2;
        const STILL_VALUE: u32 = 0xd4c;
        const CLAMP_FLAG: u32 = 0x219;
        const DIFF_HI: u32 = 0xaa4;
        const DIFF_LO: u32 = 0xaa0;
        const T224: u32 = 0x224;
        const T224_BIAS: u32 = 0x2e0;
        const P378: u32 = 0x378;
        const TGT_PHASE: u32 = 0x4c;
        const TGT_FLAGS: u32 = 0x4;
        const TGT_RATE: u32 = 0x54;
        const TGT_MODE: u32 = 0x46;
        const LATCH_BIT: u32 = 0x10000;
        const OUT68: u32 = 0x68;
        const OUT30: u32 = 0x30;
        const IN28: u32 = 0x28;
        const G_ENABLE: u32 = 0x0105_1420;
        const G_THRESH: u32 = 0x0105_1424;
        const G_MUL3: u32 = 0x0105_1428;
        const G_FLOOR: u32 = 0x0105_142c;
        const G_STARTUP: u32 = 0x0117_35bc;
        const RATE_DIV: f32 = 11.0;
        const C03183: f32 = f32::from_bits(0x3ea2_f983);
        const HALF: f32 = 0.5;
        const ONE: f32 = 1.0;
        const ZERO: f32 = 0.0;
        const SIXTEEN: f32 = 16.0;
        const PI2: f32 = f32::from_bits(0x3fc9_0fdb);
        const NPI2: f32 = f32::from_bits(0xbfc9_0fdb);
        const QTR_BITS: u32 = 0x3e80_0000;
        const FOUR: f32 = 4.0;
        const FIND_ID: u32 = 0x259;
        const C_GET: u32 = 1;
        const C_RATE: u32 = 2;
        const C_ANGLE: u32 = 3;
        const C_FIND: u32 = 4;
        const C_BLEND: u32 = 5;
        const C_CLIP: u32 = 6;
        const C_TOTAL: u32 = 7;
        const C_RESOLVE: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn or32(a: u32, v: u32) {
            unsafe { wr32(a, rd32(a) | v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Shared tail of the main and mirrored paths: latch or unlatch the
        /// target flag from the channel tag, store the scaled rate, and
        /// finish the output. Faults on a null target like the original.
        #[inline(always)]
        unsafe fn flagsec(p: u32, s1: u32, l2: f32, this: u32, startup: f32) {
            unsafe {
                let pf = core::hint::black_box(p);
                if s1 != 0 {
                    or32(pf + TGT_FLAGS, LATCH_BIT);
                } else {
                    let w = rd32(pf + TGT_FLAGS);
                    if (w >> 16) & 1 != 0 {
                        wr32(pf + TGT_FLAGS, w & !LATCH_BIT);
                    }
                }
                wrf(pf + TGT_RATE, l2);
                wrf(this + OUT30, mul(rdf(this + OUT68), startup));
            }
        }

        let state = rd32(this + STATE_OFF);
        let inner = rd32(state + INNER_OFF);
        // Channel tag: scan, or the caller's tag when nonzero.
        let s1: u32 = if a1 == 0 {
            let mut v = 0u32;
            for ch in [0x15u32, 0x16, 0x13, 0x14] {
                v = lf_checker_rt::callee_thiscall!(C_GET, u32, inner, ch);
                if v != 0 {
                    break;
                }
            }
            v
        } else {
            a1
        };
        wr32(this + TAG_OFF, 0xffff_ffff);
        let rate: f32 = lf_checker_rt::callee_thiscall!(C_RATE, f32, state);
        let rate11 = div(rate, RATE_DIV);
        let startup = gf(G_STARTUP);
        // Early-exit gates, in the original's test order (each only read
        // when the earlier ones pass, matching the original's faults).
        if (a3 & 0xff) != 0
            || rd32(this + FLAGS_OFF) & DISABLE_BIT != 0
            || rd8(lf_checker_rt::relocated(G_ENABLE)) == 0
            || (rd32(a0 + P378) >> 8) & 1 == 0
            || (rd8(state + MODE_BYTE) & 0x0f) >= 2 && rdf(state + STILL_VALUE) == 0.0
        {
            let t = mul(mul(mul(rdf(this + IN28), gf(G_MUL3)), startup), rate11);
            wrf(this + OUT30, t);
            let w4 = rd32(a0 + 4);
            let c = if w4 != 0xffff_ffff { w4 } else { 0x31 };
            let _: u32 = lf_checker_rt::callee_cdecl!(
                C_BLEND, u32, inner, rd32(a0), 0x0fu32, ONE.to_bits(), c);
            return 1;
        }
        let mut l2 = rate11;
        if rd8(state + CLAMP_FLAG) != 0 {
            let floor = gf(G_FLOOR);
            if !(l2 > floor) {
                l2 = floor;
            }
        }
        let diff = sub(rdf(state + DIFF_HI), rdf(state + DIFF_LO));
        let s3: f32 = lf_checker_rt::callee_cdecl!(C_ANGLE, f32, diff.to_bits());
        // Clamp value kept for the blend calls below.
        let ax = f32::from_bits(s3.to_bits() & 0x7fff_ffff);
        let t = mul(mul(ax, C03183), HALF);
        let c = if t > ONE { ONE } else { t };
        let l4 = mul(c, SIXTEEN);
        let esi5: u32 = lf_checker_rt::callee_thiscall!(C_GET, u32, inner, 0x10u32);
        let edx6: u32 = lf_checker_rt::callee_thiscall!(C_GET, u32, inner, 0x11u32);
        let a2f = f32::from_bits(a2);
        // Second tail: exact-zero angle, or the mode bit cleared on re-read.
        if s3 == 0.0 || (rd32(a0 + P378) >> 8) & 1 == 0 {
            let e8: u32 = lf_checker_rt::callee_thiscall!(C_RESOLVE, u32, state);
            let si = rd32(a0);
            let _: u32 = lf_checker_rt::callee_cdecl!(
                C_BLEND, u32, inner, si, 0x0fu32, a2f.to_bits(), e8);
            let t = mul(mul(mul(rdf(this + IN28), gf(G_MUL3)), startup), l2);
            wrf(this + OUT30, t);
            return 1;
        }
        let thresh = gf(G_THRESH);
        if !(s3 > thresh) {
            // Below-threshold path: middle tail or the mirrored branch.
            let neg = -thresh;
            if !(neg > s3) {
                let t7: u32 = lf_checker_rt::callee_thiscall!(C_GET, u32, inner, 0x10u32);
                let x = if t7 != 0 { FOUR } else { a2f };
                let e8: u32 = lf_checker_rt::callee_thiscall!(C_RESOLVE, u32, state);
                let si = rd32(a0);
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    C_BLEND, u32, inner, si, 0x0fu32, x.to_bits(), e8);
                let t = mul(mul(mul(rdf(this + IN28), gf(G_MUL3)), startup), l2);
                wrf(this + OUT30, t);
                return 1;
            }
            or32(this + FLAGS_OFF, ACTIVE_BIT);
            or32(state + 0x29c, 0x00c0_0000);
            if edx6 != 0 {
                let ed = core::hint::black_box(edx6);
                if (rd8(ed + TGT_MODE) >> 2) & 1 != 0 {
                    let dv: f32 = lf_checker_rt::callee_thiscall!(C_TOTAL, f32, ed);
                    wrf(this + OUT68, div(s3, dv));
                }
                flagsec(ed, s1, l2, this, startup);
                return 1;
            }
            let p: u32 = lf_checker_rt::callee_cdecl!(
                C_BLEND, u32, inner, rd32(a0), 0x11u32, l4.to_bits(), rd32(a0 + 4));
            if p != 0 {
                let pl = core::hint::black_box(p);
                if s1 != 0 {
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(C_CLIP, u32, pl, QTR_BITS);
                }
                let x1 = mul(sub(ONE, rdf(pl + TGT_PHASE)), NPI2);
                if x1 > s3 {
                    let d = sub(s3, x1);
                    if ZERO > d {
                        let dv: f32 =
                            lf_checker_rt::callee_thiscall!(C_TOTAL, f32, pl);
                        wrf(this + OUT68, div(d, dv));
                    }
                } else {
                    wr32(this + OUT68, 0);
                }
            }
            flagsec(p, s1, l2, this, startup);
            return 1;
        }
        // Main path.
        or32(this + FLAGS_OFF, ACTIVE_BIT);
        let find_this = rd32(state + T224).wrapping_add(T224_BIAS);
        let fr: u32 = lf_checker_rt::callee_thiscall!(C_FIND, u32, find_this, FIND_ID, 0u32);
        if fr & 0xff == 0 {
            or32(state + 0x29c, 0x00c0_0000);
        }
        if esi5 != 0 {
            let ed = core::hint::black_box(esi5);
            if (rd8(ed + TGT_MODE) >> 2) & 1 != 0 {
                let dv: f32 = lf_checker_rt::callee_thiscall!(C_TOTAL, f32, ed);
                wrf(this + OUT68, div(s3, dv));
            }
            flagsec(ed, s1, l2, this, startup);
            return 1;
        }
        let p: u32 = lf_checker_rt::callee_cdecl!(
            C_BLEND, u32, inner, rd32(a0), 0x10u32, l4.to_bits(), rd32(a0 + 4));
        if p != 0 {
            let pl = core::hint::black_box(p);
            if s1 != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(C_CLIP, u32, pl, QTR_BITS);
            }
            let x1 = mul(sub(ONE, rdf(pl + TGT_PHASE)), PI2);
            if s3 > x1 {
                let d = sub(s3, x1);
                if d > ZERO {
                    let dv: f32 = lf_checker_rt::callee_thiscall!(C_TOTAL, f32, pl);
                    wrf(this + OUT68, div(d, dv));
                }
            } else {
                wr32(this + OUT68, 0);
            }
        }
        flagsec(p, s1, l2, this, startup);
        1
    }
});
