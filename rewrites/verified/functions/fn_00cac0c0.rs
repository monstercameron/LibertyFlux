// original: 0x00cac0c0 CTaskComplexMoveGoToPointRelativeToEntityAndStandStill ctor overload 2 (symbols)

/// Second constructor of the "go to a point relative to an entity, then stand
/// still" complex move task: initialise the task object, look up the target
/// entity, and transform the goal offset into world space.
///
/// `this` (ECX) is the task object (at least 0x58 bytes). The five stack
/// arguments are: `entity` (stored at `+0x40`, re-read after the lookup
/// call; null means "no target yet"), `rate` (float, forwarded to the base
/// constructor), `src` (pointer to three words copied to `+0x20..0x28`, the
/// local-space goal vector), `blend` (float stored at `+0x44`) and `mode`
/// (stored at `+0x48`).
///
/// Behaviour: call the base constructor (callee 1) with `rate`; plant the
/// two vtable ids (`VTABLE_MAIN` at `+0x00`, `VTABLE_SUB` at `+0x14`); copy
/// the goal vector; store `entity`/`blend`/`mode` and zero `+0x4c`, `+0x50`
/// and the half-word at `+0x54`. Call the entity lookup (callee 2) with the
/// address of the `+0x40` slot and re-read it: a null entity ends the work
/// here. Otherwise read the entity's matrix slot at `+0x20`: when it is
/// null, create the matrix (callee 3, writes the slot) and initialise it
/// (callee 4, taking the slot value). Finally transform the goal vector by
/// the entity's 3x4 matrix (rows `M0`..`M2` of four floats at stride 0x10,
/// translation row at `+0x30`), accumulating in the original's exact order
/// `((row1*mid + row0*first) + row2*last) + translation`, and store the
/// result at `+0x30..0x38`. The word at `+0x3c` is whatever uninitialised
/// stack scratch the original happens to read; under the checker's defined
/// zero stack fill that is `0.0`, which is what this rewrite stores.
/// Returns `this`.
///
/// Original: 0x00cac0c0 (thiscall, ECX plus five stack words, callee cleans
/// 0x14). The two vtable ids are relocated immediates and are derived with
/// `relocated()`, never hard-coded as mapped addresses.
lf_checker_rt::export!(thiscall, rw_00cac0c0(this: u32, entity: u32, rate: u32, src: u32, blend: u32, mode: u32) -> u32 {
    unsafe {
        const VTABLE_MAIN_FILE: u32 = 0x00ed840c;
        const VTABLE_SUB_FILE: u32 = 0x00ed8464;
        const GOAL_VEC: u32 = 0x20;
        const WORLD_PT: u32 = 0x30;
        const WORLD_W: u32 = 0x3c;
        const ENTITY_SLOT: u32 = 0x40;
        const BLEND_SLOT: u32 = 0x44;
        const MODE_SLOT: u32 = 0x48;
        const ZERO_A: u32 = 0x4c;
        const ZERO_B: u32 = 0x50;
        const ZERO_H: u32 = 0x54;
        const MATRIX_SLOT: u32 = 0x20;
        const MATRIX_ROW_STRIDE: u32 = 0x10;
        const MATRIX_TRANS: u32 = 0x30;

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

        // Base-class initialisation; the answer is not used.
        lf_checker_rt::callee_thiscall!(1, u32, this, rate);
        wr32(this, lf_checker_rt::relocated(VTABLE_MAIN_FILE));
        wr32(this + 0x14, lf_checker_rt::relocated(VTABLE_SUB_FILE));
        wr32(this + GOAL_VEC, rd32(src));
        wr32(this + GOAL_VEC + 4, rd32(src + 4));
        wr32(this + GOAL_VEC + 8, rd32(src + 8));
        wr32(this + ENTITY_SLOT, entity);
        wr32(this + BLEND_SLOT, blend);
        wr32(this + MODE_SLOT, mode);
        wr32(this + ZERO_A, 0);
        wr32(this + ZERO_B, 0);
        (this as *mut u16).wrapping_byte_add(ZERO_H as usize).write_unaligned(0);

        // Entity lookup; re-read the slot afterwards.
        let slot = this + ENTITY_SLOT;
        lf_checker_rt::callee_thiscall!(2, u32, rd32(slot), slot);
        let obj = rd32(slot);
        if obj == 0 {
            return this;
        }
        let mut mat = rd32(obj + MATRIX_SLOT);
        if mat == 0 {
            lf_checker_rt::callee_thiscall!(3, u32, obj);
            lf_checker_rt::callee_thiscall!(4, u32, obj + 0x10, rd32(obj + MATRIX_SLOT));
            mat = rd32(obj + MATRIX_SLOT);
        }

        // out[i] = ((row1[i]*v1 + row0[i]*v0) + row2[i]*v2) + trans[i].
        let v0 = rdf(this + GOAL_VEC);
        let v1 = rdf(this + GOAL_VEC + 4);
        let v2 = rdf(this + GOAL_VEC + 8);
        let mut i = 0u32;
        while i < 3 {
            let r0 = rdf(mat + i * 4);
            let r1 = rdf(mat + MATRIX_ROW_STRIDE + i * 4);
            let r2 = rdf(mat + 2 * MATRIX_ROW_STRIDE + i * 4);
            let t = rdf(mat + MATRIX_TRANS + i * 4);
            let acc = add(mul(r1, v1), mul(r0, v0));
            let acc = add(acc, mul(r2, v2));
            wrf(this + WORLD_PT + i * 4, add(acc, t));
            i += 1;
        }
        // Uninitialised stack scratch in the original; 0.0 under stack_fill 0.
        wrf(this + WORLD_W, 0.0);
        this
    }
});
