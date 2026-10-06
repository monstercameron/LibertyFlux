// original: 0x00adc280 input_ui_float_table_build (proposed)

/// Update a float parameter table, derive packed colors from it, and issue
/// draw calls through game helpers.
///
/// Cdecl, one stack word `a0` (an object pointer or null; never dereferenced,
/// only forwarded), plain `ret`. Returns the last helper answer (the tail
/// `put2` answer). All helper calls are intercepted; the two `memset`-shaped
/// and arithmetic helpers run only through their scripted answers.
///
/// Phases in order. (1) Flag-gated one-time fills keyed on bits 0-2 of
/// `FLAGS`: bit 0 stores `sin`/`cos` of `PHASE` into `T0`/`T1` and a constant
/// into `T2`; bit 1 copies `F_DC` into `C3`; bit 2 stores a scaled `DSCALE`
/// value into `C4`. (2) Three `get1` answers are saved; `put2` runs with
/// literal pairs (5,1), (6,0), (8,0). (3) Eight floats are copied from
/// `ARR` into a frame array. (4) A divisor from `polldiv` (scripted nonzero)
/// divides `DIVIDEND`; a nonzero remainder skips phase 5. (5) Five `poll`
/// answers steer float selects with UNSIGNED `<= 1` compares, `rand0`
/// answers scale into colors, and one arm converts through x87: truncate to
/// i64 with round-toward-zero, take the low 32 bits as unsigned via the
/// `{0, 2^32}` table, narrow to f32. Out-of-range, infinite and NaN inputs
/// take the x87 indefinite path (low word 0). (6) An object method `m3` runs
/// with `a0` (or `ARG0_DEF` when null). (7) Two `m6` calls take frame
/// buffers: the float array (input-only) and a two-word out-buffer. A scale
/// from `SHAMT` (`0x80 << (cl & 31)`, wrapping shift) feeds the second
/// buffer's setup. (8) A sum of three squares normalizes `T0..T2` by
/// `K / sqrt(s)` with an exact `s == 0.0` skip (`-0.0` skips, NaN and
/// negatives take the square-root path, matching `ucomiss`+`lahf`+`test`).
/// (9) `t3` gates the draw block on its answer equaling 1: `t1`, `n2`, then
/// four `draw7` calls, each packing one color word from `C0..C4` scaled by
/// 255 with truncation toward zero (NaN and out-of-i32-range give byte 0,
/// matching `cvttss2si`+`movzx`). (10) Tail calls, a 0/1 toggle of `TOGGLE`,
/// and `put2` with the saved `get1` answers.
///
/// Edge cases: every `poll` compare is unsigned (a signed rewrite of any of
/// them is the honesty mutant); the divisor is never zero by contract; one
/// frame word is read uninitialized (defined fill 0, used as 0.0); two draw
/// temporaries feed dead stores and are not reproduced.
lf_checker_rt::export!(cdecl, rw_00adc280(a0: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x15932f0;
        const PHASE: u32 = 0x1550e00;
        const T0: u32 = 0x15932e0;
        const T1: u32 = 0x15932e4;
        const T2: u32 = 0x15932e8;
        const T3W: u32 = 0x15932ec;
        const C0: u32 = 0x15932f4;
        const C1: u32 = 0x15932f8;
        const C2: u32 = 0x15932fc;
        const C4: u32 = 0x1593300;
        const DCELL: u32 = 0x15932dc;
        const F_DC: u32 = 0x103f4dc;
        const DSCALE: u32 = 0x12ddeb4;
        const DIVIDEND: u32 = 0x103f4cc;
        const ARR: u32 = 0x1550e5c;
        const OBJ: u32 = 0x1550e84;
        const ARG0_DEF: u32 = 0x1550dfc;
        const M3_ARG1: u32 = 0x154e2bc;
        const M6A_ARG1: u32 = 0x154e2f4;
        const M6B_ARG1: u32 = 0x154e2fc;
        const TOGGLE: u32 = 0x154e2f8;
        const T3_ARG2: u32 = 0x1550e8c;
        const FLAG_51C: u32 = 0x103f51c;
        const SHAMT: u32 = 0x1160eb4;
        const X87_IN: u32 = 0x11735bc;
        const X87_TAB: u32 = 0xfe8f50;
        const K_4D4: u32 = 0x103f4d4;
        const K_518: u32 = 0x103f518;
        const K_520: u32 = 0x103f520;
        const K_A: u32 = 0x103f4d8;
        const K_EA6D40: u32 = 0xea6d40;
        const K_8684: u32 = 0xfe8684;
        const K_870C: u32 = 0xfe870c;
        const K_8728: u32 = 0xfe8728;
        const K_87D0: u32 = 0xfe87d0;
        const K_87E4: u32 = 0xfe87e4;
        const K_8830: u32 = 0xfe8830;
        const K_888C: u32 = 0xfe888c;
        const K_88E8: u32 = 0xfe88e8;
        const K_8978: u32 = 0xfe8978;
        const K_8C08: u32 = 0xfe8c08;
        const K_8C0C: u32 = 0xfe8c0c;
        const K_8C58: u32 = 0xfe8c58;
        const K_8D7C: u32 = 0xfe8d7c;
        const CAL_SIN: u32 = 1;
        const CAL_COS: u32 = 2;
        const CAL_GET1A: u32 = 3;
        const CAL_GET1B: u32 = 4;
        const CAL_GET1C: u32 = 5;
        const CAL_PUT2: u32 = 6;
        const CAL_POLLDIV: u32 = 7;
        const CAL_POLLA: u32 = 8;
        const CAL_POLLB: u32 = 9;
        const CAL_POLLC: u32 = 10;
        const CAL_POLLD: u32 = 11;
        const CAL_POLLE: u32 = 12;
        const CAL_RAND: u32 = 13;
        const CAL_M3: u32 = 14;
        const CAL_M6A: u32 = 15;
        const CAL_M6B: u32 = 16;
        const CAL_T3: u32 = 17;
        const CAL_T1: u32 = 18;
        const CAL_N2: u32 = 19;
        const CAL_DRAW: u32 = 20;
        const CAL_Z0A: u32 = 21;
        const CAL_Z0B: u32 = 22;
        const CAL_Z0C: u32 = 23;
        const CAL_COOKIE: u32 = 24;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
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
        unsafe fn g32(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn gf(file_va: u32) -> f32 {
            unsafe { f32::from_bits(g32(file_va)) }
        }
        #[inline(always)]
        unsafe fn gbyte(file_va: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn set(file_va: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(file_va), v) }
        }
        #[inline(always)]
        unsafe fn setf(file_va: u32, v: f32) {
            unsafe { set(file_va, v.to_bits()) }
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
        /// `cvttss2si` low byte: truncate toward zero; NaN and values whose
        /// truncation does not fit i32 give the indefinite 0x80000000, low
        /// byte 0. (Rust `as` saturates instead, hence the explicit guard.)
        #[inline(always)]
        fn trunc_byte(x: f32) -> u8 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0
            } else {
                (x as i32) as u8
            }
        }
        /// The x87 block: `fistp` truncates toward zero to i64, or stores the
        /// indefinite 0x8000000000000000 for NaN, infinities and
        /// out-of-range values; the low 32 bits are then read as unsigned
        /// through the `{0, 2^32}` table.
        #[inline(always)]
        unsafe fn x87_conv(x: f32) -> (f64, u32) {
            unsafe {
                let q: i64 = if x.is_nan()
                    || x >= 9223372036854775808.0
                    || x < -9223372036854775808.0
                {
                    i64::MIN
                } else {
                    x as i64
                };
                let low = q as u32;
                let tab = f64::from_bits(rd64(
                    lf_checker_rt::relocated(X87_TAB)
                        .wrapping_add((low >> 31) * 8),
                ));
                ((low as i32) as f64 + tab, (q as u64 >> 32) as u32)
            }
        }
        #[inline(always)]
        unsafe fn sin(x: f32) -> f32 {
            unsafe {
                f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_SIN, u32, x.to_bits()))
            }
        }
        #[inline(always)]
        unsafe fn cos(x: f32) -> f32 {
            unsafe {
                f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_COS, u32, x.to_bits()))
            }
        }
        /// One packed color word: four table floats scaled and truncated,
        /// first byte on top.
        #[inline(always)]
        unsafe fn pack_color(scale: f32) -> u32 {
            unsafe {
                let b0 = trunc_byte(mul(gf(C4), scale));
                let b1 = trunc_byte(mul(gf(C0), scale));
                let b2 = trunc_byte(mul(gf(C1), scale));
                let b3 = trunc_byte(mul(gf(C2), scale));
                ((b0 as u32) << 24) | ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32)
            }
        }
        #[inline(always)]
        unsafe fn draw7(c0: u32, c1: u32, c2: u32, color: u32) {
            unsafe {
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    CAL_DRAW, u32, c2, c1, c0, g32(T0), g32(T1), g32(T2), color
                );
            }
        }

        // Phase 1: flag-gated fills.
        let mut flags = g32(FLAGS);
        if flags & 1 == 0 {
            let x = gf(PHASE);
            flags |= 1;
            set(FLAGS, flags);
            setf(T0, sin(x));
            setf(T1, cos(x));
            set(T2, 0xbdcccccd);
        }
        if flags & 2 == 0 {
            let v = gf(F_DC);
            flags |= 2;
            set(FLAGS, flags);
            setf(C2, v);
        }
        if flags & 4 == 0 {
            let v = mul(gf(DSCALE), gf(K_888C));
            flags |= 4;
            set(FLAGS, flags);
            let v = add(v, gf(K_87E4));
            let v = mul(v, gf(K_A));
            setf(C4, v);
        }
        // Phase 2: get/put prologue (literals, not the saved answers).
        let g5: u32 = lf_checker_rt::callee_cdecl!(CAL_GET1A, u32, 5);
        let g6: u32 = lf_checker_rt::callee_cdecl!(CAL_GET1B, u32, 6);
        let g8: u32 = lf_checker_rt::callee_cdecl!(CAL_GET1C, u32, 8);
        let _: u32 = lf_checker_rt::callee_cdecl!(CAL_PUT2, u32, 5, 1);
        let _: u32 = lf_checker_rt::callee_cdecl!(CAL_PUT2, u32, 6, 0);
        let _: u32 = lf_checker_rt::callee_cdecl!(CAL_PUT2, u32, 8, 0);
        // Phase 3: frame array.
        let mut farr = [0u32; 8];
        for i in 0..8u32 {
            farr[i as usize] = g32(ARR.wrapping_add(i * 4));
        }
        // Phase 4: divide and branch.
        let pd: u32 = lf_checker_rt::callee_cdecl!(CAL_POLLDIV, u32,);
        let esi = g32(DIVIDEND);
        let rem = esi % pd;
        let mut join_xmm0: f32;
        // High word of the x87 store (E+12 at the m6b call; 0 if no fistp ran).
        let mut m6b_hi = 0u32;
        if rem == 0 {
            // Phase 5: the long fall-through middle.
            let pa: u32 = lf_checker_rt::callee_cdecl!(CAL_POLLA, u32,);
            let mut sweep: f32;
            if pa <= 1 {
                let a = gf(K_4D4);
                let t0 = mul(a, gf(K_8D7C));
                let t1 = mul(a, gf(K_8830));
                let r: u32 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,);
                let mut d = sub(t1, t0);
                let f = mul((r as i32) as f32, gf(K_8684));
                d = mul(d, f);
                d = add(d, t0);
                d = add(d, gf(PHASE));
                d = mul(d, gf(K_8728));
                sweep = d;
            } else {
                sweep = gf(K_8978);
            }
            setf(T0, sin(sweep));
            setf(T1, cos(sweep));
            set(T2, 0xbdcccccd);
            set(T3W, 0);
            let pb: u32 = lf_checker_rt::callee_cdecl!(CAL_POLLB, u32,);
            let e4a: f32 = if pb <= 1 { gf(F_DC) } else { gf(K_87D0) };
            let pc: u32 = lf_checker_rt::callee_cdecl!(CAL_POLLC, u32,);
            let e8a: f32 = if pc <= 1 {
                mul(add(mul(gf(DSCALE), gf(K_888C)), gf(K_87E4)), gf(K_A))
            } else {
                gf(K_EA6D40)
            };
            let pdd: u32 = lf_checker_rt::callee_cdecl!(CAL_POLLD, u32,);
            if pdd <= 1 {
                let r: u32 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,);
                setf(C0, mul((r as i32) as f32, gf(K_8684)));
                let r2: u32 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,);
                setf(C1, mul((r2 as i32) as f32, gf(K_8684)));
            } else {
                set(C0, 0x3f000000);
                set(C1, 0x3f000000);
            }
            setf(C2, e4a);
            setf(C4, mul(e8a, gf(K_870C)));
            let pe: u32 = lf_checker_rt::callee_cdecl!(CAL_POLLE, u32,);
            if pe <= 1 {
                let (d, hi) = x87_conv(mul(gf(X87_IN), gf(K_8C58)));
                m6b_hi = hi;
                setf(DCELL, mul(d as f32, gf(K_520)));
                join_xmm0 = gf(DCELL);
            } else {
                setf(DCELL, mul(gf(K_520), gf(K_518)));
                join_xmm0 = gf(DCELL);
            }
        } else {
            join_xmm0 = gf(DCELL);
        }
        // Join.
        if gbyte(FLAG_51C) != 0 {
            farr[7] = join_xmm0.to_bits();
        }
        let obj = g32(OBJ);
        let obj18 = rd32(obj.wrapping_add(0x18));
        let m3a2 = if a0 != 0 { a0 } else { g32(ARG0_DEF) };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_M3, u32, obj18, obj.wrapping_add(0x14), g32(M3_ARG1), m3a2
        );
        // m6a over the frame array (input-only).
        let arr_ptr = farr.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_M6A, u32, obj18, obj.wrapping_add(0x14), g32(M6A_ARG1), arr_ptr, 4, 8, 2
        );
        // m6b setup.
        let cl = (g32(SHAMT) & 0xff) as u8;
        let scaled = (0x80u32.wrapping_shl(cl as u32) as i32) as f32;
        let inv = div(gf(K_88E8), scaled);
        let mut m6b_buf = [0u32; 2];
        m6b_buf[0] = mul(inv, gf(K_8C0C)).to_bits();
        m6b_buf[1] = m6b_hi;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            CAL_M6B, u32, obj18, obj.wrapping_add(0x14), g32(M6B_ARG1),
            m6b_buf.as_mut_ptr() as u32, 4, 1, 2
        );
        // Post: e4 chain, sumsq normalize.
        let mut e4 = mul(inv, gf(K_8830));
        let e0 = gf(T0);
        let e1 = gf(T1);
        let e2 = gf(T2);
        let mut sumsq = mul(e0, e0);
        sumsq = add(sumsq, mul(e1, e1));
        sumsq = add(sumsq, mul(e2, e2));
        let k: f32 = if sumsq == 0.0 {
            0.0
        } else {
            div(gf(K_88E8), sumsq.sqrt())
        };
        setf(T0, mul(k, e0));
        setf(T1, mul(k, e1));
        setf(T2, mul(k, e2));
        let t3: u32 = lf_checker_rt::callee_thiscall!(
            CAL_T3, u32, obj, 0, 1, g32(T3_ARG2)
        );
        if t3 == 1 {
            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_T1, u32, obj, 0);
            let _: u32 = lf_checker_rt::callee_cdecl!(CAL_N2, u32, 4, 4);
            let scale = gf(K_8C08);
            draw7(0x3f800000, 0x3f800000, 0xbf800000, pack_color(scale));
            let e8b = add(e4, gf(K_88E8));
            draw7(0x3f800000, 0xbf800000, 0xbf800000, pack_color(scale));
            e4 = add(e4, gf(K_88E8));
            let _ = e8b;
            draw7(0x3f800000, 0x3f800000, 0x3f800000, pack_color(scale));
            draw7(0x3f800000, 0xbf800000, 0x3f800000, pack_color(scale));
            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_Z0A, u32, obj);
            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_Z0B, u32, obj);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_Z0C, u32, obj);
        // Tail.
        let tog = g32(TOGGLE);
        set(TOGGLE, 1u32.wrapping_sub(tog));
        let _: u32 = lf_checker_rt::callee_cdecl!(CAL_PUT2, u32, 5, g5);
        let _: u32 = lf_checker_rt::callee_cdecl!(CAL_PUT2, u32, 6, g6);
        let last: u32 = lf_checker_rt::callee_cdecl!(CAL_PUT2, u32, 8, g8);
        let _: u32 = lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
        last
    }
});
