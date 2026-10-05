// original: 0x009ABA40 audio_positional_resolve (proposed)

/// Audio positional resolve: turns a listener position into a placed sample
/// frame, or falls back to a default frame.
///
/// Converts the three floats at `pos` with x86 `cvttss2si` truncation
/// (NaN and out-of-range give `i32::MIN`, not saturation) and asks
/// resolver callee 1 with (iz, iy, ix, out P3, out P2, out P1). The callee
/// answers success in `al` and fills seven out-words: six at P1 (two index
/// words, three spares, one float word) and one at P3. When it fails, the
/// default frame (four words at `this + 0x10`) is copied to `out` instead.
/// On success an index `3 * (w1 + 8 * (32 * u0 + w0))` selects three
/// signed words at `this + 0xc250`, each converted to float and multiplied
/// by the constant 1/256 (`SCALE`) in (sample, scale) order; the frame
/// (s0, s1, s2, w5) is stored to `out` and passed to placer callee 2 with
/// `this`. The original aligns its stack frame and passes frame pointers;
/// the rewrite uses locals, so the pointer arguments are skipped in the
/// contract while every value they carry is compared. Returns nothing.
/// Original: 0x009ABA40 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_009ABA40(this: u32, pos: u32, out: u32) -> u32 {
    unsafe {
        const WORDS_BASE: u32 = 0xc250;
        const FALLBACK_BASE: u32 = 0x10;
        const SCALE: u32 = 0x00fe86e0;
        const RESOLVER: u32 = 1;
        const PLACER: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Truncate exactly like x86 `cvttss2si` (differs from Rust `as`).
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x > -2147483648.0 && x < 2147483648.0 {
                x as i32
            } else {
                i32::MIN
            }
        }

        let ix = cvtt(f32::from_bits(rd32(pos)));
        let iy = cvtt(f32::from_bits(rd32(pos.wrapping_add(4))));
        let iz = cvtt(f32::from_bits(rd32(pos.wrapping_add(8))));
        let mut p1 = [0u32; 6];
        let mut p3 = [0u32; 1];
        let ok: u32 = lf_checker_rt::callee_stdcall!(
            RESOLVER,
            u32,
            iz as u32,
            iy as u32,
            ix as u32,
            &mut p3 as *mut u32 as u32,
            &mut p1 as *mut u32 as u32,
            &mut p1 as *mut u32 as u32
        );
        if ok & 0xff == 0 {
            wr32(out, rd32(this.wrapping_add(FALLBACK_BASE)));
            wr32(out.wrapping_add(4), rd32(this.wrapping_add(FALLBACK_BASE + 4)));
            wr32(out.wrapping_add(8), rd32(this.wrapping_add(FALLBACK_BASE + 8)));
            wr32(out.wrapping_add(12), rd32(this.wrapping_add(FALLBACK_BASE + 12)));
            return 0;
        }
        let c = p3[0].wrapping_shl(5).wrapping_add(p1[0]);
        let a = p1[1].wrapping_add(c.wrapping_mul(8));
        let c2 = a.wrapping_add(a.wrapping_mul(2));
        let base = this.wrapping_add(WORDS_BASE).wrapping_add(c2.wrapping_mul(2));
        let s0 = ((base) as *const i16).read_unaligned() as i32;
        let s1 = ((base.wrapping_add(2)) as *const i16).read_unaligned() as i32;
        let s2 = ((base.wrapping_add(4)) as *const i16).read_unaligned() as i32;
        let scale = f32::from_bits(rd32(lf_checker_rt::relocated(SCALE)));
        wr32(out, mul(s0 as f32, scale).to_bits());
        wr32(out.wrapping_add(4), mul(s1 as f32, scale).to_bits());
        wr32(out.wrapping_add(8), mul(s2 as f32, scale).to_bits());
        wr32(out.wrapping_add(12), p1[5]);
        let _: u32 = lf_checker_rt::callee_thiscall!(PLACER, u32, this, out);
        0
    }
});
