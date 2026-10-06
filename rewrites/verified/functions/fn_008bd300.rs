// original: 0x008bd300 ui_brightness_update (proposed)

/// Update a UI brightness/color level from two polled table floats.
///
/// When the mode word is not 1, return at once (the original leaks the
/// security cookie xored with the stack pointer as its return value there,
/// which a Rust rewrite cannot observe, so this proof does not compare the
/// return value). When the override byte is set, the level starts at 0xFF.
/// Otherwise two table lookups (tags 0x3F and 0x40) fetch floats; the first
/// plus 1430.0 must exceed the limit global while the limit stays below the
/// doubled sum, else control falls to the checks below. On the main path a
/// magic-constant sequence (subtract the limit, divide by the second float,
/// scale by 255, extract the sign, round with a 2^23 add and subtract, then
/// a sign-dependent -1 correction: a floor operation for the non-negative
/// values that reach it) yields an integer level by truncation. A zero level
/// goes straight to a fallback lookup (tag 0x0C) whose low byte becomes the
/// level; losing either ordered comparison instead first re-checks the
/// inverted comparison (ordered greater-or-equal takes the 0xFF override,
/// else the same fallback lookup runs). A non-positive level after the
/// fallback (signed test) ends the update, and all values that reach either
/// signed test are non-negative, so signed and unsigned agree there.
/// The level is then refined by a fourth lookup unless it already covers
/// that lookup's low byte (signed test, likewise non-negative), in which
/// case a fifth lookup replaces it. Six fixed helper calls run, four
/// constant float pairs and the level shifted to the top byte are passed to
/// a five-pointer setup helper, and a finalizer runs.
///
/// Original: 0x008bd300 (cdecl, no arguments; callee-saved ebx preserved).
/// All floating-point comparisons use the original's ordered semantics: a
/// `jbe` after `comiss` is taken for NaN, written as `!(a > b)`.
lf_checker_rt::export!(cdecl, rw_008bd300() -> u32 {
    unsafe {
        const MODE_WORD: u32 = 0x1030b7c;
        const OVERRIDE_BYTE: u32 = 0x11609f7;
        const LIMIT_WORD: u32 = 0x11609cc;
        const BASE_OFF: f32 = f32::from_bits(0x44b2c000); // 1430.0
        const TWO_P23: f32 = f32::from_bits(0x4b000000); // 8388608.0
        const SCALE: f32 = f32::from_bits(0x437f0000); // 255.0
        const ONE: f32 = 1.0;
        const SIGN_BIT: u32 = 0x80000000;

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
        /// Truncate toward zero exactly as `cvttss2si`: NaN and
        /// out-of-range values yield 0x80000000 instead of saturating.
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0f32 || x < -2147483648.0f32 {
                0x80000000u32 as i32
            } else {
                x as i32
            }
        }

        // Mode gate: the early path's return value is the cookie residue,
        // which is not compared; return a placeholder.
        if *lf_checker_rt::global::<u32>(MODE_WORD) != 1 {
            lf_checker_rt::callee_cdecl!(8, u32,);
            return 0;
        }
        // One frame slot shared by lookups 2-5, as the original reuses its
        // own frame slot for them; the stub's writes chain through it.
        let mut slot2: [u32; 2] = [0, 0];
        let mut ebx: u32;
        if *lf_checker_rt::global::<u8>(OVERRIDE_BYTE) != 0 {
            ebx = 0xff;
        } else {
            let mut slot1: [u32; 2] = [0, 0];
            let got1: u32 =
                lf_checker_rt::callee_cdecl!(1, u32, slot1.as_mut_ptr() as u32, 0x3fu32);
            let w = f32::from_bits(*(got1 as *const u32));
            let x0 = add(w, BASE_OFF);
            let got2: u32 =
                lf_checker_rt::callee_cdecl!(1, u32, slot2.as_mut_ptr() as u32, 0x40u32);
            let x1 = f32::from_bits(*lf_checker_rt::global::<u32>(LIMIT_WORD));
            let x2 = f32::from_bits(*(got2 as *const u32));
            // Ordered greater-than decides each branch; NaN falls through.
            if x1 > x0 {
                let x4pre = add(x2, x0);
                if x4pre > x1 {
                    let mut x4 = sub(x4pre, x1);
                    x4 = div(x4, x2);
                    x4 = mul(x4, SCALE);
                    let sign = x4.to_bits() & SIGN_BIT;
                    let ax = f32::from_bits(x4.to_bits() ^ sign);
                    let m: u32 = if ax < TWO_P23 { 0xFFFFFFFF } else { 0 };
                    let x2b = f32::from_bits((TWO_P23.to_bits() & m) | sign);
                    let mut x1b = add(x4, x2b);
                    x1b = sub(x1b, x2b);
                    let x0b = sub(x1b, x4);
                    let signf = f32::from_bits(sign);
                    // cmpnless: set when NOT less (unordered counts as set).
                    let m2: u32 = if !(x0b < signf) { 0xFFFFFFFF } else { 0 };
                    let adj = f32::from_bits(ONE.to_bits() & m2);
                    x1b = sub(x1b, adj);
                    ebx = cvtt(x1b) as u32;
                    if ebx == 0 {
                        // Straight to the fallback lookup.
                        let got3: u32 = lf_checker_rt::callee_cdecl!(
                            1,
                            u32,
                            slot2.as_mut_ptr() as u32,
                            0x0cu32
                        );
                        let c = cvtt(f32::from_bits(*(got3 as *const u32)));
                        ebx = (c as u8) as u32;
                        if (ebx as i32) <= 0 {
                            lf_checker_rt::callee_cdecl!(8, u32,);
                            return c as u32;
                        }
                    } else if (ebx as i32) <= 0 {
                        // Signed test; unreachable for non-negative
                        // levels, kept for faithfulness.
                        lf_checker_rt::callee_cdecl!(8, u32,);
                        return ebx;
                    }
                } else if x0 >= x1 {
                    ebx = 0xff;
                } else {
                    let got3: u32 = lf_checker_rt::callee_cdecl!(
                        1,
                        u32,
                        slot2.as_mut_ptr() as u32,
                        0x0cu32
                    );
                    let c = cvtt(f32::from_bits(*(got3 as *const u32)));
                    ebx = (c as u8) as u32;
                    if (ebx as i32) <= 0 {
                        lf_checker_rt::callee_cdecl!(8, u32,);
                        return c as u32;
                    }
                }
            } else if x0 >= x1 {
                ebx = 0xff;
            } else {
                let got3: u32 = lf_checker_rt::callee_cdecl!(
                    1,
                    u32,
                    slot2.as_mut_ptr() as u32,
                    0x0cu32
                );
                let c = cvtt(f32::from_bits(*(got3 as *const u32)));
                ebx = (c as u8) as u32;
                if (ebx as i32) <= 0 {
                    lf_checker_rt::callee_cdecl!(8, u32,);
                    return c as u32;
                }
            }
        }
        // Shared tail: refine against a fourth lookup, then helpers.
        let got4: u32 =
            lf_checker_rt::callee_cdecl!(1, u32, slot2.as_mut_ptr() as u32, 0x0cu32);
        let e4 = cvtt(f32::from_bits(*(got4 as *const u32)));
        let e4b = (e4 as u8) as u32;
        // Signed test; both sides are bytes, so the sign never matters.
        if (ebx as i32) < (e4b as i32) {
            let got5: u32 =
                lf_checker_rt::callee_cdecl!(1, u32, slot2.as_mut_ptr() as u32, 0x0cu32);
            ebx = (cvtt(f32::from_bits(*(got5 as *const u32))) as u8) as u32;
        }
        lf_checker_rt::callee_cdecl!(2, u32,);
        lf_checker_rt::callee_cdecl!(3, u32, 0u32);
        lf_checker_rt::callee_cdecl!(4, u32, 7u32, 1u32);
        lf_checker_rt::callee_cdecl!(4, u32, 0x0au32, 1u32);
        lf_checker_rt::callee_cdecl!(4, u32, 2u32, 0u32);
        lf_checker_rt::callee_cdecl!(4, u32, 0x0fu32, 0x0fu32);
        lf_checker_rt::callee_cdecl!(5, u32,);
        let q0: [u32; 2] = [0, ONE.to_bits()];
        let q1: [u32; 2] = [0, 0];
        let q2: [u32; 2] = [ONE.to_bits(), ONE.to_bits()];
        let q3: [u32; 2] = [ONE.to_bits(), 0];
        let q4: [u32; 1] = [(ebx & 0xff) << 24];
        lf_checker_rt::callee_cdecl!(
            6,
            u32,
            q0.as_ptr() as u32,
            q1.as_ptr() as u32,
            q2.as_ptr() as u32,
            q3.as_ptr() as u32,
            q4.as_ptr() as u32
        );
        let r: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
        lf_checker_rt::callee_cdecl!(8, u32,);
        r
    }
});
