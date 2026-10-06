// original: 0x009063A0 input_ui_element_from_coords (proposed)

/// Build a UI/overlay element from a 2D coordinate pair and three scalars,
/// through a thread-gated branch.
///
/// Arguments (thiscall, the callee pops 0x14 bytes): `this` in ECX (dword at +4 is compared
/// against a global bound, dword at +0 is forwarded to constructors);
/// `pa` points at two floats (the coordinates); `fc` is a scale float;
/// `f10` selects the geometry path (`!= 0.0`, which is also true for NaN,
/// tested by the original with a lahf/test/jp parity trick); `u14` is an
/// opaque word (copied through a byte shuffle that is the identity);
/// `b18`'s low byte selects whether `fc` is doubled first. Returns in EAX
/// the last helper answer on the path taken (or bound+1 on early exit).
///
/// Algorithm. Entry derives two working floats (`s1c` = `fc` or `fc * 2`,
/// `s20` = `fc`). If the mode byte is set, a setup helper runs and `s1c`
/// is scaled by 0.75; otherwise a threshold compare may call a predicate
/// helper and then SIGNED-compare `[this+4]` against `bound + 1` (`jg`),
/// exiting early with the bound when greater. The `f10 != 0.0` branch
/// selects the geometry path: the zero path expands the coordinates with
/// `s1c`/`s20` into a 16-float table; the nonzero path runs two scalar
/// helpers (one takes its argument in XMM0) and a 4-step rotation loop
/// into 8 output floats. Both paths then read a thread-local struct
/// (TLS index from a global, flag at +0x8CC): when set, small objects are
/// allocated, stamped with the global counter, constructed and sunk (the
/// nonzero path also runs the SIGNED id-fold pair from 0x00901C10 twice);
/// when clear, a fixed helper pair runs and the table/outputs are handed
/// to a wide call (9 or 5 frame-pointer arguments). Every comparison is an
/// equality/nonzero/float-order test except the SIGNED `jg` on `[this+4]`
/// and the SIGNED `% 16`/`/ 16` in the id-fold.
///
/// Edge cases: null allocator answers are sunk as null (zero path) or
/// fault on the vtable load (nonzero path, same fault both sides); a NaN
/// `f10` takes the nonzero path; NaN/infinite table floats propagate
/// bit-exactly through the pinned float order.
///
/// Original: 0x009063A0 (thiscall, five stack words).
unsafe fn body2(signed_jg: bool, this: u32, pa: u32, fc: u32, f10: u32, u14: u32, b18: u32) -> u32 {
    unsafe {
        const G116B: u32 = 0x0116_09F6;
        const G490: u32 = 0x0103_4490;
        const TLSIDX: u32 = 0x017A_BA14;
        const CTR: u32 = 0x0103_27A0;
        const C8A24: u32 = 0x00FE_8A24;
        const C888C: u32 = 0x00FE_888C;
        const C8734: u32 = 0x00FE_8734;
        const VT_BASE: u32 = 0x00E7_E048;
        const VT2: u32 = 0x00E8_4C94;
        const D2: u32 = 0x0059_D890;
        const TLS_FLAG: u32 = 0x8CC;
        const C_NEW: u32 = 1;
        const C_C10: u32 = 2;
        const C_C6: u32 = 3;
        const C_FMB: u32 = 4;
        const C_3220: u32 = 5;
        const C_4C80: u32 = 6;
        const C_4040: u32 = 7;
        const C_45E0: u32 = 8;
        const C_N9E: u32 = 9;
        const C_GS: u32 = 10;
        const C_93AC: u32 = 11;
        const C_94F9: u32 = 12;
        const C_260: u32 = 13;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn glob(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn globf(va: u32) -> f32 {
            unsafe { f32::from_bits(glob(va)) }
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
        unsafe fn vcall(obj: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(8)) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn fold_pair(obj: u32) -> u32 {
            unsafe {
                let r1 = vcall(obj) as i32;
                let a = (16i32.wrapping_sub(r1.wrapping_rem(16))).wrapping_rem(16);
                let r2 = vcall(obj) as i32;
                let q = r2.wrapping_add(a).wrapping_div(16);
                let m = rd32(obj.wrapping_add(4));
                let bits = ((q as u32) << 14 ^ m) & 0x01FF_C000;
                wr32(obj.wrapping_add(4), m ^ bits);
                bits
            }
        }
        #[inline(always)]
        unsafe fn stamp(obj: u32) {
            unsafe {
                wr32(obj, lf_checker_rt::relocated(VT_BASE));
                let m = rd32(obj.wrapping_add(4));
                let ctr = glob(CTR);
                wr32(obj.wrapping_add(4), m ^ ((m ^ ctr) & 0x3FFF));
                (lf_checker_rt::relocated(CTR) as *mut u32).write_unaligned(ctr.wrapping_add(1));
            }
        }

        let fc = f32::from_bits(fc);
        let f10 = f32::from_bits(f10);
        // Entry floats: s20 is always fc; s1c is fc, doubled when b18 == 0.
        let mut s1c = if (b18 as u8) != 0 { fc } else { mul(fc, globf(C8A24)) };
        let s20 = fc;
        let g116 = (lf_checker_rt::relocated(G116B) as *const u8).read();
        if g116 != 0 {
            // The scaled value is stored (and pointed at) BEFORE the call.
            s1c = mul(s1c, globf(C888C));
            lf_checker_rt::callee_cdecl!(C_260, u32, 1, 0, &mut s1c as *mut f32 as u32, 0);
        } else if globf(C8734) > s1c {
            if lf_checker_rt::callee_cdecl!(C_FMB, u32,) as u8 == 0 {
                let bound = glob(G490).wrapping_add(1);
                let this4 = rd32(this.wrapping_add(4));
                let take = if signed_jg {
                    (this4 as i32) > (bound as i32)
                } else {
                    this4 > bound
                };
                if take {
                    lf_checker_rt::callee_cdecl!(C_GS, u32,);
                    return bound;
                }
            }
        }
        let x5 = s1c;
        // The byte shuffle of u14 is the identity (verified by hand).
        let edx = u14;
        if f10 != 0.0 {
            // Nonzero path (NaN included): scalar helpers + rotation loop.
            let c: [f32; 8] = [
                f32::from_bits(0x3F80_0000), f32::from_bits(0xBF80_0000),
                f32::from_bits(0x3F80_0000), f32::from_bits(0x3F80_0000),
                f32::from_bits(0xBF80_0000), f32::from_bits(0x3F80_0000),
                f32::from_bits(0xBF80_0000), f32::from_bits(0xBF80_0000),
            ];
            let x7 = rdf(pa);
            let x4 = f32::from_bits(lf_checker_rt::callee_cdecl!(C_93AC, u32,));
            let x5n = f32::from_bits(lf_checker_rt::callee_cdecl!(C_94F9, u32, f10.to_bits()));
            let x6 = s1c;
            let vec1 = rdf(pa.wrapping_add(4));
            let mut o = [0.0f32; 8];
            let mut edi = 0usize;
            while edi < 4 {
                let x2 = c[2 * edi];
                let x3 = c[2 * edi + 1];
                let mut x1 = mul(x2, x4);
                let mut x0 = mul(x5n, x3);
                x1 = sub(x1, x0);
                x0 = mul(x4, x3);
                x1 = mul(x1, x6);
                x1 = add(x1, x7);
                o[2 * edi] = x1;
                x1 = mul(x5n, x2);
                x1 = add(x1, x0);
                x1 = mul(x1, s20);
                x1 = add(x1, vec1);
                o[2 * edi + 1] = x1;
                edi += 1;
            }
            let ob: [u32; 8] = [o[0].to_bits(), o[1].to_bits(), o[2].to_bits(), o[3].to_bits(),
                                o[4].to_bits(), o[5].to_bits(), o[6].to_bits(), o[7].to_bits()];
            let idx = glob(TLSIDX);
            let flag = rd32(lf_checker_rt::tls_slot(idx as usize).wrapping_add(TLS_FLAG));
            let ret: u32;
            if flag != 0 {
                let o1 = lf_checker_rt::callee_cdecl!(C_NEW, u32, 0x10, 0);
                if o1 != 0 {
                    stamp(o1);
                    wr32(o1, lf_checker_rt::relocated(VT2));
                    wr32(o1.wrapping_add(8), lf_checker_rt::relocated(D2));
                    (o1.wrapping_add(0x0C) as *mut u8).write(1);
                }
                fold_pair(o1);
                let o2 = lf_checker_rt::callee_cdecl!(C_NEW, u32, 0x60, 0);
                if o2 != 0 {
                    let c6 = lf_checker_rt::callee_thiscall!(C_C6, u32, o2,
                        ob.as_ptr().add(0) as u32, ob.as_ptr().add(2) as u32,
                        ob.as_ptr().add(4) as u32, ob.as_ptr().add(6) as u32,
                        edx, rd32(this));
                    ret = fold_pair(c6);
                } else {
                    ret = fold_pair(o2);
                }
            } else {
                lf_checker_rt::callee_cdecl!(C_3220, u32, 7, 1);
                lf_checker_rt::callee_thiscall!(C_4C80, u32, this);
                let mut u14c = edx;
                ret = lf_checker_rt::callee_cdecl!(C_45E0, u32,
                    ob.as_ptr().add(0) as u32, ob.as_ptr().add(6) as u32,
                    ob.as_ptr().add(4) as u32, ob.as_ptr().add(2) as u32,
                    &mut u14c as *mut u32 as u32);
            }
            lf_checker_rt::callee_cdecl!(C_GS, u32,);
            ret
        } else {
            // Zero path: coordinate table, then the thread-gated branch.
            let x4 = rdf(pa);
            let x3 = rdf(pa.wrapping_add(4));
            let x1 = add(x4, x5);
            let x2 = add(x3, s20);
            let x3 = sub(x3, s20);
            let x4 = sub(x4, x5);
            let one = f32::from_bits(0x3F80_0000);
            let t: [u32; 16] = [x1.to_bits(), x2.to_bits(), x1.to_bits(), x3.to_bits(),
                                x4.to_bits(), x2.to_bits(), x4.to_bits(), x3.to_bits(),
                                0, one.to_bits(), 0, 0,
                                one.to_bits(), one.to_bits(), one.to_bits(), 0];
            let idx = glob(TLSIDX);
            let flag = rd32(lf_checker_rt::tls_slot(idx as usize).wrapping_add(TLS_FLAG));
            let ret: u32;
            if flag != 0 {
                let o1 = lf_checker_rt::callee_cdecl!(C_NEW, u32, 0x10, 0);
                if o1 != 0 {
                    stamp(o1);
                    wr32(o1, lf_checker_rt::relocated(VT2));
                    wr32(o1.wrapping_add(8), lf_checker_rt::relocated(D2));
                    (o1.wrapping_add(0x0C) as *mut u8).write(1);
                }
                lf_checker_rt::callee_cdecl!(C_N9E, u32, o1);
                let o2 = lf_checker_rt::callee_cdecl!(C_NEW, u32, 0x50, 0);
                if o2 != 0 {
                    let c10 = lf_checker_rt::callee_thiscall!(C_C10, u32, o2,
                        t.as_ptr().add(0) as u32, t.as_ptr().add(2) as u32,
                        t.as_ptr().add(4) as u32, t.as_ptr().add(6) as u32,
                        t.as_ptr().add(8) as u32, t.as_ptr().add(10) as u32,
                        t.as_ptr().add(12) as u32, t.as_ptr().add(14) as u32,
                        edx, rd32(this));
                    ret = lf_checker_rt::callee_cdecl!(C_N9E, u32, c10);
                } else {
                    ret = lf_checker_rt::callee_cdecl!(C_N9E, u32, 0);
                }
            } else {
                lf_checker_rt::callee_cdecl!(C_3220, u32, 7, 1);
                lf_checker_rt::callee_thiscall!(C_4C80, u32, this);
                let mut s24 = edx;
                ret = lf_checker_rt::callee_cdecl!(C_4040, u32,
                    t.as_ptr().add(0) as u32, t.as_ptr().add(2) as u32,
                    t.as_ptr().add(4) as u32, t.as_ptr().add(6) as u32,
                    t.as_ptr().add(8) as u32, t.as_ptr().add(10) as u32,
                    t.as_ptr().add(12) as u32, t.as_ptr().add(14) as u32,
                    &mut s24 as *mut u32 as u32);
            }
            lf_checker_rt::callee_cdecl!(C_GS, u32,);
            ret
        }
    }
}

lf_checker_rt::export!(thiscall, rw_009063A0(this: u32, pa: u32, fc: u32, f10: u32, u14: u32, b18: u32) -> u32 {
    unsafe { body2(true, this, pa, fc, f10, u14, b18) }
});
