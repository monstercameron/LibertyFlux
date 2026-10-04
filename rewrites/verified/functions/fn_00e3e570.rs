// original: 0x00E3E570 setup_bounded_emit (proposed)

/// Build float bounds from two inputs and emit them through a call sequence.
///
/// `this` points to an object with a count at `+0x30`: when it is not
/// positive the function only runs the stack-cookie check and returns a
/// cookie-derived residue (see below); otherwise it runs the sequence.
/// `a1` contributes only its low byte, `a2` and `a3` are float bit
/// patterns.
///
/// Behaviour (main path): `x0 = a3 + C0` and `x2 = a2 - C1` are formed
/// with two global float-table constants (`+0.03`, `-0.02`-ish). A priming
/// callee's nonzero low byte combined with a global selector dword not
/// being `2` picks a trailing constant (`0x41`, else `0x3b`) for a lookup
/// callee that takes the scratch float pair's address; the lookup's answer
/// points at a dword kept for later. A float-provider callee is then asked
/// twice (same constant tag, same scratch address): the first answer
/// truncates toward zero to an integer whose low byte becomes an index
/// `0..255`, the second answer is a float; the running value is the index
/// as a float clamped at `0.0` from below (dead: the index is never
/// negative) and then the minimum of it and the second float, truncated
/// again to a low byte that becomes the top byte of the kept dword. The
/// low byte of `a1` being zero picks an object offset 44, else 40, for a
/// no-stack-arg call; then a nine-pointer call carries the original and
/// shifted `a2` bits (twice each), two `1.0` words, two zero words and the
/// combined dword; a final no-arg call's answer is the return value. A
/// stack-cookie check closes both paths; it preserves all registers.
///
/// The early path's return value is the stack cookie XORed with the entry
/// stack pointer, which no Rust rewrite can observe or recompute, so the
/// proof contract pins the count positive (main path only) and checks the
/// return there; a supplementary early-path run covers the early branch
/// with the return unchecked. The float operation order is the original's.
///
/// Original: 0x00E3E570 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00E3E570(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const OFF_COUNT: u32 = 0x30;
        const PLUS_TAB: u32 = 0xfe8748;
        const MINUS_TAB: u32 = 0xfe8734;
        const SELECTOR: u32 = 0x11d6fd4;
        const ENABLE: u32 = 0x1161548;
        const COOKIE: u32 = 0x1057fb4;
        const CAL_PRIME: u32 = 1;
        const CAL_LOOKUP: u32 = 2;
        const CAL_FLOAT: u32 = 3;
        const CAL_REGISTRY: u32 = 4;
        const CAL_OFFSET: u32 = 5;
        const CAL_EMIT: u32 = 6;
        const CAL_FINAL: u32 = 7;
        const CAL_COOKIE: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// The original's `cvttss2si` (round toward zero, indefinite
        /// 0x80000000 on NaN or range error), low byte only, as used.
        #[inline(always)]
        fn cvtt_low8(f: f32) -> u8 {
            if f.is_nan() {
                return 0;
            }
            let t = f.trunc();
            if t >= 2147483648.0 || t < -2147483648.0 {
                0
            } else {
                (t as i32 & 0xff) as u8
            }
        }

        // Both paths end at the cookie check with ECX holding the cookie
        // value (the original's XOR dance cancels out).
        if (rd32(this.wrapping_add(OFF_COUNT)) as i32) <= 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CAL_COOKIE,
                u32,
                rd32(lf_checker_rt::relocated(COOKIE))
            );
            // The original returns cookie^ESP residue here; unrepresentable
            // (see doc comment). Unchecked by the contract.
            return 0;
        }

        let a2f = f32::from_bits(a2);
        let a3f = f32::from_bits(a3);
        let _x0 = add(a3f, f32::from_bits(rd32(lf_checker_rt::relocated(PLUS_TAB))));
        let x2 = sub(a2f, f32::from_bits(rd32(lf_checker_rt::relocated(MINUS_TAB))));
        // Scratch float pair shared by the lookup and float callees.
        let mut fbuf: [u32; 2] = [x2.to_bits(), a3];
        let prime: u32 = lf_checker_rt::callee_cdecl!(CAL_PRIME, u32, 0);
        let tail = if prime & 0xff != 0
            && rd32(lf_checker_rt::relocated(SELECTOR)) != 2
        {
            0x41
        } else {
            0x3b
        };
        let cell_ptr: u32 = lf_checker_rt::callee_cdecl!(
            CAL_LOOKUP,
            u32,
            fbuf.as_mut_ptr() as u32,
            tail
        );
        let cell = rd32(cell_ptr);
        let fptr_a: u32 = lf_checker_rt::callee_cdecl!(
            CAL_FLOAT,
            u32,
            fbuf.as_mut_ptr() as u32,
            0x37
        );
        let index = cvtt_low8(f32::from_bits(rd32(fptr_a)));
        if rd8(lf_checker_rt::relocated(ENABLE)) != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CAL_REGISTRY,
                u32,
                lf_checker_rt::relocated(ENABLE)
            );
        }
        let fptr_b: u32 = lf_checker_rt::callee_cdecl!(
            CAL_FLOAT,
            u32,
            fbuf.as_mut_ptr() as u32,
            0x37
        );
        let other = f32::from_bits(rd32(fptr_b));
        let ifloat = index as f32;
        // Clamp below at 0 (never fires: the index is unsigned), then take
        // the minimum; an unordered compare keeps the index float.
        let picked = if ifloat < 0.0 {
            0.0
        } else if ifloat > other {
            other
        } else {
            ifloat
        };
        let combined = (cell & 0xffffff) | ((cvtt_low8(picked) as u32) << 24);
        let n = if a1 & 0xff == 0 { 11u32 } else { 10u32 };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_OFFSET,
            u32,
            this.wrapping_add(n.wrapping_mul(4))
        );
        let one = 1.0f32.to_bits();
        let mut s0 = a2;
        let mut s1 = a2;
        let mut s2 = x2.to_bits();
        let mut s3 = x2.to_bits();
        let mut s4 = one;
        let mut s5 = one;
        let mut s6 = 0u32;
        let mut s7 = 0u32;
        let mut s8 = combined;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            CAL_EMIT,
            u32,
            &mut s0 as *mut u32 as u32,
            &mut s1 as *mut u32 as u32,
            &mut s2 as *mut u32 as u32,
            &mut s3 as *mut u32 as u32,
            &mut s4 as *mut u32 as u32,
            &mut s5 as *mut u32 as u32,
            &mut s6 as *mut u32 as u32,
            &mut s7 as *mut u32 as u32,
            &mut s8 as *mut u32 as u32
        );
        let answer: u32 = lf_checker_rt::callee_cdecl!(CAL_FINAL, u32,);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_COOKIE,
            u32,
            rd32(lf_checker_rt::relocated(COOKIE))
        );
        answer
    }
});
