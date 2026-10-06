// original: 0x005eddc0 text_format_measure_and_dispatch (proposed)

/// Measure a text-format box and dispatch it to one of two sinks.
///
/// `this` is the format context (two float divisors at `+0x18`/`+0x1c` and a
/// float bias at `+0x14`, all read as bits). `arg0` points to a style block
/// whose enable byte at `+0xcc` gates everything: when it is zero the function
/// returns `arg0` unchanged and touches nothing else.
///
/// Otherwise `ebx = arg0 + 0x14` is the box record. Four edge floats are taken
/// from it (`+0xc`, `+0x10`, `+0x18`, `+0x1c`) and callee 1 resolves a node
/// object from `(arg0, 0, G1, G2, 0)` where `G1`/`G2` are two relocated data
/// addresses passed by value. When the node's kind word at `+0xd8` equals 3
/// (compared as an exact integer), callee 2 (thiscall on the node's word at
/// `+0x8`) rewrites the four edges through four frame pointers; if not, each
/// of the `+0xc`/`+0x10` edges is instead adjusted unless it is below zero (a
/// `comiss` below-or-unordered test skips the adjustment, so NaN skips it)
/// using the trim floats at `+0x88`..`+0x94` and the context bias.
///
/// A global function pointer is then polled four times with no arguments; each
/// answer is compared for equality against a global sentinel and picks one of
/// two global integers (four picks: `ebp_sel`, `ebx_sel`, `edi_sel`, `ecx_sel`
/// in call order). The edges are projectively scaled by `1/[this+0x18]` and
/// `1/[this+0x1c]` and multiplied by the picks converted with `cvtdq2ps`
/// (exact `i32 as f32`), giving four products.
///
/// A flag byte (1 when the node kind equalled 3) is stored over the low byte
/// of a frame word whose upper bytes keep whatever was there (callee 2's
/// second word, or the `+0x1c` edge bits). Thread-local slot 0 points to a
/// state block whose dword at `+0x8cc` selects the sink: non-zero calls callee
/// 4 with the context selector in ECX (dead to the callee, which reads EDX
/// and four frame pointers) while zero calls callee 5 (cdecl) with two edge
/// bits, a frame pointer, `ebx` and the flag word (only its low byte is
/// meaningful to the callee). The sink's answer is returned.
///
/// Original: 0x005eddc0 (thiscall, ECX = this, one stack word; callee 4 takes
/// EDX plus four stack words with caller cleanup).
lf_checker_rt::export!(thiscall, rw_005eddc0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const ENABLE_OFF: u32 = 0xcc;
        const BOX_OFF: u32 = 0x14;
        const EDGE_A: u32 = 0x0c;
        const EDGE_B: u32 = 0x10;
        const EDGE_C: u32 = 0x18;
        const EDGE_D: u32 = 0x1c;
        const TRIM0: u32 = 0x88;
        const TRIM1: u32 = 0x8c;
        const TRIM2: u32 = 0x90;
        const TRIM3: u32 = 0x94;
        const NODE_KIND: u32 = 0xd8;
        const NODE_THIS: u32 = 0x08;
        const KIND_WANT: u32 = 3;
        const CTX_BIAS: u32 = 0x14;
        const CTX_DIV0: u32 = 0x18;
        const CTX_DIV1: u32 = 0x1c;
        const TLS_FLAG: u32 = 0x8cc;
        const G_CALLEE_PTR: u32 = 0x00e731ac;
        const G_SENTINEL: u32 = 0x0110dd14;
        const G_SELP0: u32 = 0x0105c880;
        const G_SELP1: u32 = 0x0105c87c;
        const G_SELQ0: u32 = 0x0105c884;
        const G_SELQ1: u32 = 0x0105c888;
        const G_ARG1: u32 = 0x0114e780;
        const G_ARG2: u32 = 0x0114e798;
        const G_ONE: u32 = 0x00fe88e8;
        const CALLEE_RESOLVE: u32 = 1;
        const CALLEE_EDGES: u32 = 2;
        // Callee 3 (the polled function pointer) is reached through the
        // global like the original, not through the stub table.
        const CALLEE_SINK_A: u32 = 4;
        const CALLEE_SINK_B: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
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

        if rd8(arg0.wrapping_add(ENABLE_OFF)) == 0 {
            return arg0;
        }
        let bx = arg0.wrapping_add(BOX_OFF);
        // Prefill of the four edge slots (each read twice by the original).
        let pre_a = rdf(bx.wrapping_add(EDGE_A));
        let pre_b = rdf(bx.wrapping_add(EDGE_B));
        let pre_c = rdf(bx.wrapping_add(EDGE_C));
        let pre_d = rdf(bx.wrapping_add(EDGE_D));

        let node: u32 = lf_checker_rt::callee_cdecl!(
            CALLEE_RESOLVE, u32, arg0, 0,
            lf_checker_rt::relocated(G_ARG1), lf_checker_rt::relocated(G_ARG2), 0
        );
        let kind_hit = rd32(node.wrapping_add(NODE_KIND)) == KIND_WANT;
        // Edge slots s14 (from C), s10 (from D), s18 (from A), s1c (from B).
        let (mut s14, mut s10, mut s18, mut s1c) = (pre_c, pre_d, pre_a, pre_b);
        // Low-three-bytes source of the flag word's upper bytes.
        let upper: u32;
        if kind_hit {
            let mut w28 = pre_c;
            let mut w2c = pre_d;
            let mut w24 = pre_a;
            let mut w20 = pre_b;
            let node_this = rd32(node.wrapping_add(NODE_THIS));
            lf_checker_rt::callee_thiscall!(
                CALLEE_EDGES, u32, node_this,
                &mut w28 as *mut f32 as u32,
                &mut w2c as *mut f32 as u32,
                &mut w24 as *mut f32 as u32,
                &mut w20 as *mut f32 as u32
            );
            s14 = w28;
            s10 = w2c;
            s18 = w24;
            s1c = w20;
            upper = w2c.to_bits() & 0xffff_ff00;
        } else {
            // comiss below-or-unordered skips the adjustment: adjust only
            // when (v >= 0.0), NaN excluded.
            if pre_a >= 0.0 {
                let trim = rdf(bx.wrapping_add(TRIM1));
                s14 = sub(s14, trim);
                let mut t = add(rdf(bx.wrapping_add(TRIM2)), trim);
                t = add(t, s18);
                s18 = t;
            }
            if pre_b >= 0.0 {
                let trim = rdf(bx.wrapping_add(TRIM3));
                let mut t = add(rdf(bx.wrapping_add(TRIM0)), trim);
                s10 = sub(s10, trim);
                t = add(t, s1c);
                s1c = t;
            }
            s10 = sub(s10, rdf(this.wrapping_add(CTX_BIAS)));
            upper = pre_d.to_bits() & 0xffff_ff00;
        }

        // Four polls of the global function pointer, each picking an integer.
        let fp = g32(G_CALLEE_PTR);
        let poll: extern "cdecl" fn() -> u32 = core::mem::transmute(fp as usize);
        let sent = g32(G_SENTINEL);
        let pick = |ans: u32, alt: u32| {
            if sent == ans {
                g32(alt)
            } else {
                g32(if alt == G_SELP1 { G_SELP0 } else { G_SELQ0 })
            }
        };
        let ebp_sel = pick(poll(), G_SELP1);
        let ebx_sel = pick(poll(), G_SELQ1);
        let edi_sel = pick(poll(), G_SELP1);
        let ecx_sel = pick(poll(), G_SELQ1);

        let one: f32 = f32::from_bits(g32(G_ONE));
        let x2 = div(one, rdf(this.wrapping_add(CTX_DIV0)));
        let x3 = div(one, rdf(this.wrapping_add(CTX_DIV1)));
        // Four products in the original's exact per-product order.
        let p0 = mul(x2, s14);
        let s38v = mul(ecx_sel as i32 as f32, p0);
        let p1 = mul(x3, s10);
        let s44v = mul(p1, edi_sel as i32 as f32);
        let p2a = add(s18, s14);
        let p2b = mul(p2a, x2);
        let s40v = mul(p2b, ebx_sel as i32 as f32);
        let p3a = add(s1c, s10);
        let p3b = mul(p3a, x3);
        let s3cv = mul(p3b, ebp_sel as i32 as f32);

        let flag: u32 = if kind_hit { 1 } else { 0 };
        let flag_word = upper | flag;
        let tls0 = lf_checker_rt::tls_slot(0);
        let tls_flag = rd32(tls0.wrapping_add(TLS_FLAG));
        if tls_flag != 0 {
            let mut slot20 = s1c;
            let mut slot38 = s38v;
            let mut slot24 = s18;
            let mut slot2c = f32::from_bits(flag_word);
            let r: u32 = lf_checker_rt::callee_fastcall!(
                CALLEE_SINK_A, u32, ecx_sel,
                &mut slot24 as *mut f32 as u32,
                &mut slot20 as *mut f32 as u32,
                &mut slot38 as *mut f32 as u32,
                bx,
                &mut slot2c as *mut f32 as u32
            );
            let _ = (s44v, s40v, s3cv);
            r
        } else {
            let mut slot38 = s38v;
            lf_checker_rt::callee_cdecl!(
                CALLEE_SINK_B, u32,
                s18.to_bits(), s1c.to_bits(),
                &mut slot38 as *mut f32 as u32,
                bx, flag_word
            )
        }
    }
});
