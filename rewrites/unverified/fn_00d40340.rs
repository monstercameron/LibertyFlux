// original: 0x00D40340 task_steer_blend_submit (proposed)

/// Steer a three-word blend block down one of two paths, scale it, submit it
/// unless its kind is `0xF`, and set the done flag.
///
/// `this` (in ECX) carries the flag byte at `+0x92` (bit `0x20` selects the
/// steering branch) and three floats at `+0x80`/`+0x84`/`+0x88`; `obj` (the
/// stack argument) is the task object. The blend block starts as
/// `[0.0, 0.0, 0x40333333]`.
///
/// Steering branch: subtract the sub-block at `obj+0x20` (`+0x30`/`+0x34`/
/// `+0x38`) from the three `this` floats, call virtual slot 59 on `obj`
/// with a frame word, and run the returned triple through a quadratic
/// solve, a max-select and up to two normalisations, each guarded by an
/// unordered-aware float comparison that skips the rest of the block.
/// Fallback branch: call the same slot, scale its two written words by the
/// file constant, fold in slot 59 of `obj+0xAB0` when non-null, and, unless
/// `obj+0x228` is 0 or `-0x70`, scale by the cdecl helper's x87 result.
///
/// Tail: virtual slot 9 of the tag object scales the block by its x87
/// result; virtual slot 1 of `obj+0x38` decides whether the block is
/// submitted (thiscall with the block address); the flag word at `+0x26C`
/// becomes `(flags & ~1) | 0x2000`, which is also the exit value.
///
/// All integer comparisons are equalities or null/zero tests. The float
/// operation order is the original's. Original: thiscall, one stack word,
/// returns `eax`.
lf_checker_rt::export!(thiscall, rw_00d40340(this: u32, obj: u32) -> u32 {
    unsafe {
        const THIS_FLAG_OFF: u32 = 0x92;
        const FLAG_STEER: u8 = 0x20;
        const THIS_X: u32 = 0x80;
        const THIS_Y: u32 = 0x84;
        const THIS_Z: u32 = 0x88;
        const OBJ_SUB_LINK: u32 = 0x20;
        const SUB_X: u32 = 0x30;
        const SUB_Y: u32 = 0x34;
        const SUB_Z: u32 = 0x38;
        const OBJ_AB_LINK: u32 = 0xab0;
        const OBJ_E_LINK: u32 = 0x228;
        const E_SKIP_ADD: u32 = 0x70;
        const OBJ_T3_LINK: u32 = 0x38;
        const OBJ_FLAGS: u32 = 0x26c;
        const SLOT_SOLVE: u32 = 0xec;
        const SLOT_EMIT: u32 = 0x24;
        const SLOT_KIND: u32 = 0x04;
        const SKIP_KIND: u32 = 0x0f;
        const INIT_B2: u32 = 0x4033_3333;
        const SIGN: u32 = 0x8000_0000;
        const K_A1: u32 = 0x00ee_3ee4;
        const K_A2: u32 = 0x00ee_3eec;
        const K_A3: u32 = 0x00fe_876c;
        const K_A4: u32 = 0x00fe_88e8;
        const K_A5: u32 = 0x00fe_8bb0;
        const K_A6: u32 = 0x00fe_8b08;
        const K_A7: u32 = 0x00ee_3ee8;
        const K_A8: u32 = 0x00eb_117c;
        const K_B1: u32 = 0x0105_4bb8;
        const SCALE_ARG: u32 = 2;
        const C_SCALE: u32 = 3;
        const C_TAG: u32 = 4;
        const C_SUBMIT: u32 = 7;
        // Frame word indexes (byte offset / 4).
        const F08: usize = 2;
        const F10: usize = 4;
        const F2C: usize = 11;
        const F30: usize = 12;
        const F34: usize = 13;
        const F38: usize = 14;
        const F40: usize = 16;
        const F44: usize = 17;
        const F50: usize = 20;

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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn kc(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        fn bits(fr: &[u32; 24], i: usize) -> f32 {
            f32::from_bits(fr[i])
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }
        /// Virtual slot 59 (thiscall, one stack word) through any object.
        #[inline(always)]
        unsafe fn solve(o: u32, p: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(o).wrapping_add(SLOT_SOLVE));
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(o, p)
            }
        }
        /// The tail's tag fetch and slot 9 scale factor (x87 result).
        #[inline(always)]
        unsafe fn fetch_scale(obj: u32) -> f32 {
            unsafe {
                let tag: u32 = lf_checker_rt::callee_thiscall!(C_TAG, u32, obj);
                let slot = rd32(rd32(tag).wrapping_add(SLOT_EMIT));
                let emit: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(slot as usize);
                emit(tag)
            }
        }

        // Frame mirror, zero-filled like the checker's stack fill.
        let mut fr = [0u32; 24];
        fr[F30] = 0;
        fr[F34] = 0;
        fr[F38] = INIT_B2;
        if rd8(this.wrapping_add(THIS_FLAG_OFF)) & FLAG_STEER != 0 {
            let subobj = rd32(obj.wrapping_add(OBJ_SUB_LINK));
            fr[F10] = sub(
                rdf(this.wrapping_add(THIS_X)),
                rdf(subobj.wrapping_add(SUB_X)),
            )
            .to_bits();
            fr[F2C] = sub(
                rdf(this.wrapping_add(THIS_Y)),
                rdf(subobj.wrapping_add(SUB_Y)),
            )
            .to_bits();
            fr[F08] = sub(
                rdf(this.wrapping_add(THIS_Z)),
                rdf(subobj.wrapping_add(SUB_Z)),
            )
            .to_bits();
            let p40 = fr.as_mut_ptr().wrapping_add(F40) as u32;
            let ret = solve(obj, p40);
            let mut x2 = add(rdf(ret.wrapping_add(8)), bits(&fr, F38));
            let x0 = mul(bits(&fr, F08), kc(K_A1));
            let x6 = add(rdf(ret), bits(&fr, F30));
            let x7 = add(rdf(ret.wrapping_add(4)), bits(&fr, F34));
            let mut x1 = sub(mul(x2, x2), x0);
            if x1 >= 0.0 {
                x1 = x1.sqrt();
                let mut x3 = sub(x1, x2);
                x2 = sub(neg(x2), x1);
                x3 = mul(x3, kc(K_A2));
                x2 = mul(x2, kc(K_A2));
                x3 = if x3 > x2 { x3 } else { x2 };
                if x3 > kc(K_A3) {
                    x2 = div(kc(K_A4), x3);
                    let mut x4 = mul(bits(&fr, F10), x2);
                    x3 = mul(x2, bits(&fr, F2C));
                    x2 = mul(x2, 0.0);
                    x1 = add(add(mul(x3, x3), mul(x4, x4)), mul(x2, x2));
                    if x1 > kc(K_A5) {
                        let r = x1.sqrt();
                        let xn = div(kc(K_A6), r);
                        x4 = mul(xn, x4);
                        let xn2 = mul(xn, x2);
                        x3 = mul(x3, xn);
                        x2 = xn2;
                    }
                    x2 = add(x2, bits(&fr, F38));
                    x4 = sub(x4, x6);
                    x3 = sub(x3, x7);
                    fr[F38] = x2.to_bits();
                    x4 = add(x4, bits(&fr, F30));
                    x3 = add(x3, bits(&fr, F34));
                    fr[F30] = x4.to_bits();
                    x1 = mul(x3, x3);
                    fr[F34] = x3.to_bits();
                    x1 = add(x1, mul(x4, x4));
                    x1 = add(x1, mul(x2, x2));
                    if x1 > kc(K_A7) {
                        let r = x1.sqrt();
                        let xn = div(kc(K_A8), r);
                        fr[F30] = mul(xn, x4).to_bits();
                        let m0 = mul(xn, x3);
                        let m1 = mul(xn, x2);
                        fr[F38] = m1.to_bits();
                        fr[F34] = m0.to_bits();
                    }
                }
            }
            let s = fetch_scale(obj);
            fr[F2C] = s.to_bits();
            fr[F30] = mul(s, bits(&fr, F30)).to_bits();
            fr[F34] = mul(s, bits(&fr, F34)).to_bits();
            fr[F38] = mul(s, bits(&fr, F38)).to_bits();
        } else {
            let p40 = fr.as_mut_ptr().wrapping_add(F40) as u32;
            let _ret = solve(obj, p40);
            fr[F30] = mul(bits(&fr, F40), kc(K_B1)).to_bits();
            fr[F34] = mul(bits(&fr, F44), kc(K_B1)).to_bits();
            let ab = rd32(obj.wrapping_add(OBJ_AB_LINK));
            if ab != 0 {
                let p50 = fr.as_mut_ptr().wrapping_add(F50) as u32;
                let r2 = solve(ab, p50);
                fr[F30] = add(rdf(r2), bits(&fr, F30)).to_bits();
                fr[F34] = add(rdf(r2.wrapping_add(4)), bits(&fr, F34)).to_bits();
                fr[F38] = add(rdf(r2.wrapping_add(8)), bits(&fr, F38)).to_bits();
            }
            let e = rd32(obj.wrapping_add(OBJ_E_LINK));
            if e != 0 && e.wrapping_add(E_SKIP_ADD) != 0 {
                // The helper call is cdecl and balanced: past it the frame
                // references are the same as in every other path.
                let v: f32 = lf_checker_rt::callee_cdecl!(C_SCALE, f32, SCALE_ARG,);
                fr[F2C] = v.to_bits();
                let f30 = bits(&fr, F30);
                let f34 = bits(&fr, F34);
                let f38 = bits(&fr, F38);
                fr[F30] = mul(v, f30).to_bits();
                fr[F34] = mul(v, f34).to_bits();
                fr[F38] = mul(v, f38).to_bits();
            }
            let s = fetch_scale(obj);
            fr[F2C] = s.to_bits();
            fr[F30] = mul(s, bits(&fr, F30)).to_bits();
            fr[F34] = mul(s, bits(&fr, F34)).to_bits();
            fr[F38] = mul(s, bits(&fr, F38)).to_bits();
        }
        let sub3 = rd32(obj.wrapping_add(OBJ_T3_LINK));
        let kslot = rd32(rd32(sub3).wrapping_add(SLOT_KIND));
        let kind: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(kslot as usize);
        if kind(sub3) != SKIP_KIND {
            let ps = fr.as_mut_ptr().wrapping_add(F30) as u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(C_SUBMIT, u32, obj, ps,);
        }
        let flags = (rd32(obj.wrapping_add(OBJ_FLAGS)) & !1) | 0x2000;
        wr32(obj.wrapping_add(OBJ_FLAGS), flags);
        flags
    }
});
