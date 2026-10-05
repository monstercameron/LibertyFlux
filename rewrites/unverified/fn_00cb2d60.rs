// original: 0x00cb2d60 CTaskComplexMoveGoToPointRelativeToEntityAndStandStill::vf19

/// Transform the stored offset by the entity's matrix, then order the
/// stand-still subtask.
///
/// `this` is the complex task: input offset at `+0x20` (3 floats), entity at
/// `+0x40`, seat limit at `+0x48`, transformed point at `+0x30` (4 floats).
/// The function's one stack word is ignored. Callee 1 prepares the entity,
/// callee 2 prepares its matrix (filling the matrix pointer as its
/// out-parameter), callee 3 fetches the behaviour object for the global at
/// `0x167e2a0`, callee 4 builds the subtask.
///
/// Behaviour: returns 0 when there is no entity. When the seat limit is
/// positive, the global tick at `0x11735b4` and the limit are saved at
/// `+0x4c`/`+0x50` and the seeded byte at `+0x54` is set. When the entity
/// has no matrix yet, callees 1 and 2 prepare it. The offset is then
/// transformed by the matrix row by row (`(m10*y + m0*x) + m20*z) + m30`
/// and likewise for the other two rows) into `+0x30`, `+0x34` and `+0x38`;
/// `+0x3c` receives an uninitialised stack word, which is zero under the
/// checker's zero stack fill. Callee 3 fetches the object (a null answer
/// returns 0); otherwise callee 4 is ordered with (this[0x18], anchor,
/// this[0x44], 2.0, 0, 0) and its answer is returned.
///
/// Float order is the original's scalar-SSE order per row.
///
/// Original: 0x00cb2d60 (thiscall, one ignored stack word; callee 1 is
/// thiscall with no stack words, callee 2 is thiscall with one, callee 3 is
/// thiscall with none and takes the global in ECX, callee 4 is thiscall with
/// six stack words and takes callee 3's answer in ECX).
lf_checker_rt::export!(thiscall, rw_00cb2d60(this: u32, _arg: u32) -> u32 {
    unsafe {
        const IN_X: u32 = 0x20;
        const IN_Y: u32 = 0x24;
        const IN_Z: u32 = 0x28;
        const OUT_BASE: u32 = 0x30;
        const ENT: u32 = 0x40;
        const LIMIT: u32 = 0x48;
        const SAVED_TICK: u32 = 0x4c;
        const SAVED_LIMIT: u32 = 0x50;
        const SEEDED: u32 = 0x54;
        const D_ARG0F: u32 = 0x18;
        const D_ARG2F: u32 = 0x44;
        const ENT_MAT: u32 = 0x20;
        const GLOBAL_TICK: u32 = 0x11735b4;
        const GLOBAL_STATE: u32 = 0x167e2a0;
        const CALLEE_PREP: u32 = 1;
        const CALLEE_MAT: u32 = 2;
        const CALLEE_FETCH: u32 = 3;
        const CALLEE_BUILD: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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

        if rd32(this + ENT) == 0 {
            return 0;
        }
        let limit = rd32(this + LIMIT) as i32;
        if limit > 0 {
            let tick = (lf_checker_rt::global::<u32>(GLOBAL_TICK) as *const u32).read_unaligned();
            wr32(this + SAVED_TICK, tick);
            wr32(this + SAVED_LIMIT, limit as u32);
            wr8(this + SEEDED, 1);
        }
        let ent = rd32(this + ENT);
        if ent != 0 {
            if rd32(ent + ENT_MAT) == 0 {
                lf_checker_rt::callee_thiscall!(CALLEE_PREP, u32, ent);
                let m0 = rd32(ent + ENT_MAT);
                lf_checker_rt::callee_thiscall!(CALLEE_MAT, u32, ent.wrapping_add(0x10), m0);
            }
            let m = rd32(ent + ENT_MAT);
            let (x, y, z) = (rdf(this + IN_X), rdf(this + IN_Y), rdf(this + IN_Z));
            let o0 = add(add(add(mul(rdf(m + 0x10), y), mul(rdf(m), x)), mul(rdf(m + 0x20), z)), rdf(m + 0x30));
            let o1 = add(add(add(mul(rdf(m + 0x14), y), mul(rdf(m + 4), x)), mul(rdf(m + 0x24), z)), rdf(m + 0x34));
            let o2 = add(add(add(mul(rdf(m + 0x18), y), mul(rdf(m + 8), x)), mul(rdf(m + 0x28), z)), rdf(m + 0x38));
            wrf(this + OUT_BASE, o0);
            wrf(this + OUT_BASE + 4, o1);
            // The original copies an uninitialised stack word here; the
            // contract pins the stack fill to zero, so this is zero.
            wrf(this + OUT_BASE + 12, 0.0);
            wrf(this + OUT_BASE + 8, o2);
        }
        let g = (lf_checker_rt::global::<u32>(GLOBAL_STATE) as *const u32).read_unaligned();
        let c = lf_checker_rt::callee_thiscall!(CALLEE_FETCH, u32, g);
        if c == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            CALLEE_BUILD, u32, c, rd32(this + D_ARG0F),
            this.wrapping_add(OUT_BASE), rd32(this + D_ARG2F),
            0x4000_0000, 0, 0
        )
    }
});
