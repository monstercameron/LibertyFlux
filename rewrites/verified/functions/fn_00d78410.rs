// original: 0x00d78410 CRenderPhaseWarpShadow::vf4 (symbols)

/// Set up one warp-shadow render phase from a source object and a config row.
///
/// `this` is the render phase, `src` the source object. Bit 2 of the source
/// flag byte at `SRC_FLAG` becomes the phase flag at `DST_FLAG`; the row
/// index at `INDEX` selects one config row (`ROW_STRIDE` bytes each) from the
/// table at `TABLE`. Callee 1 first copies the source block at `SRC_BLOCK`
/// into the phase block at `BLK_A`. If the row's enable byte is not 1 nothing
/// else happens. Otherwise the row counter is incremented and the case word
/// picks a mode constant (ored with 2 when the global limit is exceeded) for
/// `OUT_MODE`, with a fixed fallback for one sub-case. Then two running
/// argument words are accumulated across three calls: callee 2 or 3 (by the
/// row mode word) takes the phase block and the row index, the row's float
/// pair is combined (sum and difference, or a fixed pair with the row's third
/// float as the vector input), callee 4 takes the vector input plus 1.0 and
/// its result is scaled twice, and callee 5 takes the second phase block at
/// `BLK_B`, the scaled result, 1.0 and the two running words. Callees 6 and 7
/// finish the two blocks, an optional callee 8 runs unless the row id is -1,
/// and the row's done byte becomes the phase flag at `OUT_FLAG`.
///
/// Original: 0x00d78410 (thiscall, one stack word; no return value).
lf_checker_rt::export!(thiscall, rw_00d78410(this: u32, src: u32) -> u32 {
    unsafe {
        const SRC_FLAG: u32 = 0x558;
        const DST_FLAG: u32 = 0x1d;
        const INDEX: u32 = 0x940;
        const SRC_BLOCK: u32 = 0x10;
        const BLK_A: u32 = 0x4a0;
        const BLK_B: u32 = 0xb0;
        const OUT_MODE: u32 = 0x8d0;
        const OUT_FLAG: u32 = 0x44;
        const LIMIT_GLOBAL: u32 = 0x0116_0ea8;
        const TABLE: u32 = 0x119f_100;
        const ROW_STRIDE: u32 = 0x110;
        const R_MODE: u32 = 0x00;
        const R_F2: u32 = 0x0c;
        const R_F0: u32 = 0x10;
        const R_F1: u32 = 0x14;
        const R_BLK: u32 = 0x40;
        const R_DONE: u32 = 0xec;
        const R_ENABLE: u32 = 0xed;
        const R_ID: u32 = 0xf0;
        const R_CASE: u32 = 0xfc;
        const R_COUNT: u32 = 0x100;
        const K1_GLOBAL: u32 = 0x00e7_c2a8;
        const K2_GLOBAL: u32 = 0x00fe_8a24;
        const ID_BASE: u32 = 0x11a0_b10;
        const GRACE: u32 = 0x100;
        const COPY_BLK: u32 = 1;
        const PREP_EQ2: u32 = 2;
        const PREP_OTHER: u32 = 3;
        const SCALE_CALLEE: u32 = 4;
        const APPLY5: u32 = 5;
        const FIN_BLK: u32 = 6;
        const FIN_SRC: u32 = 7;
        const OPT_ID: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { (lf_checker_rt::relocated(va) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
        }
        #[inline(always)]
        unsafe fn gw32(va: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(va) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rt32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rt8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rtf(a: u32) -> f32 {
            unsafe { f32::from_bits(rt32(a)) }
        }
        #[inline(always)]
        unsafe fn rtw32(a: u32, v: u32) {
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

        wr8(this.wrapping_add(DST_FLAG), (rd8(src.wrapping_add(SRC_FLAG)) >> 2) & 1);
        let v = rd32(this.wrapping_add(INDEX));
        let blk_a = this.wrapping_add(BLK_A);
        lf_checker_rt::callee_thiscall!(COPY_BLK, u32, blk_a, src.wrapping_add(SRC_BLOCK));
        let code = if (g32(LIMIT_GLOBAL) as i32) > 2 { 2u32 } else { 0u32 };
        let row = lf_checker_rt::relocated(TABLE).wrapping_add(v.wrapping_mul(ROW_STRIDE));
        if rt8(row.wrapping_add(R_ENABLE)) != 1 {
            return 0;
        }
        let case = rt32(row.wrapping_add(R_CASE));
        rtw32(row.wrapping_add(R_COUNT), rt32(row.wrapping_add(R_COUNT)).wrapping_add(1));
        if case == 5 {
            wr32(this.wrapping_add(OUT_MODE), code | 0x0180_0087);
        } else if case == 3 {
            const SUB_GLOBAL: u32 = 0x119d_012;
            if g8(SUB_GLOBAL) == 1 {
                wr32(this.wrapping_add(OUT_MODE), code | 0x0180_0285);
            } else {
                wr32(this.wrapping_add(OUT_MODE), 0x0080_0280);
            }
        } else if case == 4 {
            wr32(this.wrapping_add(OUT_MODE), code | 0x0180_0007);
        }
        let blk_b = this.wrapping_add(BLK_B);
        let (fa, fb, vec_in): (f32, f32, f32);
        if rt32(row.wrapping_add(R_MODE)) == 2 {
            lf_checker_rt::callee_cdecl!(PREP_EQ2, u32, blk_a, v);
            fb = rtf(row.wrapping_add(R_F1));
            fa = f32::from_bits(0x3dcc_cccd);
            vec_in = rtf(row.wrapping_add(R_F2));
        } else {
            lf_checker_rt::callee_cdecl!(PREP_OTHER, u32, blk_a, v);
            let f0 = rtf(row.wrapping_add(R_F0));
            let f1 = rtf(row.wrapping_add(R_F1));
            fb = add(f0, f1);
            fa = sub(f0, f1);
            vec_in = div(f1, fa);
        }
        let scaled_bits: u32 = lf_checker_rt::callee_cdecl!(SCALE_CALLEE, u32, vec_in.to_bits());
        let scaled = mul(f32::from_bits(scaled_bits), gf(K1_GLOBAL));
        let scaled = mul(scaled, gf(K2_GLOBAL));
        lf_checker_rt::callee_stdcall!(
            APPLY5, u32, blk_b, scaled.to_bits(), 0x3f80_0000u32, fa.to_bits(), fb.to_bits()
        );
        lf_checker_rt::callee_thiscall!(FIN_BLK, u32, blk_b, row.wrapping_add(R_BLK));
        lf_checker_rt::callee_thiscall!(FIN_SRC, u32, blk_b, blk_a);
        let id = rt32(row.wrapping_add(R_ID));
        if id != 0xffff_ffff {
            lf_checker_rt::callee_thiscall!(
                OPT_ID, u32,
                lf_checker_rt::relocated(ID_BASE).wrapping_add(id.wrapping_mul(GRACE)),
                row.wrapping_add(R_MODE)
            );
        }
        wr8(this.wrapping_add(OUT_FLAG), (rt8(row.wrapping_add(R_DONE)) == 1) as u8);
        0
    }
});
