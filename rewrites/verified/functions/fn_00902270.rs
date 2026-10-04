// original: 0x00902270 draw_arc_strip
/// Draw a caller-supplied polyline as an arc strip: `count` rows of six
/// floats are built from one base point plus cosine/sine offsets, then each
/// row is emitted as three draw calls.
///
/// `pts` points at the base point (two floats, read once and reused for every
/// row). `scales` points at the cosine/sine amplitudes. `count` is a signed
/// count: zero or less skips both loops (the setup calls still run).
/// `id_slot` points at a word that is copied into the draw-id global and
/// passed to every draw call. `depth` is passed through to every draw call.
///
/// Per row `i` (angle step `2*pi/count`, first angle 0): the row holds the
/// raw base point, then the base point plus the amplitude scaled by
/// cosine/sine of angle `i`, then of angle `i+1`. The three draw calls take
/// the three pairs with the shared depth, state and id words.
///
/// The original builds the rows in a variable-size stack buffer; every word
/// of every row is written by its own iteration.
///
/// Original: 0x00902270 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00902270(pts: u32, scales: u32, count: u32, id_slot: u32, depth: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        // The f32xmm0 stub sets both XMM0 (which the original reads) and EAX
        // to the answer bits; a Rust `-> f32` return would read X87 ST0
        // instead, which the stub never sets (measured: always +0.0).
        #[inline(always)]
        unsafe fn cos_f(x: f32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::callee_cdecl!(COS_CALLEE, u32, x.to_bits())) }
        }

        #[inline(always)]
        unsafe fn sin_f(x: f32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::callee_cdecl!(SIN_CALLEE, u32, x.to_bits())) }
        }

        const COS_CALLEE: u32 = 1;
        const SIN_CALLEE: u32 = 2;
        const CTX_BEGIN: u32 = 3;
        const CTX_ENTER: u32 = 4;
        const DRAW_BEGIN: u32 = 5;
        const DRAW_SEG: u32 = 6;
        const CTX_LEAVE: u32 = 7;
        const CTX_END: u32 = 8;
        const TWO_PI: f32 = f32::from_bits(0x40c9_0fdb); // .rdata 2pi
        const ZERO_MUL: f32 = 0.0; // .rdata zero multiplier
        const ID_GLOBAL: u32 = 0x110_dfb0;
        const ST_E: u32 = 0x17f_59ec;
        const ST_A: u32 = 0x17f_59f0;
        const ST_B: u32 = 0x17f_59f4;
        const ST_D: u32 = 0x17f_59f8;
        const ST_C: u32 = 0x17f_59fc;

        let base_x = rdf(pts);
        let base_y = rdf(pts + 4);
        let amp_c = rdf(scales);
        let amp_s = rdf(scales + 4);
        let n = count as i32;
        let step = fdiv(TWO_PI, n as f32);
        let first_angle = fmul(step, ZERO_MUL);
        let mut acc_c = fmul(cos_f(first_angle), amp_c);
        let mut acc_s = fmul(sin_f(first_angle), amp_s);
        let mut rows: Vec<[f32; 6]> = Vec::new();
        if n > 0 {
            rows.reserve(n as usize);
            let mut k = 1i32;
            let mut rem = n;
            while rem != 0 {
                let angle = fmul(k as f32, step);
                let tc = fmul(cos_f(angle), amp_c);
                let ts = fmul(sin_f(angle), amp_s);
                rows.push([
                    base_x,
                    base_y,
                    fadd(acc_c, base_x),
                    fadd(base_y, acc_s),
                    fadd(base_x, tc),
                    fadd(base_y, ts),
                ]);
                acc_c = tc;
                acc_s = ts;
                k += 1;
                rem -= 1;
            }
        }
        lf_checker_rt::callee_cdecl!(CTX_BEGIN, u32, 0);
        lf_checker_rt::callee_cdecl!(CTX_ENTER, u32,);
        lf_checker_rt::callee_cdecl!(DRAW_BEGIN, u32, 3, (n.wrapping_mul(3)) as u32);
        let id = rd32(id_slot);
        (lf_checker_rt::relocated(ID_GLOBAL) as *mut u32).write(id);
        if n > 0 {
            let mut rem = n;
            let mut ri = 0usize;
            while rem != 0 {
                let r = rows[ri];
                // State words are re-read per call like the original.
                let st = |va: u32| (lf_checker_rt::relocated(va) as *const u32).read();
                lf_checker_rt::callee_cdecl!(DRAW_SEG, u32,
                    r[0].to_bits(), r[1].to_bits(), depth, st(ST_E), st(ST_D), st(ST_C), id, st(ST_B), st(ST_A));
                let st = |va: u32| (lf_checker_rt::relocated(va) as *const u32).read();
                lf_checker_rt::callee_cdecl!(DRAW_SEG, u32,
                    r[2].to_bits(), r[3].to_bits(), depth, st(ST_E), st(ST_D), st(ST_C), id, st(ST_B), st(ST_A));
                let st = |va: u32| (lf_checker_rt::relocated(va) as *const u32).read();
                lf_checker_rt::callee_cdecl!(DRAW_SEG, u32,
                    r[4].to_bits(), r[5].to_bits(), depth, st(ST_E), st(ST_D), st(ST_C), id, st(ST_B), st(ST_A));
                ri += 1;
                rem -= 1;
            }
        }
        lf_checker_rt::callee_cdecl!(CTX_LEAVE, u32,);
        lf_checker_rt::callee_cdecl!(CTX_END, u32,)
    }
});
