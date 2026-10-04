// original: 0x00c65560 CCutsceneObject::vf27

/// Bounding box of a cutscene object, transformed into world space.
///
/// `this` points to the cutscene object. Two corner triples are read from it
/// (`CORNER_AX..AZ` at `+0x2f0` and `CORNER_BX..BZ` at `+0x300`), and a world
/// matrix is produced by callee 1 (called with `this + 0x10` in ECX and a
/// pointer to a 16-word scratch buffer) whose 16 words are scripted by the
/// contract. Three scale factors come from globals (`SCALE_X/Y/Z`), plus a
/// bit mask (`ABS_MASK_G`) applied to nine of the matrix words.
///
/// The two corners are added and subtracted pairwise and scaled, giving a
/// center triple (`px_sum`, `py_sum`, `pz_sum`) and an extent triple
/// (`px_dif`, `py_dif`, `pz_dif`). The center is pushed through the matrix
/// (words 0,1,2 / 4,5,6 / 8,9,10 with translations 12,13,14); the extent is
/// pushed through the masked (absolute) matrix the same way. `out` receives
/// eight words: center-minus-extent, matrix word 3, center-plus-extent,
/// matrix word 7. Straight-line code: no branches, no loops.
///
/// Original: thiscall, one stack word (the out pointer), returns it in EAX.
/// Float operation order is the original's, pinned through `black_box`.
lf_checker_rt::export!(thiscall, rw_00c65560(this: u32, out: u32) -> u32 {
    unsafe {
        const CORNER_AX: u32 = 0x2f0;
        const CORNER_AY: u32 = 0x2f4;
        const CORNER_AZ: u32 = 0x2f8;
        const CORNER_BX: u32 = 0x300;
        const CORNER_BY: u32 = 0x304;
        const CORNER_BZ: u32 = 0x308;
        const CALLEE_THIS_OFF: u32 = 0x10;
        const MATRIX_CALLEE: u32 = 1;
        const SCALE_X_G: u32 = 0x0110_db60;
        const SCALE_Y_G: u32 = 0x0110_db64;
        const SCALE_Z_G: u32 = 0x0110_db68;
        const ABS_MASK_G: u32 = 0x00fe_8f80;

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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        fn masked(word: u32, mask: u32) -> f32 {
            f32::from_bits(word & mask)
        }

        let ax = rdf(this + CORNER_AX);
        let ay = rdf(this + CORNER_AY);
        let az = rdf(this + CORNER_AZ);
        let bx = rdf(this + CORNER_BX);
        let by = rdf(this + CORNER_BY);
        let bz = rdf(this + CORNER_BZ);

        let mut buf = [0u32; 16];
        lf_checker_rt::callee_thiscall!(
            MATRIX_CALLEE,
            u32,
            this.wrapping_add(CALLEE_THIS_OFF),
            core::ptr::addr_of_mut!(buf) as u32
        );
        let m = |i: usize| f32::from_bits(buf[i]);

        let gx = f32::from_bits(rd32(lf_checker_rt::global::<u32>(SCALE_X_G) as u32));
        let gy = f32::from_bits(rd32(lf_checker_rt::global::<u32>(SCALE_Y_G) as u32));
        let gz = f32::from_bits(rd32(lf_checker_rt::global::<u32>(SCALE_Z_G) as u32));
        let mask = rd32(lf_checker_rt::global::<u32>(ABS_MASK_G) as u32);

        // Scaled corner sums (center) and differences (extent).
        let px_sum = mul(gx, add(bx, ax));
        let pz_sum = mul(add(bz, az), gz);
        let py_sum = mul(gy, add(by, ay));
        let px_dif = mul(sub(bx, ax), gx);
        let pz_dif = mul(sub(bz, az), gz);
        let py_dif = mul(sub(by, ay), gy);

        // Masked matrix words.
        let ab0 = masked(buf[0], mask);
        let ab1 = masked(buf[1], mask);
        let ab2 = masked(buf[2], mask);
        let ab4 = masked(buf[4], mask);
        let ab5 = masked(buf[5], mask);
        let ab6 = masked(buf[6], mask);
        let ab8 = masked(buf[8], mask);
        let ab9 = masked(buf[9], mask);
        let ab10 = masked(buf[10], mask);

        // Center through the matrix.
        let b0_px = mul(m(0), px_sum);
        let b4_py = mul(m(4), py_sum);
        let b1_px = mul(m(1), px_sum);
        let cx_a = add(b4_py, b0_px);
        let b8_pz = mul(m(8), pz_sum);
        let b2_px = mul(m(2), px_sum);
        let cx_b = add(cx_a, b8_pz);
        let b9_pz = mul(m(9), pz_sum);
        let b10_pz = mul(m(10), pz_sum);
        let cx = add(cx_b, m(12));

        let b5_py = mul(m(5), py_sum);
        let cy_a = add(b5_py, b1_px);
        let cy_b = add(cy_a, b9_pz);
        let b6_py = mul(m(6), py_sum);
        let cy = add(cy_b, m(13));

        let cz_a = add(b6_py, b2_px);
        let cz_b = add(cz_a, b10_pz);
        let cz = add(cz_b, m(14));

        // Extent through the masked matrix.
        let ab0_px = mul(ab0, px_dif);
        let ab1_py = mul(ab1, py_dif);
        let ex_a = add(ab1_py, ab0_px);
        let ab2_pz = mul(ab2, pz_dif);
        let ex = add(ex_a, ab2_pz);

        let ab5_py = mul(ab5, py_dif);
        let ab4_px = mul(ab4, px_dif);
        let ey_a = add(ab5_py, ab4_px);
        let ab6_pz = mul(ab6, pz_dif);
        let ey = add(ey_a, ab6_pz);

        let ab9_py = mul(ab9, py_dif);
        let ab8_px = mul(ab8, px_dif);
        let ez_a = add(ab9_py, ab8_px);
        let ab10_pz = mul(ab10, pz_dif);
        let ez = add(ez_a, ab10_pz);

        wrf(out, sub(cx, ex));
        wrf(out + 4, sub(cy, ey));
        wrf(out + 8, sub(cz, ez));
        wr32(out + 0x0c, buf[3]);
        wrf(out + 0x10, add(cx, ex));
        wrf(out + 0x14, add(cy, ey));
        wrf(out + 0x18, add(cz, ez));
        wr32(out + 0x1c, buf[7]);
        out
    }
});
