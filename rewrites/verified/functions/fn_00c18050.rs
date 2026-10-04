// original: 0x00c18050 cam_vec_filter

/// Filter and normalize a camera vector through two sampled helpers.
///
/// Scales the input scalar, samples two helpers at that point, scales the
/// input vector by the first sample, runs the scaled vector and the stored
/// vector through a bilinear form, then normalizes the result (or zeroes it
/// when its squared length is exactly zero). The two helpers take the scaled
/// point in XMM0 and return a float in XMM0; the contract transports that
/// register explicitly. Returns nothing meaningful.
///
/// NaN note: every multiply and add runs through `fmul`/`fadd`. Plain
/// `a * b` lets LLVM emit `b * a` (identical for finite values, but SSE
/// propagates the first operand's NaN, so a swapped multiply picks the wrong
/// NaN payload when both operands are different NaNs). The helpers keep both
/// operands live past the operation, which stops the backend from reusing a
/// dead operand's register as the destination; with neither reusable it
/// copies the first, preserving the order. Neither `black_box` on the first
/// operand alone nor the `_mm_set_ss`/`_mm_mul_ss`/`_mm_cvtss_f32` idiom
/// holds the order (both degenerate back to a commutable multiply).
/// Subtraction, division and square root are never commuted and stay plain.
export!(cdecl, rw_00c18050(out: u32, inp: u32, scalar_bits: u32) -> u32 {
    #[inline(always)]
    fn fmul(a: f32, b: f32) -> f32 {
        // Both operands stay live past the multiply so the backend cannot
        // reuse a dead operand's register as the destination (which is what
        // swaps the NaN winner); with neither reusable it copies the first.
        let r = core::hint::black_box(a) * core::hint::black_box(b);
        core::hint::black_box(a);
        core::hint::black_box(b);
        r
    }
    #[inline(always)]
    fn fadd(a: f32, b: f32) -> f32 {
        let r = core::hint::black_box(a) + core::hint::black_box(b);
        core::hint::black_box(a);
        core::hint::black_box(b);
        r
    }
    unsafe {
        let k1 = global::<f32>(0x00fe_8830).read();
        let k2 = global::<f32>(0x00fe_8628).read();
        let k3 = global::<f32>(0x00fe_88e8).read();
        // Scaled sample point. (The original also writes it back over the
        // incoming argument slot, a compiler spill the rewrite cannot
        // reproduce; the contract disables the stack check for that word.)
        let scalar = f32::from_bits(scalar_bits);
        let s = fmul(scalar, k1);
        let r1 = f32::from_bits(callee_cdecl!(1, u32, s.to_bits()));
        // Scaled input triple.
        let ix = ((inp + 0) as *const f32).read();
        let iy = ((inp + 4) as *const f32).read();
        let iz = ((inp + 8) as *const f32).read();
        let t0 = fmul(ix, r1);
        let t1 = fmul(iy, r1);
        let t2 = fmul(iz, r1);
        let w = f32::from_bits(callee_cdecl!(2, u32, s.to_bits()));
        // Stored triple.
        let vx = ((out + 0) as *const f32).read();
        let vy = ((out + 4) as *const f32).read();
        let vz = ((out + 8) as *const f32).read();
        // Residual scalar: w*k2 minus the three cross terms, left to right.
        let q1 = fmul(t0, vx);
        let mut h = fmul(w, k2);
        h = h - q1;
        let q2 = fmul(vy, t1);
        h = h - q2;
        let q3 = fmul(t2, vz);
        h = h - q3;
        // First form row.
        let mut p = fmul(w, vx);
        let pk = fmul(t0, k2);
        p = fadd(p, pk);
        let pv = fmul(t1, vz);
        p = fadd(p, pv);
        let pw = fmul(t2, vy);
        p = p - pw;
        let vy_t0 = fmul(vy, t0);
        // Second form row.
        let mut q = fmul(w, vy);
        let qk = fmul(t1, k2);
        q = fadd(q, qk);
        let qx = fmul(t2, vx);
        q = fadd(q, qx);
        let qz = fmul(t0, vz);
        q = q - qz;
        // Third form row.
        let mut u = fmul(w, vz);
        let uk = fmul(t2, k2);
        u = fadd(u, uk);
        u = fadd(u, vy_t0);
        let ux = fmul(t1, vx);
        u = u - ux;
        // Output triple: each row against the scaled triple, left to right.
        let ox0 = fmul(w, p);
        let mut ox = ox0;
        let ox1 = fmul(t0, h);
        ox = ox - ox1;
        let ox2 = fmul(t2, q);
        ox = ox - ox2;
        let ox3 = fmul(t1, u);
        ox = fadd(ox, ox3);
        let t1p = fmul(t1, p);
        let t2h = fmul(t2, h);
        let oy0 = fmul(w, q);
        let mut oy = oy0;
        let oy1 = fmul(t1, h);
        oy = oy - oy1;
        let oy2 = fmul(t0, u);
        oy = oy - oy2;
        let oy3 = fmul(t2, p);
        oy = fadd(oy, oy3);
        let oz0 = fmul(w, u);
        let mut oz = oz0;
        oz = oz - t2h;
        oz = oz - t1p;
        let oz1 = fmul(t0, q);
        oz = fadd(oz, oz1);
        // Normalize, or zero when the squared length is exactly zero.
        let nxx = fmul(ox, ox);
        let mut n2 = fmul(oy, oy);
        n2 = fadd(n2, nxx);
        let nzz = fmul(oz, oz);
        n2 = fadd(n2, nzz);
        let inv = if n2 == 0.0 { 0.0 } else { k3 / n2.sqrt() };
        ((out + 0) as *mut f32).write(fmul(inv, ox));
        ((out + 4) as *mut f32).write(fmul(inv, oy));
        ((out + 8) as *mut f32).write(fmul(inv, oz));
    }
    0
});
