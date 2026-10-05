// original: 0x00C7DE30 CTaskComplexMobileMakeCall::vf19

/// Mobile-make-call task update: advance the call scenario for the ped.
///
/// `this` is the call task, `ped` the ped performing it. Callee 1 resolves
/// the ped's call state; a null answer ends the tick with 0 and its value
/// is otherwise carried into the final chaining call. The tick then splits
/// on the task's phase word at `+0x14`: phase 1 runs the proximity check,
/// phase 0 skips straight to the wait task, and any other phase skips to
/// the final chaining call.
///
/// The proximity check compares the ped's ground position (from its matrix
/// at `[ped+0x20]+0x30/0x34`) against the other party's position: the party
/// link at `this+0x40`, when null, substitutes the ped's own position (so
/// the distance is 0); otherwise the base is `[party+0x20]+0x30` if the
/// party has a matrix, else `party+0x10`. The squared ground distance must
/// be strictly below `G_A*G_A*G_B` (an unordered comparison falls through
/// to the wait task). When close, the base's four position words are
/// copied into a stack buffer and offered, with the globals `G_D`/`G_C`,
/// to the scenario builder (callee 3, reached through the task-system
/// singleton from `G_TASK_SYS`); its result becomes the pending sub-task
/// and is stamped with `G_A` at `+0x58`. A null singleton on this path
/// writes the stamp through a null pointer and faults, exactly as the
/// original does.
///
/// The wait path (phase 0 or out of range) asks the singleton for an
/// 8-second wait task (callee 4); a null singleton leaves a null pending
/// sub-task. Every path converges on the chaining call (callee 5) with the
/// pending sub-task and the carried call state, whose result the tick
/// returns; a null singleton there ends the tick with 0.
///
/// Original: 0x00C7DE30 (thiscall, one stack word, returns eax).
lf_checker_rt::export!(thiscall, rw_00c7de30(this: u32, ped: u32) -> u32 {
    unsafe {
        const PHASE: u32 = 0x14;
        const PARTY: u32 = 0x40;
        const PED_MATRIX: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const POS_Z: u32 = 0x38;
        const POS_W: u32 = 0x3C;
        const PARTY_MATRIX: u32 = 0x20;
        const PARTY_LOCAL: u32 = 0x10;
        const STAMP: u32 = 0x58;
        const G_RANGE: u32 = 0x0104B94C;
        const G_RANGE_MUL: u32 = 0x00FE87E4;
        const G_C: u32 = 0x00EE1EB4;
        const G_D: u32 = 0x00EE1EB0;
        const BUILD_MODE: u32 = 3;
        const C_STATE: u32 = 1;
        const C_BUILD: u32 = 3;
        const C_CHAIN: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        unsafe fn wait_task() -> u32 {
            unsafe {
                const G_TASK_SYS: u32 = 0x0167E2A0;
                const WAIT_SECS_BITS: u32 = 0x41000000;
                const C_SINGLETON: u32 = 2;
                const C_WAIT: u32 = 4;
                let sys: u32 = lf_checker_rt::callee_thiscall!(
                    C_SINGLETON,
                    u32,
                    lf_checker_rt::global::<u32>(G_TASK_SYS).read()
                );
                if sys == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(C_WAIT, u32, sys, 0, 1, 0, WAIT_SECS_BITS)
            }
        }
        #[inline(always)]
        unsafe fn singleton() -> u32 {
            unsafe {
                const G_TASK_SYS: u32 = 0x0167E2A0;
                const C_SINGLETON: u32 = 2;
                lf_checker_rt::callee_thiscall!(
                    C_SINGLETON,
                    u32,
                    lf_checker_rt::global::<u32>(G_TASK_SYS).read()
                )
            }
        }

        let state: u32 = lf_checker_rt::callee_thiscall!(C_STATE, u32, this, ped);
        if state == 0 {
            return 0;
        }
        let phase = rd32(this.wrapping_add(PHASE));
        let mut pending: u32 = 0;
        if phase == 1 {
            let pedm = rd32(ped.wrapping_add(PED_MATRIX));
            let ped_x = rdf(pedm.wrapping_add(POS_X));
            let ped_y = rdf(pedm.wrapping_add(POS_Y));
            let party = rd32(this.wrapping_add(PARTY));
            let mut buf = [0u32; 4];
            let close = if party == 0 {
                buf[0] = ped_x.to_bits();
                buf[1] = ped_y.to_bits();
                buf[2] = rd32(pedm.wrapping_add(POS_Z));
                buf[3] = rd32(pedm.wrapping_add(POS_W));
                true
            } else {
                let partm = rd32(party.wrapping_add(PARTY_MATRIX));
                let base = if partm == 0 {
                    party.wrapping_add(PARTY_LOCAL)
                } else {
                    partm.wrapping_add(POS_X)
                };
                let dx = sub(ped_x, rdf(base));
                let dy = sub(ped_y, rdf(base.wrapping_add(4)));
                let dist2 = add(mul(dy, dy), mul(dx, dx));
                let range = rdf(lf_checker_rt::relocated(G_RANGE));
                let mut thr = mul(range, range);
                thr = mul(thr, rdf(lf_checker_rt::relocated(G_RANGE_MUL)));
                if thr > dist2 {
                    buf[0] = rd32(base);
                    buf[1] = rd32(base.wrapping_add(4));
                    buf[2] = rd32(base.wrapping_add(8));
                    buf[3] = rd32(base.wrapping_add(12));
                    true
                } else {
                    false
                }
            };
            if close {
                let sys = singleton();
                let stamp = rd32(lf_checker_rt::relocated(G_RANGE));
                if sys == 0 {
                    // The original stamps through the null pending
                    // sub-task here and faults; keep the fault.
                    (pending.wrapping_add(STAMP) as *mut u32)
                        .write_unaligned(stamp);
                    return 0;
                }
                let built: u32 = lf_checker_rt::callee_thiscall!(
                    C_BUILD,
                    u32,
                    sys,
                    BUILD_MODE,
                    buf.as_ptr() as u32,
                    rd32(lf_checker_rt::relocated(G_D)),
                    rd32(lf_checker_rt::relocated(G_C)),
                    0xFFFFFFFF,
                    1,
                    1,
                    0,
                    0,
                    1
                );
                pending = built;
                (pending.wrapping_add(STAMP) as *mut u32).write_unaligned(stamp);
            } else {
                pending = wait_task();
            }
        } else if phase == 0 {
            pending = wait_task();
        }
        let sys = singleton();
        if sys == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(C_CHAIN, u32, sys, pending, state, 0, 0)
    }
});
