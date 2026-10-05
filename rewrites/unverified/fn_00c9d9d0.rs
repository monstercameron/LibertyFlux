// original: 0x00C9D9D0 aim_error_test (proposed)

/// Resolve two table-driven handles, project the aim error onto the second
/// handle's frame, and test two solver results against the given bounds.
///
/// `this` points to the task object whose word at `+0x40` is the controller.
/// The signed word at `ctl+0x2e` indexes the global dispatch table twice;
/// each selected entry's slot `0x38` is invoked with the entry and the
/// constants 7 and 6, and each answer is resolved to an object through the
/// shared lookup. With `d` the difference of the triple at `a1` and the
/// triple at the first object's `+0x30`, three dot products against rows of
/// the second object form `A`, `B`, `C`; when both `|B|` and `|C|` are below
/// 0.001 they are replaced by 0.001 and zero. The solver runs twice (as
/// doubles: `C` with `B`, then `-A` with the root of `C*C+B*B`) and each
/// single-precision result must lie within its bound pair (`[a3]`/`[a2]`
/// and `[a3+8]`/`[a2+8]`): strictly above the first bound fails, strictly
/// below the second passes only when the first comparison already failed.
/// The result is 1 when both tests pass and 0 otherwise, except that a second result strictly above its bound returns `a2` with
/// its low byte cleared.
///
/// NOTE: the two solver calls take their arguments only in vector registers
/// (pairs of doubles in xmm0/xmm1) and the checker cannot observe them (see
/// the lane report), so the whole float dataflow above is unverified.
///
/// Original: 0x00C9D9D0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00C9D9D0(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const CTL_OFF: u32 = 0x40;
        const DISPATCH_TABLE: u32 = 0x1295CD8;
        const VT_SLOT_SUBTYPE: u32 = 0x38;
        const FABS_MASK: u32 = 0x7FFF_FFFF;
        const TINY_BITS: u32 = 0x3A83126F; // 0.001
        const SIGN_BIT: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        fn fabsb(x: f32) -> f32 {
            f32::from_bits(x.to_bits() & FABS_MASK)
        }

        let ctl = rd32(this + CTL_OFF);
        let idx = rd16(ctl + 0x2E) as i16 as i32;
        let entry1 = rd32(
            lf_checker_rt::relocated(DISPATCH_TABLE)
                .wrapping_add((idx as u32).wrapping_mul(4)),
        );
        let sub1: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(entry1) + VT_SLOT_SUBTYPE) as usize);
        let r1 = sub1(entry1, 7);
        let r2: u32 = lf_checker_rt::callee_thiscall!(2, u32, ctl, r1);
        let idx2 = rd16(ctl + 0x2E) as i16 as i32;
        let entry2 = rd32(
            lf_checker_rt::relocated(DISPATCH_TABLE)
                .wrapping_add((idx2 as u32).wrapping_mul(4)),
        );
        let sub3: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(entry2) + VT_SLOT_SUBTYPE) as usize);
        let r3 = sub3(entry2, 6);
        let r4: u32 = lf_checker_rt::callee_thiscall!(4, u32, ctl, r3);

        // Aim-error triple and its three projections.
        let d0 = sub(rdf(a1), rdf(r2 + 0x30));
        let d1 = sub(rdf(a1 + 4), rdf(r2 + 0x34));
        let d2 = sub(rdf(a1 + 8), rdf(r2 + 0x38));
        let t4 = mul(rdf(r4), d0);
        let mut t7 = mul(rdf(r4 + 4), d1);
        let t3 = mul(rdf(r4 + 8), d2);
        t7 = add(t7, t4);
        let a_val = add(t7, t3);
        let t6 = mul(rdf(r4 + 0x10), d0);
        let t5 = mul(rdf(r4 + 0x18), d2);
        let mut b_val = mul(rdf(r4 + 0x14), d1);
        b_val = add(b_val, t6);
        b_val = add(b_val, t5);
        let mut c_val = mul(rdf(r4 + 0x24), d1);
        let t0 = mul(rdf(r4 + 0x20), d0);
        let t7b = mul(rdf(r4 + 0x28), d2);
        c_val = add(c_val, t0);
        c_val = add(c_val, t7b);
        let tiny = f32::from_bits(TINY_BITS);
        if tiny > fabsb(b_val) && tiny > fabsb(c_val) {
            b_val = tiny;
            c_val = 0.0;
        }

        // Solver results. The proof scripts doubles with a zero high word;
        // the stub leaves the low word in eax, which is all the rewrite can
        // read (the vector arguments the original passes cannot be observed
        // by the checker at all).
        let s1lo: u32 = lf_checker_rt::callee_thiscall!(5, u32, r4);
        let s1 = f64::from_bits(s1lo as u64);
        let q = add(mul(c_val, c_val), mul(b_val, b_val));
        let f1 = s1 as f32;
        let neg_a = f32::from_bits(a_val.to_bits() ^ SIGN_BIT);
        core::hint::black_box(q);
        core::hint::black_box(neg_a);
        let s2lo: u32 = lf_checker_rt::callee_cdecl!(6, u32,);
        let s2 = f64::from_bits(s2lo as u64);
        let f2 = s2 as f32;

        // Bound tests. A first result above its bound only clears the
        // flag; a second result above its bound returns a2 masked.
        let flag1 = if f1 > rdf(a3) {
            0u32
        } else if rdf(a2) > f1 {
            0u32
        } else {
            1u32
        };
        if f2 > rdf(a3 + 8) {
            return a2 & 0xFFFF_FF00;
        }
        let e = if rdf(a2 + 8) > f2 { 0u32 } else { 1u32 };
        if flag1 == 0 || e == 0 {
            0
        } else {
            1
        }
    }
});
