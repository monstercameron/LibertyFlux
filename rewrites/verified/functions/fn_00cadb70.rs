// original: 0x00cadb70 advance_toward_point (proposed)
/// Step the position at `+0x30` toward a target point by a scaled stride.
///
/// `this` (ECX) holds the current position at `+0x20`/`+0x24`/`+0x28` and a
/// step length at `+0x50`; `other` points to an object whose dword at `+0x20`
/// points at the target's coordinates (`+0x30`/`+0x34`/`+0x38` there). Computes
/// the displacement `d = target - pos`, its squared length (accumulated as
/// `(dy*dy + dx*dx) + dz*dz`), and a scale of `1.0/sqrt(len2)` (exactly 0 when
/// `len2 == 0`, including negative zero; NaN propagates through the root and
/// division). Writes the stepped position back to `+0x30`/`+0x34`/`+0x38` as
/// `pos + step*scale*d`, with the original's exact per-lane operation order
/// (`s*(scale*dx)`, `(dy*scale)*s`, `(dz*scale)*s`), pinned through `black_box`.
/// Also stores a word at `+0x3C` which the original reads from uninitialized
/// stack; the contract defines that fill as 0 on both sides, so the rewrite
/// stores 0 (see the narrowed note). Original is thiscall(`this`, `other`).
lf_checker_rt::export!(thiscall, rw_00cadb70(this: u32, other: u32) -> u32 {
    unsafe {
        const POS: u32 = 0x20;
        const STEP: u32 = 0x50;
        const OUT: u32 = 0x30;
        const OTHER_VEC_PTR: u32 = 0x20;
        const VEC: u32 = 0x30;
        const ONE: u32 = 0x00FE_88E8;
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn dw(base: u32, off: u32) -> u32 {
            unsafe { ((base.wrapping_add(off)) as *const u32).read_unaligned() }
        }
        let px = f32::from_bits(dw(this, POS));
        let py = f32::from_bits(dw(this, POS.wrapping_add(4)));
        let pz = f32::from_bits(dw(this, POS.wrapping_add(8)));
        let step = f32::from_bits(dw(this, STEP));
        let vec = dw(other, OTHER_VEC_PTR);
        let tx = f32::from_bits(dw(vec, VEC));
        let ty = f32::from_bits(dw(vec, VEC.wrapping_add(4)));
        let tz = f32::from_bits(dw(vec, VEC.wrapping_add(8)));
        let dx = sub(tx, px);
        let dy = sub(ty, py);
        let dz = sub(tz, pz);
        let len2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let scale = if len2 == 0.0 {
            0.0f32
        } else {
            let one = f32::from_bits(
                (lf_checker_rt::global::<u32>(ONE) as *const u32).read());
            let root = core::hint::black_box(len2).sqrt();
            core::hint::black_box(one) / core::hint::black_box(root)
        };
        let ox = add(px, mul(step, mul(scale, dx)));
        let oy = add(py, mul(mul(dy, scale), step));
        let oz = add(pz, mul(mul(dz, scale), step));
        ((this.wrapping_add(OUT)) as *mut u32).write_unaligned(ox.to_bits());
        ((this.wrapping_add(OUT.wrapping_add(4))) as *mut u32).write_unaligned(oy.to_bits());
        ((this.wrapping_add(OUT.wrapping_add(8))) as *mut u32).write_unaligned(oz.to_bits());
        ((this.wrapping_add(OUT.wrapping_add(12))) as *mut u32).write_unaligned(0);
        0
    }
});
