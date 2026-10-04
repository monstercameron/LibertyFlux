// original: 0x00D6F750 CReplayProgressBar::vf2
/// Draw the replay progress bar and its chapter markers.
///
/// Renders the progress track, the played-versus-total fill, the position
/// thumb and one marker per chapter onto the screen-space rectangles
/// derived from the object's layout fields. All coordinates are scaled by
/// the display dimensions chosen per block through a scripted display-mode
/// query, times are normalized through a scripted duration lookup, colours
/// come from a scripted HUD palette lookup, and the resulting quads are
/// submitted through scripted draw-list calls. When the chapter list is
/// empty the bar draws nothing; a global progress-display switch selects
/// between the compact bar and the full bar with markers.
export!(thiscall, rw_00d6f750(this: u32) -> u32 {
    unsafe {
        const Q: u32 = 1;
        const DUR: u32 = 2;
        const HUD: u32 = 3;
        const SETUP: u32 = 4;
        const QUAD: u32 = 5;
        const DONE: u32 = 6;
        const CHAPTERS: u32 = 7;
        const FIT: u32 = 8;
        const BEGIN: u32 = 9;
        const EMIT: u32 = 10;
        const LAYOUT: u32 = 11;
        /// Display width choices and height choices.
        const DIM_W0: u32 = 0x0105C884;
        const DIM_W1: u32 = 0x0105C888;
        const DIM_H0: u32 = 0x0105C880;
        const DIM_H1B: u32 = 0x0105C87C;
        /// Progress-display switch.
        const SHOW: u32 = 0x0103F690;
        /// Seconds-to-ticks scale, half-unit and margin constants.
        const K_TICKS: u32 = 0x00FE8C58;
        const K_HALF: u32 = 0x00FE8830;
        const K_MARGIN: u32 = 0x00FE86EC;
        /// Offscreen placeholder coordinate.
        const BIG: u32 = 0x49742400;
        const NEG_BIG: u32 = 0xC9742400;

        /// Truncate toward zero to a full 64-bit word pair, matching the
        /// original's convert-with-chop then store-doubleword-pair sequence:
        /// NaN and magnitudes at or above 2^63 store the indefinite value.
        fn trunc_pair(x: f32) -> (u32, u32) {
            const TWO63: f32 = 9223372036854775808.0;
            if x.is_nan() || x >= TWO63 || x <= -TWO63 {
                (0, 0x80000000)
            } else {
                let v = x as i64;
                (v as u32, (v >> 32) as u32)
            }
        }
        /// The original's integer-to-float path converts the stored word
        /// as an unsigned 32-bit value through a wider float and back.
        fn word_to_f32(w: u32) -> f32 {
            (w as f64) as f32
        }
        /// Scalar add/multiply with FORCED operand order. Plain `+`/`*`
        /// (and even the SSE intrinsics, which lower to plain nodes) let
        /// LLVM commute the operands, which changes NaN payloads on some
        /// hosts; the call boundary keeps the order exactly as written
        /// (verified in the built DLL). Subtraction and division cannot
        /// be commuted and stay plain operators.
        #[inline(never)]
        fn fadd(a: f32, b: f32) -> f32 {
            a + b
        }
        #[inline(never)]
        fn fmul(a: f32, b: f32) -> f32 {
            a * b
        }
        fn shown() -> bool {
            unsafe { *(relocated(SHOW) as *const u8) != 0 }
        }

        let uld = |off: u32| unsafe { *((this.wrapping_add(off)) as *const u32) };
        let fld = |off: u32| unsafe { *((this.wrapping_add(off)) as *const f32) };
        let ust = |off: u32, v: u32| unsafe { *((this.wrapping_add(off)) as *mut u32) = v };
        let dim_w = || unsafe {
            let q: u32 = callee_cdecl!(Q, u32,);
            let v = if (q as u8) != 0 {
                *(relocated(DIM_W1) as *const i32)
            } else {
                *(relocated(DIM_W0) as *const i32)
            };
            v as f32
        };
        let dim_h = || unsafe {
            let q: u32 = callee_cdecl!(Q, u32,);
            let v = if (q as u8) != 0 {
                *(relocated(DIM_H1B) as *const i32)
            } else {
                *(relocated(DIM_H0) as *const i32)
            };
            v as f32
        };
        let ticks = || unsafe { *(relocated(K_TICKS) as *const f32) };
        let half = || unsafe { *(relocated(K_HALF) as *const f32) };
        let margin = || unsafe { *(relocated(K_MARGIN) as *const f32) };

        if uld(0x9C) == 0 {
            return 0;
        }
        let mut fr = [0u32; 48];
        fr[0x6C >> 2] = BIG;
        fr[0x78 >> 2] = BIG;
        fr[0x74 >> 2] = NEG_BIG;
        fr[0x70 >> 2] = NEG_BIG;
        fr[0x7C >> 2] = BIG;
        fr[0x88 >> 2] = BIG;
        fr[0x84 >> 2] = NEG_BIG;
        fr[0x80 >> 2] = NEG_BIG;
        fr[0x8C >> 2] = BIG;
        fr[0x98 >> 2] = BIG;
        fr[0x94 >> 2] = NEG_BIG;
        fr[0x90 >> 2] = NEG_BIG;
        fr[0x9C >> 2] = BIG;
        fr[0xA8 >> 2] = BIG;
        fr[0xA4 >> 2] = NEG_BIG;
        fr[0xA0 >> 2] = NEG_BIG;
        fr[0x68 >> 2] = 0xFFFFFFFF;
        fr[0x4C >> 2] = 0xFFFFFFFF;
        fr[0x64 >> 2] = 0xFFFFFFFF;
        fr[0x60 >> 2] = 0xFFFFFFFF;
        fr[0x6C >> 2] = (fmul(dim_w(), fld(8))).to_bits();
        fr[0x74 >> 2] = (fmul(fadd(fld(0x10), fld(8)), dim_w())).to_bits();
        fr[0x70 >> 2] = (fmul(dim_h(), fld(0xC))).to_bits();
        fr[0x78 >> 2] = (fmul(fadd(fld(0xC), fld(0x14)), dim_h())).to_bits();
        fr[0x68 >> 2] = 0xFF6D6D6D;
        let e1: u32 = callee_thiscall!(DUR, u32, this, uld(0xB8), 0);
        let e2: u32 = callee_thiscall!(DUR, u32, this, uld(0xCC), 0);
        let (lo, hi) = trunc_pair(fmul(fld(0x20), ticks()));
        fr[0x30 >> 2] = lo;
        fr[0x34 >> 2] = hi;
        fr[0x18 >> 2] = (word_to_f32(e1) / word_to_f32(lo)).to_bits();
        fr[0x8C >> 2] = fld(0xE4).to_bits();
        fr[0x94 >> 2] = (fmul(fadd(fmul(fld(0x10), f32::from_bits(fr[0x18 >> 2])), fld(8)), dim_w())).to_bits();
        fr[0x90 >> 2] = fld(0xAC).to_bits();
        fr[0x98 >> 2] = fld(0xB4).to_bits();
        let (lo, hi) = trunc_pair(fmul(fld(0x20), ticks()));
        fr[0x30 >> 2] = lo;
        fr[0x34 >> 2] = hi;
        fr[0x18 >> 2] = (word_to_f32(e2) / word_to_f32(lo)).to_bits();
        fr[0x9C >> 2] = (fmul(fadd(fmul(fld(0x10), f32::from_bits(fr[0x18 >> 2])), fld(8)), dim_w())).to_bits();
        fr[0xA4 >> 2] = fld(0xD8).to_bits();
        fr[0xA0 >> 2] = fld(0xC0).to_bits();
        fr[0xA8 >> 2] = fld(0xC8).to_bits();
        fr[0x60 >> 2] = 0xFF202020;
        let (lo, hi) = trunc_pair(fmul(fld(0x1C), ticks()));
        fr[0x30 >> 2] = lo;
        fr[0x34 >> 2] = hi;
        let num = word_to_f32(lo);
        let (lo, hi) = trunc_pair(fmul(fld(0x20), ticks()));
        fr[0x30 >> 2] = lo;
        fr[0x34 >> 2] = hi;
        fr[0x38 >> 2] = (num / word_to_f32(lo)).to_bits();
        fr[0x7C >> 2] = (fmul(dim_w(), fld(8))).to_bits();
        fr[0x84 >> 2] = (fmul(fadd(fmul(fld(0x10), f32::from_bits(fr[0x38 >> 2])), fld(8)), dim_w())).to_bits();
        fr[0x80 >> 2] = (fmul(fadd(fld(0xC), margin()), dim_h())).to_bits();
        fr[0x88 >> 2] = (fmul(fadd(fld(0xC), fld(0x14)) - margin(), dim_h())).to_bits();
        let hud_slot = fr.as_mut_ptr().add(0x24 >> 2) as u32;
        let _: u32 = callee_cdecl!(HUD, u32, hud_slot, 0x3E);
        let color = fr[0x24 >> 2];
        fr[0x64 >> 2] = color;
        if !shown() {
            let v = fr[0x8C >> 2];
            fr[0x7C >> 2] = v;
            fr[0x6C >> 2] = v;
            fr[0x74 >> 2] = fr[0xA4 >> 2];
        }
        let _: u32 = callee_cdecl!(SETUP, u32, 0, 1);
        let a0 = fr.as_mut_ptr().add(0x6C >> 2) as u32;
        let a1 = fr.as_mut_ptr().add(0x68 >> 2) as u32;
        let _: u32 = callee_cdecl!(QUAD, u32, a0, a1);
        let _: u32 = callee_cdecl!(DONE, u32,);
        let _: u32 = callee_thiscall!(CHAPTERS, u32, this,);
        let _: u32 = callee_cdecl!(SETUP, u32, 0, 1);
        let a0 = fr.as_mut_ptr().add(0x7C >> 2) as u32;
        let a1 = fr.as_mut_ptr().add(0x64 >> 2) as u32;
        let _: u32 = callee_cdecl!(QUAD, u32, a0, a1);
        let _: u32 = callee_cdecl!(DONE, u32,);
        if shown() {
            let _: u32 = callee_cdecl!(SETUP, u32, 0, 1);
            let a0 = fr.as_mut_ptr().add(0x8C >> 2) as u32;
            let a1 = fr.as_mut_ptr().add(0x60 >> 2) as u32;
            let _: u32 = callee_cdecl!(QUAD, u32, a0, a1);
            let _: u32 = callee_cdecl!(DONE, u32,);
            let _: u32 = callee_cdecl!(SETUP, u32, 0, 1);
            let a0 = fr.as_mut_ptr().add(0x9C >> 2) as u32;
            let a1 = fr.as_mut_ptr().add(0x60 >> 2) as u32;
            let _: u32 = callee_cdecl!(QUAD, u32, a0, a1);
            let _: u32 = callee_cdecl!(DONE, u32,);
            fr[0x3C >> 2] = BIG;
            fr[0x48 >> 2] = BIG;
            fr[0x44 >> 2] = NEG_BIG;
            fr[0x40 >> 2] = NEG_BIG;
            fr[0x50 >> 2] = BIG;
            fr[0x5C >> 2] = BIG;
            fr[0x58 >> 2] = NEG_BIG;
            fr[0x54 >> 2] = NEG_BIG;
            fr[0x2C >> 2] = (fld(0xD0) - fld(0xEC)).to_bits();
            fr[0x14 >> 2] = 0;
            fr[0x20 >> 2] = 0;
            fr[0x28 >> 2] = 0;
            fr[0x18 >> 2] = 0x3CC49BA6;
            let p1 = fr.as_mut_ptr().add(0x14 >> 2) as u32;
            let p2 = fr.as_mut_ptr().add(0x20 >> 2) as u32;
            let p3 = fr.as_mut_ptr().add(0x28 >> 2) as u32;
            let p4 = fr.as_mut_ptr().add(0x18 >> 2) as u32;
            let _: u32 = callee_thiscall!(FIT, u32, this, uld(0x88), p1, p2, p3, p4);
            let (lo, hi) = trunc_pair(fmul(fld(0x20), ticks()));
            fr[0x30 >> 2] = lo;
            fr[0x34 >> 2] = hi;
            let e3: u32 = callee_thiscall!(DUR, u32, this, uld(0xB8), 0);
            fr[0x20 >> 2] = (word_to_f32(e3) / word_to_f32(lo)).to_bits();
            let r2 = fmul(f32::from_bits(fr[0x20 >> 2]), f32::from_bits(fr[0x2C >> 2]));
            fr[0x20 >> 2] = r2.to_bits();
            ust(0xA8, (fadd(fmul((fld(8) - f32::from_bits(fr[0x28 >> 2])), dim_w()), r2)).to_bits());
            fr[0x3C >> 2] = uld(0xA8);
            let x1 = fadd(fmul(dim_w(), fld(8)), f32::from_bits(fr[0x20 >> 2]));
            let (lo, hi) = trunc_pair(fmul(fld(0x20), ticks()));
            fr[0x30 >> 2] = lo;
            fr[0x34 >> 2] = hi;
            ust(0xB0, x1.to_bits());
            fr[0x44 >> 2] = x1.to_bits();
            let e4: u32 = callee_thiscall!(DUR, u32, this, uld(0xCC), 0);
            fr[0x20 >> 2] = (word_to_f32(e4) / word_to_f32(lo)).to_bits();
            let r3 = fmul(f32::from_bits(fr[0x20 >> 2]), f32::from_bits(fr[0x2C >> 2]));
            fr[0x20 >> 2] = r3.to_bits();
            ust(0xBC, (fadd(fmul(dim_w(), fld(8)), r3)).to_bits());
            let r4 = fadd(
                fmul(fadd(fld(8), f32::from_bits(fr[0x28 >> 2])), dim_w()),
                f32::from_bits(fr[0x20 >> 2]),
            );
            fr[0x50 >> 2] = uld(0xBC);
            fr[0x40 >> 2] = fld(0xAC).to_bits();
            fr[0x48 >> 2] = fld(0xB4).to_bits();
            fr[0x54 >> 2] = fld(0xC0).to_bits();
            ust(0xC4, r4.to_bits());
            fr[0x58 >> 2] = r4.to_bits();
            fr[0x5C >> 2] = fld(0xC8).to_bits();
            let _: u32 = callee_thiscall!(BEGIN, u32, uld(0x88),);
            let a0 = fr.as_mut_ptr().add(0x3C >> 2) as u32;
            let a1 = fr.as_mut_ptr().add(0x4C >> 2) as u32;
            let _: u32 = callee_cdecl!(EMIT, u32, a0, a1);
            let _: u32 = callee_cdecl!(DONE, u32,);
            let _: u32 = callee_thiscall!(BEGIN, u32, uld(0x8C),);
            let a0 = fr.as_mut_ptr().add(0x50 >> 2) as u32;
            let a1 = fr.as_mut_ptr().add(0x4C >> 2) as u32;
            let _: u32 = callee_cdecl!(EMIT, u32, a0, a1);
            let _: u32 = callee_cdecl!(DONE, u32,);
        }
        let _: u32 = callee_thiscall!(LAYOUT, u32, this.wrapping_add(0x1C),);
        fr[0x28 >> 2] = 0;
        fr[0x20 >> 2] = 0;
        fr[0x2C >> 2] = 0;
        fr[0x18 >> 2] = 0x3CAC0831;
        let p1 = fr.as_mut_ptr().add(0x28 >> 2) as u32;
        let p2 = fr.as_mut_ptr().add(0x20 >> 2) as u32;
        let p3 = fr.as_mut_ptr().add(0x2C >> 2) as u32;
        let p4 = fr.as_mut_ptr().add(0x18 >> 2) as u32;
        let _: u32 = callee_thiscall!(FIT, u32, this, uld(0x90), p1, p2, p3, p4);
        fr[0x50 >> 2] = BIG;
        fr[0x5C >> 2] = BIG;
        fr[0x58 >> 2] = NEG_BIG;
        fr[0x54 >> 2] = NEG_BIG;
        fr[0x54 >> 2] = (fmul(dim_h(), fld(0xC))).to_bits();
        fr[0x5C >> 2] = (fmul(fadd(fld(0xC), fld(0x14)), dim_h())).to_bits();
        let half_scaled = fmul(f32::from_bits(fr[0x2C >> 2]), half());
        fr[0x18 >> 2] = half_scaled.to_bits();
        fr[0x50 >> 2] =
            (fmul(fadd(fmul(fld(0x10), f32::from_bits(fr[0x38 >> 2])), fld(8)) - half_scaled, dim_w())).to_bits();
        fr[0x58 >> 2] = fmul(
            fadd(
                fadd(
                    fmul(fld(0x10), f32::from_bits(fr[0x38 >> 2])),
                    fld(8),
                ),
                f32::from_bits(fr[0x18 >> 2]),
            ),
            dim_w(),
        )
        .to_bits();
        let _: u32 = callee_thiscall!(BEGIN, u32, uld(0x90),);
        let a0 = fr.as_mut_ptr().add(0x50 >> 2) as u32;
        let a1 = fr.as_mut_ptr().add(0x4C >> 2) as u32;
        let _: u32 = callee_cdecl!(EMIT, u32, a0, a1);
        let _: u32 = callee_cdecl!(DONE, u32,);
        if !shown() {
            return 0;
        }
        fr[0x3C >> 2] = BIG;
        fr[0x48 >> 2] = BIG;
        fr[0x44 >> 2] = NEG_BIG;
        fr[0x40 >> 2] = NEG_BIG;
        let hud_slot = fr.as_mut_ptr().add(0x2C >> 2) as u32;
        let _: u32 = callee_cdecl!(HUD, u32, hud_slot, 0x3D);
        fr[0x28 >> 2] = 0xFFFFFFFF;
        fr[0x14 >> 2] = 0;
        fr[0x18 >> 2] = 0;
        fr[0x20 >> 2] = 0;
        fr[0x38 >> 2] = 0x3D1374BC;
        let p1 = fr.as_mut_ptr().add(0x14 >> 2) as u32;
        let p2 = fr.as_mut_ptr().add(0x18 >> 2) as u32;
        let p3 = fr.as_mut_ptr().add(0x20 >> 2) as u32;
        let p4 = fr.as_mut_ptr().add(0x38 >> 2) as u32;
        let _: u32 = callee_thiscall!(FIT, u32, this, uld(0x94), p1, p2, p3, p4);
        let arr = uld(0x9C);
        let count = unsafe { *((arr.wrapping_add(4)) as *const u16) };
        if count != 0 {
            fr[0x24 >> 2] = (fmul(f32::from_bits(fr[0x20 >> 2]), half())).to_bits();
            let base = unsafe { *(arr as *const u32) };
            let mut i = 0u32;
            while i < count as u32 {
                let item = unsafe {
                    *((base.wrapping_add(i.wrapping_mul(4))) as *const u32)
                };
                let (idx, slot): (u32, usize) = if i == uld(0xA0) {
                    (0x3E, 0xB4 >> 2)
                } else if i == uld(0xA4) {
                    (1, 0xB0 >> 2)
                } else {
                    (0x3B, 0x30 >> 2)
                };
                let hud_ptr = fr.as_mut_ptr().add(slot) as u32;
                let _: u32 = callee_cdecl!(HUD, u32, hud_ptr, idx);
                fr[0x28 >> 2] = fr[slot];
                fr[0xAC >> 2] = (fmul(fld(0x20), ticks())).to_bits();
                let (lo, hi) = trunc_pair(f32::from_bits(fr[0xAC >> 2]));
                fr[0x18 >> 2] = lo;
                fr[0x1C >> 2] = hi;
                let item_arg = unsafe { *((item.wrapping_add(0x14)) as *const u32) };
                let e: u32 = callee_thiscall!(DUR, u32, this, item_arg, 0);
                fr[0x18 >> 2] = (word_to_f32(e) / word_to_f32(lo)).to_bits();
                fr[0x14 >> 2] = fmul(
                    fadd(
                        fmul(fld(0x10), f32::from_bits(fr[0x18 >> 2])),
                        fld(8),
                    ) - f32::from_bits(fr[0x24 >> 2]),
                    dim_w(),
                )
                .to_bits();
                fr[0x18 >> 2] =
                    fmul(fld(0xC) - f32::from_bits(fr[0x38 >> 2]), dim_h()).to_bits();
                fr[0x3C >> 2] = fr[0x14 >> 2];
                fr[0x44 >> 2] =
                    fadd(fmul(dim_w(), f32::from_bits(fr[0x20 >> 2])), f32::from_bits(fr[0x14 >> 2]))
                        .to_bits();
                fr[0x40 >> 2] = fr[0x18 >> 2];
                fr[0x48 >> 2] =
                    fadd(fmul(dim_h(), f32::from_bits(fr[0x38 >> 2])), f32::from_bits(fr[0x18 >> 2]))
                        .to_bits();
                let _: u32 = callee_thiscall!(BEGIN, u32, uld(0x94),);
                let a0 = fr.as_mut_ptr().add(0x3C >> 2) as u32;
                let a1 = fr.as_mut_ptr().add(0x2C >> 2) as u32;
                let _: u32 = callee_cdecl!(EMIT, u32, a0, a1);
                let _: u32 = callee_cdecl!(DONE, u32,);
                let _: u32 = callee_thiscall!(BEGIN, u32, uld(0x98),);
                let a0 = fr.as_mut_ptr().add(0x3C >> 2) as u32;
                let a1 = fr.as_mut_ptr().add(0x28 >> 2) as u32;
                let _: u32 = callee_cdecl!(EMIT, u32, a0, a1);
                let _: u32 = callee_cdecl!(DONE, u32,);
                i = i.wrapping_add(1);
            }
        }
        if (uld(0xA0) as i32) >= 0 {
            let base = unsafe { *(uld(0x9C) as *const u32) };
            let item = unsafe {
                *((base.wrapping_add(uld(0xA0).wrapping_mul(4))) as *const u32)
            };
            let hud_ptr = fr.as_mut_ptr().add(0x30 >> 2) as u32;
            let _: u32 = callee_cdecl!(HUD, u32, hud_ptr, 0x3E);
            fr[0x28 >> 2] = fr[0x30 >> 2];
            let (lo, hi) = trunc_pair(fmul(fld(0x20), ticks()));
            fr[0x30 >> 2] = lo;
            fr[0x34 >> 2] = hi;
            let item_arg = unsafe { *((item.wrapping_add(0x14)) as *const u32) };
            let e: u32 = callee_thiscall!(DUR, u32, this, item_arg, 0);
            fr[0x24 >> 2] = (word_to_f32(e) / word_to_f32(lo)).to_bits();
            let r = fmul(
                fadd(
                    fmul(fld(0x10), f32::from_bits(fr[0x24 >> 2])),
                    fld(8),
                ) - fmul(f32::from_bits(fr[0x20 >> 2]), half()),
                dim_w(),
            );
            fr[0x24 >> 2] = r.to_bits();
            fr[0x3C >> 2] = r.to_bits();
            fr[0x44 >> 2] =
                fadd(fmul(dim_w(), f32::from_bits(fr[0x20 >> 2])), f32::from_bits(fr[0x24 >> 2])).to_bits();
            let _: u32 = callee_thiscall!(BEGIN, u32, uld(0x94),);
            let a0 = fr.as_mut_ptr().add(0x3C >> 2) as u32;
            let a1 = fr.as_mut_ptr().add(0x2C >> 2) as u32;
            let _: u32 = callee_cdecl!(EMIT, u32, a0, a1);
            let _: u32 = callee_cdecl!(DONE, u32,);
            let _: u32 = callee_thiscall!(BEGIN, u32, uld(0x98),);
            let a0 = fr.as_mut_ptr().add(0x3C >> 2) as u32;
            let a1 = fr.as_mut_ptr().add(0x28 >> 2) as u32;
            let _: u32 = callee_cdecl!(EMIT, u32, a0, a1);
            let _: u32 = callee_cdecl!(DONE, u32,);
        }
        if (uld(0xA4) as i32) >= 0 {
            let base = unsafe { *(uld(0x9C) as *const u32) };
            let item = unsafe {
                *((base.wrapping_add(uld(0xA4).wrapping_mul(4))) as *const u32)
            };
            let hud_ptr = fr.as_mut_ptr().add(0x30 >> 2) as u32;
            let _: u32 = callee_cdecl!(HUD, u32, hud_ptr, 1);
            fr[0x28 >> 2] = fr[0x30 >> 2];
            let (lo, hi) = trunc_pair(fmul(fld(0x20), ticks()));
            fr[0x30 >> 2] = lo;
            fr[0x34 >> 2] = hi;
            let item_arg = unsafe { *((item.wrapping_add(0x14)) as *const u32) };
            let e: u32 = callee_thiscall!(DUR, u32, this, item_arg, 0);
            fr[0x24 >> 2] = (word_to_f32(e) / word_to_f32(lo)).to_bits();
            let r = fmul(
                fadd(
                    fmul(fld(0x10), f32::from_bits(fr[0x24 >> 2])),
                    fld(8),
                ) - fmul(f32::from_bits(fr[0x20 >> 2]), half()),
                dim_w(),
            );
            fr[0x24 >> 2] = r.to_bits();
            fr[0x3C >> 2] = r.to_bits();
            fr[0x44 >> 2] =
                fadd(fmul(dim_w(), f32::from_bits(fr[0x20 >> 2])), f32::from_bits(fr[0x24 >> 2])).to_bits();
            let _: u32 = callee_thiscall!(BEGIN, u32, uld(0x94),);
            let a0 = fr.as_mut_ptr().add(0x3C >> 2) as u32;
            let a1 = fr.as_mut_ptr().add(0x2C >> 2) as u32;
            let _: u32 = callee_cdecl!(EMIT, u32, a0, a1);
            let _: u32 = callee_cdecl!(DONE, u32,);
            let _: u32 = callee_thiscall!(BEGIN, u32, uld(0x98),);
            let a0 = fr.as_mut_ptr().add(0x3C >> 2) as u32;
            let a1 = fr.as_mut_ptr().add(0x28 >> 2) as u32;
            let _: u32 = callee_cdecl!(EMIT, u32, a0, a1);
            let _: u32 = callee_cdecl!(DONE, u32,);
        }
        0
    }
});
