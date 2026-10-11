// original: 0x00902570 draw_quad_arcs
/// Draw four quadrant arcs around a centre: for each of the four sign
/// combinations of the radii, seven arc points are computed and emitted as
/// eight draw calls (the centre first, then the seven points).
///
/// `cx`, `cy` are the centre, `rx`, `ry` the radii (all float bits). `color`
/// points at one color word whose bytes are scaled and passed to every draw
/// call. The mode word read from its global is clamped to zero when positive
/// and passed to the mode call.
///
/// Per quadrant the point angle is `k * (pi/2) / 6` for `k` in 0..7,
/// ascending on odd quadrants and descending on even ones; each point is the
/// centre plus cosine/sine of the angle times the quadrant's signed radii.
/// Each draw call takes the point, three zero words, -1.0, the scaled color
/// and two zero words.
///
/// Original: 0x00902570 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00902570(cx: u32, cy: u32, rx: u32, ry: u32, color: u32) -> u32 {
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
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        #[inline(always)]
        fn fneg_bits(v: f32) -> f32 {
            f32::from_bits(v.to_bits() ^ 0x8000_0000)
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
        const CTX_FINI: u32 = 9;
        const CTX_MODE: u32 = 10;
        const HALF_PI: f32 = f32::from_bits(0x3fc9_0fdb); // .rdata pi/2
        const INV_SIX: f32 = f32::from_bits(0x3e2a_aaab); // .rdata 1/6
        const INV_255: f32 = f32::from_bits(0x3b80_8081); // .rdata 1/255
        const C_255: f32 = f32::from_bits(0x437f_0000); // .rdata 255.0
        const NEG_ONE_BITS: u32 = 0xbf80_0000;
        const MODE_GLOBAL: u32 = 0x106_b310;

        #[inline(always)]
        unsafe fn draw6(x: u32, y: u32, scaled: u32) {
            unsafe {
                lf_checker_rt::callee_cdecl!(DRAW_SEG, u32,
                    x, y, 0, 0, 0, NEG_ONE_BITS, scaled, 0, 0);
            }
        }

        #[inline(always)]
        fn scale_byte(b: u32) -> u32 {
            let t = fmul(fmul(b as f32, INV_255), C_255);
            // Truncation; the value is always a finite in-range non-negative
            // float, so `as` matches cvttss2si exactly.
            (t as i32) as u32 & 0xff
        }

        lf_checker_rt::callee_cdecl!(CTX_BEGIN, u32, 0);
        let mode = (lf_checker_rt::relocated(MODE_GLOBAL) as *const u32).read();
        let mode_arg = if (mode as i32) < 0 { mode } else { 0 };
        lf_checker_rt::callee_cdecl!(CTX_MODE, u32, 1, mode_arg);
        lf_checker_rt::callee_cdecl!(CTX_ENTER, u32,);
        let centre_x = f32::from_bits(cx);
        let centre_y = f32::from_bits(cy);
        let rad_x = f32::from_bits(rx);
        let rad_y = f32::from_bits(ry);
        let neg_x = fneg_bits(rad_x);
        let neg_y = fneg_bits(rad_y);
        let quads = [
            (rad_x, rad_y),
            (neg_x, rad_y),
            (rad_x, neg_y),
            (neg_x, neg_y),
        ];
        let mut outer = 0i32;
        while outer < 4 {
            let (qx, qy) = quads[outer as usize];
            let mut pairs = [(0.0f32, 0.0f32); 7];
            let mut t = 0usize;
            while t < 7 {
                let k = if outer & 1 == 1 { t as i32 } else { 6 - t as i32 };
                let base = fmul(fmul(k as f32, HALF_PI), INV_SIX);
                let px = fadd(fmul(cos_f(base), qx), centre_x);
                let py = fadd(fmul(sin_f(base), qy), centre_y);
                pairs[t] = (px, py);
                t += 1;
            }
            lf_checker_rt::callee_cdecl!(DRAW_BEGIN, u32, 5, 8);
            let raw = rd32(color);
            let scaled = scale_byte(raw >> 24) << 24
                | scale_byte((raw >> 16) & 0xff) << 16
                | scale_byte((raw >> 8) & 0xff) << 8
                | scale_byte(raw & 0xff);
            draw6(cx, cy, scaled);
            let mut i = 0usize;
            while i < 7 {
                draw6(pairs[i].0.to_bits(), pairs[i].1.to_bits(), scaled);
                i += 1;
            }
            lf_checker_rt::callee_cdecl!(CTX_LEAVE, u32,);
            outer += 1;
        }
        lf_checker_rt::callee_cdecl!(CTX_END, u32,);
        lf_checker_rt::callee_cdecl!(CTX_FINI, u32,)
    }
});
