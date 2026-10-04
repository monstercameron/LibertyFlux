// original: 0x00d839d0 aim_steer_solver
use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

/// length guard is "not equal to zero").
export!(cdecl, rw_d839d0(obj_a: u32, obj_b: u32, out1: u32, out2: u32, out3: u32, out_b: u32) -> u32 {

    #[inline(always)]
    fn fadd(a: f32, b: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(a) + core::hint::black_box(b))
    }
    #[inline(always)]
    fn fsub(a: f32, b: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(a) - core::hint::black_box(b))
    }
    #[inline(always)]
    fn fmul(a: f32, b: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(a) * core::hint::black_box(b))
    }
    #[inline(always)]
    fn fdiv(a: f32, b: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(a) / core::hint::black_box(b))
    }
    #[inline(always)]
    fn fsqrt(a: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(a).sqrt())
    }
    #[inline(always)]
    fn fneg(a: f32) -> f32 {
        core::hint::black_box(-core::hint::black_box(a))
    }
    #[inline(always)]
    unsafe fn fr(base: u32, off: u32) -> f32 {
        unsafe { ((base.wrapping_add(off)) as *const f32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn ur(base: u32, off: u32) -> u32 {
        unsafe { ((base.wrapping_add(off)) as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn vcall_0ec(obj: u32, arg: u32) -> u32 {
        unsafe {
            let vt = ur(obj, 0);
            let fptr = ur(vt, 0xEC);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(fptr as usize);
            f(obj, arg)
        }
    }

    const PI_F: f32 = f32::from_bits(0x40490FDB);
    const FRAC_PI_2: f32 = f32::from_bits(0x3FC90FDB);
    const NEG_FRAC_PI_2: f32 = f32::from_bits(0xBFC90FDB);

    unsafe {
        (out1 as *mut u32).write_unaligned(0);
        (out_b as *mut u8).write(0);
        let p = callee_thiscall!(1, u32, obj_a);
        let q = ur(p, 0x20);
        let x = fr(q, 0);
        let y = fr(q, 4);
        let n2 = fadd(fmul(x, x), fmul(y, y));
        let k = if n2 != 0.0 { fdiv(1.0, fsqrt(n2)) } else { 0.0 };
        let sx = fmul(x, k);
        let sy = fmul(y, k);
        let sz = fmul(k, 0.0);
        let p2 = callee_thiscall!(1, u32, obj_a);
        let mut slot = 0u32;
        let slot_ptr = (&mut slot as *mut u32) as u32;
        let f1 = vcall_0ec(p2, slot_ptr);
        let t0 = fr(f1, 0);
        let t1 = fr(f1, 4);
        let t2 = fr(f1, 8);
        let p3 = callee_thiscall!(1, u32, obj_a);
        let r = ur(p3, 0x20);
        let (u0, u1, u2) = if r != 0 {
            (fadd(fr(r, 0x30), t0), fadd(fr(r, 0x34), t1), fadd(fr(r, 0x38), t2))
        } else {
            (fadd(fr(p3, 0x10), t0), fadd(fr(p3, 0x14), t1), fadd(fr(p3, 0x18), t2))
        };
        let sib = ur(obj_b, 0x20);
        let m10 = fr(sib, 0x10);
        let m14 = fr(sib, 0x14);
        let m18 = fr(sib, 0x18);
        let d0 = fsub(fr(sib, 0x30), u0);
        let d1 = fsub(fr(sib, 0x34), u1);
        let d2 = fsub(fr(sib, 0x38), u2);
        let mut w = fadd(fadd(fmul(d0, sx), fmul(d1, sy)), fmul(d2, sz));
        let v = fadd(fadd(fmul(m14, sy), fmul(m10, sx)), fmul(m18, sz));
        if v > 0.0 {
            w = fneg(w);
        }
        let f2 = vcall_0ec(obj_b, slot_ptr);
        let s = fadd(fadd(fmul(m14, fr(f2, 4)), fmul(fr(f2, 0), m10)), fmul(fr(f2, 8), m18));
        let w2 = fsub(w, s);
        if w2 > 0.0 {
            let t = fmul(w2, 0.5);
            let res = if t > 1.0 { 1.0 } else { t };
            (out2 as *mut f32).write_unaligned(res);
            (out3 as *mut u32).write_unaligned(0);
        } else {
            let f3 = vcall_0ec(obj_b, slot_ptr);
            let s3 = fadd(fadd(fmul(fr(f3, 4), m14), fmul(fr(f3, 0), m10)), fmul(fr(f3, 8), m18));
            if 3.0 > s3 {
                let t = fmul(w2, 0.5);
                let res = if -1.0 > t { -1.0 } else { t };
                (out2 as *mut f32).write_unaligned(res);
                (out3 as *mut u32).write_unaligned(0);
            } else {
                let t = fmul(w2, -0.5);
                let res = if t > 1.0 { 1.0 } else { t };
                (out2 as *mut u32).write_unaligned(0);
                (out3 as *mut f32).write_unaligned(res);
            }
        }
        let p4 = callee_thiscall!(1, u32, obj_a);
        let e1 = ur(p4, 0x20);
        let p5 = callee_thiscall!(1, u32, obj_a);
        let e2 = ur(p5, 0x20);
        let h1 = callee_cdecl!(3, f32, ur(e2, 0), ur(e1, 4));
        let h2 = callee_cdecl!(3, f32, ur(sib, 0x10), ur(sib, 0x14));
        let mut d = fsub(h1, h2);
        while NEG_FRAC_PI_2 > d {
            d = fadd(d, PI_F);
        }
        while d > FRAC_PI_2 {
            d = fsub(d, PI_F);
        }
        d = fmul(d, 0.5);
        (out1 as *mut f32).write_unaligned(d);
        let f4 = vcall_0ec(obj_b, slot_ptr);
        let s4 = fadd(fadd(fmul(fr(f4, 4), m14), fmul(fr(f4, 0), m10)), fmul(fr(f4, 8), m18));
        if 0.0 > s4 {
            let cur = (out1 as *const f32).read_unaligned();
            (out1 as *mut f32).write_unaligned(fneg(cur));
        }
        let mut h2s = h2;
        let mut h1s = h1;
        callee_cdecl!(4, u32, obj_b, out1, (&mut h2s as *mut f32) as u32, (&mut h1s as *mut f32) as u32)
    }
});
