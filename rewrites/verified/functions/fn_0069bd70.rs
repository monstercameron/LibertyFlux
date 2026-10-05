// original: 0x0069BD70 rage::crAnimChannelCurveFloat::sample

/// Samples a curve-float channel at time `t` into an output.
///
/// `this` is the channel object (element base at `+8`, element count as an
/// unsigned word at `+0xC`, output scale at `+0x10`, output bias at `+0x14`);
/// the stack arguments are `t` as `f32` bits and the output pointer. Each
/// element is 8 bytes: a `u16` key at `+0`, the segment order byte at `+2`
/// and a coefficient pointer at `+4`. Negative `t` is clamped to zero (NaN
/// is not clamped); the search target is `trunc(t) + 1` with the x87-style
/// indefinite value (`0x80000000`) for NaN and overflow, matching
/// `cvttss2si` where a Rust cast would saturate. The first `count - 1`
/// elements are scanned for the first key at or past the target (signed
/// compare, so a negative target matches the first element); the match is
/// evaluated by the segment callee (callee 1: coefficient in ecx, order in
/// edx, local `t` in xmm0, output on the stack; the stack words are
/// skipped by the xmm0 transport, so the output address is uncompared and
/// listed as narrowed, while the coefficient, order and local `t` compare
/// and the result is scripted). With no match the last element is used:
/// `t` is clamped down to the last key (`min` with NaN mapping to the key),
/// made local by subtracting the previous key, and evaluated inline (copy,
/// linear, quadratic or cubic Horner forms, or a generic loop for higher
/// orders; the linear multiply is `t * c0` as written). Every path finishes
/// with `out = scale * result + bias` (scale first). No return value.
///
/// Original: 0x0069BD70 (thiscall, two stack words, callee pops 8).
lf_checker_rt::export!(thiscall, rw_0069BD70(this: u32, tbits: u32, out: u32) -> u32 {
    unsafe {
        const BASE_OFF: u32 = 8;
        const COUNT_OFF: u32 = 0x0C;
        const SCALE_OFF: u32 = 0x10;
        const BIAS_OFF: u32 = 0x14;
        const ELEM_SIZE: u32 = 8;
        const ORDER_OFF: u32 = 2;
        const COEFF_OFF: u32 = 4;
        const EVAL: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let mut t = f32::from_bits(tbits);
        if t < 0.0 {
            t = 0.0;
        }
        let count = rd16(this + COUNT_OFF);
        let base = rd32(this + BASE_OFF);
        let mut isi: i32 = if t.is_nan() || t > 2147483647.0 {
            i32::MIN
        } else {
            t as i32
        };
        isi = isi.wrapping_add(1);
        let ebx = count.wrapping_sub(1);
        let mut ebp: u32 = 0;
        let mut found: u32 = 0;
        let mut fcoeff: u32 = 0;
        let mut forder: u32 = 0;
        if (ebx as i32) > 0 {
            let mut eax: u32 = 0;
            let mut ecx = base;
            loop {
                let key = rd16(ecx);
                if (key as i32) >= isi {
                    found = 1;
                    forder = unsafe { ((ecx + ORDER_OFF) as *const u8).read() as u32 };
                    fcoeff = rd32(ecx + COEFF_OFF);
                    break;
                }
                eax = eax.wrapping_add(1);
                ecx = ecx.wrapping_add(ELEM_SIZE);
                ebp = key;
                if !((eax as i32) < (ebx as i32)) {
                    break;
                }
            }
        }
        let scale = f32::from_bits(rd32(this + SCALE_OFF));
        let bias = f32::from_bits(rd32(this + BIAS_OFF));
        if found != 0 {
            let tl = fsub(t, ebp as f32);
            let _ = lf_checker_rt::callee_fastcall!(EVAL, u32, fcoeff, forder, out, tl.to_bits());
            let r = f32::from_bits(rd32(out));
            wr32(out, fadd(fmul(scale, r), bias).to_bits());
            return 0;
        }
        let last = base.wrapping_add(count.wrapping_mul(ELEM_SIZE)).wrapping_sub(ELEM_SIZE);
        let lk = rd16(last) as f32;
        if !(lk > t) {
            t = lk;
        }
        let typ = unsafe { ((last + ORDER_OFF) as *const u8).read() as u32 };
        let coeff = rd32(last + COEFF_OFF);
        let tl = fsub(t, ebp as f32);
        if typ == 0 {
            wr32(out, rd32(coeff));
        } else if typ == 1 {
            let r = fadd(fmul(tl, f32::from_bits(rd32(coeff))), f32::from_bits(rd32(coeff + 4)));
            wr32(out, r.to_bits());
        } else if typ == 2 {
            let s1 = fadd(fmul(tl, f32::from_bits(rd32(coeff))), f32::from_bits(rd32(coeff + 4)));
            let r = fadd(fmul(s1, tl), f32::from_bits(rd32(coeff + 8)));
            wr32(out, r.to_bits());
        } else if typ == 3 {
            let s1 = fadd(fmul(tl, f32::from_bits(rd32(coeff))), f32::from_bits(rd32(coeff + 4)));
            let s2 = fadd(fmul(s1, tl), f32::from_bits(rd32(coeff + 8)));
            let r = fadd(fmul(s2, tl), f32::from_bits(rd32(coeff + 12)));
            wr32(out, r.to_bits());
        } else {
            let mut s = f32::from_bits(rd32(coeff));
            let mut p = coeff.wrapping_add(4);
            let mut left = typ;
            while left != 0 {
                s = fmul(s, tl);
                p = p.wrapping_add(4);
                s = fadd(s, f32::from_bits(rd32(p.wrapping_sub(4))));
                left -= 1;
            }
            wr32(out, s.to_bits());
        }
        let r = f32::from_bits(rd32(out));
        wr32(out, fadd(fmul(scale, r), bias).to_bits());
        0
    }
});
