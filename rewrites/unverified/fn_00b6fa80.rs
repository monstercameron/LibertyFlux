// original: 0x00b6fa80 task_advance_door_step (proposed)

/// Advance one step of the go-to-car-door task, dispatching on the step kind.
///
/// `task` points to the task object, `kind` selects the step, `ped` is the
/// pedestrian. Layout read: `+0x14` the vehicle (its state word at `+0x1304`
/// doubles the speed at `+0x20` when 4 or 5); `+0x18` a direction word;
/// `+0x40`/`+0x44`/`+0x48` written with a global, an animation id and the
/// status byte 1 on the locating steps; `+0x60` a scratch position vector;
/// `+0x77` selects the 10.0/0.5 blend on the far step; `+0x7c` a pointer to
/// a counter that gates the door step. Globals: the id base and the
/// task-list head, both read fresh for every gate.
///
/// Behaviour by kind: 0x384 locates the door, records the animation and
/// builds the near task (gated twice); 0xCB/0xCA build the small tasks
/// (8.0 / push-1) when the gate answers; 0x387 locates then builds the
/// 2.0 task; 0x3AE builds the far task (0.5 floor, 10.0 blend when the
/// flag is set); 0x389 builds the door task when the counter is positive;
/// anything else, or a gate that answers null, falls back to the idle call
/// and returns 0. A built task id of 0 also falls back; otherwise it is
/// returned. The float arguments are the (possibly doubled) speed and the
/// small constants 8.0, 2.0, 10.0, 0.5, 5.0 and -1.0.
///
/// Original: 0x00b6fa80 (thiscall, ecx = task, two stack words = kind, ped).
lf_checker_rt::export!(thiscall, rw_00b6fa80(task: u32, kind: u32, ped: u32) -> u32 {
    unsafe {
        const VEHICLE: u32 = 0x14;
        const VEH_STATE: u32 = 0x1304;
        const SPEED: u32 = 0x20;
        const DIR_WORD: u32 = 0x18;
        const ID_SLOT: u32 = 0x40;
        const ANIM_SLOT: u32 = 0x44;
        const STATUS_SLOT: u32 = 0x48;
        const POS_SCRATCH: u32 = 0x60;
        const BLEND_FLAG: u32 = 0x77;
        const COUNTER_PTR: u32 = 0x7c;
        const PED_FLAG_OFF: u32 = 0x26c;
        const PED_FLAG_BIT: u32 = 0x80000;
        const GLOBAL_ID_BASE: u32 = 0x11735b4;
        const GLOBAL_TASK_HEAD: u32 = 0x167e2a0;
        const LOCATE: u32 = 0;
        const LOCATE_USE: u32 = 1;
        const ANIM_OF: u32 = 2;
        const GATE_A1: u32 = 3;
        const GATE_SHARED: u32 = 4;
        const IDLE: u32 = 5;
        const GATE_C: u32 = 6;
        const BUILD_SMALL8: u32 = 7;
        const GATE_D: u32 = 8;
        const BUILD_PUSH1: u32 = 9;
        const GATE_E1: u32 = 10;
        const BUILD_WIDE2: u32 = 11;
        const STATE_F: u32 = 12;
        const GATE_F1: u32 = 13;
        const BUILD_FAR: u32 = 14;
        const STATE_G: u32 = 15;
        const GATE_G1: u32 = 16;
        const BUILD_DOOR: u32 = 17;
        const GATE_G2: u32 = 18;
        const BUILD_NEAR: u32 = 19;
        const BUILD_FINAL: u32 = 20;
        const DBL_SPEED: f32 = 2.0;
        const WIDE_ARG: f32 = 2.0;
        const SMALL8_ARG: f32 = 8.0;
        const FAR_BLEND_OFF: f32 = 0.5;
        const FAR_BLEND_ON: f32 = 10.0;
        const FAR_FLOOR: f32 = 0.5;
        const DOOR_ARG: f32 = 5.0;
        const DOOR_LAST: f32 = -1.0;

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
        unsafe fn glob(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// The idle fallback: call and return 0.
        #[inline(always)]
        unsafe fn idle(ped: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(IDLE, u32, ped);
                0
            }
        }
        /// The final build: 0 falls back, otherwise the id is returned.
        #[inline(always)]
        unsafe fn finish(gate: u32, built: u32, ped: u32) -> u32 {
            unsafe {
                let r = lf_checker_rt::callee_thiscall!(BUILD_FINAL, u32, gate, built, 0, 0, 0);
                if r == 0 { idle(ped) } else { r }
            }
        }
        /// The shared second gate: non-null goes final, null idles.
        #[inline(always)]
        unsafe fn shared_tail(built: u32, ped: u32) -> u32 {
            unsafe {
                let b = lf_checker_rt::callee_thiscall!(GATE_SHARED, u32, glob(GLOBAL_TASK_HEAD));
                if b != 0 { finish(b, built, ped) } else { idle(ped) }
            }
        }
        /// Locate the door position and feed it to the locate consumer.
        #[inline(always)]
        unsafe fn locate(task: u32, ped: u32, out: u32) {
            unsafe {
                let f: f32 = lf_checker_rt::callee_cdecl!(LOCATE, f32, rd32(task.wrapping_add(DIR_WORD)));
                lf_checker_rt::callee_cdecl!(LOCATE_USE, u32, ped, out, f.to_bits());
            }
        }

        let vehicle = rd32(task.wrapping_add(VEHICLE));
        let state = rd32(vehicle.wrapping_add(VEH_STATE));
        let mut speed = rdf(task.wrapping_add(SPEED));
        if state == 5 || state == 4 {
            speed = fmul(speed, DBL_SPEED);
        }
        let pos = task.wrapping_add(POS_SCRATCH);
        if kind > 0x387 {
            let t = kind.wrapping_sub(0x389);
            if t == 0 {
                // Door step.
                let counter = rd32(task.wrapping_add(COUNTER_PTR));
                if (rd32(counter) as i32) > 0 {
                    locate(task, ped, counter.wrapping_add(0x10));
                }
                let anim = lf_checker_rt::callee_thiscall!(ANIM_OF, u32, task, ped);
                let st = lf_checker_rt::callee_thiscall!(STATE_G, u32, vehicle);
                if st != 4 {
                    wr32(task.wrapping_add(ID_SLOT), glob(GLOBAL_ID_BASE));
                    wr32(task.wrapping_add(ANIM_SLOT), anim);
                    wr8(task.wrapping_add(STATUS_SLOT), 1);
                }
                let a = lf_checker_rt::callee_thiscall!(GATE_G1, u32, glob(GLOBAL_TASK_HEAD));
                let built = if a == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(
                        BUILD_DOOR, u32, a,
                        rd32(task.wrapping_add(DIR_WORD)),
                        counter, 0, speed.to_bits(), DOOR_ARG.to_bits(),
                        0, 0, 0, 0, DOOR_LAST.to_bits()
                    )
                };
                let b = lf_checker_rt::callee_thiscall!(GATE_G2, u32, glob(GLOBAL_TASK_HEAD));
                if b != 0 { finish(b, built, ped) } else { idle(ped) }
            } else if t == 0x25 {
                // Far step.
                let anim = lf_checker_rt::callee_thiscall!(ANIM_OF, u32, task, ped);
                let st = lf_checker_rt::callee_thiscall!(STATE_F, u32, vehicle);
                if st != 4 {
                    wr32(task.wrapping_add(ID_SLOT), glob(GLOBAL_ID_BASE));
                    wr32(task.wrapping_add(ANIM_SLOT), anim);
                    wr8(task.wrapping_add(STATUS_SLOT), 1);
                }
                let a = lf_checker_rt::callee_thiscall!(GATE_F1, u32, glob(GLOBAL_TASK_HEAD));
                if a == 0 {
                    shared_tail(0, ped)
                } else {
                    let blend = if rd8(task.wrapping_add(BLEND_FLAG)) != 0 {
                        FAR_BLEND_ON
                    } else {
                        FAR_BLEND_OFF
                    };
                    let built = lf_checker_rt::callee_thiscall!(
                        BUILD_FAR, u32, a,
                        rd32(task.wrapping_add(DIR_WORD)),
                        pos, speed.to_bits(), FAR_FLOOR.to_bits(),
                        0xffff_ffff, 1, 0, 0, blend.to_bits(), 0
                    );
                    shared_tail(built, ped)
                }
            } else {
                idle(ped)
            }
        } else if kind == 0x387 {
            // Wide step.
            locate(task, ped, pos);
            let a = lf_checker_rt::callee_thiscall!(GATE_E1, u32, glob(GLOBAL_TASK_HEAD));
            let built = if a == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    BUILD_WIDE2, u32, a,
                    rd32(task.wrapping_add(DIR_WORD)),
                    pos, speed.to_bits(), WIDE_ARG.to_bits(), 0, 0
                )
            };
            shared_tail(built, ped)
        } else {
            let t = kind.wrapping_sub(0xca);
            if t == 0 {
                // Push-1 step.
                let a = lf_checker_rt::callee_thiscall!(GATE_D, u32, glob(GLOBAL_TASK_HEAD));
                if a == 0 {
                    idle(ped)
                } else {
                    let r = lf_checker_rt::callee_thiscall!(BUILD_PUSH1, u32, a, 1);
                    if r == 0 { idle(ped) } else { r }
                }
            } else if t == 1 {
                // Small-8 step.
                let a = lf_checker_rt::callee_thiscall!(GATE_C, u32, glob(GLOBAL_TASK_HEAD));
                if a == 0 {
                    idle(ped)
                } else {
                    let r = lf_checker_rt::callee_thiscall!(
                        BUILD_SMALL8, u32, a, 1, 0, 0, SMALL8_ARG.to_bits()
                    );
                    if r == 0 { idle(ped) } else { r }
                }
            } else if t == 0x2ba {
                // Near step.
                locate(task, ped, pos);
                let anim = lf_checker_rt::callee_thiscall!(ANIM_OF, u32, task, ped);
                wr32(task.wrapping_add(ID_SLOT), glob(GLOBAL_ID_BASE));
                wr32(task.wrapping_add(ANIM_SLOT), anim);
                wr8(task.wrapping_add(STATUS_SLOT), 1);
                wr32(ped.wrapping_add(PED_FLAG_OFF), rd32(ped.wrapping_add(PED_FLAG_OFF)) | PED_FLAG_BIT);
                let a = lf_checker_rt::callee_thiscall!(GATE_A1, u32, glob(GLOBAL_TASK_HEAD));
                let built = if a == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(
                        BUILD_NEAR, u32, a,
                        rd32(task.wrapping_add(DIR_WORD)),
                        pos, speed.to_bits(), 1, 0
                    )
                };
                shared_tail(built, ped)
            } else {
                idle(ped)
            }
        }
    }
});
