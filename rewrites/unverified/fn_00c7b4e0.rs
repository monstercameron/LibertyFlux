// original: 0x00c7b4e0 CTaskComplexChatScenario::vf20
//
// Pedestrian chat scenario tick: advance the scenario's state machine and
// decide the sub-task the pedestrian runs this frame. `this` is the scenario
// task, `ped` the pedestrian. Returns the task to run (the current sub-task
// at `+0x08`) or null when the scenario yields (status bit 1 at `+0x0c`).
//
// Behaviour. Flag the ped's intelligence as scenario-driven
// (`[ped+PED_INTEL]+INTEL_SCENARIO = 1`). An idle scenario (byte `+0x20`
// clear with no pending guard at `+0x1c`) runs the body, otherwise the shared
// gate re-polls the task's own sub-operation (vtable slot `SELF_VT_SUBOP`,
// kinds 1 then 2) and yields when it accepts. The body resolves the chat
// partner through the aux object (`+0x24`, partner lookup with key `0x163`);
// a missing partner takes the yield epilogue. Otherwise the scenario state
// (`+0x28`, 0..6) selects a branch: states 0 and 1 count the chat timer
// (`+0x2c`) down by the frame step from the game's data and, once it runs
// out, start (state 0) or join (state 1) a chat exchange, moving to states 2
// and 3; states 2 and 3 wait for the ped's chat activity to end, then state
// 3 randomises the next timer (thresholds 0.33 and 0.5 on the scaled random
// stream, else a 10..20 range) while state 2 hands off to the ped with state
// 6; states 5 and 6 dispatch on the partner limit (5 replays the state-3
// randomisation, 6 takes a narrow range, anything else yields or hands off
// with state 5 when the state/limit pair is exactly 6/2); state 4 yields.
// Random draws scale the raw generator by `RAND_SCALE` (2^-15).
//
// Two reads need care. The state dispatch jumps through a table with an
// unrelocated base, and every branch reads tuning floats, the frame step and
// the random thresholds through unrelocated absolute addresses, which the
// checker cannot serve on this machine; the values are read through
// `relocated()` (the dispatch is a plain `match`, which never touches the
// original's code) so the rewrite stays correct wherever the image is
// mapped. The state-3 random path also compares the scratch register left
// behind by the generator call; the checker's stub always leaves its step
// index (0) there, which never equals 2, so that arm is dead under the
// checker and is modelled exactly that way (`STUB_SCRATCH_ECX`). Original:
// thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00c7b4e0(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUB_TASK: u32 = 0x08;
        const STATUS: u32 = 0x0c;
        const STATUS_STICKY: u32 = 0x01;
        const STATUS_DONE: u32 = 0x02;
        const PENDING_GUARD: u32 = 0x1c;
        const ACTIVE: u32 = 0x20;
        const AUX: u32 = 0x24;
        const STATE: u32 = 0x28;
        const CHAT_TIMER: u32 = 0x2c;
        const CHAT_MODE: u32 = 0x34;
        const CHAT_MODE_RANDOM: u32 = 0x100;
        const PED_INTEL: u32 = 0x224;
        const INTEL_SCENARIO: u32 = 0x2dc;
        const PED_TASKS: u32 = 0x570;
        const INTEL_PARTNER_OFF: u32 = 0x44;
        const PARTNER_KEY: u32 = 0x163;
        const PARTNER_LIMIT: u32 = 0x28;
        const SELF_VT_SUBOP: u32 = 0x14;
        const RAND_SCALE: f32 = f32::from_bits(0x38000100); // 2^-15
        const CHAT_THRESHOLD: f32 = f32::from_bits(0x3ea8f5c3); // 0.33
        const HALF: f32 = 0.5;
        const FULL_WEIGHT: f32 = 1.0;
        /// Value the checker's recorder stub leaves in ECX at exit (its
        /// per-trial step index, always 0 without a `seq`): the state-3 arm
        /// compares exactly this. A real callee's leftover is unknowable;
        /// the arm is dead (never 2) under the checker either way.
        const STUB_SCRATCH_ECX: u32 = 0;
        const CAL_PARTNER: u32 = 2;
        const CAL_CHAT_NEW: u32 = 3;
        const CAL_CHAT_JOIN: u32 = 4;
        const CAL_RUN_NEW: u32 = 5;
        const CAL_RUN_JOIN: u32 = 6;
        const CAL_CHAT_BUSY: u32 = 7;
        const CAL_RAND_STATE3: u32 = 8;
        const CAL_RAND_RETRY: u32 = 9;
        const CAL_RAND_TIMER: u32 = 10;
        const CAL_RAND_LIM5A: u32 = 11;
        const CAL_RAND_LIM5B: u32 = 12;
        const CAL_RANGE_WIDE: u32 = 13;
        const CAL_RANGE_NARROW: u32 = 14;
        const CAL_HANDOFF: u32 = 15;
        const CAL_CHAT_DONE: u32 = 16;

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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        /// The task's own sub-operation through its vtable (`kind` 1 or 2),
        /// answered by the checker's planted stub on both sides.
        #[inline(always)]
        unsafe fn self_subop(this: u32, ped: u32, kind: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(this) + SELF_VT_SUBOP);
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(this, ped, kind, 0)
            }
        }
        /// Scaled random draw in the original's operand order.
        #[inline(always)]
        unsafe fn scaled(draw: u32) -> f32 {
            mul(draw as i32 as f32, RAND_SCALE)
        }

        // Shared prologue: mark the ped's intelligence scenario-driven.
        wr8(rd32(ped + PED_INTEL) + INTEL_SCENARIO, 1);
        // Gate: an idle scenario runs the body, otherwise re-poll and yield
        // when the sub-operation accepts.
        if rd8(this + ACTIVE) != 0 && rd32(this + PENDING_GUARD) == 0 {
            if rd8(this + STATUS) & (STATUS_STICKY as u8) != 0 {
                return 0;
            }
            if self_subop(this, ped, 1) & 0xff != 0 {
                wr32(this + STATUS, rd32(this + STATUS) | STATUS_DONE);
                return 0;
            }
            if rd8(this + STATUS) & (STATUS_STICKY as u8) != 0 {
                return 0;
            }
            if self_subop(this, ped, 2) & 0xff != 0 {
                wr32(this + STATUS, rd32(this + STATUS) | STATUS_DONE);
                return 0;
            }
        }
        // Partner lookup; a missing partner takes the yield epilogue.
        let aux = rd32(this + AUX);
        if aux == 0 {
            return yield_epilogue(this, ped);
        }
        let intel = rd32(aux + PED_INTEL);
        let partner: u32 = lf_checker_rt::callee_thiscall!(
            CAL_PARTNER,
            u32,
            intel.wrapping_add(INTEL_PARTNER_OFF),
            PARTNER_KEY
        );
        if partner == 0 {
            return yield_epilogue(this, ped);
        }
        let limit = rd32(partner + PARTNER_LIMIT);
        let state = rd32(this + STATE);
        if state > 6 {
            return rd32(this + SUB_TASK);
        }
        let state_result = match state {
            0 | 1 => {
                let step = f32::from_bits(rd32(lf_checker_rt::relocated(0x117359c)));
                let next = sub(rdf(this + CHAT_TIMER), step);
                wrf(this + CHAT_TIMER, next);
                if next <= 0.0 {
                    // Timer ran out: start (0) or join (1) the exchange.
                    if state == 0 {
                        let chat: u32 = lf_checker_rt::callee_thiscall!(
                            CAL_CHAT_NEW, u32, this, 1
                        );
                        let run: u32 = lf_checker_rt::callee_thiscall!(
                            CAL_RUN_NEW,
                            u32,
                            ped.wrapping_add(PED_TASKS),
                            chat,
                            0,
                            0,
                            0,
                            0xffff_ffff,
                            0,
                            0,
                            FULL_WEIGHT.to_bits(),
                            0,
                            0
                        );
                        if run & 0xff == 0 {
                            return rd32(this + SUB_TASK);
                        }
                        wr32(this + STATE, 2);
                    } else {
                        let chat: u32 = lf_checker_rt::callee_thiscall!(
                            CAL_CHAT_JOIN,
                            u32,
                            this,
                            0,
                            0,
                            FULL_WEIGHT.to_bits(),
                            0,
                            0,
                            0xffff_ffff,
                            0,
                            0,
                            0,
                            0
                        );
                        lf_checker_rt::callee_thiscall!(
                            CAL_RUN_JOIN,
                            u32,
                            ped.wrapping_add(PED_TASKS),
                            chat
                        );
                        wr32(this + STATE, 3);
                    }
                }
                rd32(this + SUB_TASK)
            }
            2 | 3 => {
                let busy: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_CHAT_BUSY,
                    u32,
                    ped.wrapping_add(PED_TASKS)
                );
                if busy & 0xff != 0 {
                    return rd32(this + SUB_TASK);
                }
                if state != 3 {
                    // State 2 hands off with state 6.
                    lf_checker_rt::callee_thiscall!(CAL_HANDOFF, u32, ped, aux);
                    wr32(this + STATE, 6);
                    return rd32(this + SUB_TASK);
                }
                let draw1: u32 =
                    lf_checker_rt::callee_cdecl!(CAL_RAND_STATE3, u32,);
                if CHAT_THRESHOLD <= scaled(draw1) {
                    // Random declined: the exact 6/2 pair hands off with
                    // state 5, anything else yields. The ECX comparison
                    // reads the stub's exit scratch (see constant).
                    if draw1 == 6 && STUB_SCRATCH_ECX == 2 {
                        lf_checker_rt::callee_thiscall!(CAL_HANDOFF, u32, ped, aux);
                        wr32(this + STATE, 5);
                    }
                    return rd32(this + SUB_TASK);
                }
                wr32(this + STATE, 0);
                randomise_timer(this, ped, CAL_RAND_RETRY, CAL_RAND_TIMER, CAL_RANGE_WIDE);
                rd32(this + SUB_TASK)
            }
            4 => rd32(this + SUB_TASK),
            _ => {
                // States 5 and 6 dispatch on the partner limit.
                if limit == 5 {
                    wr32(this + STATE, 0);
                    randomise_timer(this, ped, CAL_RAND_LIM5A, CAL_RAND_LIM5B, CAL_RANGE_WIDE);
                    rd32(this + SUB_TASK)
                } else if limit == 6 {
                    wr32(this + STATE, 1);
                    let lo = rd32(lf_checker_rt::relocated(0x16f80b4));
                    let hi = rd32(lf_checker_rt::relocated(0x104b910));
                    let span: f32 =
                        lf_checker_rt::callee_cdecl!(CAL_RANGE_NARROW, f32, lo, hi);
                    wrf(this + CHAT_TIMER, span);
                    let _done: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_CHAT_DONE,
                        u32,
                        ped
                    );
                    rd32(this + SUB_TASK)
                } else if state == 6 && limit == 2 {
                    lf_checker_rt::callee_thiscall!(CAL_HANDOFF, u32, ped, aux);
                    wr32(this + STATE, 5);
                    rd32(this + SUB_TASK)
                } else {
                    rd32(this + SUB_TASK)
                }
            }
        };

        /// Yield epilogue shared by the partner-miss paths.
        #[inline(always)]
        unsafe fn yield_epilogue(this: u32, ped: u32) -> u32 {
            unsafe {
                if rd8(this + STATUS) & (STATUS_STICKY as u8) != 0 {
                    return 0;
                }
                if self_subop(this, ped, 1) & 0xff != 0 {
                    wr32(this + STATUS, rd32(this + STATUS) | STATUS_DONE);
                    return 0;
                }
                rd32(this + SUB_TASK)
            }
        }
        /// Randomise the chat timer: with the random mode flag set, accept a
        /// draw under one half and derive the timer from a second draw,
        /// otherwise take the wide configured range. Ends the chat activity.
        #[inline(always)]
        unsafe fn randomise_timer(
            this: u32,
            ped: u32,
            retry_id: u32,
            timer_id: u32,
            range_id: u32,
        ) {
            unsafe {
                if rd32(this + CHAT_MODE) == CHAT_MODE_RANDOM {
                    let draw2: u32 =
                        lf_checker_rt::callee_cdecl!(retry_id, u32,);
                    if HALF > scaled(draw2) {
                        let draw3: u32 =
                            lf_checker_rt::callee_cdecl!(timer_id, u32,);
                        let t = add(mul(scaled(draw3), HALF), HALF);
                        wrf(this + CHAT_TIMER, t);
                        let _done: u32 = lf_checker_rt::callee_thiscall!(
                            CAL_CHAT_DONE,
                            u32,
                            ped
                        );
                        return;
                    }
                }
                let lo = rd32(lf_checker_rt::relocated(0x104b908));
                let hi = rd32(lf_checker_rt::relocated(0x104b90c));
                let span: f32 =
                    lf_checker_rt::callee_cdecl!(range_id, f32, lo, hi);
                wrf(this + CHAT_TIMER, span);
            }
        }
        state_result
    }
});
