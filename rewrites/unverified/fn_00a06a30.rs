// original: 0x00a06a30 NativeImpl_SET_OBJECT_INITIAL_ROTATION_VELOCITY (native)
/// Give an object its initial rotation velocity from Euler rates.
///
/// Scales the three rate words by the shared time step (the data global
/// times the read-only constant) and folds each through the object's 3x3
/// inertia basis at object+0x20, in the original's SSE order (pinned with
/// black_box so the compiler cannot reassociate): out0 is
/// (m10*sy + m0*sx) + m20*sz, out1 (m14*sy + m4*sx) + m24*sz and out2
/// (m18*sy + m8*sx) + m28*sz. The vector is applied by the callee; its
/// answer is returned. Cdecl, four words.
lf_checker_rt::export!(cdecl, rw_00a06a30(handle: u32, sx: u32, sy: u32, sz: u32) -> u32 {
    unsafe {
        const OBJ_POOL: u32 = 0x01632c60;
        const STEP_GLOBAL: u32 = 0x011735bc;
        const STEP_CONST: u32 = 0x00fe8b68;
        const BASIS_OFF: u32 = 0x20;
        const LOOKUP: u32 = 0;
        const APPLY: u32 = 1;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        let pool = (lf_checker_rt::global::<u32>(OBJ_POOL) as *const u32).read_unaligned();
        let step_g = f32::from_bits(
            (lf_checker_rt::global::<u32>(STEP_GLOBAL) as *const u32).read_unaligned());
        let step_c = f32::from_bits(
            (lf_checker_rt::relocated(STEP_CONST) as *const u32).read_unaligned());
        let base = mul(step_g, step_c);
        let fx = f32::from_bits(sx);
        let fy = f32::from_bits(sy);
        let fz = f32::from_bits(sz);
        let s2 = mul(base, fx);
        let s3 = mul(base, fy);
        let s4 = mul(base, fz);
        let obj = lf_checker_rt::callee_thiscall!(LOOKUP, u32, pool, handle);
        let m = ((obj + BASIS_OFF) as *const u32).read_unaligned();
        let t0 = mul(rdf(m), s2);
        let mut t4 = mul(rdf(m + 0x10), s3);
        let mut t2 = mul(rdf(m + 0x14), s3);
        t4 = add(t4, t0);
        let t0b = mul(rdf(m + 0x20), s4);
        let mut t1 = mul(rdf(m + 0x18), s3);
        let out0 = add(t4, t0b);
        let q0 = mul(rdf(m + 4), s2);
        t2 = add(t2, q0);
        let q1 = mul(rdf(m + 0x24), s4);
        let out1 = add(t2, q1);
        let r0 = mul(rdf(m + 8), s2);
        t1 = add(t1, r0);
        let r1 = mul(rdf(m + 0x28), s4);
        let out2 = add(t1, r1);
        let vel = [out0.to_bits(), out1.to_bits(), out2.to_bits()];
        lf_checker_rt::callee_cdecl!(APPLY, u32, vel.as_ptr() as u32)
    }
});
