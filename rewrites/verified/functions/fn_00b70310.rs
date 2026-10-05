// original: 0x00b70310 door_approach_solve (proposed)

/// Solve the door-approach point and report which step comes next.
///
/// `task` is the task object, `arg0`/`arg1` the two actors (each with a
/// position triple at `+0x20` -> `+0x30`/`+0x34`/`+0x38`), `arg2` a scratch
/// vector. Layout read: `task+0x14` the vehicle (state at `+0x1304`, matrix
/// object at `+0x20`); `task+0x08` the controller (step id at vtable slot
/// `+0x0c`, step object at `+0x14`); `task+0x70` a parameter word;
/// `task+0x7c` a counter pointer (signed count at `+0x0`); `arg1+0xab0` the
/// follow target, equal to `arg0` when already tracking it.
///
/// Behaviour: reject up front (return 0x3ae) when the vehicle state is 4,
/// the squared distance exceeds 2500, or the height gap exceeds 2.5 without
/// the tracking flag. Otherwise run one of two solvers picked by a second
/// state query: the near solver transforms two frames and normalises the
/// offset to a quarter-step blend (a zero-length offset normalises to zero,
/// ordered-equal, so NaN still divides), the matrix solver combines the
/// vehicle matrix with the scratch vector twice to pick two 1-or-3 mode
/// flags. A validation call (0.25 blend, kind 0x86) rejects again on a null
/// answer; a vehicle state of 1 returns 0x384 directly. The solve call takes
/// nine words (ids, the gate object, the position triple and two frame
/// triples, the second chosen by another state query) and its answer
/// selects the tail: 5 with both step ids matching (0x11d, 0x389) and a live
/// counter rebuilds through the list call and returns 0x389; 0 returns
/// 0x384 after dropping the gate and counter; anything else, or a failed
/// gate, falls to the rebuild tail returning 0x389. A security-cookie check
/// runs before every return.
///
/// Float order is the original's throughout (dy²+dx² addends first, matrix
/// products in load order).
///
/// Original: 0x00b70310 (thiscall, ecx = task, three stack words).
lf_checker_rt::export!(thiscall, rw_00b70310(task: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const VEHICLE: u32 = 0x14;
        const VEH_STATE: u32 = 0x1304;
        const MATRIX_OBJ: u32 = 0x20;
        const CTRL: u32 = 0x08;
        const STEP_SLOT: u32 = 0x0c;
        const STEP_OBJ: u32 = 0x14;
        const PARAM: u32 = 0x70;
        const COUNTER_PTR: u32 = 0x7c;
        const POS_PTR: u32 = 0x20;
        const FOLLOW: u32 = 0xab0;
        const GLOBAL_LIST: u32 = 0x179d114;
        const CAL_STATE0: u32 = 0;
        const CAL_STATE1: u32 = 1;
        const CAL_SETUP: u32 = 2;
        const CAL_XF_A: u32 = 3;
        const CAL_XF_B: u32 = 4;
        const CAL_XF_C: u32 = 5;
        const CAL_XF_D: u32 = 6;
        const CAL_VALIDATE: u32 = 7;
        const CAL_STATE2: u32 = 8;
        const CAL_SOLVE: u32 = 9;
        const CAL_STATE3: u32 = 10;
        const CAL_GATE: u32 = 11;
        const CAL_REBUILD: u32 = 14;
        const CAL_COOKIE: u32 = 15;
        const FAR2: f32 = 2500.0;
        const HIGH: f32 = 2.5;
        const UNIT: f32 = 1.0;
        const QUARTER: f32 = 0.25;
        const LIFT: f32 = 0.3;
        const FAIL: u32 = 0x3ae;
        const SEATED: u32 = 0x384;
        const ADVANCE: u32 = 0x389;

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
        unsafe fn glob(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn fneg(a: f32) -> f32 {
            -core::hint::black_box(a)
        }
        #[inline(always)]
        fn fabs(a: f32) -> f32 {
            core::hint::black_box(a).abs()
        }
        #[inline(always)]
        unsafe fn cookie() {
            unsafe {
                lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            }
        }
        #[inline(always)]
        unsafe fn step_id(obj: u32) -> u32 {
            unsafe {
                // Vtable slot 0x0c on the object; each object's planted
                // vtable routes to its own scripted stub.
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + STEP_SLOT) as usize);
                f(obj)
            }
        }
        /// Rebuild tail: drop the counter object if present, link the gate,
        /// check the cookie and advance. Shared by two paths.
        #[inline(always)]
        unsafe fn rebuild_tail(task: u32, gate: u32) -> u32 {
            unsafe {
                let c = rd32(task.wrapping_add(COUNTER_PTR));
                if c != 0 {
                    lf_checker_rt::callee_thiscall!(CAL_REBUILD, u32, glob(GLOBAL_LIST), c);
                }
                wr32(task.wrapping_add(COUNTER_PTR), gate);
                cookie();
                ADVANCE
            }
        }

        // Frame scratch: mirrors of the original's slots. v_scratch is the
        // arg2 vector home (also the near-solver output); t_near the near
        // triple; t_far the matrix triple; v_wide the six-word solve input.
        let mut v_scratch = [
            rd32(arg2),
            rd32(arg2.wrapping_add(4)),
            rd32(arg2.wrapping_add(8)),
        ];
        let mut t_near = [0u32; 3];
        let mut t_far = [0u32; 3];
        let mut v_wide = [0u32; 6];
        let mut dummy = [0u32; 4];
        let dummy_ptr = dummy.as_mut_ptr() as u32;

        let vehicle = rd32(task.wrapping_add(VEHICLE));
        let pos_a = rd32(arg0.wrapping_add(POS_PTR));
        let pos_b = rd32(arg1.wrapping_add(POS_PTR));
        let dx = fsub(rdf(pos_a.wrapping_add(0x30)), rdf(pos_b.wrapping_add(0x30)));
        let dy = fsub(rdf(pos_a.wrapping_add(0x34)), rdf(pos_b.wrapping_add(0x34)));
        let dz = fsub(rdf(pos_a.wrapping_add(0x38)), rdf(pos_b.wrapping_add(0x38)));
        let follow = rd32(arg1.wrapping_add(FOLLOW));
        let tracking = follow != 0 && follow == arg0;
        let st0 = lf_checker_rt::callee_thiscall!(CAL_STATE0, u32, arg0);
        if st0 == 4 {
            cookie();
            return FAIL;
        }
        let dist2 = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
        if dist2 > FAR2 {
            cookie();
            return FAIL;
        }
        if fabs(dz) > HIGH && !tracking {
            cookie();
            return FAIL;
        }
        lf_checker_rt::callee_thiscall!(
            CAL_SETUP, u32, dummy_ptr,
            arg0,
            rdf(pos_b.wrapping_add(0x38)).to_bits(),
            QUARTER.to_bits(),
            0, 0, 0, 0, 0, 0
        );
        let st1 = lf_checker_rt::callee_thiscall!(CAL_STATE1, u32, vehicle);
        if st1 != 3 {
            // Near solver.
            lf_checker_rt::callee_thiscall!(
                CAL_XF_A, u32, dummy_ptr,
                v_scratch.as_mut_ptr() as u32, t_near.as_mut_ptr() as u32, 0xffff_ffff
            );
            lf_checker_rt::callee_thiscall!(
                CAL_XF_B, u32, dummy_ptr,
                pos_b.wrapping_add(0x30), dummy_ptr, 0xffff_ffff
            );
            let ox = fsub(f32::from_bits(t_near[0]), f32::from_bits(v_scratch[0]));
            let oy = fsub(f32::from_bits(t_near[1]), f32::from_bits(v_scratch[1]));
            let oz = fsub(f32::from_bits(t_near[2]), f32::from_bits(v_scratch[2]));
            let len2 = fadd(fadd(fmul(oy, oy), fmul(ox, ox)), fmul(oz, oz));
            let inv = if len2 == 0.0 { 0.0 } else { fdiv(UNIT, len2.sqrt()) };
            let bx = f32::from_bits(v_scratch[0]);
            let by = f32::from_bits(v_scratch[1]);
            let bz = f32::from_bits(v_scratch[2]);
            v_scratch[0] = fadd(fmul(fmul(ox, inv), QUARTER), bx).to_bits();
            v_scratch[1] = fadd(fmul(fmul(oy, inv), QUARTER), by).to_bits();
            v_scratch[2] = fadd(fmul(fmul(oz, inv), QUARTER), bz).to_bits();
            // v_wide models the validate out-buffer: zeros on entry in both
            // paths (path A never fills it, path B scratch), scripted after.
            let va = lf_checker_rt::callee_cdecl!(
                CAL_VALIDATE, u32,
                v_wide.as_mut_ptr() as u32, t_near.as_mut_ptr() as u32,
                QUARTER.to_bits(), arg1, 0x86, 0, 0
            );
            if va as u8 == 0 {
                cookie();
                return FAIL;
            }
        } else {
            // Matrix solver.
            let mat = rd32(vehicle.wrapping_add(MATRIX_OBJ));
            let m00 = rdf(mat);
            let m04 = rdf(mat.wrapping_add(4));
            let m08 = rdf(mat.wrapping_add(8));
            let m30 = rdf(mat.wrapping_add(0x30));
            let m34 = rdf(mat.wrapping_add(0x34));
            let m38 = rdf(mat.wrapping_add(0x38));
            let s30 = rdf(pos_b.wrapping_add(0x30));
            let s34 = rdf(pos_b.wrapping_add(0x34));
            let s38 = rdf(pos_b.wrapping_add(0x38));
            let mut x2 = fmul(m04, m34);
            let x0a = fmul(m30, m00);
            let mut x1 = fmul(s34, m04);
            x2 = fadd(x2, x0a);
            let x0b = fmul(m08, m38);
            x2 = fadd(x2, x0b);
            let x0c = fmul(m00, s30);
            x2 = fneg(x2);
            x1 = fadd(x1, x0c);
            let x0d = fmul(s38, m08);
            let f24 = x2;
            x1 = fadd(x1, x0d);
            x1 = fadd(x1, f24);
            let flag1 = if x1 < 0.0 { 1u32 } else { 3u32 };
            lf_checker_rt::callee_thiscall!(
                CAL_XF_C, u32, dummy_ptr,
                pos_b.wrapping_add(0x30), dummy_ptr, flag1
            );
            let v0 = f32::from_bits(v_scratch[0]);
            let v1 = f32::from_bits(v_scratch[1]);
            let v2 = f32::from_bits(v_scratch[2]);
            let mut y1 = fmul(m04, v1);
            let y0a = fmul(v0, m00);
            y1 = fadd(y1, y0a);
            let y0b = fmul(m08, v2);
            y1 = fadd(y1, y0b);
            y1 = fadd(y1, f24);
            let flag2 = if y1 < 0.0 { 1u32 } else { 3u32 };
            lf_checker_rt::callee_thiscall!(
                CAL_XF_D, u32, dummy_ptr,
                v_scratch.as_mut_ptr() as u32, t_far.as_mut_ptr() as u32, flag2
            );
            let va = lf_checker_rt::callee_cdecl!(
                CAL_VALIDATE, u32,
                v_wide.as_mut_ptr() as u32, t_far.as_mut_ptr() as u32,
                QUARTER.to_bits(), arg1, 0x86, 0, 0
            );
            if va as u8 == 0 {
                cookie();
                return FAIL;
            }
        }
        if rd32(vehicle.wrapping_add(VEH_STATE)) == 1 {
            cookie();
            return SEATED;
        }
        let gate = lf_checker_rt::callee_thiscall!(CAL_GATE, u32, glob(GLOBAL_LIST));
        if gate != 0 {
            wr32(gate, 0);
        }
        v_scratch[2] = fsub(f32::from_bits(v_scratch[2]), LIFT).to_bits();
        let st2 = lf_checker_rt::callee_thiscall!(CAL_STATE2, u32, vehicle);
        let (p_deep, p_shallow) = if st1 != 3 {
            (
                if st2 == 3 { t_near.as_mut_ptr() as u32 } else { v_scratch.as_mut_ptr() as u32 },
                v_scratch.as_mut_ptr() as u32,
            )
        } else {
            (
                if st2 == 3 { t_far.as_mut_ptr() as u32 } else { v_scratch.as_mut_ptr() as u32 },
                v_scratch.as_mut_ptr() as u32,
            )
        };
        let solved = lf_checker_rt::callee_cdecl!(
            CAL_SOLVE, u32,
            vehicle, arg1, gate, pos_b.wrapping_add(0x30), p_deep, p_shallow,
            0x8e, 1, rd32(task.wrapping_add(PARAM))
        );
        let st3 = lf_checker_rt::callee_thiscall!(CAL_STATE3, u32, arg0);
        if st3 == 3 && solved != 5 && solved != 0 {
            wr32(gate.wrapping_add(0x10), v_wide[0]);
            wr32(gate.wrapping_add(0x14), v_wide[1]);
            wr32(gate.wrapping_add(0x18), v_wide[2]);
            wr32(gate.wrapping_add(0x1c), v_wide[3]);
        }
        let ctrl = rd32(task.wrapping_add(CTRL));
        let mut cl = 0u8;
        if ctrl != 0 {
            let v1 = step_id(ctrl);
            if v1 == 0x11d {
                let step = rd32(ctrl.wrapping_add(STEP_OBJ));
                if step != 0 {
                    let v2 = step_id(step);
                    if v2 == 0x389 {
                        cl = 1;
                    }
                }
            }
        }
        if solved == 5 {
            if cl == 0 {
                return rebuild_tail(task, gate);
            }
            let c = rd32(task.wrapping_add(COUNTER_PTR));
            if c == 0 || (rd32(c) as i32) <= 0 {
                return rebuild_tail(task, gate);
            }
            lf_checker_rt::callee_thiscall!(CAL_REBUILD, u32, glob(GLOBAL_LIST), c);
            wr32(task.wrapping_add(COUNTER_PTR), gate);
            cookie();
            ADVANCE
        } else if solved == 0 {
            if gate != 0 {
                lf_checker_rt::callee_thiscall!(CAL_REBUILD, u32, glob(GLOBAL_LIST), gate);
            }
            let c = rd32(task.wrapping_add(COUNTER_PTR));
            if c != 0 {
                wr32(c, 0);
            }
            cookie();
            SEATED
        } else {
            rebuild_tail(task, gate)
        }
    }
});
