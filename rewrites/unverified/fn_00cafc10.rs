// original: 0x00cafc10 CTaskSimpleMoveGoToPointOnRoute::vf22 (symbols)

/// Advance a move-to-point-on-route task's progress estimate by one step.
///
/// `this` is the task, `a` the owner object, `b` the current target point
/// (two floats). Returns nothing (thiscall: `this` in ECX, the rest on the
/// stack, callee pops 8); the outcome is the task word at `+0x18`.
///
/// Layout read: the enable byte at `this+0xE0`, the current estimate at
/// `this+0x24`, two anchor floats at `this+0xD0`/`+0xD4`; `a+0xA80` is a
/// state object whose dword at `+0x50` (bit 1 after a shift) vetoes the step;
/// `a+0x20` is a matrix row read at `+0x30`/`+0x34`. Four globals form a
/// lazily initialized cache: a flags word with three one-time bits and three
/// cached floats. Callee 1 (direct, no stack arguments, float input in XMM0,
/// `f64` answer in XMM0) refines the third cached float on its first use.
///
/// Algorithm: when the enable byte is clear the function returns without
/// writing anything. When the veto bit is set, or the constant 1.0 is
/// ordered-greater-or-equal to the estimate (a NaN estimate proceeds), the
/// estimate is copied to `+0x18` and the function
/// returns. Otherwise the cache bits are filled from constants (1.5, 2.0)
/// and, once, from callee 1 (whose `f64` answer is narrowed to `f32`); a
/// bound of 4.0 or 3.0 is picked by comparing the estimate against 2.0. Two
/// planar vectors (target minus matrix row, anchors minus target) are each
/// scaled by 1.0 over their length (a zero length yields factor 0, computed
/// through a `ucomiss`/`lahf`/parity idiom that treats NaN as nonzero), the
/// scaled projections are combined and divided by the refined cache float
/// (exactly zero skips the division and uses 1.0), the quotient is clamped
/// into [0, 1.0], and the estimate is re-blended from it and clamped twice
/// (at most the old estimate, at least the constant 1.1) before the final
/// store to `+0x18`.
///
/// Edge cases: every comparison uses ordered `comiss` semantics (unordered
/// is false, so NaN inputs flow to the else sides, except the parity idiom
/// which takes the square-root path for NaN); all arithmetic runs in the
/// original's operand order, including the two length sums which add their
/// squares in opposite orders. The callee's `f64` input (a fixed constant)
/// is loaded but cannot be observed at the call: Rust cannot place it in
/// XMM0, so the call carries no vector register on the rewrite side. The
/// `f64` answer reaches the rewrite through the stub's registers: the low
/// half in EAX plus the high half read from the script entry the stub leaves
/// in EDX (stock-worker behavior, relied on explicitly; see the lane
/// report). Only the final store to each address is emitted: the checker
/// compares final memory, so the original's repeated stores to `+0x18` and
/// the flags word collapse to one value each.
lf_checker_rt::export!(thiscall, rw_00cafc10(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const ENABLE_OFF: u32 = 0xE0;
        const EST_OFF: u32 = 0x24;
        const OUT_OFF: u32 = 0x18;
        const ANCH_X: u32 = 0xD0;
        const ANCH_Y: u32 = 0xD4;
        const STATE_OFF: u32 = 0xA80;
        const STATE_WORD: u32 = 0x50;
        const MAT_OFF: u32 = 0x20;
        const ROW_OFF: u32 = 0x30;
        const C_ONE: u32 = 0x00FE88E8;
        const C_1P5: u32 = 0x00FE8960;
        const C_TWO: u32 = 0x00FE8A24;
        const C_FOUR: u32 = 0x00FE8AB8;
        const C_THREE: u32 = 0x00FE8A94;
        const C_1P1: u32 = 0x00FE8914;
        const C_F64: u32 = 0x00E9B9E0;
        const G_F0: u32 = 0x0171BF64;
        const G_FLAGS: u32 = 0x0171BF68;
        const G_F1: u32 = 0x0171BF6C;
        const G_F2: u32 = 0x0171BF70;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        if rd8(this + ENABLE_OFF) == 0 {
            return 0;
        }
        let state = rd32(a + STATE_OFF);
        if ((rd32(state + STATE_WORD) >> 1) & 1) != 0 {
            wr32(this + OUT_OFF, rd32(this + EST_OFF));
            return 0;
        }
        let est = rdf(this + EST_OFF);
        let one = rdf(lf_checker_rt::relocated(C_ONE));
        if one >= est {
            wr32(this + OUT_OFF, rd32(this + EST_OFF));
            return 0;
        }
        let g = lf_checker_rt::relocated(G_F0);
        let mut flags = rd32(g + 4);
        let f0 = if (flags & 1) == 0 {
            let v = rdf(lf_checker_rt::relocated(C_1P5));
            flags |= 1;
            wr32(g + 4, flags);
            wrf(g, v);
            v
        } else {
            rdf(g)
        };
        let c_two = rdf(lf_checker_rt::relocated(C_TWO));
        let mut slot = if (flags & 2) == 0 {
            flags |= 2;
            wr32(g + 4, flags);
            wrf(g + 8, c_two);
            c_two
        } else {
            rdf(g + 8)
        };
        let bound = if est > c_two {
            rdf(lf_checker_rt::relocated(C_FOUR))
        } else {
            slot = f0;
            rdf(lf_checker_rt::relocated(C_THREE))
        };
        let mat = rd32(a + MAT_OFF);
        let dx = fsub(rdf(b), rdf(mat + ROW_OFF));
        let dy = fsub(rdf(b + 4), rdf(mat + ROW_OFF + 4));
        let mut n1 = fadd(fmul(dy, dy), fmul(dx, dx));
        n1 = if n1 != 0.0 {
            fdiv(one, core::hint::black_box(n1).sqrt())
        } else {
            0.0
        };
        let w20 = fmul(n1, dx);
        let w34 = fmul(dy, n1);
        let w38 = fmul(n1, 0.0);
        let inv = fdiv(one, n1);
        if (flags & 4) == 0 {
            let input = rd64(lf_checker_rt::relocated(C_F64));
            core::hint::black_box(input);
            flags |= 4;
            wr32(g + 4, flags);
            // The stub answers the f64 in XMM0 (for the original) and leaves
            // the low half in EAX and the script entry address in EDX; the
            // high half is read from that entry. Stock-worker behavior.
            let callee: extern "thiscall" fn(u32) -> u64 = unsafe {
                core::mem::transmute(lf_checker_rt::callee_addr(1) as usize)
            };
            let ans = callee(flags);
            let hi = rd32((ans >> 32) as u32 + 4);
            let fref = f64::from_bits(((hi as u64) << 32) | (ans as u32 as u64)) as f32;
            wrf(g + 12, fref);
        }
        if !(bound > inv) {
            wrf(this + OUT_OFF, est);
            return 0;
        }
        let ex = fsub(rdf(this + ANCH_X), rdf(b));
        let ey = fsub(rdf(this + ANCH_Y), rdf(b + 4));
        let mut n2 = fadd(fmul(ey, ey), fmul(ex, ex));
        n2 = if n2 != 0.0 {
            fdiv(one, core::hint::black_box(n2).sqrt())
        } else {
            0.0
        };
        let g2 = rdf(g + 12);
        let mut q = fmul(ex, n2);
        let mut r = fmul(ey, n2);
        let mut s = fmul(n2, 0.0);
        if g2 == 0.0 {
            q = one;
        } else {
            q = fmul(q, w20);
            r = fmul(r, w34);
            s = fmul(s, w38);
            q = fdiv(fadd(fadd(q, r), s), g2);
            if 0.0f32 > q {
                q = 0.0;
            } else if q > one {
                q = one;
            }
        }
        let t = fsub(est, slot);
        q = fmul(q, q);
        let u = fsub(est, t);
        q = fadd(fmul(q, t), u);
        let est2 = if q > est { est } else { q };
        let c11 = rdf(lf_checker_rt::relocated(C_1P1));
        wrf(this + OUT_OFF, if c11 > est2 { c11 } else { est2 });
        0
    }
});
