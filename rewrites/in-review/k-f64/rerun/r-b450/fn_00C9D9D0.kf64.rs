// original: 0x00C9D9D0 aim_error_test (proposed)
// Adapted by lane k-f64 from r-b450's deferred rewrite for the doubles
// extension (unadopted): both solver calls now cross with the 8-byte
// transport and full double answers. Ready for a stock re-run, not verified.

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
/// NOTE (k-f64): the two solver calls take their arguments only in vector
/// registers (pairs of doubles in xmm0/xmm1). With the doubles extension
/// the low doubles compare bit for bit and the double answers arrive whole;
/// the upper halves of the argument registers are conversion residue the
/// callee never reads, so they are not compared.
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
        // Bit-exact f32->f64 widening, as cvtss2sd does (k-f64).
        #[inline(always)]
        fn cvtss2sd_bits(a: u32) -> u64 {
            let sign = u64::from(a >> 31) << 63;
            let e = (a >> 23) & 0xFF;
            let f = u64::from(a & 0x7F_FFFF);
            if e == 0xFF {
                if f == 0 {
                    return sign | 0x7FF0_0000_0000_0000;
                }
                return sign | 0x7FF8_0000_0000_0000 | (f << 29);
            }
            if e == 0 {
                if f == 0 {
                    return sign;
                }
                let lz = (f as u32).leading_zeros() - 9;
                let exp = 896 - lz;
                let frac = ((f << (lz + 1)) & 0x7F_FFFF) << 29;
                return sign | (u64::from(exp) << 52) | frac;
            }
            sign | (u64::from(e + 896) << 52) | (f << 29)
        }
        #[inline(always)]
        fn lo_hi(d: u64) -> (u32, u32) {
            ((d & 0xFFFF_FFFF) as u32, ((d >> 32) & 0xFFFF_FFFF) as u32)
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

        // Solver results. Each solver call takes its two doubles in
        // xmm0/xmm1 (compared on the low 8 bytes) and answers a double as
        // the callee's u64 return, narrowed with the same single cvtsd2ss
        // the original uses. First call: C with B; second: -A with the
        // root of C*C+B*B (f32 root, widened).
        let (c0, c1) = lo_hi(cvtss2sd_bits(c_val.to_bits()));
        let (b0, b1) = lo_hi(cvtss2sd_bits(b_val.to_bits()));
        let s1: u64 = lf_checker_rt::callee_thiscall!(5, u64, r4, c0, c1, b0, b1);
        let q = add(mul(c_val, c_val), mul(b_val, b_val));
        let f1 = core::hint::black_box(f64::from_bits(s1)) as f32;
        let neg_a = f32::from_bits(a_val.to_bits() ^ SIGN_BIT);
        let root = core::hint::black_box(q).sqrt();
        let (n0, n1) = lo_hi(cvtss2sd_bits(neg_a.to_bits()));
        let (rt0, rt1) = lo_hi(cvtss2sd_bits(root.to_bits()));
        let s2: u64 = lf_checker_rt::callee_cdecl!(6, u64, n0, n1, rt0, rt1);
        let f2 = core::hint::black_box(f64::from_bits(s2)) as f32;

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
