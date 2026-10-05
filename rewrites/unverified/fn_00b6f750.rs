// original: 0x00b6f750 CTaskComplexGoToCarDoorAndStandStill::vf18

/// Decide the next step of a "go to car door and stand still" complex task.
///
/// `task` points to the task object, `ped` to the pedestrian the task acts
/// on. Layout read (all offsets from `task` unless said): `+0x00` vtable with
/// the sub-task hook at slot `+0x4c`; `+0x08` controller object (its vtable
/// slot `+0x0c` reports the current step id, its word `+0x14` points at the
/// step object or is null); `+0x14` the target vehicle, null when there is
/// none; `+0x20` the walk speed; `+0x30` a direction word; `+0x60` a scratch
/// position vector filled by the locator callee; `+0x70` written with the
/// animation id; bytes `+0x74` (reached-door flag), `+0x75` (arrived flag,
/// written), `+0x76`/`+0x77` (door-open gates). The vehicle word at `+0x1304`
/// is its state; `ped+0x20` points at the target position triple
/// (`+0x30`/`+0x34`/`+0x38`).
///
/// Behaviour: with no vehicle, return 0. Otherwise double the speed when the
/// vehicle state is 4 or 5, resolve the direction word and animation id
/// through two callees, then dispatch on the step id: 0xCA checks arrival at
/// the stored door position (within 0.5 of the target, 2D), 0xCB skips
/// straight to the advance call, 0x11D dispatches again on the step object's
/// id minus 0x384 through a 43-entry index (offsets 0, 3, 5 and 42 select the
/// live cases, anything else returns 0), any other id returns 0. The distance
/// cases locate the door position, skip when the target is 3.0 or more above
/// (or the height difference is unordered), else compare the 2D distance
/// against the (possibly raised) speed; far means calling the sub-task hook
/// and returning its value, near sets the arrived flag and advances. The
/// advance call passes (0x516, ped) and its value is returned.
///
/// Float order is the original's (2D distance as dy²+dx², height gate as
/// `!(3.0 > dy)` so NaN skips, arrival as `!(0.5 > dist)` so NaN advances).
/// The switch index values come from the original's index table.
///
/// Original: 0x00b6f750 (thiscall, ecx = task, one stack word = ped).
lf_checker_rt::export!(thiscall, rw_00b6f750(task: u32, ped: u32) -> u32 {
    unsafe {
        const VEHICLE: u32 = 0x14;
        const VEH_STATE: u32 = 0x1304;
        const SPEED: u32 = 0x20;
        const DIR_WORD: u32 = 0x30;
        const POS_SCRATCH: u32 = 0x60;
        const ANIM_ID: u32 = 0x70;
        const FLAG_REACHED: u32 = 0x74;
        const FLAG_ARRIVED: u32 = 0x75;
        const GATE_A: u32 = 0x76;
        const GATE_B: u32 = 0x77;
        const CTRL: u32 = 0x08;
        const STEP_ID_SLOT: u32 = 0x0c;
        const STEP_OBJ: u32 = 0x14;
        const SUBTASK_SLOT: u32 = 0x4c;
        const TARGET_POS: u32 = 0x20;
        const STEP_ARRIVED_CHECK: u32 = 0xca;
        const STEP_ADVANCE: u32 = 0xcb;
        const STEP_SWITCH: u32 = 0x11d;
        const MISSING_STEP: u32 = 0xc8;
        const SWITCH_BASE: u32 = 0x384;
        const ADVANCE_KIND: u32 = 0x516;
        const CAL_DIR: u32 = 0;
        const CAL_ANIM: u32 = 1;
        const CAL_LOCATE: u32 = 4;
        const CAL_ADVANCE: u32 = 6;
        const DBL_SPEED: f32 = 2.0;
        const HIGH_ABOVE: f32 = 3.0;
        const TOUCH_DIST: f32 = 0.5;
        /// Switch index table (43 entries): step-base offset -> case.
        const SWITCH_INDEX: [u8; 43] = [
            0, 4, 4, 1, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3,
        ];

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        /// The sub-task hook: vtable slot 0x4c on the task, one stack word.
        #[inline(always)]
        unsafe fn subtask(task: u32, ped: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(task) + SUBTASK_SLOT) as usize);
                f(task, ped)
            }
        }
        /// A step-id query: vtable slot 0x0c on the object, no stack words.
        #[inline(always)]
        unsafe fn step_id(obj: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + STEP_ID_SLOT) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn advance(task: u32, ped: u32) -> u32 {
            unsafe { lf_checker_rt::callee_thiscall!(CAL_ADVANCE, u32, task, ADVANCE_KIND, ped) }
        }
        /// Locate the door position, then measure (dx, dy, dz) to the target.
        #[inline(always)]
        unsafe fn locate(dir: u32, vehicle: u32, pos: u32, ped: u32) -> (f32, f32, f32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(CAL_LOCATE, u32, dir, vehicle, pos);
                let tgt = rd32(ped.wrapping_add(TARGET_POS));
                let dx = fsub(rdf(pos), rdf(tgt.wrapping_add(0x30)));
                let dy = fsub(rdf(pos.wrapping_add(4)), rdf(tgt.wrapping_add(0x34)));
                let dz = fsub(rdf(pos.wrapping_add(8)), rdf(tgt.wrapping_add(0x38)));
                (dx, dy, dz)
            }
        }

        let vehicle = rd32(task.wrapping_add(VEHICLE));
        if vehicle == 0 {
            return 0;
        }
        let state = rd32(vehicle.wrapping_add(VEH_STATE));
        let mut speed = rdf(task.wrapping_add(SPEED));
        if state == 5 || state == 4 {
            speed = fmul(speed, DBL_SPEED);
        }
        let dir = lf_checker_rt::callee_thiscall!(CAL_DIR, u32, vehicle, rd32(task.wrapping_add(DIR_WORD)));
        let anim = lf_checker_rt::callee_thiscall!(CAL_ANIM, u32, dir, vehicle);
        wr32(task.wrapping_add(ANIM_ID), anim);
        let ctrl = rd32(task.wrapping_add(CTRL));
        let kind = step_id(ctrl);
        if kind == STEP_ARRIVED_CHECK {
            // Arrival check against the stored door position.
            if rd8(task.wrapping_add(FLAG_REACHED)) == 0 {
                return advance(task, ped);
            }
            let tgt = rd32(ped.wrapping_add(TARGET_POS));
            let dx = fsub(rdf(task.wrapping_add(POS_SCRATCH)), rdf(tgt.wrapping_add(0x30)));
            let dy = fsub(rdf(task.wrapping_add(POS_SCRATCH + 4)), rdf(tgt.wrapping_add(0x34)));
            let dist = fadd(fmul(dy, dy), fmul(dx, dx)).sqrt();
            if !(TOUCH_DIST > dist) {
                return advance(task, ped);
            }
            wr8(task.wrapping_add(FLAG_ARRIVED), 1);
            return advance(task, ped);
        }
        if kind == STEP_ADVANCE {
            return advance(task, ped);
        }
        if kind != STEP_SWITCH {
            return 0;
        }
        let step_obj = rd32(ctrl.wrapping_add(STEP_OBJ));
        let sub = if step_obj == 0 { MISSING_STEP } else { step_id(step_obj) };
        let t = sub.wrapping_sub(SWITCH_BASE);
        if t > 0x2a {
            return 0;
        }
        let pos = task.wrapping_add(POS_SCRATCH);
        match SWITCH_INDEX[t as usize] {
            0 => {
                let (dx, dy, dz) = locate(dir, vehicle, pos, ped);
                if !(HIGH_ABOVE > dz) {
                    return advance(task, ped);
                }
                let dist = fadd(fmul(dy, dy), fmul(dx, dx)).sqrt();
                if !(speed >= dist) {
                    return advance(task, ped);
                }
                wr8(task.wrapping_add(FLAG_ARRIVED), 1);
                advance(task, ped)
            }
            1 => subtask(task, ped),
            2 => {
                let (dx, dy, dz) = locate(dir, vehicle, pos, ped);
                if !(HIGH_ABOVE > dz) {
                    return subtask(task, ped);
                }
                let dist = fadd(fmul(dy, dy), fmul(dx, dx)).sqrt();
                if speed >= dist {
                    wr8(task.wrapping_add(FLAG_ARRIVED), 1);
                    advance(task, ped)
                } else {
                    subtask(task, ped)
                }
            }
            3 => {
                if rd8(task.wrapping_add(GATE_B)) != 0 && rd8(task.wrapping_add(GATE_A)) != 0 {
                    wr8(task.wrapping_add(FLAG_ARRIVED), 0);
                    return advance(task, ped);
                }
                let (dx, dy, dz) = locate(dir, vehicle, pos, ped);
                if !(HIGH_ABOVE > dz) {
                    return subtask(task, ped);
                }
                let mut near = speed;
                if !(near > TOUCH_DIST) {
                    near = TOUCH_DIST;
                }
                near = fadd(near, TOUCH_DIST);
                let dist = fadd(fmul(dy, dy), fmul(dx, dx)).sqrt();
                if near >= dist {
                    wr8(task.wrapping_add(FLAG_ARRIVED), 1);
                    advance(task, ped)
                } else {
                    subtask(task, ped)
                }
            }
            _ => 0,
        }
    }
});
