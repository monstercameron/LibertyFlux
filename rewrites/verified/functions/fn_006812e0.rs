// original: 0x006812E0 euphoria_solve_quat (proposed)

/// Solve one clamped step of a two-stage animation blend and combine the results.
///
/// `this`+0x0c holds a float limit offset `c`. The entry vector registers carry
/// the step parameter `t` (XMM1, low word) and a clamp bound `lim` (XMM2, low
/// word); their upper words must be zero (the callee call log compares the
/// whole XMM1 and the rewrite side can only transport the low word).
/// `a` and `b` are passed through to the helper; `q_out` receives a
/// quaternion, `v_out` a 3-vector (the helper also writes one word past it).
///
/// Behaviour: both outputs are initialised first (identity quaternion, zero
/// vector). If `lim` is an ordered plus or minus zero the function returns 1
/// with no calls. Otherwise a bound `x2` is formed: for `lim > 0` it is the
/// smaller of `lim` and `t` (NaN-safe: NaN keeps the old value); otherwise the
/// larger of `lim` and `t - c`. The helper (callee id 1, then id 2) is asked
/// for a quaternion and a vector, first into scratch with argument `t - x2`,
/// then into the outputs with argument `t`; either answer of 0 returns 0.
/// On success the scratch and output quaternions are combined into `q_out`
/// with the exact multiply-add order below, the scratch vector is subtracted
/// from `v_out`, and 1 is returned.
///
/// The helper takes four stack words plus a float in XMM1 and pops 16 bytes.
/// A Rust rewrite cannot set XMM1 for a call, so the contract transports it:
/// the stub loads XMM1 from a stack word on the rewrite side only. The word
/// used is slot 1 (`b`'s position): the rewrite passes the XMM1 bits there
/// instead of `b`, and the checker skips every stack word of a transported
/// call on both sides, so no comparison sees slot 1 either way. Slot 1 is
/// otherwise unused by the stub (snapshots and out-param writes go through
/// slots 0, 2 and 3), and the real helper's use of `b` is behind the stub,
/// so passing `b` itself through is unverifiable with this callee shape (see
/// `narrowed`). A fifth stack word instead would unbalance the callee-cleaned
/// call by 4 bytes per call, which faults under LLVM's stack-pointer-relative
/// locals; the four-word call keeps both sides balanced.
///
/// Original: 0x006812E0 (thiscall, ECX + four stack words, callee pops 16,
/// returns AL).
lf_checker_rt::export!(thiscall, rw_006812e0(ecx: u32, a: u32, b: u32, q_out: u32, v_out: u32) -> u32 {
    unsafe {
        const THIS_LIMIT: u32 = 0x0c;
        const IDENT_W: u32 = 0x3f80_0000;
        const SITE_SCRATCH: u32 = 1;
        const SITE_OUTPUT: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }

        // Slot 1 carries the XMM1 bits (see doc comment); b is not passed.
        let _ = b;
        let t = f32::from_bits(lf_checker_rt::xmm_word(1, 0));
        let lim = f32::from_bits(lf_checker_rt::xmm_word(2, 0));

        wr32(q_out, 0);
        wr32(q_out + 4, 0);
        wr32(q_out + 8, 0);
        wr32(q_out + 12, IDENT_W);
        wr32(v_out, 0);
        wr32(v_out + 4, 0);
        wr32(v_out + 8, 0);

        // ucomiss lim,0 / lahf / (an instruction of the original)
        // is an ordered zero (unordered/NaN sets both ZF and PF).
        if lim == 0.0 {
            return 1;
        }
        let c = rdf(ecx + THIS_LIMIT);
        // comiss+jbe becomes `>` with the old value kept: NaN keeps it too.
        let mut x2 = lim;
        if lim > 0.0 {
            if x2 > t {
                x2 = t;
            }
        } else {
            let x1 = sub(t, c);
            if x1 > x2 {
                x2 = x1;
            }
        }
        let x1a = sub(t, x2);

        let mut q_tmp = [0u32; 4];
        let mut v_tmp = [0u32; 4];
        let ok1: u8 = lf_checker_rt::callee_stdcall!(
            SITE_SCRATCH, u8, a, x1a.to_bits(),
            q_tmp.as_mut_ptr() as u32, v_tmp.as_mut_ptr() as u32
        );
        if ok1 == 0 {
            return 0;
        }
        let ok2: u8 = lf_checker_rt::callee_stdcall!(
            SITE_OUTPUT, u8, a, t.to_bits(), q_out, v_out
        );
        if ok2 == 0 {
            return 0;
        }

        let q0 = rdf(q_out);
        let q1 = rdf(q_out + 4);
        let q2 = rdf(q_out + 8);
        let q3 = rdf(q_out + 12);
        let f0 = f32::from_bits(q_tmp[0]);
        let f1 = f32::from_bits(q_tmp[1]);
        let f2 = f32::from_bits(q_tmp[2]);
        let f3 = f32::from_bits(q_tmp[3]);

        let mut acc = mul(q0, f0);
        let p = mul(q3, f3);
        let mut x5 = mul(q0, f3);
        acc = add(acc, p);
        acc = add(acc, mul(q1, f1));
        acc = add(acc, mul(q2, f2));
        let qw = acc;

        x5 = sub(x5, mul(q3, f0));
        let mut x1b = mul(q1, f3);
        x5 = sub(x5, mul(q1, f2));
        x5 = add(x5, mul(q2, f1));
        let x7 = mul(q3, f2);
        x1b = sub(x1b, mul(q3, f1));
        x1b = sub(x1b, mul(q2, f0));
        let x0i = mul(q0, f2);
        let x2b = mul(q0, f1);
        x1b = add(x1b, x0i);
        let mut x0j = mul(q2, f3);
        wrf(q_out, x5);
        x0j = sub(x0j, x7);
        x0j = sub(x0j, x2b);
        x0j = add(x0j, mul(q1, f0));
        wrf(q_out + 4, x1b);
        wrf(q_out + 8, x0j);
        wrf(q_out + 12, qw);

        wrf(v_out, sub(rdf(v_out), f32::from_bits(v_tmp[0])));
        wrf(v_out + 4, sub(rdf(v_out + 4), f32::from_bits(v_tmp[1])));
        wrf(v_out + 8, sub(rdf(v_out + 8), f32::from_bits(v_tmp[2])));
        1
    }
});

/// Deliberately wrong version of [`rw_006812e0`]: the final vector subtract
/// is left out, so `v_out` keeps the helper's raw output. The heap comparison
/// must catch it.
