// original: 0x00d8c990 audio_listener_emit_grid (proposed)

/// Emit one audio listener's contribution grid: scale the listener position,
/// then per active slot combine it with the slot's triple and send six
/// seven-word commands.
///
/// Takes no arguments (cdecl, no stack words) and returns nothing meaningful.
/// If the enable byte at `FLAG_ADDR` is clear, or the slot count at
/// `COUNT_ADDR` is zero, it returns at once. Otherwise it calls the allocator
/// as `alloc(3, 6 * count)`, scales the three listener coordinates found
/// through `LISTENER_PTR` (`+0x40/0x44/0x48`) by the constant at `SCALE_ADDR`,
/// and loops over the slots: slot `i` reads its triple from
/// `TRIPLE_BASE + i * 0x10` (`[0]`, `[-4]`, `[-8]`), its handle from
/// `HANDLE_BASE + i * 4`, and the shared triple at `COEFF_BASE`, and issues
/// six `emit(f0, f1, f2, 0, 0, -1.0, handle)` commands whose float arguments
/// are sums and differences of those inputs (see the body for the exact
/// derivation). A final bare `finish()` call closes the sequence. All float
/// arithmetic is branch-free add/mul/sub in the original's operand order.
///
/// Two redundancies of the original are omitted: a re-check of the count that
/// can never fail (the stubbed allocator leaves globals alone), and two
/// scratch words that are written but never read.
///
/// Original: 0x00d8c990 (cdecl, no arguments; no meaningful return).
lf_checker_rt::export!(cdecl, rw_00d8c990() -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const EMIT: u32 = 2;
        const FINISH: u32 = 3;
        const FLAG_ADDR: u32 = 0x0179d120;
        const LISTENER_PTR: u32 = 0x017f583c;
        const COUNT_ADDR: u32 = 0x0179d124;
        const HANDLE_BASE: u32 = 0x0179d128;
        const TRIPLE_BASE: u32 = 0x0179d938;
        const TRIPLE_STRIDE: u32 = 0x10;
        const COEFF_BASE: u32 = 0x018d2170;
        const SCALE_ADDR: u32 = 0x00fe87c8;
        const K_ADDR: u32 = 0x00fe888c;

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
        unsafe fn emit(f0: f32, f1: f32, f2: f32, esi: u32) {
            unsafe {
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    EMIT, u32, f0.to_bits(), f1.to_bits(), f2.to_bits(),
                    0, 0, 0xbf800000, esi
                );
            }
        }

        let flag = (lf_checker_rt::relocated(FLAG_ADDR) as *const u8).read();
        if flag == 0 {
            return 0;
        }
        let p = rd32(lf_checker_rt::relocated(LISTENER_PTR));
        let count = rd32(lf_checker_rt::relocated(COUNT_ADDR));
        if count == 0 {
            return 0;
        }
        let s = rdf(lf_checker_rt::relocated(SCALE_ADDR));
        let k = rdf(lf_checker_rt::relocated(K_ADDR));
        let l80 = mul(rdf(p + 0x40), s);
        let l64 = mul(rdf(p + 0x44), s);
        let l88 = mul(rdf(p + 0x48), s);
        let _: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, 3, count.wrapping_mul(6));
        let coeff = lf_checker_rt::relocated(COEFF_BASE);
        let triples = lf_checker_rt::relocated(TRIPLE_BASE);
        let handles = lf_checker_rt::relocated(HANDLE_BASE);
        let mut ebx = 0u32;
        while ebx < count {
            let edi = triples + ebx * TRIPLE_STRIDE;
            let e0 = rdf(edi);
            let e1 = rdf(edi - 4);
            let e2 = rdf(edi - 8);
            let x5 = mul(rdf(coeff), k);
            let x4 = mul(rdf(coeff + 4), k);
            let x3 = mul(rdf(coeff + 8), k);
            let esi = rd32(handles + ebx * 4);
            let d0 = sub(e0, l88);
            let d1 = sub(e1, l64);
            let d2 = sub(e2, l80);
            emit(add(d2, x5), add(d1, x4), add(d0, x3), esi);
            let s0 = add(l88, e0);
            let t1 = add(l64, e1);
            let t2 = add(l80, e2);
            let (l30, l24, l20) = (add(s0, x3), add(t1, x4), add(t2, x5));
            emit(l20, l24, l30, esi);
            let (l3c, l38, l34) = (sub(d0, x3), sub(d1, x4), sub(d2, x5));
            emit(l34, l38, l3c, esi);
            emit(l20, l24, l30, esi);
            emit(sub(t2, x5), sub(t1, x4), sub(s0, x3), esi);
            emit(l34, l38, l3c, esi);
            ebx += 1;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(FINISH, u32,);
        0
    }
});
