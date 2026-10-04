// original: 0x0096D230 basis_from_target_pos (proposed)

#![allow(unsafe_code)]

/// Rebuild the direction basis of an object from a target point.
///
/// `this` points to an object holding a position at `+0x2f40` (three floats)
/// and three output vector slots: direction at `+0x2f30`, normal at `+0x2f10`
/// and binormal at `+0x2f20`. `target` points to three floats. The function
/// sets the flag byte at `+0x2ef5` to 1, forms `d = pos - target`,
/// normalises it (a zero or denormal-squared length yields a zero vector,
/// while NaN propagates through the normalising path), then builds
/// `t = up x dir` with the global up vector, normalises that the same way,
/// and stores a third vector from the two. Every float operation runs in the
/// original's operand order. The store to `+0x2f3c` replays the original's
/// read of its own uninitialised frame scratch, which the contract defines
/// as zero. Original: 0x0096D230 (thiscall, one stack word; the sole caller
/// ignores EAX, so no value is returned).
lf_checker_rt::export!(thiscall, rw_0096d230(this: u32, target: u32) -> u32 {
    unsafe {
        const POS: u32 = 0x2f40;
        const DIR: u32 = 0x2f30;
        const NRM: u32 = 0x2f10;
        const BIN: u32 = 0x2f20;
        const OUT_W: u32 = 0x2f3c;
        const FLAG: u32 = 0x2ef5;
        const UP: u32 = 0x0110db50;
        const ONE: f32 = 1.0;

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
        #[inline(always)]
        unsafe fn fsqrt(x: f32) -> f32 {
            unsafe {
                use core::arch::x86::{_mm_cvtss_f32, _mm_set_ss, _mm_sqrt_ss};
                _mm_cvtss_f32(_mm_sqrt_ss(_mm_set_ss(core::hint::black_box(x))))
            }
        }

        ((this + FLAG) as *mut u8).write(1);
        let dx = sub(rdf(this + POS), rdf(target));
        let dy = sub(rdf(this + POS + 4), rdf(target + 4));
        let dz = sub(rdf(this + POS + 8), rdf(target + 8));
        wrf(this + OUT_W, 0.0);
        let up = lf_checker_rt::relocated(UP);
        let g0 = rdf(up);
        let g1 = rdf(up + 4);
        let g2 = rdf(up + 8);
        let len2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let inv = if len2 != 0.0 { div(ONE, fsqrt(len2)) } else { 0.0 };
        let dirx = mul(inv, dx);
        let diry = mul(inv, dy);
        let dirz = mul(inv, dz);
        wrf(this + DIR, dirx);
        wrf(this + DIR + 4, diry);
        wrf(this + DIR + 8, dirz);
        let t0 = sub(mul(dirz, g1), mul(diry, g2));
        let t1 = sub(mul(dirx, g2), mul(dirz, g0));
        let t2 = sub(mul(diry, g0), mul(dirx, g1));
        let acc = add(add(mul(t0, t0), mul(t1, t1)), mul(t2, t2));
        let inv2 = if acc != 0.0 { div(ONE, fsqrt(acc)) } else { 0.0 };
        let nx = mul(t0, inv2);
        let ny = mul(t1, inv2);
        let nz = mul(inv2, t2);
        wrf(this + NRM, nx);
        wrf(this + NRM + 4, ny);
        wrf(this + NRM + 8, nz);
        wrf(this + BIN, sub(mul(nz, diry), mul(ny, dirz)));
        wrf(this + BIN + 4, sub(mul(nx, dirz), mul(dirx, nz)));
        wrf(this + BIN + 8, sub(mul(ny, dirx), mul(nx, diry)));
        0
    }
});
