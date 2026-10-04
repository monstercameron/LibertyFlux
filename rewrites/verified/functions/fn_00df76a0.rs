// original: 0x00DF76A0 UIFileViewer::vf94
/// File-viewer visibility probe: resolves two related view objects, samples
/// four float gauges from each through virtual getters, and forwards to the
/// pane-activation routine when every range check passes.
///
/// `this` is the viewer, `gen` an opaque generation tag forwarded to the
/// lookup helper. The lookup result and the resolved entry each expose four
/// gauges (low edge, low span, high edge, high span); each gauge pair is
/// combined with the global half-scale factor into a bound, the two objects'
/// bounds are reduced pairwise (outer edges take the maximum, inner edges
/// take the minimum), and two globally-configured limits (integer magnitude
/// scaled by a global factor, clamped to the unit ceiling, NaN passing
/// through) are tested against the four reduced bounds with strict ordered
/// comparisons. If all four hold and the shared viewer-state flag word is
/// clear, the pane-activation routine is tail-called with the state object
/// and tag 1; otherwise the function returns nothing meaningful.
///
/// Note: the original reuses its incoming argument word as float scratch
/// (every intermediate store lands on it) and the tail path overwrites it
/// with 1. A faithful Rust rewrite cannot address that word, so the proof
/// contract pins the argument to 1 and the one gauge feeding that store to
/// matching bits; the meaningful value (the tail tag 1) is still verified
/// through the tail call's logged argument. See the lane report.
lf_checker_rt::export!(thiscall, rb97_fn1(this: u32, gen: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_LOOKUP: u32 = 1; // object lookup (thiscall/1: global, gen)
    const CAL_RESOLVE: u32 = 2; // entry resolve (thiscall/0)
    const CAL_TAIL: u32 = 7; // pane activation, tail call (thiscall/1: state, tag)

    // Globals (file VAs; resolved through the worker's image base).
    const GCTX_VA: u32 = 0x01981A4C; // shared UI context for CAL_LOOKUP
    const GSTATE_VA: u32 = 0x018B6C8C; // shared viewer state pointer
    const K_VA: u32 = 0x00FE8830; // gauge scale factor (0.5)
    const HI_VA: u32 = 0x00FE88E8; // limit ceiling (1.0)
    const INT1_VA: u32 = 0x018B7A8C; // first limit magnitude (integer)
    const FLT1_VA: u32 = 0x017ACCF0; // first limit scale (float)
    const INT2_VA: u32 = 0x018B7A80; // second limit magnitude (integer)
    const FLT2_VA: u32 = 0x017ACCE8; // second limit scale (float)

    const STATE_FLAG: u32 = 0x200; // activation gate (nonzero blocks the tail call)
    const TAG: u32 = 1; // generation tag forwarded to the tail call
    const VT_LO_EDGE: u32 = 0xB8; // low-edge gauge (thiscall/0, float result)
    const VT_LO_SPAN: u32 = 0xC8; // low-span gauge (thiscall/0, float result)
    const VT_HI_EDGE: u32 = 0xC0; // high-edge gauge (thiscall/0, float result)
    const VT_HI_SPAN: u32 = 0xD0; // high-span gauge (thiscall/0, float result)

    /// Call a planted vtable slot exactly like the original does and read
    /// its floating-point result. Both sides land on the same stub.
    #[inline(always)]
    unsafe fn vget(object: u32, slot: u32) -> f32 {
        let vtable = *(object as *const u32);
        let target = *((vtable.wrapping_add(slot)) as *const u32);
        let f: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(target as usize);
        f(object)
    }

    #[inline(always)]
    unsafe fn g32(va: u32) -> u32 {
        *(lf_checker_rt::global::<u32>(va))
    }

    unsafe {
        let gctx = lf_checker_rt::relocated(GCTX_VA);
        let first = lf_checker_rt::callee_thiscall!(CAL_LOOKUP, u32, gctx, gen);
        let second = lf_checker_rt::callee_thiscall!(CAL_RESOLVE, u32, this);
        if second == 0 {
            return 0;
        }
        let k = *(lf_checker_rt::global::<f32>(K_VA));
        // Gauges are sampled in the original's exact call order (span, edge,
        // edge, span per pair); each pair's two samples answer identically.
        // Low bound = first-span - first-edge*k, high = second-span + second-edge*k.
        let a = vget(first, VT_LO_SPAN);
        let b = vget(first, VT_LO_EDGE);
        let c = vget(first, VT_LO_EDGE);
        let d = vget(first, VT_LO_SPAN);
        let e = vget(first, VT_HI_SPAN);
        let f = vget(first, VT_HI_EDGE);
        let g = vget(first, VT_HI_EDGE);
        let h = vget(first, VT_HI_SPAN);
        let lo0 = a - b * k;
        let hi0 = d + c * k;
        let lo1 = e - f * k;
        let hi1 = h + g * k;
        // Second object: same gauge pairs in the same order.
        let a2 = vget(second, VT_LO_SPAN);
        let b2 = vget(second, VT_LO_EDGE);
        let c2 = vget(second, VT_LO_EDGE);
        let d2 = vget(second, VT_LO_SPAN);
        let e2 = vget(second, VT_HI_SPAN);
        let f2 = vget(second, VT_HI_EDGE);
        let g2 = vget(second, VT_HI_EDGE);
        let h2 = vget(second, VT_HI_SPAN);
        let lo2 = a2 - b2 * k;
        let hi2 = d2 + c2 * k;
        let lo3 = e2 - f2 * k;
        let hi3 = h2 + g2 * k;
        // Pairwise reduction: outer edges maximise, inner edges minimise.
        // Each `>` is the exact ordered-greater of the original's
        // comiss/jbe pair (unordered/NaN keeps the old value).
        let mut edge_lo = lo0;
        if lo2 > edge_lo {
            edge_lo = lo2;
        }
        let mut edge_hi = hi0;
        if edge_hi > hi2 {
            edge_hi = hi2;
        }
        let mut span_lo = lo1;
        if lo3 > span_lo {
            span_lo = lo3;
        }
        let mut span_hi = hi1;
        if span_hi > hi3 {
            span_hi = hi3;
        }
        // Globally-configured limits, clamped to [0, ceiling]; NaN passes
        // through both clamps exactly as in the original.
        let hi = *(lf_checker_rt::global::<f32>(HI_VA));
        let mut lim0 = (g32(INT1_VA) as i32) as f32 * *(lf_checker_rt::global::<f32>(FLT1_VA));
        if 0.0 > lim0 {
            lim0 = 0.0;
        } else if lim0 > hi {
            lim0 = hi;
        }
        let raw1 = (g32(INT2_VA) as i32) as f32 * *(lf_checker_rt::global::<f32>(FLT2_VA));
        let lim1 = if 0.0 > raw1 {
            0.0
        } else if raw1 > hi {
            hi
        } else {
            raw1
        };
        // All four range checks must hold (strict, ordered).
        if !(lim1 > edge_lo) {
            return 0;
        }
        if !(edge_hi > lim1) {
            return 0;
        }
        if !(lim0 > span_lo) {
            return 0;
        }
        if !(span_hi > lim0) {
            return 0;
        }
        let state = *(lf_checker_rt::global::<u32>(GSTATE_VA));
        if *((state.wrapping_add(STATE_FLAG)) as *const u32) != 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CAL_TAIL, u32, state, TAG)
    }
});
