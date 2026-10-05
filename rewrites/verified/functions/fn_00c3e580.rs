// original: 0x00c3e580 suspension_residual_update (proposed)
/// Solve one suspension residual and store the result on the object.
///
/// `this` (ECX) points at a 0x74-byte block; `flag` is a stack word whose
/// low byte selects the path. When the byte is zero the stored travel at
/// `+0x6c` is copied back to `+0x50`, the status byte at `+0x70` is cleared
/// and no call is made. Otherwise, with `r = f32[0x54]`:
/// `d = f32[0x40] - (r*f32[0x10] + f32[0x30])`,
/// `e = f32[0x44] - (f32[0x34] + f32[0x14]*r)`,
/// `f = f32[0x48] - (f32[0x38] + f32[0x18]*r)`,
/// `spill = d*d + e*e + f*f`; helper id 1 is called with
/// `f32[0x50]*0.5*(pi/180)` in XMM0 (its float answer is scripted);
/// stores `f32[0x50]` at `+0x6c`, `sqrt(spill)*answer` at `+0x68` and the
/// flag byte at `+0x70`. The accumulator holds the flag byte on exit but
/// its high bytes are entry garbage, so the function is void.
///
/// Original: 0x00c3e580 (thiscall, one stack word, one call).
lf_checker_rt::export!(thiscall, rw_00c3e580(this: u32, flag: u32) -> u32 {
    unsafe {
        const L1: u32 = 0x3f00_0000; // 0.5 (text const, embedded)
        const L2: u32 = 0x3c8e_fa35; // pi/180 (text const, embedded)
        #[inline(always)]
        unsafe fn rd(p: u32) -> f32 {
            unsafe { f32::from_bits((p as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn w(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        if (flag & 0xFF) == 0 {
            w(this + 0x50, rd(this + 0x6c).to_bits());
            (this as *mut u8).wrapping_add(0x70).write_unaligned(0);
            return 0;
        }
        let r = rd(this + 0x54);
        let t = fadd(fmul(r, rd(this + 0x10)), rd(this + 0x30));
        let x1 = fmul(rd(this + 0x14), r);
        let x0 = fadd(rd(this + 0x34), x1);
        let x2 = fmul(rd(this + 0x18), r);
        let x1b = fadd(rd(this + 0x38), x2);
        let dx = fsub(rd(this + 0x40), t);
        let dy = fsub(rd(this + 0x44), x0);
        let dz = fsub(rd(this + 0x48), x1b);
        let spill = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
        let l1 = f32::from_bits(L1);
        let l2 = f32::from_bits(L2);
        let arg = fmul(fmul(rd(this + 0x50), l1), l2);
        let f1 = f32::from_bits(lf_checker_rt::callee_cdecl!(1, u32, arg.to_bits()));
        let res = fmul(spill.sqrt(), f1);
        w(this + 0x6c, rd(this + 0x50).to_bits());
        w(this + 0x68, res.to_bits());
        (this as *mut u8).wrapping_add(0x70).write_unaligned((flag & 0xFF) as u8);
        0
    }
});
