// original: 0x005ee1a0 text_box_scale_and_dispatch (proposed)

/// Scale a text box by the context and dispatch it to one of two emitters.
///
/// `this` is the format context (float bias at `+0x14`, divisors at `+0x18`
/// and `+0x1c`); `arg0 + 0x14` is the box record. Four kind words of the box
/// (`+0x68`, `+0x74`, `+0x80`, `+0x5c`) are compared exactly against 0x14 and
/// the matches counted (the running count is kept in the incoming argument
/// slot as scratch). A zero count returns 1 at once; the count is signed but
/// never negative.
///
/// Otherwise five scaled values are formed from the box floats with the
/// context (`x6a = ([+0x18]-[+0x6c])/[ctx+0x18]`,
/// `x5 = ([+0x10]+[+0x1c]+[+0x60]-1-[ctx+0x14])/[ctx+0x1c]`,
/// `x7 = ([+0x1c]-[+0x84]-[ctx+0x14])/[ctx+0x1c]`,
/// `x6b = ([+0xc]+[+0x18]+[+0x78]-1)/[ctx+0x18]`, each in the original's
/// operation order) and thread-local slot 0's dword at `+0x8cc` picks the
/// emitter: non-zero calls callee 1 with the context in ECX, the box in EDX
/// and five frame pointers (x7, x5, x6a, x6b slots and the count slot) with
/// caller cleanup, while zero calls callee 2 (cdecl) with
/// `(box, x7, x5, x6a, x6b, count)` by value. The emitter's answer is returned.
///
/// Original: 0x005ee1a0 (thiscall, ECX = this, one stack word).
lf_checker_rt::export!(thiscall, rw_005ee1a0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const KIND_WANT: u32 = 0x14;
        const COUNT_OFFS: [u32; 4] = [0x68, 0x74, 0x80, 0x5c];
        const CTX_BIAS: u32 = 0x14;
        const CTX_DIV0: u32 = 0x18;
        const CTX_DIV1: u32 = 0x1c;
        const TLS_FLAG: u32 = 0x8cc;
        const G_ONE: u32 = 0x00fe88e8;
        const CALLEE_DIRECT: u32 = 1;
        const CALLEE_EMIT: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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

        let bx = arg0.wrapping_add(0x14);
        let mut count = 0u32;
        for off in COUNT_OFFS {
            if rd32(bx.wrapping_add(off)) == KIND_WANT {
                count = count.wrapping_add(1);
            }
        }
        if (count as i32) <= 0 {
            return 1;
        }
        let one: f32 =
            f32::from_bits((lf_checker_rt::relocated(G_ONE) as *const u32).read_unaligned());
        let x1 = div(one, rdf(this.wrapping_add(CTX_DIV0)));
        let x3 = div(one, rdf(this.wrapping_add(CTX_DIV1)));
        let x6a = mul(sub(rdf(bx.wrapping_add(0x18)), rdf(bx.wrapping_add(0x6c))), x1);
        let x5t = add(rdf(bx.wrapping_add(0x10)), rdf(bx.wrapping_add(0x1c)));
        let x5t = add(x5t, rdf(bx.wrapping_add(0x60)));
        let x5t = sub(x5t, one);
        let x5t = sub(x5t, rdf(this.wrapping_add(CTX_BIAS)));
        let x5 = mul(x5t, x3);
        let x7t = sub(rdf(bx.wrapping_add(0x1c)), rdf(bx.wrapping_add(0x84)));
        let x7t = sub(x7t, rdf(this.wrapping_add(CTX_BIAS)));
        let x7 = mul(x7t, x3);
        let x6t = add(rdf(bx.wrapping_add(0x0c)), rdf(bx.wrapping_add(0x18)));
        let x6t = add(x6t, rdf(bx.wrapping_add(0x78)));
        let x6t = sub(x6t, one);
        let x6b = mul(x6t, x1);

        let tls0 = lf_checker_rt::tls_slot(0);
        if rd32(tls0.wrapping_add(TLS_FLAG)) != 0 {
            let mut s_x7 = x7;
            let mut s_x5 = x5;
            let mut s_x6a = x6a;
            let mut s_x6b = x6b;
            let mut s_count = count;
            lf_checker_rt::callee_fastcall!(
                CALLEE_DIRECT, u32, this, bx,
                &mut s_x7 as *mut f32 as u32,
                &mut s_x5 as *mut f32 as u32,
                &mut s_x6a as *mut f32 as u32,
                &mut s_x6b as *mut f32 as u32,
                &mut s_count as *mut u32 as u32
            )
        } else {
            lf_checker_rt::callee_cdecl!(
                CALLEE_EMIT, u32,
                bx, x7.to_bits(), x5.to_bits(), x6a.to_bits(), x6b.to_bits(), count
            )
        }
    }
});
