// original: 0x00CBC310 ped_task_acquire_move_target (proposed)

/// Steer a task toward a move target, or fall back to a table or a fresh fix.
///
/// `this` is a movement task (flag byte/word at `+0xB4`, table base at
/// `+0x20`, home position at `+0x90`, result pose at `+0x30`, result words
/// at `+0x60`); `ped` is the ped it acts on (matrix at `+0x20`, slot base
/// at `+0x224`, state words at `+0xA80`/`+0xAA0`). Returns 1 except on the
/// three early exits, which return 0.
///
/// The main path clears flag bit 3, then runs a chain of gates: a validity
/// helper must answer nonzero; a table index from the first phase must be
/// zero; two bit checks (on the ped's and on a fetched object's state
/// words, each `(word >> 1) & 1`) must be clear; a fetched speed must lie
/// strictly between 0 and 1.5; a direction helper's answer is discarded and
/// a heading helper's absolute answer must be below its limit. A score
/// `s`, the dot product of the target matrix's first row with the
/// direction triple, picks an approach point: `pos + 1.3*row` when `s` is
/// strictly negative, `pos - 1.3*row` otherwise (NaN takes the second).
/// When the squared distance from home exceeds 4 one placement helper
/// runs, otherwise another; a confirm helper must then answer nonzero.
/// Success sets flag bit 3, stores the approach point and a 1.0 weight,
/// resolves an object through the ped's slot table and dispatches on its
/// kind code: `0x384` continues into an initialiser call, anything else
/// returns at once.
///
/// Every gate failure funnels into one fail path: a reset call, then, when
/// the entry flag bit 0 was set, a copy of table row `index` (stride 24)
/// into the result pose and words; otherwise a fresh fix (object fetch,
/// matrix copy, two float helpers into the result words). All float gates
/// are `comiss` + `jbe`, i.e. "continue" means strictly greater with NaN
/// taking the exit; callee answers are compared for exact equality only.
/// The float operation order is the original's.
///
/// Three reads take uninitialised stack: the flag byte the second fail
/// path tests (written to a different slot while the call arguments are
/// pushed), and the two words stored into `+0x3C`. In the worker the flag
/// byte residue is stably nonzero, so the reset call there is always
/// skipped, and the two words read 0; this rewrite hard-codes all three,
/// and the proof covers that residue only. One gate (the speed re-check
/// against 5.0) is dead when reached, since the speed is already below 1.5.
///
/// Original: 0x00CBC310 (thiscall, one stack word, returns 0 or 1 in `al`).
lf_checker_rt::export!(thiscall, rw_00CBC310(this: u32, ped: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0xb4;
        const ENTRY_BIT: u32 = 0x01;
        const PLACED_BIT: u32 = 0x08;
        const TABLE_BASE: u32 = 0x20;
        const TABLE_ROWS: u32 = 0x50;
        const HOME_X: u32 = 0x90;
        const HOME_Y: u32 = 0x94;
        const HOME_Z: u32 = 0x98;
        const RES_X: u32 = 0x30;
        const RES_Y: u32 = 0x34;
        const RES_Z: u32 = 0x38;
        const RES_W: u32 = 0x3c;
        const RES_A: u32 = 0x60;
        const RES_B: u32 = 0x64;
        const PED_MATRIX: u32 = 0x20;
        const PED_SLOT: u32 = 0x224;
        const SLOT_BIAS: u32 = 0x44;
        const PED_STATE: u32 = 0xa80;
        const STATE_BITS: u32 = 0x50;
        const PED_HEAD: u32 = 0xaa0;
        const OBJ_STATE: u32 = 0xa80;
        const OBJ_MATRIX: u32 = 0x20;
        const OBJ_HEAD: u32 = 0xaa0;
        const OBJ_WORD: u32 = 0xaa4;
        const MAT_POS_X: u32 = 0x30;
        const MAT_POS_Y: u32 = 0x34;
        const MAT_POS_Z: u32 = 0x38;
        const MAT_POS_W: u32 = 0x3c;
        const VTABLE_KIND: u32 = 0x0c;
        const INIT_SLOT: u32 = 0x28;
        const KIND_CONTINUE: u32 = 0x384;
        const ONE_BITS: u32 = 0x3f800000;
        const SPEED_LO_VA: u32 = 0x00fe8628;
        const SPEED_HI_VA: u32 = 0x00fe8960;
        const SPEED_RECHECK_VA: u32 = 0x00fe8ad8;
        const HEAD_LIMIT_VA: u32 = 0x00ed8efc;
        const REACH_VA: u32 = 0x00fe892c;
        const FAR2_VA: u32 = 0x00fe8ab8;
        const ABS_MASK: u32 = 0x7fffffff;
        const ROW_STRIDE: u32 = 24;
        const IDX_CALLEE: u32 = 1;
        const IDX_OK_CALLEE: u32 = 2;
        const VALID_CALLEE: u32 = 3;
        const FETCH_CALLEE: u32 = 4;
        const SPEED_CALLEE: u32 = 5;
        const DIR_CALLEE: u32 = 6;
        const HEAD_CALLEE: u32 = 7;
        const PLACE_FAR_CALLEE: u32 = 8;
        const PLACE_NEAR_CALLEE: u32 = 9;
        const CONFIRM_CALLEE: u32 = 10;
        const RESOLVE_CALLEE: u32 = 11;
        const RESET_CALLEE: u32 = 14;
        const REFRESH_CALLEE: u32 = 15;
        const AIM_CALLEE: u32 = 16;
        const PITCH_CALLEE: u32 = 17;

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
        #[inline(always)]
        unsafe fn live_f32(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        /// Shared fail path: reset unless skipped, then the table copy
        /// (entry bit 0 set) or a fresh fix. Returns 1 like the
        /// original's fail exits.
        unsafe fn fail_path(this: u32, ped: u32, idx: u32, reset: bool) -> u32 {
            unsafe {
                if reset {
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(RESET_CALLEE, u32, this);
                }
                if rd8(this.wrapping_add(FLAG_OFF)) & (ENTRY_BIT as u8) != 0 {
                    let base = rd32(this.wrapping_add(TABLE_BASE));
                    let rows = rd32(base.wrapping_add(TABLE_ROWS));
                    let row = rows.wrapping_add(idx.wrapping_mul(ROW_STRIDE));
                    wrf(this.wrapping_add(RES_X), rdf(row.wrapping_add(0x18)));
                    wrf(this.wrapping_add(RES_Y), rdf(row.wrapping_add(0x1c)));
                    wrf(this.wrapping_add(RES_Z), rdf(row.wrapping_add(0x20)));
                    // Uninitialised stack slot: 0 under the zero fill.
                    wr32(this.wrapping_add(RES_W), 0);
                    wr32(this.wrapping_add(RES_A), rd32(row.wrapping_add(0x24)));
                    wr32(this.wrapping_add(RES_B), rd32(row.wrapping_add(0x28)));
                } else {
                    let objz: u32 =
                        lf_checker_rt::callee_thiscall!(REFRESH_CALLEE, u32, this);
                    let matc = rd32(objz.wrapping_add(OBJ_MATRIX));
                    wr32(this.wrapping_add(RES_X), rd32(matc.wrapping_add(MAT_POS_X)));
                    wr32(this.wrapping_add(RES_Y), rd32(matc.wrapping_add(MAT_POS_Y)));
                    wr32(this.wrapping_add(RES_Z), rd32(matc.wrapping_add(MAT_POS_Z)));
                    wr32(this.wrapping_add(RES_W), rd32(matc.wrapping_add(MAT_POS_W)));
                    let mata = rd32(ped.wrapping_add(PED_MATRIX));
                    let aim: f32 = lf_checker_rt::callee_cdecl!(
                        AIM_CALLEE,
                        f32,
                        rd32(matc.wrapping_add(MAT_POS_X)),
                        rd32(matc.wrapping_add(MAT_POS_Y)),
                        rd32(mata.wrapping_add(MAT_POS_X)),
                        rd32(mata.wrapping_add(MAT_POS_Y))
                    );
                    wrf(this.wrapping_add(RES_A), aim);
                    let pitch: f32 =
                        lf_checker_rt::callee_thiscall!(PITCH_CALLEE, f32, this);
                    wrf(this.wrapping_add(RES_B), pitch);
                }
                1
            }
        }

        let mut idx = 0u32;
        if rd8(this.wrapping_add(FLAG_OFF)) & (ENTRY_BIT as u8) != 0 {
            let base = rd32(this.wrapping_add(TABLE_BASE));
            let probe = base.wrapping_add(8);
            if probe == 0 {
                return 0;
            }
            let a1: u32 = lf_checker_rt::callee_thiscall!(IDX_CALLEE, u32, probe, ped);
            idx = a1;
            if a1 == 0xffffffff {
                return 0;
            }
            let probe2 = rd32(this.wrapping_add(TABLE_BASE)).wrapping_add(8);
            let a2: u32 = lf_checker_rt::callee_thiscall!(IDX_OK_CALLEE, u32, probe2, a1);
            if a2 == 0 {
                return 0;
            }
        }
        wr32(
            this.wrapping_add(FLAG_OFF),
            rd32(this.wrapping_add(FLAG_OFF)) & !PLACED_BIT,
        );
        let valid: u32 = lf_checker_rt::callee_thiscall!(VALID_CALLEE, u32, this, ped);
        if valid & 0xff == 0 {
            return fail_path(this, ped, idx, true);
        }
        if idx != 0 {
            return fail_path(this, ped, idx, true);
        }
        if rd8(this.wrapping_add(FLAG_OFF)) & (ENTRY_BIT as u8) != 0 {
            let bits = rd32(rd32(ped.wrapping_add(PED_STATE)).wrapping_add(STATE_BITS));
            if (bits >> 1) & 1 != 0 {
                return fail_path(this, ped, idx, true);
            }
        }
        let objx: u32 = lf_checker_rt::callee_thiscall!(FETCH_CALLEE, u32, this);
        let xbits = rd32(rd32(objx.wrapping_add(OBJ_STATE)).wrapping_add(STATE_BITS));
        if (xbits >> 1) & 1 != 0 {
            return fail_path(this, ped, idx, true);
        }
        let speed: f32 = lf_checker_rt::callee_thiscall!(SPEED_CALLEE, f32, objx);
        if !(speed > live_f32(SPEED_LO_VA)) {
            return fail_path(this, ped, idx, true);
        }
        if !(live_f32(SPEED_HI_VA) > speed) {
            return fail_path(this, ped, idx, true);
        }
        let mata = rd32(ped.wrapping_add(PED_MATRIX));
        let matb = rd32(objx.wrapping_add(OBJ_MATRIX));
        let dx = sub(rdf(matb.wrapping_add(MAT_POS_X)), rdf(mata.wrapping_add(MAT_POS_X)));
        let dy = sub(rdf(matb.wrapping_add(MAT_POS_Y)), rdf(mata.wrapping_add(MAT_POS_Y)));
        let dz = sub(rdf(matb.wrapping_add(MAT_POS_Z)), rdf(mata.wrapping_add(MAT_POS_Z)));
        let mut triple = [dx, dy, dz];
        let _: f32 = lf_checker_rt::callee_cdecl!(DIR_CALLEE, f32, triple.as_mut_ptr() as u32);
        let (ex, ey, ez) = (triple[0], triple[1], triple[2]);
        // Dead when reached (speed is already below 1.5), mirrored anyway.
        if !(live_f32(SPEED_RECHECK_VA) > speed) {
            return fail_path(this, ped, idx, true);
        }
        // Argument order: the object's head word is at [esp], the ped's
        // at [esp+4].
        let head: f32 = lf_checker_rt::callee_cdecl!(
            HEAD_CALLEE,
            f32,
            rd32(objx.wrapping_add(OBJ_HEAD)),
            rd32(ped.wrapping_add(PED_HEAD))
        );
        let ahead = f32::from_bits(head.to_bits() & ABS_MASK);
        if !(live_f32(HEAD_LIMIT_VA) > ahead) {
            return fail_path(this, ped, idx, true);
        }
        let rx = rdf(matb);
        let ry = rdf(matb.wrapping_add(4));
        let rz = rdf(matb.wrapping_add(8));
        let score = add(add(mul(rx, ex), mul(ry, ey)), mul(rz, ez));
        let reach = live_f32(REACH_VA);
        let px = rdf(matb.wrapping_add(MAT_POS_X));
        let py = rdf(matb.wrapping_add(MAT_POS_Y));
        let pz = rdf(matb.wrapping_add(MAT_POS_Z));
        // `comiss 0, s` + `jbe`: the upper branch runs only on s < 0.
        let (cx, cy, cz) = if 0.0 > score {
            (add(mul(rx, reach), px), add(mul(ry, reach), py), add(mul(rz, reach), pz))
        } else {
            (sub(px, mul(rx, reach)), sub(py, mul(ry, reach)), sub(pz, mul(rz, reach)))
        };
        let qx = sub(cx, rdf(this.wrapping_add(HOME_X)));
        let qy = sub(cy, rdf(this.wrapping_add(HOME_Y)));
        let qz = sub(cz, rdf(this.wrapping_add(HOME_Z)));
        let dist2 = add(add(mul(qy, qy), mul(qx, qx)), mul(qz, qz));
        // The first placement argument is not the ped matrix: `eax` was
        // reloaded to the target matrix plus 0x30 before the pushes.
        let place_arg = matb.wrapping_add(MAT_POS_X);
        let mut vec = [cx, cy, cz];
        if dist2 > live_f32(FAR2_VA) {
            let _: u32 =
                lf_checker_rt::callee_thiscall!(PLACE_FAR_CALLEE, u32, this, place_arg, vec.as_mut_ptr() as u32);
        } else {
            let _: u32 =
                lf_checker_rt::callee_thiscall!(PLACE_NEAR_CALLEE, u32, this, place_arg, vec.as_mut_ptr() as u32);
        }
        let confirm: u32 = lf_checker_rt::callee_thiscall!(CONFIRM_CALLEE, u32, this);
        if confirm & 0xff == 0 {
            // The flag byte the original tests reads uninitialised stack
            // (written to another slot while the arguments are pushed). The
            // worker's residue there is stably nonzero, so the original
            // always skips its reset call here; mirrored.
            return fail_path(this, ped, idx, false);
        }
        wr32(
            this.wrapping_add(FLAG_OFF),
            rd32(this.wrapping_add(FLAG_OFF)) | PLACED_BIT,
        );
        wrf(this.wrapping_add(RES_X), cx);
        wrf(this.wrapping_add(RES_Y), cy);
        wrf(this.wrapping_add(RES_Z), cz);
        // Uninitialised stack slot: 0 under the zero fill.
        wr32(this.wrapping_add(RES_W), 0);
        wr32(this.wrapping_add(RES_A), rd32(objx.wrapping_add(OBJ_WORD)));
        wr32(this.wrapping_add(RES_B), ONE_BITS);
        let slot = rd32(ped.wrapping_add(PED_SLOT));
        let objy: u32 =
            lf_checker_rt::callee_thiscall!(RESOLVE_CALLEE, u32, slot.wrapping_add(SLOT_BIAS));
        let vth = rd32(objy);
        let sloth = rd32(vth.wrapping_add(VTABLE_KIND));
        let kind_fn: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(sloth as usize);
        if kind_fn(objy) != KIND_CONTINUE {
            return 1;
        }
        let one = rdf(this.wrapping_add(RES_B));
        let memi = rd32(objy.wrapping_add(0x14));
        let sloti = rd32(memi.wrapping_add(INIT_SLOT));
        let init_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(sloti as usize);
        init_fn(objy.wrapping_add(0x14), one.to_bits());
        1
    }
});
