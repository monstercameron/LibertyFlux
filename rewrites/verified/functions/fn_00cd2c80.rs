// original: 0x00CD2C80 aim_cone_test

/// Cone/aim test between a probe point from a callee and two direction sets.
///
/// Arguments (cdecl, five stack words): `p1` points at a record whose
/// `[+0x20]` slot holds a center triple at `[+0x30/+0x34/+0x38]`, `p2` is the
/// subject record, `flag` selects the gated path by its low byte, `arg3` is
/// a cosine-like threshold and `arg4` a radius, both as float bits. Returns
/// 0 or 1 in AL.
///
/// A callee writes a probe triple to a stack slot; `d` is the probe minus
/// the center. The normalizer is 0 when the squared length is +0.0/-0.0 and
/// otherwise 1.0 over its square root (NaN passes through); `n` is `d`
/// scaled by it, computed lane by lane in the original's order.
///
/// When the flag byte is nonzero, a direction triple is loaded from the
/// `[p2+0x20]` record (`[+0x14/+0x10/+0x18]`, crossed against `n` as
/// `(v0*n1 + v1*n0) + v2*n2`); the threshold gate returns 0 unless the dot
/// is strictly above the threshold for a non-negative threshold, or the
/// threshold strictly above the dot for a negative one (NaN on either side
/// returns 0). A null `[p2+0x20]` would take a sine/cosine fallback whose
/// callees consume their float in XMM0 with no stack argument, which the
/// checker cannot stub; the contract always provides the record.
///
/// The tail reads selector bits `([p2+0x28] >> 6) & 0xf`: values 2, 3 or 4
/// call the `[p2]` vtable slot +0xEC for a triple, else the triple is zero.
/// A squared radius strictly above the triple's squared length (ordered)
/// returns 0, else the same threshold gate on the second dot product
/// becomes the 0/1 return value.

lf_checker_rt::export!(cdecl, rw_00cd2c80(p1: u32, p2: u32, flag: u32, arg3: u32, arg4: u32) -> u32 {
    unsafe {
        const C_ONE: u32 = 0xFE88E8;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 { unsafe { rd32(lf_checker_rt::relocated(va)) } }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 { core::hint::black_box(a) * core::hint::black_box(b) }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 { core::hint::black_box(a) - core::hint::black_box(b) }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 { core::hint::black_box(a) + core::hint::black_box(b) }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 { core::hint::black_box(a) / core::hint::black_box(b) }
        let cs = rd32(p1.wrapping_add(0x20));
        let c0 = f32::from_bits(rd32(cs.wrapping_add(0x30)));
        let c1 = f32::from_bits(rd32(cs.wrapping_add(0x34)));
        let c2 = f32::from_bits(rd32(cs.wrapping_add(0x38)));
        let mut out = [0u32; 3];
        lf_checker_rt::callee_cdecl!(1, u32, out.as_mut_ptr() as u32, p2);
        let o0 = f32::from_bits(out[0]);
        let o1 = f32::from_bits(out[1]);
        let o2 = f32::from_bits(out[2]);
        let d0 = fsub(o0, c0);
        let d1 = fsub(o1, c1);
        let d2 = fsub(o2, c2);
        let l2 = fadd(fadd(fmul(d0, d0), fmul(d1, d1)), fmul(d2, d2));
        let cone = f32::from_bits(g32(C_ONE));
        let inv = if l2 == 0.0 { 0.0f32 } else { fdiv(cone, core::hint::black_box(l2).sqrt()) };
        let n0 = fmul(d0, inv);
        let n1 = fmul(inv, d1);
        let n2 = fmul(d2, inv);
        let a3 = f32::from_bits(arg3);
        if (flag & 0xff) != 0 {
            let vs = rd32(p2.wrapping_add(0x20));
            let v1 = f32::from_bits(rd32(vs.wrapping_add(0x10)));
            let v0 = f32::from_bits(rd32(vs.wrapping_add(0x14)));
            let v2 = f32::from_bits(rd32(vs.wrapping_add(0x18)));
            let dot = fadd(fadd(fmul(v0, n1), fmul(v1, n0)), fmul(v2, n2));
            let pass = if a3.is_nan() || a3 >= 0.0 { dot > a3 } else { a3 > dot };
            if !pass { return 0; }
        }
        let sel = (rd32(p2.wrapping_add(0x28)) >> 6) & 0xf;
        let (w0, w1, w2);
        if sel == 2 || sel == 3 || sel == 4 {
            let vt = rd32(p2);
            let _ = vt;
            let mut tmp = [0u32; 3];
            let rp = lf_checker_rt::callee_thiscall!(2, u32, p2, tmp.as_mut_ptr() as u32);
            w0 = f32::from_bits(rd32(rp));
            w1 = f32::from_bits(rd32(rp.wrapping_add(4)));
            w2 = f32::from_bits(rd32(rp.wrapping_add(8)));
        } else {
            w0 = 0.0f32; w1 = 0.0f32; w2 = 0.0f32;
        }
        let wl2 = fadd(fadd(fmul(w1, w1), fmul(w0, w0)), fmul(w2, w2));
        let a4 = f32::from_bits(arg4);
        if fmul(a4, a4) > wl2 { return 0; }
        let dot2 = fadd(fadd(fmul(w1, n1), fmul(w0, n0)), fmul(w2, n2));
        let r = if a3.is_nan() || a3 >= 0.0 { dot2 > a3 } else { a3 > dot2 };
        if r { 1 } else { 0 }
    }
});

// original: 0x00cd2c80 (in-crate draft; shipped file has the spec header)
