// original: 0x0069B8A0 rage::crAnimChannelStaticVector3::compress

/// Compresses a static Vector3 channel when all samples match the first.
///
/// `this` holds the destination slot at `[this+8]`; the stack arguments are
/// the source array (16-byte records: `u32, f32, f32, u32`), the signed
/// record `count` and the `tolerance` as `f32` bits. Record 0 is always
/// copied. When `count > 1` (signed), each further record's componentwise
/// distance from record 0 is squared and the largest square must not exceed
/// `tolerance * tolerance`, compared with `comiss`+`ja` semantics (unordered
/// counts as not-above, matching `f32`'s `>`). Returns 1 in `al` when every
/// record passes (or `count <= 1`), else 0. Call-free.
///
/// Original: 0x0069B8A0 (thiscall, three stack words, callee pops 12).
lf_checker_rt::export!(thiscall, rw_0069B8A0(this: u32, src: u32, count: i32, tolbits: u32) -> u32 {
    unsafe {
        const SLOT_OFF: u32 = 8;
        const REC_SIZE: u32 = 0x10;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
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
        let dst = rd32(this + SLOT_OFF);
        let x0 = f32::from_bits(rd32(src));
        let y0 = f32::from_bits(rd32(src.wrapping_add(4)));
        let z0 = f32::from_bits(rd32(src.wrapping_add(8)));
        unsafe {
            ((dst) as *mut u32).write_unaligned(rd32(src));
            ((dst + 4) as *mut u32).write_unaligned(rd32(src + 4));
            ((dst + 8) as *mut u32).write_unaligned(rd32(src + 8));
            ((dst + 12) as *mut u32).write_unaligned(rd32(src + 12));
        }
        if count <= 1 {
            return 1;
        }
        let tol = f32::from_bits(tolbits);
        let limit = fmul(tol, tol);
        let mut i: i32 = 1;
        while i < count {
            let rec = src.wrapping_add((i as u32).wrapping_mul(REC_SIZE));
            let dx = fsub(x0, f32::from_bits(rd32(rec)));
            let dy = fsub(y0, f32::from_bits(rd32(rec.wrapping_add(4))));
            let dz = fsub(z0, f32::from_bits(rd32(rec.wrapping_add(8))));
            let mut m = fmul(dx, dx);
            let my = fmul(dy, dy);
            if !(m > my) {
                m = my;
            }
            let mz = fmul(dz, dz);
            if !(m > mz) {
                m = mz;
            }
            if m > limit {
                return 0;
            }
            i += 1;
        }
        1
    }
});
