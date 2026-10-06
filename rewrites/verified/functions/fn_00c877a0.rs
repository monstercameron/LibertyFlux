// original: 0x00c877a0 audio_triangle_area (proposed)
///
/// Area of the triangle A-B-C from SIMD cross-product length: `tri` points
/// at three packed points (A at +0x00, B at +0x10, C at +0x20, each three
/// floats x, y, z; the words at +0x0c, +0x1c, +0x2c are padding and are
/// never read). Computes u = B - A, v = C - A, then |u x v| * 0.5 with
/// the component order (cy, cx, cz): cx = uy*vz - uz*vy,
/// cy = uz*vx - vz*ux, cz = vy*ux - vx*uy, sum of squares accumulated as
/// cy*cy + cx*cx + cz*cz, square root, times the file constant 0.5.
/// Every operation is pinned to the original's operand order (NaN payloads
/// included). Returns the float on the x87 stack. Cdecl, one argument.
/// (The original spills the result over its own incoming argument slot;
/// that store is the return value, not extra behaviour.)

lf_checker_rt::export!(cdecl, rw_00c877a0(tri: u32) -> f32 {
    unsafe {
    #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
    #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits(rd32(a)) }
    }
    #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
    #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
    #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        const HALF: f32 = 0.5;
        let ax = rdf(tri.wrapping_add(0x00));
        let ay = rdf(tri.wrapping_add(0x04));
        let az = rdf(tri.wrapping_add(0x08));
        let ux = fsub(rdf(tri.wrapping_add(0x10)), ax);
        let uy = fsub(rdf(tri.wrapping_add(0x14)), ay);
        let uz = fsub(rdf(tri.wrapping_add(0x18)), az);
        let vx = fsub(rdf(tri.wrapping_add(0x20)), ax);
        let vy = fsub(rdf(tri.wrapping_add(0x24)), ay);
        let vz = fsub(rdf(tri.wrapping_add(0x28)), az);
        let cx = fsub(fmul(uy, vz), fmul(uz, vy));
        let cy = fsub(fmul(uz, vx), fmul(vz, ux));
        let cz = fsub(fmul(vy, ux), fmul(vx, uy));
        let sum = fadd(fadd(fmul(cy, cy), fmul(cx, cx)), fmul(cz, cz));
        fmul(core::hint::black_box(sum).sqrt(), HALF)
    }
});
