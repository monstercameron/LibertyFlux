// original: 0x005d4720 CViewportPrimaryOrtho::vf1
//
// Present one primary-ortho viewport frame: configure two view stages from
// the viewport size, then draw either the scene or a full-screen clear.
//
// `this` is the viewport object. The width and height words at `+0x2c4`
// and `+0x2c0` (signed integers) are converted to float and truncated back
// to integers (x87-free SSE conversion with truncation toward zero and the
// indefinite 0x80000000 for out-of-range results, replicated exactly), and
// together with two computed zeroes and the constants 0 and 1.0 they are
// passed to the first stage callee with `this + 0x10`. The second stage
// callee takes `this + 0x10` and six constant words. A base setup callee
// runs first with `this`. Then flag bit 0 of `+0x558` is set, `+0x544` is
// set to 1000, and the pending flag at `+0x560` is tested: when set it is
// cleared and the scene path runs, otherwise a status callee decides
// between the scene path (answer 1) and the clear path. Both final paths
// call the draw callee with `this + 0x424` and nine words; the clear path
// additionally passes a pointer to its own 0xff000000 slot (compared by a
// one-word snapshot, since the address itself differs per side) and the
// colour and size constants 0x3e800000, 1.0, 0x50 and 0xff000000, while
// the scene path passes 0x186a0 among zeroes.
//
// Original: 0x005d4720 (thiscall, no stack arguments; returns the draw
// callee's answer).
lf_checker_rt::export!(thiscall, rw_005d4720(this: u32) -> u32 {
    unsafe {
        const ID_SETUP: u32 = 1;
        const ID_STAGE1: u32 = 2;
        const ID_STAGE2: u32 = 3;
        const ID_STATUS: u32 = 4;
        const ID_DRAW_A: u32 = 5;
        const ID_DRAW_B: u32 = 6;
        const ONE_BITS: u32 = 0x3f80_0000;
        const INDEFINITE: i32 = i32::MIN;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        /// Integer to float exactly as `cvtdq2ps` (round to nearest even).
        #[inline(always)]
        fn i2f(x: i32) -> f32 {
            core::hint::black_box(x as f32)
        }
        /// Float to integer exactly as `cvttss2si`: truncate toward zero,
        /// with the indefinite value for NaN and out-of-range inputs (Rust's
        /// `as` would saturate instead, so the edges are handled by hand).
        #[inline(always)]
        fn f2i_trunc(x: f32) -> i32 {
            const HI: f32 = 2147483648.0;
            const LO: f32 = -2147483648.0;
            if x.is_nan() || x >= HI || x < LO {
                INDEFINITE
            } else {
                core::hint::black_box(x) as i32
            }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(ID_SETUP, u32, this);
        let f_w = i2f(rd32(this.wrapping_add(0x2c4)) as i32);
        let f_h = i2f(rd32(this.wrapping_add(0x2c0)) as i32);
        let c_w = f2i_trunc(f_w) as u32;
        let c_h = f2i_trunc(f_h) as u32;
        let c_w0 = f2i_trunc(mul(f_w, 0.0)) as u32;
        let c_h0 = f2i_trunc(mul(f_h, 0.0)) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            ID_STAGE1, u32, this.wrapping_add(0x10), c_h0, c_w0, c_h, c_w, 0, ONE_BITS
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            ID_STAGE2, u32, this.wrapping_add(0x10), 0, ONE_BITS, ONE_BITS, 0, 0, ONE_BITS
        );
        wr8(this.wrapping_add(0x558), rd8(this.wrapping_add(0x558)) | 1);
        wr32(this.wrapping_add(0x544), 0x3e8);
        if rd8(this.wrapping_add(0x560)) != 0 {
            wr8(this.wrapping_add(0x560), 0);
            lf_checker_rt::callee_thiscall!(
                ID_DRAW_A, u32, this.wrapping_add(0x424), 0, 0x186a0, 0, 0, 0, 0, ONE_BITS, 1, 0
            )
        } else {
            let status: u32 = lf_checker_rt::callee_cdecl!(ID_STATUS, u32,);
            if status == 1 {
                lf_checker_rt::callee_thiscall!(
                    ID_DRAW_A, u32, this.wrapping_add(0x424), 0, 0x186a0, 0, 0, 0, 0, ONE_BITS, 1, 0
                )
            } else {
                let mut slot: u32 = 0xff00_0000;
                lf_checker_rt::callee_thiscall!(
                    ID_DRAW_B, u32, this.wrapping_add(0x424), 0, 0x50, 1, 0,
                    &mut slot as *mut u32 as u32, 0x3e80_0000, ONE_BITS, 0, 0
                )
            }
        }
    }
});
