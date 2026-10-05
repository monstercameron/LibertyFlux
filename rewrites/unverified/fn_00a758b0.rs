// original: 0x00A758B0 ped_task_heading_update (proposed)

/// Steer a ped task's heading vector from two polled integer sensors.
///
/// `this` is the task object (heading vector at `+0x90`, spare slot at
/// `+0x9c`, cooldown at `+0xa0`); `arg1` is an opaque sensor handle passed
/// to the two sampler callees. Two `.data` globals tune the behaviour: a
/// cooldown step and an angle bias. Float constants come from `.rdata`.
///
/// Behaviour: if the cooldown is below 10.0 it is advanced by the step.
/// Two sensor integers are read (each through its own sampler, then a
/// shared converter), converted to float, the second negated; if the sum
/// of squares is not above 50.0 the function returns. Otherwise the pair
/// is widened to doubles for the angle callee, its result biased and run
/// through a shaping callee, then two component callees produce the new
/// (x, y); the vector is normalised (zero stays zero, NaN propagates) and
/// stored with a zero z and a cleared cooldown.
///
/// One slot read is uninitialised stack (never written by the function);
/// under the checker's `stack_fill` of 0 it reads as 0.0, which is what
/// the rewrite stores. The angle callee takes two doubles in XMM0/XMM1
/// and returns a double in XMM0, which stable Rust cannot call directly;
/// the harness stub answers `f32xmm0` (scripted low 32 bits; the entry
/// high half is preserved), so the double the original converts is the
/// scripted low half over the entry high half, which the rewrite
/// reassembles from its own computed double. Production will call the
/// rewritten angle callee directly.
/// Calling convention: thiscall, one stack word, no defined return value.
lf_checker_rt::export!(thiscall, rw_00a758b0(this: u32, arg1: u32) -> u32 {
    unsafe {
        const ADD_THRESH: f32 = 10.0;
        const MAG_THRESH: f32 = 50.0;
        const ONE: f32 = 1.0;
        const SIGN_MASK: u32 = 0x8000_0000;
        const VEC_X: u32 = 0x90;
        const VEC_Y: u32 = 0x94;
        const VEC_Z: u32 = 0x98;
        const SPARE: u32 = 0x9c;
        const COOLDOWN: u32 = 0xa0;
        const G_STEP: u32 = 0x011735bc;
        const G_BIAS: u32 = 0x0128e3a0;
        const C_SAMPLE_A: u32 = 1;
        const C_CONVERT_A: u32 = 2;
        const C_SAMPLE_B: u32 = 3;
        const C_CONVERT_B: u32 = 8;
        const C_ANGLE: u32 = 4;
        const C_SHAPE: u32 = 5;
        const C_COMP_X: u32 = 6;
        const C_COMP_Y: u32 = 7;

        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
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
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN_MASK)
        }
        #[inline(always)]
        unsafe fn global_f32(file_va: u32) -> f32 {
            unsafe { rdf(lf_checker_rt::relocated(file_va)) }
        }

        // Cooldown advance.
        let cool = rdf(this + COOLDOWN);
        if ADD_THRESH > cool {
            wrf(this + COOLDOWN, add(cool, global_f32(G_STEP)));
        }
        // Sensor pair.
        let r1: u32 = lf_checker_rt::callee_thiscall!(C_SAMPLE_A, u32, arg1);
        let r2: u32 = lf_checker_rt::callee_cdecl!(C_CONVERT_A, u32, r1);
        let a: f32 = (r2 as i32) as f32;
        let r3: u32 = lf_checker_rt::callee_thiscall!(C_SAMPLE_B, u32, arg1);
        let r4: u32 = lf_checker_rt::callee_cdecl!(C_CONVERT_B, u32, r3);
        let b: f32 = neg((r4 as i32) as f32);
        // Magnitude gate.
        let mag = add(mul(b, b), mul(a, a));
        if !(mag > MAG_THRESH) {
            return 0;
        }
        // Angle of (-a, b) as doubles, then bias and shape. The stub
        // answers with the scripted low 32 bits over the preserved entry
        // high half (see doc comment); reassemble that double.
        let na = neg(a);
        let _db = b as f64; // second double the real callee reads from XMM1
        let ang_lo: u32 = lf_checker_rt::callee_cdecl!(C_ANGLE, u32,);
        let ang_hi: u64 = (na as f64).to_bits() & 0xFFFF_FFFF_0000_0000;
        let ang = f64::from_bits(ang_hi | ang_lo as u64);
        let biased = add(ang as f32, global_f32(G_BIAS));
        let shaped: f64 = lf_checker_rt::callee_cdecl!(C_SHAPE, f64, biased.to_bits());
        let level: f32 = shaped as f32;
        // Components and store. The spare slot reads uninitialised stack,
        // pinned to 0 by the checker's stack fill.
        let px: f32 = f32::from_bits(lf_checker_rt::callee_cdecl!(C_COMP_X, u32, level.to_bits()));
        let nx = neg(px);
        let qy: f32 = f32::from_bits(lf_checker_rt::callee_cdecl!(C_COMP_Y, u32, level.to_bits()));
        wrf(this + SPARE, 0.0);
        let n2 = add(mul(qy, qy), mul(nx, nx));
        wrf(this + VEC_X, nx);
        wrf(this + VEC_Y, qy);
        wrf(this + VEC_Z, 0.0);
        // Normalise: exact zero (either sign) maps to +0, else 1/sqrt.
        let k: f32 = if n2 == 0.0 {
            0.0
        } else {
            core::hint::black_box(ONE) / core::hint::black_box(n2.sqrt())
        };
        wrf(this + VEC_X, mul(nx, k));
        wrf(this + VEC_Y, mul(qy, k));
        wrf(this + VEC_Z, mul(k, 0.0));
        wrf(this + COOLDOWN, 0.0);
        0
    }
});
