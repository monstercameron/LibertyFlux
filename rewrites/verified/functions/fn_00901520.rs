// original: 0x00901520 project_and_clamp_point (proposed)

/// Project a 2D point through a matrix, then centre, limit and clamp it.
///
/// Arguments (cdecl, four stack words): `p_in` points to the two input
/// floats (x, y); `p_out` receives the two output floats; `p_mat` points to a
/// structure whose matrix block at `+0x200` holds three rows of projection
/// coefficients; the low byte of `flags` forces the early exit when nonzero.
///
/// The function first forms three row sums in the original's exact order:
/// row0 = (M210*y + M200*x) + M220*0 + M230, row1 the same shape from the
/// +0x204 column with bias M234, row2 from the +0x20c column with bias M23c
/// (each `*0` multiplies by positive zero, so an infinite coefficient still
/// yields NaN). It then divides the scale global by row2 and multiplies that
/// quotient by row0 and row1, storing the pair to the output: a perspective
/// divide whose result is handed to the callee, not returned.
///
/// It calls the helper (callee id 1) with a scratch pointer and the output
/// pointer, then copies the two doublewords the helper's return pointer
/// addresses over the output, so whatever the projection computed only
/// survives as the helper's input.
///
/// From here on every value is a float compared with unordered-aware
/// semantics (a NaN takes the "not above" path, as `comiss`+`ja` does):
/// both outputs are shifted by the centre global, the radius
/// sqrt(cy*cy + cx*cx) is taken in that operand order, and a limit is formed
/// as min(bound_y - centre, bound_x - centre) plus the margin global. The
/// early exit (restore the centre shift, return 1) is taken when the flag
/// byte is set, when the radius is not strictly above the limit, or when the
/// mode-select global is set. Otherwise the mode word decides: 0 scales both
/// centred outputs by limit/radius, 1 clamps them into
/// [-bound_x, +bound_x] x [-bound_y, +bound_y] with strict comparisons (a NaN
/// keeps its value), and any other mode leaves them; then the centre shift
/// is added back and 0 is returned. The sign-flip mask for the lower clamp
/// bounds is read from its global like every other constant.
///
/// Original: 0x00901520 (cdecl, four stack words), one direct callee.
lf_checker_rt::export!(cdecl, rw_00901520(p_in: u32, p_out: u32, p_mat: u32, flags: u32) -> u8 {
    unsafe {
        const M_R0_X: u32 = 0x200;
        const M_R1_X: u32 = 0x204;
        const M_R2_X: u32 = 0x20c;
        const M_R0_Y: u32 = 0x210;
        const M_R1_Y: u32 = 0x214;
        const M_R2_Y: u32 = 0x21c;
        const M_R0_Z: u32 = 0x220;
        const M_R1_Z: u32 = 0x224;
        const M_R2_Z: u32 = 0x22c;
        const M_R0_B: u32 = 0x230;
        const M_R1_B: u32 = 0x234;
        const M_R2_B: u32 = 0x23c;
        const G_SCALE: u32 = 0x00fe88e8;
        const G_CENTRE: u32 = 0x00fe8830;
        const G_MARGIN: u32 = 0x00fe8794;
        const G_SIGN: u32 = 0x00fe8fa0;
        const G_BOUND_X: u32 = 0x010344c8;
        const G_BOUND_Y: u32 = 0x010344cc;
        const G_MODE: u32 = 0x010344b4;
        const G_SELECT: u32 = 0x011609f6;
        const CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rd_f32(a: u32) -> f32 {
            unsafe { (a as *const f32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd_u32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr_f32(a: u32, v: f32) {
            unsafe { (a as *mut f32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn g_f32(va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(va).read() }
        }
        #[inline(always)]
        unsafe fn g_u32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let x = rd_f32(p_in);
        let y = rd_f32(p_in.wrapping_add(4));
        let zero = 0.0f32;
        // Row 0: ((M210*y + M200*x) + M220*0) + M230.
        let t200x = fmul(rd_f32(p_mat.wrapping_add(M_R0_X)), x);
        let mut row0 = fmul(rd_f32(p_mat.wrapping_add(M_R0_Y)), y);
        row0 = fadd(row0, t200x);
        row0 = fadd(row0, fmul(rd_f32(p_mat.wrapping_add(M_R0_Z)), zero));
        row0 = fadd(row0, rd_f32(p_mat.wrapping_add(M_R0_B)));
        // Row 1: ((M204*x + M214*y) + M224*0) + M234.
        let mut row1 = fmul(rd_f32(p_mat.wrapping_add(M_R1_X)), x);
        row1 = fadd(row1, fmul(rd_f32(p_mat.wrapping_add(M_R1_Y)), y));
        row1 = fadd(row1, fmul(rd_f32(p_mat.wrapping_add(M_R1_Z)), zero));
        row1 = fadd(row1, rd_f32(p_mat.wrapping_add(M_R1_B)));
        // Row 2: ((M20c*x + M21c*y) + M22c*0) + M23c.
        let mut row2 = fmul(rd_f32(p_mat.wrapping_add(M_R2_X)), x);
        row2 = fadd(row2, fmul(rd_f32(p_mat.wrapping_add(M_R2_Y)), y));
        row2 = fadd(row2, fmul(rd_f32(p_mat.wrapping_add(M_R2_Z)), zero));
        row2 = fadd(row2, rd_f32(p_mat.wrapping_add(M_R2_B)));
        // Perspective divide; the stored pair is the callee's input.
        let w = fdiv(g_f32(G_SCALE), row2);
        wr_f32(p_out, fmul(w, row0));
        wr_f32(p_out.wrapping_add(4), fmul(w, row1));

        let mut scratch = [0u32; 4];
        let answer =
            lf_checker_rt::callee_cdecl!(CALLEE, u32, scratch.as_mut_ptr() as u32, p_out);
        wr_f32(p_out, f32::from_bits(rd_u32(answer)));
        wr_f32(
            p_out.wrapping_add(4),
            f32::from_bits(rd_u32(answer.wrapping_add(4))),
        );

        let centre = g_f32(G_CENTRE);
        let mut cx = fsub(rd_f32(p_out), centre);
        let mut cy = fsub(rd_f32(p_out.wrapping_add(4)), centre);
        wr_f32(p_out, cx);
        wr_f32(p_out.wrapping_add(4), cy);

        // Radius: sqrt(cy*cy + cx*cx), in that operand order.
        let dist = fadd(fmul(cy, cy), fmul(cx, cx)).sqrt();
        let bound_x = g_f32(G_BOUND_X);
        let bound_y = g_f32(G_BOUND_Y);
        let lim_x = fsub(bound_x, centre);
        let lim_y = fsub(bound_y, centre);
        // `comiss lim_y, lim_x; ja keep`: strictly greater keeps lim_x.
        let mut lim = if lim_y > lim_x { lim_x } else { lim_y };
        lim = fadd(lim, g_f32(G_MARGIN));

        let early = (flags & 0xff) != 0
            || !(dist > lim)
            || lf_checker_rt::global::<u8>(G_SELECT).read() != 0;
        if early {
            wr_f32(p_out, fadd(cx, centre));
            wr_f32(p_out.wrapping_add(4), fadd(cy, centre));
            return 1;
        }
        let mode = g_u32(G_MODE);
        if mode == 0 {
            let f = fdiv(lim, dist);
            cx = fmul(cx, f);
            cy = fmul(cy, f);
            wr_f32(p_out, cx);
            wr_f32(p_out.wrapping_add(4), cy);
        } else if mode == 1 {
            let sign = g_u32(G_SIGN);
            let lo_x = f32::from_bits(bound_x.to_bits() ^ sign);
            if lo_x > cx {
                wr_f32(p_out, lo_x);
            }
            let lo_y = f32::from_bits(bound_y.to_bits() ^ sign);
            if lo_y > cy {
                wr_f32(p_out.wrapping_add(4), lo_y);
            }
            if rd_f32(p_out) > bound_x {
                wr_f32(p_out, bound_x);
            }
            if rd_f32(p_out.wrapping_add(4)) > bound_y {
                wr_f32(p_out.wrapping_add(4), bound_y);
            }
        }
        wr_f32(p_out, fadd(rd_f32(p_out), centre));
        wr_f32(p_out.wrapping_add(4), fadd(rd_f32(p_out.wrapping_add(4)), centre));
        0
    }
});
