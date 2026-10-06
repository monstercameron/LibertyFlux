// original: 0x00CBC190 ped_task_gate_presence_flag (proposed)

/// Gate a task's presence flag behind a chain of checks, then set it.
///
/// `this` is a task-ish object (subject pointer at `+0x04`, position at
/// `+0x40`, flag word at `+0xC4`); `ped` is the ped it acts on (matrix
/// pointer at `+0x20`, state bytes at `+0x218`/`+0x219`, slot pointer at
/// `+0x224`). The flag bit `0x200` is set only if every gate passes, in
/// order: the ped validity helper answers nonzero; the bit is not already
/// set; the ped sub-object probe answers zero; the slot-table lookup
/// answers zero; either state byte `0x218` differs from that zero answer
/// or byte `0x219` is zero, otherwise a context helper's answer must
/// differ from its singleton address (a relocated immediate: the live
/// address, not the file one); the subject (when present) answers
/// two kind codes that must differ from `0x395` and `0x3AB`; the squared
/// distance between the task position and the matrix position exceeds 9;
/// and the dot product of the normalised offset with the matrix's first
/// row is below 0. Any failure leaves the flag word untouched.
///
/// Comparisons: the callee answers are compared for exact equality only
/// (`je`), so signedness is irrelevant; the two float gates are x86
/// `comiss` + `jbe`, i.e. "continue" means strictly greater, with NaN on
/// either side taking the exit. The float operation order is the
/// original's: offset `dy*dy + dx*dx + dz*dz` folded left, dot product
/// `(row_y*ny + nx*row_x) + row_z*nz`.
///
/// Original: 0x00CBC190 (thiscall, one stack word; no meaningful return).
lf_checker_rt::export!(thiscall, rw_00CBC190(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBJ_PTR: u32 = 0x04;
        const VTASK_KIND: u32 = 0x0c;
        const POS_X: u32 = 0x40;
        const POS_Y: u32 = 0x44;
        const POS_Z: u32 = 0x48;
        const TASK_FLAGS: u32 = 0xc4;
        const PRESENCE_BIT: u32 = 0x200;
        const PED_MATRIX: u32 = 0x20;
        const PED_STATE0: u32 = 0x218;
        const PED_STATE1: u32 = 0x219;
        const PED_SLOT: u32 = 0x224;
        const PED_SUBOBJ: u32 = 0xbb0;
        const SLOT_BIAS: u32 = 0x44;
        const SLOT_INDEX: u32 = 5;
        const KIND_AVOID: u32 = 0x395;
        const KIND_SKIP: u32 = 0x3ab;
        const CONTEXT_SINGLETON: u32 = 0x01185c08;
        const DIST2_LIMIT_VA: u32 = 0x00fe8b00;
        const DOT_LIMIT_VA: u32 = 0x0171bc08;
        const MAT_ROW_X: u32 = 0x10;
        const MAT_ROW_Y: u32 = 0x14;
        const MAT_ROW_Z: u32 = 0x18;
        const MAT_POS_X: u32 = 0x30;
        const MAT_POS_Y: u32 = 0x34;
        const MAT_POS_Z: u32 = 0x38;
        const VALID_CALLEE: u32 = 1;
        const PROBE_CALLEE: u32 = 2;
        const SLOT_CALLEE: u32 = 3;
        const CONTEXT_CALLEE: u32 = 4;
        // The kind-code callee (id 5) is reached through the fabricated
        // vtable, not the stub table, so it needs no constant here.
        const NORMALISE_CALLEE: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        /// The subject's kind code through its virtual slot, exactly as the
        /// original calls it (both sides land on the same planted stub).
        #[inline(always)]
        unsafe fn kind_code(obj: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let slot = rd32(vt.wrapping_add(VTASK_KIND));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(obj)
            }
        }

        let valid: u32 =
            lf_checker_rt::callee_thiscall!(VALID_CALLEE, u32, ped, 0u32);
        if valid & 0xff == 0 {
            return 0;
        }
        if rd32(this.wrapping_add(TASK_FLAGS)) & PRESENCE_BIT != 0 {
            return 0;
        }
        let probe: u32 =
            lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, ped.wrapping_add(PED_SUBOBJ));
        if probe != 0 {
            return 0;
        }
        let slot_base = rd32(ped.wrapping_add(PED_SLOT));
        let slot: u32 = lf_checker_rt::callee_thiscall!(
            SLOT_CALLEE,
            u32,
            slot_base.wrapping_add(SLOT_BIAS),
            SLOT_INDEX
        );
        if slot != 0 {
            return 0;
        }
        // `al` still holds the slot answer's zero low byte here.
        if rd8(ped.wrapping_add(PED_STATE0)) == slot as u8 {
            if rd8(ped.wrapping_add(PED_STATE1)) != 0 {
                let ctx: u32 = lf_checker_rt::callee_cdecl!(CONTEXT_CALLEE, u32,);
                // The original compares against a relocated address
                // immediate, so the live value, not the file one.
                if ctx == lf_checker_rt::relocated(CONTEXT_SINGLETON) {
                    return 0;
                }
            }
        }
        let subj = rd32(this.wrapping_add(SUBJ_PTR));
        if subj != 0 {
            if kind_code(subj) == KIND_AVOID {
                return 0;
            }
            let subj2 = rd32(this.wrapping_add(SUBJ_PTR));
            if kind_code(subj2) == KIND_SKIP {
                return 0;
            }
        }
        let mat = rd32(ped.wrapping_add(PED_MATRIX));
        let dx = sub(rdf(this.wrapping_add(POS_X)), rdf(mat.wrapping_add(MAT_POS_X)));
        let dy = sub(rdf(this.wrapping_add(POS_Y)), rdf(mat.wrapping_add(MAT_POS_Y)));
        let dz = sub(rdf(this.wrapping_add(POS_Z)), rdf(mat.wrapping_add(MAT_POS_Z)));
        let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let dist_limit = f32::from_bits(rd32(lf_checker_rt::relocated(DIST2_LIMIT_VA)));
        // `comiss` + `jbe`: continue only on strictly greater (NaN exits).
        if !(dist2 > dist_limit) {
            return 0;
        }
        let mut triple = [dx, dy, dz];
        lf_checker_rt::callee_thiscall!(NORMALISE_CALLEE, u32, triple.as_mut_ptr() as u32);
        let (nx, ny, nz) = (triple[0], triple[1], triple[2]);
        let row_x = rdf(mat.wrapping_add(MAT_ROW_X));
        let row_y = rdf(mat.wrapping_add(MAT_ROW_Y));
        let row_z = rdf(mat.wrapping_add(MAT_ROW_Z));
        let dot = add(add(mul(row_y, ny), mul(nx, row_x)), mul(row_z, nz));
        let dot_limit = f32::from_bits(rd32(lf_checker_rt::relocated(DOT_LIMIT_VA)));
        if dot_limit > dot {
            let flags = rd32(this.wrapping_add(TASK_FLAGS));
            wr32(this.wrapping_add(TASK_FLAGS), flags | PRESENCE_BIT);
        }
        0
    }
});
