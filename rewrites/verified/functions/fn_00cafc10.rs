// original: 0x00CAFC10 CTaskSimpleMoveGoToPointOnRoute::vf22 (merged name)

/// Advance a move-to-point-on-route task and store the new progress value.
///
/// `this` is the task, `ped` a related object, `target` a two-float point.
/// The task is idle unless the byte at `this + 0xe0` is non-zero, bit 1 of
/// the word at `[[ped + 0xa80] + 0x50]` is clear, and the float at
/// `this + 0x24` is above `1.0` (NaN counts as above: an unordered compare
/// falls through). Otherwise `this + 0x18` is just refreshed from
/// `this + 0x24` (or left alone when the task is idle) and the function
/// returns.
///
/// On the working path the function shares three lazily initialised cached
/// floats with its siblings (flag word plus three slots in one four-word
/// global block; each slot is filled from a constant the first time its bit
/// is found clear): 1.5, 2.0 and a value derived from one double call.
/// With those it normalises the 2D direction from `[[ped + 0x20] + 0x30/34`
/// to `target` (a zero squared length yields a zero factor, NaN propagates),
/// optionally evaluates the double helper (callee 1: one double in `XMM0`,
/// double answer in `XMM0`) over the constant pi/8 converted back to float,
/// bails out storing the entry value when a threshold compare fails, then
/// normalises a second 2D direction from `target` to
/// `this + 0xd0/d4`, dots the two unit directions, divides by the cached
/// value (an exact-zero cache yields `1.0` instead, skipping the divide),
/// clamps to `[0, 1]` (NaN passes through), blends with the entry value and
/// the cached 2.0/1.5, and stores the result at `this + 0x18` after two
/// min-selects (against the entry value, then against 1.1), each of which
/// keeps the second operand on an unordered compare.
///
/// All float operation orders are the original's. The original's one packed
/// square root is scalar-equivalent here (its upper lanes are provably zero
/// and never observed). The function returns nothing meaningful: the value
/// left in `eax` differs by path (incoming register, a copied word, or a
/// stale pointer), so the contract does not compare it. The only integer
/// comparisons are a zero check, a bit test and flag-bit tests.
///
/// Original: 0x00CAFC10 (thiscall, two stack words, callee pops 8).
lf_checker_rt::export!(thiscall, rw_00CAFC10(this: u32, ped: u32, target: u32) -> u32 {
    unsafe {
        const ACTIVE_BYTE: u32 = 0xe0;
        const ENTRY_VAL: u32 = 0x24;
        const OUT_VAL: u32 = 0x18;
        const ANCHOR_X: u32 = 0xd0;
        const ANCHOR_Y: u32 = 0xd4;
        const PED_LINK: u32 = 0xa80;
        const LINK_FLAG_OFF: u32 = 0x50;
        const PED_POS_OFF: u32 = 0x20;
        const POS_X_OFF: u32 = 0x30;
        const POS_Y_OFF: u32 = 0x34;
        const G_CACHE_A: u32 = 0x0171BF64;
        const G_FLAGS: u32 = 0x0171BF68;
        const G_CACHE_B: u32 = 0x0171BF6C;
        const G_CACHE_C: u32 = 0x0171BF70;
        const CONST_A: f32 = 1.5;
        const CONST_B: f32 = 2.0;
        const CONST_C: f32 = 3.0;
        const CONST_D: f32 = 4.0;
        const CONST_E: f32 = f32::from_bits(0x3F8C_CCCD); // 1.1
        const DBL_ARG_LO: u32 = 0x60000000;
        const DBL_ARG_HI: u32 = 0x3FD921FB;
        const DBL_HELPER: u32 = 1;

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
        unsafe fn g32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wgl(a: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(a)).write_unaligned(v) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn inv_sqrt(d2: f32) -> f32 {
            if d2 == 0.0 {
                0.0f32
            } else {
                div(1.0f32, core::hint::black_box(d2).sqrt())
            }
        }

        if rd8(this + ACTIVE_BYTE) == 0 {
            return 0;
        }
        let link = rd32(ped + PED_LINK);
        if ((rd32(link + LINK_FLAG_OFF) >> 1) & 1) != 0 {
            wr32(this + OUT_VAL, rd32(this + ENTRY_VAL));
            return 0;
        }
        let entry = rdf(this + ENTRY_VAL);
        if entry <= 1.0f32 {
            wr32(this + OUT_VAL, entry.to_bits());
            return 0;
        }

        let mut flags = g32(G_FLAGS);
        let fa: f32 = if (flags & 1) != 0 {
            f32::from_bits(g32(G_CACHE_A))
        } else {
            flags |= 1;
            wgl(G_FLAGS, flags);
            wgl(G_CACHE_A, CONST_A.to_bits());
            CONST_A
        };
        let fb_cached: f32 = if (flags & 2) != 0 {
            f32::from_bits(g32(G_CACHE_B))
        } else {
            flags |= 2;
            wgl(G_FLAGS, flags);
            wgl(G_CACHE_B, CONST_B.to_bits());
            CONST_B
        };
        // Threshold select: !(entry > 2.0), so NaN takes the first arm.
        let (threshold, blend_b): (f32, f32) = if !(entry > CONST_B) {
            (CONST_C, fa)
        } else {
            (CONST_D, fb_cached)
        };

        let pos_base = rd32(ped + PED_POS_OFF);
        let dx = sub(rdf(target), rdf(pos_base + POS_X_OFF));
        let dy = sub(rdf(target + 4), rdf(pos_base + POS_Y_OFF));
        let d2 = add(mul(dy, dy), mul(dx, dx));
        let inv = inv_sqrt(d2);
        let nx = mul(inv, dx);
        let ny = mul(dy, inv);
        let nz = mul(inv, 0.0f32);
        let one_over = div(1.0f32, inv);

        if (flags & 4) == 0 {
            // Double helper: argument and answer travel in XMM0; the
            // rewrite passes the two words and reads the u64 answer.
            let ans: u64 =
                lf_checker_rt::callee_cdecl!(DBL_HELPER, u64, DBL_ARG_LO, DBL_ARG_HI);
            let cf = f64::from_bits(ans) as f32;
            flags |= 4;
            wgl(G_FLAGS, flags);
            wgl(G_CACHE_C, cf.to_bits());
        }
        // !(threshold > one_over): NaN one_over takes the early store.
        if !(threshold > one_over) {
            wr32(this + OUT_VAL, entry.to_bits());
            return 0;
        }

        let ex = sub(rdf(this + ANCHOR_X), rdf(target));
        let ey = sub(rdf(this + ANCHOR_Y), rdf(target + 4));
        let e2 = add(mul(ey, ey), mul(ex, ex));
        let inv2 = inv_sqrt(e2);
        let cached_c = f32::from_bits(g32(G_CACHE_C));
        // Exact-zero cache skips the divide and yields 1.0.
        let mut t: f32 = if cached_c == 0.0 {
            1.0f32
        } else {
            let px = mul(ex, inv2);
            let py = mul(ey, inv2);
            let pz = mul(inv2, 0.0f32);
            let dot = add(add(mul(px, nx), mul(py, ny)), mul(pz, nz));
            let q = div(dot, cached_c);
            if q < 0.0f32 {
                0.0f32
            } else if q > 1.0f32 {
                1.0f32
            } else {
                q
            }
        };

        let d = sub(entry, blend_b);
        t = mul(t, t);
        let e = sub(entry, d);
        t = add(mul(t, d), e);
        // Min-selects; unordered keeps the second operand in both.
        let entry2 = if t > entry { entry } else { t };
        let out = if CONST_E > entry2 { CONST_E } else { entry2 };
        wr32(this + OUT_VAL, out.to_bits());
        0
    }
});
