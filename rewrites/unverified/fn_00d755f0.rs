// original: 0x00d755f0 ui_ring_emit_7tap (proposed)

/// Emit one input sample plus seven cosine/sine taps through a sink callee,
/// repeating the whole sweep four times with flipped gain signs.
///
/// `vec` points at two floats (base levels); `gain_cos` / `gain_sin` are the
/// per-tap gain scales; `color` points at a dword whose four bytes are scaled
/// by a reciprocal pair and packed into one word. The sweep runs an outer
/// pass `o` in 0..4 and an inner tap `i` in 0..7:
///
/// * tap angle: `i * (pi/2) * (1/6)` on odd passes, `(6 - i) * (pi/2) * (1/6)`
///   on even passes, evaluated low-lane SSE in that multiply order;
/// * each angle goes through the cosine callee then the sine callee (both
///   take and return their value in `xmm0`), and the tap pair is
///   `(vec[0] + cos * cos_scale[o], vec[1] + sin * sin_scale[o])`, where the
///   scales are `(gain_cos, gain_sin)` with the signs for pass `o` flipped by
///   xoring in the sign bit (`o=1` flips cosine, `o=2` flips sine, `o=3`
///   flips both);
/// * the sink callee then receives the base pair followed by the seven taps,
///   each call carrying `(f0, f1, 0, 0, 0, -1.0, packed)`; the packed word is
///   the four color bytes each mapped through `byte * (1/255) * 255`,
///   truncated toward zero (`cvttss2si`, whose out-of-range result keeps low
///   byte `0x00`) and packed top byte first.
///
/// A two-call lock prologue (second call takes `1` and the negative-or-zero
/// part of a global flag) precedes the sweep, a two-word setup call precedes
/// each pass's sink calls, and a no-argument flush call ends each pass; the
/// function's return value is the last flush call's answer. Every callee
/// answer except the trigonometric pair and the final flush value is ignored.
///
/// Original: 0x00d755f0 (stdcall, four stack words: pointer, float, float,
/// pointer; returns `eax`).
lf_checker_rt::export!(stdcall, rw_00d755f0(vec: u32, gain_cos: u32, gain_sin: u32, color: u32) -> u32 {
    unsafe {
        const LOCK_A: u32 = 1;
        const LOCK_B: u32 = 2;
        const COS: u32 = 3;
        const SIN: u32 = 4;
        const SETUP: u32 = 5;
        const EMIT: u32 = 6;
        const FLUSH: u32 = 7;
        const COOKIE: u32 = 8;
        const SIGN: u32 = 0x8000_0000;
        const NEG_ONE_BITS: u32 = 0xbf80_0000;
        const GLOBAL_FLAG: u32 = 0x006B310 + 0x1000000;

        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn negate_bits(v: f32) -> f32 {
            f32::from_bits(v.to_bits() ^ SIGN)
        }
        /// Low byte of `cvttss2si`: truncate toward zero; NaN, infinities and
        /// out-of-range values yield the indefinite `0x80000000` (low `0x00`).
        #[inline(always)]
        fn cvtt_low8(x: f32) -> u32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0
            } else {
                (x as i32) as u8 as u32
            }
        }
        #[inline(always)]
        unsafe fn emit(f0: u32, f1: u32, packed: u32) {
            unsafe {
                lf_checker_rt::callee_cdecl!(EMIT, u32, f0, f1, 0u32, 0u32, 0u32, NEG_ONE_BITS, packed);
            }
        }

        // .rdata reciprocal pairs, read the way the original reads them.
        let half_pi = f32::from_bits(*lf_checker_rt::global::<u32>(0x00FE8978));
        let sixth = f32::from_bits(*lf_checker_rt::global::<u32>(0x00FE87C4));
        let inv_255 = f32::from_bits(*lf_checker_rt::global::<u32>(0x00FE86E8));
        let scale_255 = f32::from_bits(*lf_checker_rt::global::<u32>(0x00FE8C08));

        lf_checker_rt::callee_cdecl!(LOCK_A, u32, 0u32);
        let flag = *lf_checker_rt::global::<u32>(GLOBAL_FLAG) as i32;
        let sel = if flag < 0 { flag as u32 } else { 0 };
        lf_checker_rt::callee_cdecl!(LOCK_B, u32, 1u32, sel);

        let v0 = f32::from_bits((vec as *const u32).read_unaligned());
        let v1 = f32::from_bits(((vec + 4) as *const u32).read_unaligned());
        let gc = f32::from_bits(gain_cos);
        let gs = f32::from_bits(gain_sin);
        let color_word = (color as *const u32).read_unaligned();

        let mut answer = 0u32;
        for o in 0..4u32 {
            let cos_scale = if o & 1 != 0 { negate_bits(gc) } else { gc };
            let sin_scale = if o & 2 != 0 { negate_bits(gs) } else { gs };
            let odd = o & 1 != 0;
            let mut taps = [(0u32, 0u32); 7];
            for i in 0..7u32 {
                let idx = if odd { i } else { 6 - i };
                let angle = mul(mul(idx as f32, half_pi), sixth);
                let cosv = f32::from_bits(lf_checker_rt::callee_cdecl!(COS, u32, angle.to_bits()));
                let t = mul(cosv, cos_scale);
                let sinv = f32::from_bits(lf_checker_rt::callee_cdecl!(SIN, u32, angle.to_bits()));
                let p_cos = add(v0, t);
                let p_sin = add(v1, mul(sinv, sin_scale));
                taps[i as usize] = (p_cos.to_bits(), p_sin.to_bits());
            }
            lf_checker_rt::callee_cdecl!(SETUP, u32, 5u32, 8u32);
            let mut packed = 0u32;
            for k in 0..4u32 {
                let b = ((color_word >> (24 - k * 8)) & 0xff) as f32;
                packed = (packed << 8) | cvtt_low8(mul(mul(b, inv_255), scale_255));
            }
            emit(v0.to_bits(), v1.to_bits(), packed);
            for i in 0..7u32 {
                let (f0, f1) = taps[i as usize];
                emit(f0, f1, packed);
            }
            answer = lf_checker_rt::callee_cdecl!(FLUSH, u32,);
        }
        lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        answer
    }
});
