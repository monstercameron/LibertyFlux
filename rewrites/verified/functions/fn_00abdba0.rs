// original: 0x00ABDBA0 vec_light_kernel (proposed)

/// Probe a lighting/visibility term for one object against one vector set.
///
/// Arguments (cdecl, four stack words, plain `ret`):
/// `obj` points to the object (floats at `+0x20`..`+0x28`, `+0x30`..`+0x40`,
/// `+0x54`, `+0x5c`; an integer tag at `+0x44`), `vec` to six floats at
/// `+0x00`..`+0x18`, `out4` to four output words, `out1` to one output word.
/// Returns a float on the x87 stack (`fld` of the low word, `fldz` on the
/// early exits).
///
/// Algorithm. First half: `half = 0.5`; `cx = half*(vec[0x14]+vec[0x04])`,
/// `cy = half*(vec[0x00]+vec[0x10])`, `cz = half*(vec[0x18]+vec[0x08])`,
/// minus `obj[0x24]`, `obj[0x20]`, `obj[0x28]` respectively;
/// `len = sqrt(cx*cx+cy*cy+cz*cz)`; `inv = 1.0/len`; scale the centroid by
/// `inv` into `out4[0]`, `out4[1]`, `out4[2]`; `out1[0] = len`.
/// `out4[3]` is whatever word sits in the original's uninitialised scratch
/// (see below). Second half: `d = (half*diffs) squared and summed`,
/// `s = sqrt(d)`; exit with 0 when not (`obj[0x54] > 4.0`) or when not
/// (`obj[0x54]+s > len`). Otherwise `w = obj[0x30..0x40]` dotted with the
/// constant triple `(0.2125, 0.7154, 0.0721)`; call helper 1
/// (cdecl, two float words: `len`, `s+10.0`) which answers a float on the
/// x87 stack; exit through the tail when not (`len > 0`), not
/// (`answer > 0`), `obj[0x44] != 2` (equality; signedness immaterial) or not
/// (`obj[0x5c] > 0.01`). Otherwise call helper 2 (thiscall: scratch struct
/// in `ecx`, one float word, `obj+0x20`, `obj`) with `0.0` when
/// `0.0 > obj[0x5c]` (unreachable: `obj[0x5c] > 0.01` held to get here),
/// else `obj[0x5c]` clamped above at `1.0`; the tail returns
/// `answer * scale` where `scale` is `1.0` on the call path and the dotted
/// `w` on the skipped paths. All `comiss` + branch pairs are ordered
/// comparisons with NaN taking the false/exit side, modelled as `!(a > b)`.
/// The `sqrtps` the original uses for `s` only feeds its low lane onward,
/// which equals `sqrtss`; the upper lanes are never observed.
///
/// Edge cases: zero `len` divides to infinities; NaN anywhere in the first
/// half propagates into `len`/`inv`/outputs and takes every later exit;
/// infinities compare/arithmetic per IEEE-754, bit-exact with the original.
///
/// Fidelity note: `out4[3]` (`[a2+0xc]`) is read by the original from its
/// own stack scratch that it never wrote. Its value in the game is whatever
/// the stack held; under the checker (defined zero fill) it is `+0.0`, which
/// this rewrite emits. The comparison therefore covers that word only up to
/// the fill, disclosed in the proof record.
///
/// Original: 0x00ABDBA0 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00abdba0(obj: u32, vec: u32, out4: u32, out1: u32) -> f32 {
    unsafe {
        const C_HALF: u32 = 0x00FE8830; // 0.5
        const C_ONE: u32 = 0x00FE88E8; // 1.0
        const C_FOUR: u32 = 0x00FE8AB8; // 4.0
        const C_TINY: u32 = 0x00FE870C; // 0.01
        const C_TEN: u32 = 0x00FE8B08; // 10.0
        const C_W0: u32 = 0x0103EF00; // 0.2125
        const C_W1: u32 = 0x0103EF04; // 0.7154
        const C_W2: u32 = 0x0103EF08; // 0.0721
        const HELPER_F32: u32 = 1;
        const HELPER_STRUCT: u32 = 2;

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
        #[inline(always)]
        unsafe fn g(v: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::global::<u32>(v)).read_unaligned()) }
        }

        let half = g(C_HALF);
        let one = g(C_ONE);

        // Centroid of the vector set, relative to the object's anchor.
        let mut cx = add(rdf(vec.wrapping_add(0x14)), rdf(vec.wrapping_add(0x04)));
        let mut cy = add(rdf(vec), rdf(vec.wrapping_add(0x10)));
        let mut cz = add(rdf(vec.wrapping_add(0x18)), rdf(vec.wrapping_add(0x08)));
        cx = mul(cx, half);
        cy = mul(cy, half);
        cx = sub(cx, rdf(obj.wrapping_add(0x24)));
        cy = sub(cy, rdf(obj.wrapping_add(0x20)));
        cz = mul(cz, half);
        let mut x1 = mul(cx, cx);
        cz = sub(cz, rdf(obj.wrapping_add(0x28)));
        let mut x0 = mul(cy, cy);
        let x7 = one;
        let mut x2 = x7;
        x1 = add(x1, x0);
        x0 = mul(cz, cz);
        x1 = add(x1, x0);
        let len = x1.sqrt();
        x2 = div(x2, len);
        x0 = x2;
        x1 = x2;
        x0 = mul(x0, cx);
        let spill_len = len;
        x2 = mul(x2, cz);
        wrf(out4.wrapping_add(4), x0);
        x0 = 0.0; // original reads uninitialised scratch here; fill is 0
        wrf(out4.wrapping_add(8), x2);
        x1 = mul(x1, cy);
        wrf(out4.wrapping_add(0x0c), x0);
        wrf(out4, x1);
        wrf(out1, spill_len);

        // Spread of the vector set, then the two range gates.
        x1 = sub(rdf(vec.wrapping_add(0x10)), rdf(vec));
        x2 = sub(rdf(vec.wrapping_add(0x14)), rdf(vec.wrapping_add(0x04)));
        x0 = sub(rdf(vec.wrapping_add(0x18)), rdf(vec.wrapping_add(0x08)));
        x1 = mul(x1, half);
        x2 = mul(x2, half);
        x1 = mul(x1, x1);
        x2 = mul(x2, x2);
        x0 = mul(x0, half);
        x2 = add(x2, x1);
        x0 = mul(x0, x0);
        x2 = add(x2, x0);
        x0 = rdf(obj.wrapping_add(0x54));
        let s = x2.sqrt();
        if !(x0 > g(C_FOUR)) {
            return 0.0;
        }
        x0 = add(x0, s);
        if !(x0 > spill_len) {
            return 0.0;
        }

        // Luminance-style weighting, then the float helper call.
        x0 = mul(g(C_W0), rdf(obj.wrapping_add(0x30)));
        let mut x3 = rdf(obj.wrapping_add(0x34));
        x3 = mul(x3, g(C_W1));
        x1 = add(s, g(C_TEN));
        x3 = add(x3, x0);
        x0 = rdf(obj.wrapping_add(0x38));
        x0 = mul(x0, g(C_W2));
        let arg_len = spill_len;
        let arg_sum = x1;
        x3 = add(x3, x0);
        x3 = mul(x3, rdf(obj.wrapping_add(0x40)));
        let ans: f32 =
            lf_checker_rt::callee_cdecl!(HELPER_F32, f32, arg_len.to_bits(), arg_sum.to_bits());
        x2 = ans;
        let zero = 0.0f32;
        if !(spill_len > zero) {
            return mul(x2, x3);
        }
        if !(x2 > zero) {
            return mul(x2, x3);
        }
        if rd32(obj.wrapping_add(0x44)) != 2 {
            return mul(x2, x3);
        }
        x1 = rdf(obj.wrapping_add(0x5c));
        if !(x1 > g(C_TINY)) {
            return mul(x2, x3);
        }
        let x0sel: f32;
        if zero > x1 {
            x0sel = zero;
        } else if !(x1 > x7) {
            x0sel = x1;
        } else {
            x0sel = x7;
        }
        let mut frame = [0u32; 11];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            HELPER_STRUCT,
            u32,
            frame.as_mut_ptr() as u32,
            x0sel.to_bits(),
            obj.wrapping_add(0x20),
            obj
        );
        x3 = one;
        x2 = ans;
        mul(x2, x3)
    }
});
