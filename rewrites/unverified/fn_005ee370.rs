// original: 0x005ee370 text_pair_round_and_dispatch (proposed)

/// Round a float pair from vector registers, scale it, and route one record.
///
/// `this` is the format context (divisors at `+0x18`/`+0x1c`). Four stack
/// words arrive with two vector-register inputs: XMM3 and XMM2 (low floats
/// only). When `a1 == a2` (exact integer compare) the function returns at
/// once with the incoming EAX, which the contract fixes.
///
/// Otherwise callee 1 yields a table index; callee 2 (thiscall) copies 24
/// bytes from that 72-byte table entry to six frame slots; callee 3
/// (thiscall on `a0 + 0x14`, no stack words) runs for effect. Both vector
/// inputs then pass an identical round-and-adjust block (sign extracted with
/// a `-0.0` mask, magnitude compared against 2^23, round-trip through a
/// constructed magnitude, conditional `1.0` correction, all bit-exact
/// including NaN), are divided by the context divisors, and go to callee 4
/// (cdecl) with `(a1, a2)`; callee 5 runs with no arguments. Both answers are
/// ignored.
///
/// A selector byte at `a0 + 0xcd` picks a slot word (`a0 + 0x48` or `+0xd0`)
/// and a dispatch word (`a0 + 0x54` or `+0xd4`, exact integer compares):
/// `0x13` skips to the write-back; `0x15`/`0x17` fetch an x87 float from
/// callee 6, scale it by the context and `0.9`/`0.5`; anything else leaves
/// the accumulator zero. The accumulator plus the XMM3 input, divided by the
/// context, and `1/[ctx+0x18]`, feed one of two sinks picked by thread-local
/// slot 0's dword at `+0x8cc`: callee 7 (EDX plus three frame pointers,
/// caller cleanup) or callee 8 (four words, first a frame pointer, caller
/// cleanup); both take `(xmm2in+a3)` products as well as the accumulator.
///
/// Finally callee 1 is polled again and 18 frame words starting at callee 2's
/// six slots are copied to that table entry (the 12 words past callee 2's
/// output are uninitialized frame, defined zero by the contract). The second
/// index times 9 is returned.
///
/// Original: 0x005ee370 (thiscall, ECX = this, four stack words; callee 7 is
/// EDX plus stack with caller cleanup, callee 8 is ECX plus stack with
/// caller cleanup).
lf_checker_rt::export!(thiscall, rw_005ee370(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const FIXED_EAX: u32 = 0x12345678;
        const TABLE: u32 = 0x0119bf18;
        const ENTRY_WORDS: u32 = 18;
        const G_SIGN: u32 = 0x00fe8d1c;
        const G_MAG: u32 = 0x00fe8cf8;
        const G_ONE: u32 = 0x00fe88e8;
        const G_F09: u32 = 0x00fe88bc;
        const G_F05: u32 = 0x00fe8830;
        const CTX_DIV0: u32 = 0x18;
        const CTX_DIV1: u32 = 0x1c;
        const TLS_FLAG: u32 = 0x8cc;
        const CALLEE_INDEX: u32 = 1;
        const CALLEE_FETCH: u32 = 2;
        const CALLEE_PREP: u32 = 3;
        const CALLEE_APPLY: u32 = 4;
        const CALLEE_POST: u32 = 5;
        const CALLEE_SCALE: u32 = 6;
        const CALLEE_SINK_A: u32 = 7;
        const CALLEE_SINK_B: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// One round-and-adjust block: bit-exact, NaN included.
        #[inline(always)]
        fn block(x: f32, sign_mask: u32, mag_const: f32, one: f32) -> f32 {
            let sign = x.to_bits() & sign_mask;
            let mag = f32::from_bits(x.to_bits() ^ sign);
            let mask: u32 = if mag < mag_const { 0xffffffff } else { 0 };
            let m = f32::from_bits((mag_const.to_bits() & mask) | sign);
            let t = add(x, m);
            let t = sub(t, m);
            let d = sub(t, x);
            let signf = f32::from_bits(sign);
            // cmpnle: not less-or-equal (NaN still yields the mask).
            let nmask: u32 = if d <= signf { 0 } else { 0xffffffff };
            let adj = f32::from_bits(nmask & one.to_bits());
            sub(t, adj)
        }

        if a1 == a2 {
            // The original returns its incoming EAX unwritten; the contract
            // fixes that input, so echo the fixed value.
            return FIXED_EAX;
        }
        let ans1: u32 = lf_checker_rt::callee_cdecl!(CALLEE_INDEX, u32,);
        let src = lf_checker_rt::relocated(TABLE).wrapping_add(ans1.wrapping_mul(72));
        let mut slots = [0u32; 6];
        lf_checker_rt::callee_thiscall!(
            CALLEE_FETCH, u32,
            slots.as_mut_ptr() as u32,
            src
        );
        lf_checker_rt::callee_thiscall!(CALLEE_PREP, u32, a0.wrapping_add(0x14),);

        let sign_mask = g32(G_SIGN);
        let mag_const = f32::from_bits(g32(G_MAG));
        let one = f32::from_bits(g32(G_ONE));
        let x3e = f32::from_bits(lf_checker_rt::xmm_word(3, 0));
        let x2e = f32::from_bits(lf_checker_rt::xmm_word(2, 0));
        let ctx0 = rdf(this.wrapping_add(CTX_DIV0));
        let ctx1 = rdf(this.wrapping_add(CTX_DIV1));
        let s1 = div(block(x3e, sign_mask, mag_const, one), ctx1);
        let s0 = div(block(x2e, sign_mask, mag_const, one), ctx0);
        lf_checker_rt::callee_cdecl!(CALLEE_APPLY, u32, s0.to_bits(), s1.to_bits(), a1, a2);
        lf_checker_rt::callee_cdecl!(CALLEE_POST, u32,);

        let (slot, eaxv) = if (a0.wrapping_add(0xcd) as *const u8).read() == 0 {
            (rd32(a0.wrapping_add(0x48)), rd32(a0.wrapping_add(0x54)))
        } else {
            (rd32(a0.wrapping_add(0xd0)), rd32(a0.wrapping_add(0xd4)))
        };
        let mut x0 = 0.0f32;
        let skip_sinks = eaxv == 0x13;
        if !skip_sinks {
            if eaxv == 0x15 {
                let ans: f32 = lf_checker_rt::callee_cdecl!(CALLEE_SCALE, f32,);
                x0 = mul(mul(ans, ctx1), f32::from_bits(g32(G_F09)));
            } else if eaxv == 0x17 {
                let ans: f32 = lf_checker_rt::callee_cdecl!(CALLEE_SCALE, f32,);
                x0 = mul(mul(ans, ctx1), f32::from_bits(g32(G_F05)));
            }
            x0 = add(x0, x3e);
            let x1 = div(one, ctx0);
            x0 = div(x0, ctx1);
            let a3f = f32::from_bits(a3);
            let tls0 = lf_checker_rt::tls_slot(0);
            if rd32(tls0.wrapping_add(TLS_FLAG)) != 0 {
                let s1c = x0;
                let xa = mul(add(x2e, a3f), x1);
                let xb = mul(x1, x2e);
                let mut sl0 = xb;
                let mut sl1 = xa;
                let mut sl2 = s1c;
                let mut sl3 = slot;
                lf_checker_rt::callee_fastcall!(
                    CALLEE_SINK_A, u32, 0,
                    &mut sl3 as *mut u32 as u32,
                    &mut sl0 as *mut f32 as u32,
                    &mut sl1 as *mut f32 as u32,
                    &mut sl2 as *mut f32 as u32
                );
            } else {
                let xa = mul(add(x2e, a3f), x1);
                let xb = mul(x1, x2e);
                let mut sl3 = slot;
                lf_checker_rt::callee_thiscall!(
                    CALLEE_SINK_B, u32, 0,
                    &mut sl3 as *mut u32 as u32,
                    xb.to_bits(),
                    xa.to_bits(),
                    x0.to_bits()
                );
            }
        }
        let ans2: u32 = lf_checker_rt::callee_cdecl!(CALLEE_INDEX, u32,);
        let dest = lf_checker_rt::relocated(TABLE).wrapping_add(ans2.wrapping_mul(72));
        let mut i = 0u32;
        while i < 6 {
            wr32(dest.wrapping_add(i.wrapping_mul(4)), slots[i as usize]);
            i += 1;
        }
        // Words past callee 2's output are uninitialized frame, defined zero.
        while i < ENTRY_WORDS {
            wr32(dest.wrapping_add(i.wrapping_mul(4)), 0);
            i += 1;
        }
        ans2.wrapping_mul(9)
    }
});
