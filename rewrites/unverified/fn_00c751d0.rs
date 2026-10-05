// original: 0x00c751d0 ped_task_build_quant_table (proposed)

/// Build a scaled, quantized parameter table from three floats.
///
/// Stdcall with three float arguments (`a0`, `a1`, `a2` as bits) returning the
/// built object. A `0xf0`-byte object is allocated and constructed, configured
/// through a five-argument callee, then filled from the arguments: the low
/// lane of `a0` xored with a sign constant seeds one bound, `a1` times two
/// constants seeds two more, and sums and differences with `a2` fill the
/// object's bound words at `+0x10..0x28` (the incoming `a1` slot is clobbered
/// with the scaled `a1` on the way). A refresh callee and one vtable call
/// (slot `+0x28`, handed `a2`) run before the arithmetic below.
///
/// Four repetitions of one clamp-scale-quantize step then write twelve
/// 16-bit words into the buffer at `[obj+0xb0]`: each step clamps the three
/// base words between a lower bound (rotating per step over `a0`, the xored
/// `a0`, zero, the scaled-`a1` slot and the first scaled product) and the
/// upper words at `+0x10..0x18`, subtracts offsets at `+0xa0..0xa8`,
/// multiplies by scales formed as one constant over divisors at
/// `+0x90..0x98`, adds a bias constant and truncates toward zero. The lower
/// bounds per step for the three lanes are (`a0`, 0, first product),
/// (xored `a0`, 0, first product), (`a0`, 0, scaled `a1`) and
/// (xored `a0`, 0, scaled `a1`).
///
/// Two quirks are reproduced exactly. The fourth step's middle scale is read
/// from a frame slot that is never written, so it is zero under the checker's
/// zero stack fill (and indeterminate stack garbage in a live game). The
/// minimum/maximum selections are single-precision comparisons where an
/// unordered (NaN) pair keeps the first operand for the upper clamp and takes
/// the second for the lower clamp; the truncation follows the hardware
/// convert instruction, yielding `0x80000000` (low word zero) for NaN and
/// out-of-range values rather than saturating.
///
/// The tail issues four six-argument configuration calls carrying the object,
/// `a0`, small integers, a truncation result or prior-call leftover in one
/// slot, and pointers to sign-flipped constant triples (one triple's middle
/// word is the same never-written frame slot, hence zero), then a two-argument
/// vtable call (slot `+0x60`), a one-argument lookup callee handed a data
/// address, a data-table call reached through a global pointer with a constant
/// address and -1, and a final one-argument call through slot `+0x60` carrying
/// that call's answer.
///
/// Edge cases: a null allocator or constructor answer leaves the object null
/// and the first bound store faults, exactly as the original does; a zero
/// divisor yields infinities that flow into the truncation as zeros.
///
/// Original: 0x00c751d0 (stdcall, three stack words, returns the object).
lf_checker_rt::export!(stdcall, rw_00C751D0(a0b: u32, a1b: u32, a2b: u32) -> u32 {
    unsafe {
        const NEW: u32 = 1;
        const CTOR: u32 = 2;
        const CONFIG: u32 = 3;
        const REFRESH: u32 = 4;
        const VCALL_ARG: u32 = 5;
        const TAIL_FIRST: u32 = 6;
        const TAIL_REST: u32 = 7;
        const VCALL_DISPATCH: u32 = 8;
        const LOOKUP: u32 = 9;
        const _TABLE_CALL: u32 = 10;
        const VT_SLOT_ARG: u32 = 0x28;
        const VT_SLOT_DISPATCH: u32 = 0x60;
        const OFF_VTABLE: u32 = 0x00;
        const OFF_HI0: u32 = 0x10;
        const OFF_HI1: u32 = 0x14;
        const OFF_HI2: u32 = 0x18;
        const OFF_BASE0: u32 = 0x20;
        const OFF_BASE1: u32 = 0x24;
        const OFF_BASE2: u32 = 0x28;
        const OFF_DIV0: u32 = 0x90;
        const OFF_DIV1: u32 = 0x94;
        const OFF_DIV2: u32 = 0x98;
        const OFF_SUB0: u32 = 0xa0;
        const OFF_SUB1: u32 = 0xa4;
        const OFF_SUB2: u32 = 0xa8;
        const OFF_WORDS: u32 = 0xb0;
        const K_NEG: u32 = 0x00fe_8fa0;
        const K_A1B: u32 = 0x00fe_8d7c;
        const K_BIAS: u32 = 0x00fe_8830;
        const K_SCALE: u32 = 0x00fe_88e8;
        const C_LOW: u32 = 0x0110_db50;
        const C_HIGH: u32 = 0x0110_db70;
        const GLOBAL_TABLE: u32 = 0x018b_8968;
        const LOOKUP_ADDR: u32 = 0x01b4_b2a0;
        const TABLE_ARG: u32 = 0x00ed_3b94;
        const NONE: u32 = 0xffff_ffff;
        const I32_LIM: f32 = 2147483648.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        /// Lower clamp: `max(p, q)` with the comparison's NaN rule (an
        /// unordered pair takes `q`).
        #[inline(always)]
        fn fmax_sel(mut p: f32, q: f32) -> f32 {
            if !(p > q) {
                p = q;
            }
            p
        }
        /// Upper clamp: `min(p, q)` with the comparison's NaN rule (an
        /// unordered pair keeps `p`).
        #[inline(always)]
        fn fmin_sel(mut p: f32, q: f32) -> f32 {
            if p > q {
                p = q;
            }
            p
        }
        /// Truncate toward zero exactly like the hardware convert
        /// instruction: NaN and out-of-range values yield `0x80000000`.
        #[inline(always)]
        fn cvtt_i32(x: f32) -> i32 {
            if !(x < I32_LIM) || x < -I32_LIM {
                i32::MIN
            } else {
                x as i32
            }
        }
        #[inline(always)]
        unsafe fn wr_word(buf: u32, at: u32, v: u16) {
            unsafe { ((buf.wrapping_add(at)) as *mut u16).write_unaligned(v) }
        }

        let a0 = f32::from_bits(a0b);
        let a1 = f32::from_bits(a1b);
        let a2 = f32::from_bits(a2b);
        let k_neg = rd32(lf_checker_rt::relocated(K_NEG));
        let k_a1b = rdf(lf_checker_rt::relocated(K_A1B));
        let k_bias = rdf(lf_checker_rt::relocated(K_BIAS));
        let k_scale = rdf(lf_checker_rt::relocated(K_SCALE));

        // Object allocation, configuration, bound derivation.
        let mem: u32 = lf_checker_rt::callee_cdecl!(NEW, u32, 0xf0u32);
        let edi = if mem == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(CTOR, u32, mem)
        };
        lf_checker_rt::callee_thiscall!(CONFIG, u32, edi, 4u32, 1u32, 0u32, 4u32, 0u32);
        let g8 = f32::from_bits(a0b ^ k_neg);
        let prod_b = fmul(a1, k_a1b);
        let a1slot = fmul(a1, k_bias);
        let d0hi = fadd(a0, a2);
        let d2lo_base = fsub(prod_b, a2);
        wrf(edi.wrapping_add(OFF_BASE0), fsub(g8, a2));
        wrf(edi.wrapping_add(OFF_BASE1), fsub(g8, a2));
        wrf(edi.wrapping_add(OFF_BASE2), d2lo_base);
        wrf(edi.wrapping_add(OFF_HI0), d0hi);
        wrf(edi.wrapping_add(OFF_HI1), d0hi);
        wrf(edi.wrapping_add(OFF_HI2), fadd(a1slot, a2));
        lf_checker_rt::callee_thiscall!(REFRESH, u32, edi);
        let vtable = rd32(edi.wrapping_add(OFF_VTABLE));
        let arg_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_SLOT_ARG)) as usize);
        arg_fn(edi, a2b);

        // Shared scale and offset terms.
        let s0 = fdiv(k_scale, rdf(edi.wrapping_add(OFF_DIV0)));
        let s1 = fdiv(k_scale, rdf(edi.wrapping_add(OFF_DIV1)));
        let s2 = fdiv(k_scale, rdf(edi.wrapping_add(OFF_DIV2)));
        let o0 = rdf(edi.wrapping_add(OFF_SUB0));
        let o1 = rdf(edi.wrapping_add(OFF_SUB1));
        let o2 = rdf(edi.wrapping_add(OFF_SUB2));
        let hi0 = rdf(edi.wrapping_add(OFF_HI0));
        let hi1 = rdf(edi.wrapping_add(OFF_HI1));
        let hi2 = rdf(edi.wrapping_add(OFF_HI2));
        let base0 = rdf(edi.wrapping_add(OFF_BASE0));
        let base1 = rdf(edi.wrapping_add(OFF_BASE1));
        let base2 = rdf(edi.wrapping_add(OFF_BASE2));
        let words = rd32(edi.wrapping_add(OFF_WORDS));
        // One clamp-scale-quantize step over the three lanes.
        macro_rules! step {
            ($lo0:expr, $lo1:expr, $lo2:expr, $sc1:expr, $at:expr) => {{
                let c0 = fmin_sel(fmax_sel(base0, $lo0), hi0);
                let c1 = fmin_sel(fmax_sel(base1, $lo1), hi1);
                let c2 = fmin_sel(fmax_sel(base2, $lo2), hi2);
                let v0 = fadd(fmul(s0, fsub(c0, o0)), k_bias);
                let v1 = fadd(fmul($sc1, fsub(c1, o1)), k_bias);
                let v2 = fadd(fmul(s2, fsub(c2, o2)), k_bias);
                wr_word(words, $at, cvtt_i32(v0) as u16);
                wr_word(words, $at + 2, cvtt_i32(v1) as u16);
                wr_word(words, $at + 4, cvtt_i32(v2) as u16);
            }};
        }
        step!(a0, 0.0f32, prod_b, s1, 0);
        step!(g8, 0.0f32, prod_b, s1, 6);
        step!(a0, 0.0f32, a1slot, s1, 0x0c);
        // Fourth step: the middle scale reads a never-written frame slot.
        step!(g8, 0.0f32, a1slot, 0.0f32, 0x12);
        let tail_cvtt = cvtt_i32(fadd(fmul(s2, fsub(fmin_sel(fmax_sel(base2, a1slot), hi2), o2)), k_bias));

        // Tail configuration calls with sign-flipped constant triples.
        let c50 = lf_checker_rt::relocated(C_LOW);
        let c70 = lf_checker_rt::relocated(C_HIGH);
        let t1 = [
            rd32(c70) ^ k_neg,
            rd32(c70.wrapping_add(4)) ^ k_neg,
            rd32(c70.wrapping_add(8)) ^ k_neg,
        ];
        lf_checker_rt::callee_thiscall!(
            TAIL_FIRST, u32, edi, a0b, 1u32, 0u32, tail_cvtt as u32,
            t1.as_ptr() as u32, c50
        );
        let t2a = [
            rd32(c50) ^ k_neg,
            rd32(c50.wrapping_add(4)) ^ k_neg,
            rd32(c50.wrapping_add(8)) ^ k_neg,
        ];
        // Middle word of the second triple is the never-written frame slot.
        let t2b = [rd32(c70) ^ k_neg, 0u32, rd32(c70.wrapping_add(8)) ^ k_neg];
        lf_checker_rt::callee_thiscall!(
            TAIL_REST, u32, edi, a0b, 0u32, 1u32, 0u32,
            t2b.as_ptr() as u32, t2a.as_ptr() as u32
        );
        let t3 = [
            rd32(c50) ^ k_neg,
            rd32(c50.wrapping_add(4)) ^ k_neg,
            rd32(c50.wrapping_add(8)) ^ k_neg,
        ];
        lf_checker_rt::callee_thiscall!(
            TAIL_REST, u32, edi, a0b, 2u32, 3u32, 0u32, c70,
            t3.as_ptr() as u32
        );
        lf_checker_rt::callee_thiscall!(
            TAIL_REST, u32, edi, a0b, 3u32, 2u32, 0u32, c70, c50
        );

        // Dispatch tail: vtable, lookup, data-table and final calls.
        let disp_fn: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable.wrapping_add(VT_SLOT_DISPATCH)) as usize);
        disp_fn(edi, 0, NONE);
        lf_checker_rt::callee_thiscall!(LOOKUP, u32, edi, lf_checker_rt::relocated(LOOKUP_ADDR));
        let gt = rd32(lf_checker_rt::relocated(GLOBAL_TABLE));
        let tab_fn: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(gt).wrapping_add(0x18)) as usize);
        let answer = tab_fn(gt, lf_checker_rt::relocated(TABLE_ARG), NONE);
        disp_fn(edi, answer, 0);
        edi
    }
});
