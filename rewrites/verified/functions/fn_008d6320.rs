// original: 0x008d6320 basis_from_dir_up
/// Builds an orthonormal-ish basis vector from a direction: normalizes the
/// input vector in place (zero stays zero), notifies the worker, then writes
/// `normalize(n x G) x n` to the output, where G is the global up vector
/// (0,0,1). A zero cross product yields a zero output. Returns the output
/// pointer.
export!(cdecl, rw_008d6320(out: u32, vec: u32) -> u32 {
    unsafe {
        const K_ADDR: u32 = 0x00FE_88E8;
        const GX: u32 = 0x0110_DB70;
        const GY: u32 = 0x0110_DB74;
        let k = *global::<f32>(K_ADDR);
        let gx = *global::<f32>(GX);
        let gy = *global::<f32>(GY);
        let gz = *global::<f32>(GX + 8);
        let v = vec as *mut f32;
        let x = *v;
        let y = *v.add(1);
        let z = *v.add(2);
        // Inverse length, or 0 for a zero vector; NaN length stays NaN.
        // (Matches the original's ucomiss/lahf/test/jp ladder: normalize
        // unless the squared length compares equal to zero.)
        let len2 = fadd(fadd(fmul(x, x), fmul(y, y)), fmul(z, z));
        let s1 = if len2 > 0.0 || len2.is_nan() {
            fdiv(k, fsqrt(len2))
        } else {
            0.0
        };
        let nx = fmul(x, s1);
        let ny = fmul(y, s1);
        let nz = fmul(z, s1);
        *v = nx;
        *v.add(1) = ny;
        *v.add(2) = nz;
        callee_cdecl!(1, u32, vec);
        // t = n x G.
        let t0 = fsub(fmul(ny, gz), fmul(nz, gy));
        let t1 = fsub(fmul(nz, gx), fmul(nx, gz));
        let t2 = fsub(fmul(nx, gy), fmul(ny, gx));
        let sum2 = fadd(fadd(fmul(t1, t1), fmul(t0, t0)), fmul(t2, t2));
        let s2 = if sum2 > 0.0 || sum2.is_nan() {
            fdiv(k, fsqrt(sum2))
        } else {
            0.0
        };
        let u0 = fmul(s2, t0);
        let u1 = fmul(s2, t1);
        let u2 = fmul(s2, t2);
        // out = u x n.
        let o = out as *mut f32;
        *o = fsub(fmul(nz, u1), fmul(ny, u2));
        *o.add(1) = fsub(fmul(nx, u2), fmul(nz, u0));
        *o.add(2) = fsub(fmul(ny, u0), fmul(nx, u1));
        out
    }
});
