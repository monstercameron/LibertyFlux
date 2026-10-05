// original: 0x00A20DA0 ped_task_blend_matrix_update (proposed)

/// Cached-selector blend update: refreshes a blend weight, runs two
/// matrix-calibration callees, then pulls a target point back along the
/// calibrated 3x3 matrix.
///
/// `this` is the task object (countdown at `+0x380`, source pointer at
/// `+0x384`, rate float at `+0x2D8`, matrix block at `+0x140`), `a1`
/// points at four floats (a target point plus a spare slot, updated in
/// place). All floats are single precision, evaluated in the original's
/// SSE lane order.
///
/// Behaviour: a countdown either ticks down and reuses the cached file
/// selector, or, at zero and below, reloads the selector from the source
/// object's `+0xB80` word, stores it to the file global and resets the
/// countdown to 10. With selector 1 the rate decays by 0.9 and is zeroed
/// when the decayed value is ordered-below 0.02; otherwise the rate moves
/// a tenth of the way from its current value toward the selector-indexed
/// entry of the constant table. Two calibration callees then run against
/// the matrix block (first with a table pointer and a scale float, 0.05
/// under selector 1 else 0.1; second with the rate). Finally the target
/// target point is overwritten with the matrix applied to its own value,
/// while the spare slot receives a word of uninitialized stack (zero
/// under the proof's stack fill). Returns `a1`.
/// Original is thiscall with one stack word.
lf_checker_rt::export!(thiscall, rw_00A20DA0(this: u32, a1: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x380;
        const SRC_OFF: u32 = 0x384;
        const SRC_SEL: u32 = 0xB80;
        const COUNT_RESET: u32 = 10;
        const SEL_GLOBAL: u32 = 0x012D_D5D0;
        const RATE_OFF: u32 = 0x2D8;
        const MAT_OFF: u32 = 0x140;
        const DECAY: f32 = f32::from_bits(0x3F66_6666); // 0.9
        const FLOOR: f32 = f32::from_bits(0x3CA3_D70A); // 0.02
        const STEP: f32 = f32::from_bits(0x3DCC_CCCD); // 0.1
        const STEP_ONE: f32 = f32::from_bits(0x3D4C_CCCD); // 0.05
        const RATE_TABLE: u32 = 0x00E9_B270;
        const CAL_BASE: u32 = 0x12DD_2B0;
        const CAL_STRIDE: u32 = 160;
        const CAL1: u32 = 1;
        const CAL2: u32 = 2;

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
        let sel: u32 = if (rd32(this + COUNT_OFF) as i32) > 0 {
            wr32(this + COUNT_OFF, rd32(this + COUNT_OFF).wrapping_sub(1));
            rd32(lf_checker_rt::relocated(SEL_GLOBAL))
        } else {
            let v = rd32(rd32(this + SRC_OFF) + SRC_SEL);
            wr32(lf_checker_rt::relocated(SEL_GLOBAL), v);
            wr32(this + COUNT_OFF, COUNT_RESET);
            v
        };
        if sel == 1 {
            let x = mul(DECAY, rdf(this + RATE_OFF));
            wrf(this + RATE_OFF, x);
            if FLOOR > x {
                wr32(this + RATE_OFF, 0);
            }
        } else {
            let old = rdf(this + RATE_OFF);
            let goal = rdf(lf_checker_rt::relocated(RATE_TABLE) + sel.wrapping_mul(4));
            wrf(this + RATE_OFF, add(mul(sub(goal, old), STEP), old));
        }
        let base = sel.wrapping_mul(CAL_STRIDE).wrapping_add(CAL_BASE);
        let f2 = if sel == 1 { STEP_ONE } else { STEP };
        let mat = this + MAT_OFF;
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL1, u32, mat, base, f2.to_bits());
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL2, u32, mat, rdf(this + RATE_OFF).to_bits());
        let v0 = rdf(a1);
        let v1 = rdf(a1 + 4);
        let v2 = rdf(a1 + 8);
        let r0 = add(add(mul(rdf(mat + 0x10), v1), mul(rdf(mat), v0)), mul(rdf(mat + 0x20), v2));
        let r1 = add(add(mul(rdf(mat + 0x14), v1), mul(rdf(mat + 4), v0)), mul(rdf(mat + 0x24), v2));
        let r2 = add(add(mul(rdf(mat + 0x18), v1), mul(rdf(mat + 8), v0)), mul(rdf(mat + 0x28), v2));
        wrf(a1, r0);
        wrf(a1 + 4, r1);
        wrf(a1 + 0xC, 0.0);
        wrf(a1 + 8, r2);
        a1
    }
});
