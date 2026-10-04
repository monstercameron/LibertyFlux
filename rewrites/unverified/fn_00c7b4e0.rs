// original: 0x00c7b4e0 CTaskComplexChatScenario::vf20

/// Chat-scenario task update: advance one think tick of a ped's ambient-chat
/// state machine and report the task id, or finish the task.
///
/// `this` is the task object, `ped` the ped it runs on (thiscall: `this` in
/// ECX, `ped` the single stack word; callee pops 4). Returns the task id at
/// `this+0x08`, or 0 once the task finishes (flag bit 1 set at `this+0x0c`).
///
/// Layout read: ped `+0x224` points at an intel block whose byte `+0x2dc` is
/// set to 1 on entry; task bytes `+0x20`/`+0x1c` gate the entry check, dword
/// `+0x0c` holds finish flag bit 0 (re-entrancy guard) and bit 1 (finished),
/// `+0x24` a sub-object whose `+0x224` block (plus `0x44`) is searched for a
/// task of type `0x163`, `+0x28` the chat state (0..6, anything else returns
/// the id), `+0x2c` a countdown timer, `+0x34` a second kind field.
///
/// Behaviour: when the entry gate (`+0x20` set, `+0x1c` clear) is open, the
/// task first offers the ped to its own virtual slot `+0x14` with kinds 1
/// then 2; a true answer finishes. Otherwise the found sub-task's kind
/// (`+0x28`) and the chat state drive a jump table: states 0/1 count the
/// timer down by the frame delta and then start a speech line (state 0 needs
/// the line to start, else the id is returned; state 1 always advances);
/// states 2/3 wait while ambient speech plays, state 3 otherwise rolling two
/// random gates before drawing a fresh timer; state 4 retires the sub-task;
/// states 5/6 dispatch on the sub-task kind (5 redraws a timer or retires,
/// 6 restarts at state 1, kind 2 with state 6 retires, anything else returns
/// the id). When no sub-task is found the task offers kind 1 once more and
/// otherwise returns the id.
///
/// Float order is the original's: timer decrement, `int_bits as f32` scaled
/// by the random factor, and the double-scaled timer redraw. All `comiss`
/// branches keep NaN semantics (`jb`/`jbe` taken on unordered).
lf_checker_rt::export!(thiscall, rw_00c7b4e0(this: u32, ped: u32) -> u32 {
    /// Shared tail when no sub-task was found: offer kind 1 once, else id.
    unsafe fn tail_offer(this: u32, ped: u32, id: u32) -> u32 {
        unsafe {
            const TASK_FLAGS: u32 = 0x0c;
            const FINISH_GUARD: u32 = 0x01;
            const FINISHED: u32 = 0x02;
            const VT_OFFER: u32 = 0x14;
            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            #[inline(always)]
            unsafe fn wr32(a: u32, v: u32) {
                unsafe { (a as *mut u32).write_unaligned(v) }
            }
            if rd32(this + TASK_FLAGS) & FINISH_GUARD != 0 {
                return 0;
            }
            let slot = rd32(rd32(this) + VT_OFFER);
            let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            if (f(this, ped, 1, 0) & 0xff) != 0 {
                wr32(this + TASK_FLAGS, rd32(this + TASK_FLAGS) | FINISHED);
                return 0;
            }
            id
        }
    }

    /// Retire the sub-task from the ped.
    unsafe fn retire_sub(this: u32, ped: u32) {
        unsafe {
            const TASK_SUB: u32 = 0x24;
            const C_RETIRE_SUB: u32 = 8;
            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            lf_checker_rt::callee_thiscall!(
                C_RETIRE_SUB,
                u32,
                ped,
                rd32(this + TASK_SUB)
            );
        }
    }

    /// Draw a fresh (10, 20) timer through the float-range callee.
    unsafe fn redraw_timer(this: u32) {
        unsafe {
            const TASK_TIMER: u32 = 0x2c;
            const C_FRAND: u32 = 7;
            const G_FRAND_LO: u32 = 0x0104b908;
            const G_FRAND_HI: u32 = 0x0104b90c;
            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            let lo = rd32(lf_checker_rt::relocated(G_FRAND_LO));
            let hi = rd32(lf_checker_rt::relocated(G_FRAND_HI));
            let t = lf_checker_rt::callee_cdecl!(C_FRAND, f32, lo, hi);
            ((this + TASK_TIMER) as *mut u32).write_unaligned(t.to_bits());
        }
    }

    unsafe {
        const PED_INTEL: u32 = 0x224;
        const INTEL_CHAT_FLAG: u32 = 0x2dc;
        const TASK_ID: u32 = 0x08;
        const TASK_FLAGS: u32 = 0x0c;
        const FINISH_GUARD: u32 = 0x01;
        const FINISHED: u32 = 0x02;
        const TASK_GATE_B: u32 = 0x20;
        const TASK_GATE_W: u32 = 0x1c;
        const TASK_SUB: u32 = 0x24;
        const TASK_STATE: u32 = 0x28;
        const TASK_TIMER: u32 = 0x2c;
        const TASK_KIND2: u32 = 0x34;
        const SUB_INTEL: u32 = 0x224;
        const SUB_SEARCH_BIAS: u32 = 0x44;
        const FOUND_KIND: u32 = 0x28;
        const FIND_TYPE: u32 = 0x163;
        const KIND2_RANDOM: u32 = 0x100;
        const VT_OFFER: u32 = 0x14;
        const SPEECH_OBJ: u32 = 0x570;
        const V_OFFER: u32 = 1;
        const C_FIND_BY_TYPE: u32 = 2;
        const C_CONV_ID: u32 = 3;
        const C_PED_SAY: u32 = 4;
        const C_AMBIENT_PLAYING: u32 = 5;
        const C_RAND: u32 = 6;
        const C_FRAND: u32 = 7;
        const C_RETIRE_SUB: u32 = 8;
        const C_TOUCH_PED: u32 = 9;
        const G_DT: u32 = 0x0117359c;
        const G_RAND_K: u32 = 0x00fe8684;
        const G_GATE1: u32 = 0x00fe8800;
        const G_GATE2: u32 = 0x00fe8830;
        const G_FRAND_LO: u32 = 0x0104b908;
        const G_FRAND_HI: u32 = 0x0104b90c;
        const G_FRAND6_HI: u32 = 0x0104b910;
        const G_FRAND6_LO: u32 = 0x016f80b4;
        const SAY_VOLUME: u32 = 0x3f800000;

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
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
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
        /// Virtual slot `+0x14` offer: (ped, kind, 0) with `this` in ECX.
        #[inline(always)]
        unsafe fn offer(this: u32, ped: u32, kind: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(this) + VT_OFFER);
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(this, ped, kind, 0)
            }
        }
        #[inline(always)]
        unsafe fn finish(this: u32) -> u32 {
            unsafe {
                wr32(this + TASK_FLAGS, rd32(this + TASK_FLAGS) | FINISHED);
                0
            }
        }

        // Entry: mark the ped's intel block, then the gated double offer.
        wr8(rd32(ped + PED_INTEL) + INTEL_CHAT_FLAG, 1);
        let id = rd32(this + TASK_ID);
        if rd8(this + TASK_GATE_B) != 0 && rd32(this + TASK_GATE_W) == 0 {
            // Guard already set: return 0 without touching the flags.
            if rd32(this + TASK_FLAGS) & FINISH_GUARD != 0 {
                return 0;
            }
            if (offer(this, ped, 1) & 0xff) != 0 {
                return finish(this);
            }
            if rd32(this + TASK_FLAGS) & FINISH_GUARD != 0 {
                return 0;
            }
            if (offer(this, ped, 2) & 0xff) != 0 {
                return finish(this);
            }
        }
        // Locate the sub-task of the wanted type.
        let subtask = rd32(this + TASK_SUB);
        if subtask == 0 {
            return tail_offer(this, ped, id);
        }
        let found = lf_checker_rt::callee_thiscall!(
            C_FIND_BY_TYPE,
            u32,
            rd32(subtask + SUB_INTEL).wrapping_add(SUB_SEARCH_BIAS),
            FIND_TYPE
        );
        if found == 0 {
            return tail_offer(this, ped, id);
        }
        let kind = rd32(found + FOUND_KIND);
        let state = rd32(this + TASK_STATE);
        match state {
            0 | 1 => {
                let left = sub(rdf(this + TASK_TIMER), gf(G_DT));
                wrf(this + TASK_TIMER, left);
                // jb: taken while the timer is still positive (or NaN).
                if !(0.0f32 >= left) {
                    return id;
                }
                let speech = ped.wrapping_add(SPEECH_OBJ);
                if state == 0 {
                    let cid =
                        lf_checker_rt::callee_thiscall!(C_CONV_ID, u32, this, 1);
                    let said = lf_checker_rt::callee_thiscall!(
                        C_PED_SAY, u32, speech, cid, 0, 0, 0, 0xffff_ffff, 0,
                        0, SAY_VOLUME, 0, 0
                    );
                    if (said & 0xff) == 0 {
                        return id;
                    }
                    wr32(this + TASK_STATE, 2);
                    id
                } else {
                    let cid =
                        lf_checker_rt::callee_thiscall!(C_CONV_ID, u32, this, 0);
                    lf_checker_rt::callee_thiscall!(
                        C_PED_SAY, u32, speech, cid, 0, 0, 0, 0xffff_ffff, 0,
                        0, SAY_VOLUME, 0, 0
                    );
                    wr32(this + TASK_STATE, 3);
                    id
                }
            }
            2 | 3 => {
                let playing = lf_checker_rt::callee_thiscall!(
                    C_AMBIENT_PLAYING,
                    u32,
                    ped.wrapping_add(SPEECH_OBJ)
                );
                if (playing & 0xff) != 0 {
                    return id;
                }
                if state != 3 {
                    retire_sub(this, ped);
                    wr32(this + TASK_STATE, 6);
                    return id;
                }
                let gate1 = gf(G_GATE1);
                let r = lf_checker_rt::callee_cdecl!(C_RAND, u32,);
                let x = mul((r as i32) as f32, gf(G_RAND_K));
                // jbe: retire unless the roll is below the first gate.
                if !(gate1 > x) {
                    retire_sub(this, ped);
                    wr32(this + TASK_STATE, 5);
                    return id;
                }
                wr32(this + TASK_STATE, 0);
                if rd32(this + TASK_KIND2) != KIND2_RANDOM {
                    redraw_timer(this);
                    return id;
                }
                let gate2 = gf(G_GATE2);
                let r = lf_checker_rt::callee_cdecl!(C_RAND, u32,);
                let x = mul((r as i32) as f32, gf(G_RAND_K));
                if !(gate2 > x) {
                    redraw_timer(this);
                    return id;
                }
                let r = lf_checker_rt::callee_cdecl!(C_RAND, u32,);
                let half = gf(G_GATE2);
                let t = add(mul(mul((r as i32) as f32, gf(G_RAND_K)), half), half);
                wrf(this + TASK_TIMER, t);
                id
            }
            4 => {
                retire_sub(this, ped);
                wr32(this + TASK_STATE, 5);
                id
            }
            5 | 6 => {
                if kind == 5 {
                    wr32(this + TASK_STATE, 0);
                    if rd32(this + TASK_KIND2) == KIND2_RANDOM {
                        let gate2 = gf(G_GATE2);
                        let r = lf_checker_rt::callee_cdecl!(C_RAND, u32,);
                        let x = mul((r as i32) as f32, gf(G_RAND_K));
                        if gate2 > x {
                            let r = lf_checker_rt::callee_cdecl!(C_RAND, u32,);
                            let half = gf(G_GATE2);
                            let t = add(
                                mul(mul((r as i32) as f32, gf(G_RAND_K)), half),
                                half,
                            );
                            wrf(this + TASK_TIMER, t);
                            lf_checker_rt::callee_thiscall!(C_TOUCH_PED, u32, ped);
                            return id;
                        }
                    }
                    redraw_timer(this);
                    lf_checker_rt::callee_thiscall!(C_TOUCH_PED, u32, ped);
                    id
                } else if kind == 6 {
                    wr32(this + TASK_STATE, 1);
                    let lo = g32(G_FRAND6_LO);
                    let hi = g32(G_FRAND6_HI);
                    let t = lf_checker_rt::callee_cdecl!(C_FRAND, f32, lo, hi);
                    wrf(this + TASK_TIMER, t);
                    lf_checker_rt::callee_thiscall!(C_TOUCH_PED, u32, ped);
                    id
                } else if state == 6 && kind == 2 {
                    retire_sub(this, ped);
                    wr32(this + TASK_STATE, 5);
                    id
                } else {
                    id
                }
            }
            _ => id,
        }
    }
});
