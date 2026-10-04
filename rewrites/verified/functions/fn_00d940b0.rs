// original: 0x00d940b0 probe_trace_setup (proposed)

/// Store probe parameters into globals and run the traversal when the probe
/// direction is non-zero.
///
/// `a1` and `a2` point to four floats each, copied to the parameter globals;
/// `a3` is a probe id stored alongside; `a4` is unread here but forwarded to
/// the traversal; `a5` points to the direction vector, copied to the probe
/// globals. When the squared length of the direction's first three floats is
/// exactly +0.0 the function returns 0 without calling. Otherwise it clears
/// the result global, calls the traversal callee (thiscall, forwarded
/// incoming object pointer, `a4`, 0), copies the traversal's result id to
/// `*a6` and the four result floats to `a7`, and returns the callee's answer.
/// A NaN length takes the call path (an unordered compare is not equal).
///
/// Only the low byte of the result is behaviour: the early path leaves the
/// entry residue above AL, so the contract compares the `al` channel.
///
/// Original: 0x00d940b0 (thiscall-shaped: incoming ECX forwarded to the
/// callee, seven stack words, callee pops 0x1c, al result).
lf_checker_rt::export!(thiscall, rw_00d940b0(this: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        const VEC_A: u32 = 0x17a33c0;
        const VEC_B: u32 = 0x17a33d0;
        const PROBE_ID: u32 = 0x179fd68;
        const DIR: u32 = 0x17a33e0;
        const RESULT_ID: u32 = 0x179fd6c;
        const RESULT_VEC: u32 = 0x17a33f0;
        const TRAVERSE: u32 = 0;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        for i in 0..4u32 {
            wr32(lf_checker_rt::relocated(VEC_A + i * 4), rd32(a1 + i * 4));
            wr32(lf_checker_rt::relocated(VEC_B + i * 4), rd32(a2 + i * 4));
        }
        wr32(lf_checker_rt::relocated(PROBE_ID), a3);
        let x = f32::from_bits(rd32(a5));
        let y = f32::from_bits(rd32(a5 + 4));
        let z = f32::from_bits(rd32(a5 + 8));
        for i in 0..4u32 {
            wr32(lf_checker_rt::relocated(DIR + i * 4), rd32(a5 + i * 4));
        }
        wr32(lf_checker_rt::relocated(0x179fd70), 0);
        let len2 = fadd(fadd(fmul(x, x), fmul(y, y)), fmul(z, z));
        if len2 != 0.0 {
            wr32(lf_checker_rt::relocated(RESULT_ID), 0);
            let r: u32 = lf_checker_rt::callee_thiscall!(TRAVERSE, u32, this, a4, 0);
            wr32(a6, rd32(lf_checker_rt::relocated(RESULT_ID)));
            for i in 0..4u32 {
                wr32(a7 + i * 4, rd32(lf_checker_rt::relocated(RESULT_VEC + i * 4)));
            }
            r
        } else {
            0
        }
    }
});
