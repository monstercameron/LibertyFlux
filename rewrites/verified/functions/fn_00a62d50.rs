// original: 0x00a62d50 ped_task_pose_blend (proposed)

/// Blend the two pose vectors of a ped task object into one output.
///
/// Calls callee 0 (thiscall on `this`, three out-pointers: two 4-float
/// slots and one dword slot) to fill two vectors `a`, `b` and a dword `c`,
/// then, by flag bit 2 of `this+0x28`:
/// - set: copy path. `out1 = a` (bitwise) and `out2[0] = c * c` with `c`
///   read as a float.
/// - clear: blend path. `out1 = [avg(b0, a0), avg(b1, a1), avg(b2, a2),
///   b3]` with `avg(x, y) = 0.5 * (x + y)` through the constant at
///   `G_HALF`, and `out2[0]` is the sum of squared deviations
///   `(a1-m1)^2 + (a0-m0)^2 + (a2-m2)^2 + (c*0.5)^2` in that
///   accumulation order.
///
/// The slot `a3` and the callee's return value are never read.
///
/// Float operation order is the original's SSE order, pinned through
/// `black_box` helpers. Original: 0x00a62d50 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a62d50(this: u32, out1: u32, out2: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x28;
        const FLAG_COPY: u8 = 0x04;
        const G_HALF: u32 = 0x00fe8830;
        const CALLEE_POSE: u32 = 0;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        let mut a = [0u32; 4];
        let mut b = [0u32; 4];
        let mut c = 0u32;
        lf_checker_rt::callee_thiscall!(
            CALLEE_POSE,
            u32,
            this,
            a.as_mut_ptr() as u32,
            b.as_mut_ptr() as u32,
            &mut c as *mut u32 as u32
        );
        if rd8(this + FLAGS) & FLAG_COPY != 0 {
            wr32(out1, a[0]);
            wr32(out1 + 4, a[1]);
            wr32(out1 + 8, a[2]);
            wr32(out1 + 0x0c, a[3]);
            let cf = f32::from_bits(c);
            wr32(out2, mul(cf, cf).to_bits());
        } else {
            let half = f32::from_bits(rd32(lf_checker_rt::relocated(G_HALF)));
            let a0 = f32::from_bits(a[0]);
            let a1 = f32::from_bits(a[1]);
            let a2 = f32::from_bits(a[2]);
            let b0 = f32::from_bits(b[0]);
            let b1 = f32::from_bits(b[1]);
            let b2 = f32::from_bits(b[2]);
            let cf = f32::from_bits(c);
            let mut s3 = add(b1, a1);
            let mut s4 = add(b0, a0);
            let mut s2 = add(b2, a2);
            wr32(out1 + 0x0c, b[3]);
            let mut s0 = cf;
            s3 = mul(s3, half);
            s4 = mul(s4, half);
            let mut d7 = sub(a1, s3);
            s2 = mul(s2, half);
            let mut d6 = sub(a0, s4);
            s0 = mul(s0, half);
            d7 = mul(d7, d7);
            let mut d5 = sub(a2, s2);
            d6 = mul(d6, d6);
            s0 = mul(s0, s0);
            d7 = add(d7, d6);
            d5 = mul(d5, d5);
            wr32(out1, s4.to_bits());
            wr32(out1 + 4, s3.to_bits());
            d7 = add(d7, d5);
            wr32(out1 + 8, s2.to_bits());
            d7 = add(d7, s0);
            wr32(out2, d7.to_bits());
        }
        out2
    }
});
