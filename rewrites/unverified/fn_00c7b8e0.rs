// original: 0x00c7b8e0 CTaskComplexMobileChatScenario::vf20
//
// Mobile-phone chat scenario tick: decide the sub-task the pedestrian runs
// this frame. `this` is the scenario task, `ped` the pedestrian.
// Returns the task to run (the current sub-task at `+0x08`) or null when the
// scenario finishes a chat branch and yields (status bit 1 set at `+0x0c`).
//
// Behaviour. Flag the ped's intelligence as scenario-driven
// (`[ped+PED_INTEL]+INTEL_SCENARIO = 1`). Unless status bit 1 is already set,
// run the chat-progress branch: when the "waiting" byte (`+0x31`) is set,
// poll the ped's vehicle/phone state (expects id 7 for the phone path,
// otherwise the generic path) and remember whether it answered idle;
// otherwise count the chat timer (`+0x2c`) down by the frame step read from
// the game's data and remember whether it expired. Then ask the ped (through
// its vtable slot `PED_VT_SCENARIO`, given a scenario-global pointer) whether
// the chat partner is available; if not, retry with the urgent flag (8) and
// reset the timer to `TIMER_RETRY`. If the partner answered and the timer
// branch allowed it, build the next chat sub-task; when the current sub-task
// has the chat type id (`TYPE_CHAT`), attach the new one to it. Otherwise
// fall through to the finish checks (flag byte `+0x30`, aux ids `AUX_A/B`
// triggering ped-group calls) and the shared yield epilogue: yield (return
// null, set bit 1) when status bit 0 is set or the task's own sub-operation
// (vtable slot `SELF_VT_SUBOP`, kinds 1 then 2) accepts the ped.
//
// Reads game data through unrelocated absolute addresses (the frame-step
// float and the scenario-global pointer), which the checker cannot serve on
// this machine; the values are read through `relocated()` so the rewrite
// stays correct wherever the image is mapped. Original: thiscall, one stack
// word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00c7b8e0(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUB_TASK: u32 = 0x08;
        const STATUS: u32 = 0x0c;
        const STATUS_STICKY: u32 = 0x01;
        const STATUS_DONE: u32 = 0x02;
        const AUX_ID: u32 = 0x14;
        const AUX_A: u32 = 0x5d;
        const AUX_B: u32 = 0x2b;
        const PHONE_KIND: u32 = 7;
        const PHONE_KIND_SLOT: u32 = 0x24;
        const WAIT_FLAG: u32 = 0x31;
        const FINISH_FLAG: u32 = 0x30;
        const CHAT_TIMER: u32 = 0x2c;
        const TIMER_RETRY: u32 = 0x3c23d70a; // 0.01f
        const PED_INTEL: u32 = 0x224;
        const INTEL_SCENARIO: u32 = 0x2dc;
        const PED_PHONE: u32 = 0x3c0;
        const PED_TASKS: u32 = 0x570;
        const PED_GROUP: u32 = 0x2b0;
        const SELF_VT_SUBOP: u32 = 0x14;
        const PED_VT_SCENARIO: u32 = 0x12c;
        const SUB_VT_TYPE: u32 = 0x0c;
        const TYPE_CHAT: u32 = 0x11d;
        const URGENT: u32 = 8;
        const CAL_PHONE_IDLE: u32 = 2;
        const CAL_TASKS_IDLE: u32 = 3;
        const CAL_PARTNER_TEST: u32 = 4;
        const CAL_PARTNER_TEST2: u32 = 5;
        const CAL_MAKE_SUB: u32 = 6;
        const CAL_ATTACH_SUB: u32 = 7;
        const CAL_GROUP_OP: u32 = 8;
        const CAL_BASE_SUBOP: u32 = 9;

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
        /// Ped scenario query through the ped's vtable; `urgent` selects the
        /// one- or two-argument shape.
        #[inline(always)]
        unsafe fn ped_query(ped: u32, scenario_global: u32, urgent: bool) -> u32 {
            unsafe {
                let slot = rd32(rd32(ped) + PED_VT_SCENARIO);
                if urgent {
                    let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    f(ped, scenario_global, URGENT)
                } else {
                    let f: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    f(ped, scenario_global)
                }
            }
        }

        // Shared prologue: mark the ped's intelligence scenario-driven.
        wr8(rd32(ped + PED_INTEL) + INTEL_SCENARIO, 1);
        // Shared gate: a finished scenario (bit 1) re-polls the
        // sub-operation and yields when it accepts; otherwise run the body.
        if rd32(this + STATUS) & STATUS_DONE != 0 {
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
        // Chat-progress branch.
        let mut proceed: u8 = 0;
        if rd8(this + WAIT_FLAG) != 0 {
            let answered_idle: u32 = if rd32(this + PHONE_KIND_SLOT) == PHONE_KIND {
                lf_checker_rt::callee_thiscall!(CAL_PHONE_IDLE, u32, ped.wrapping_add(PED_PHONE))
            } else {
                lf_checker_rt::callee_thiscall!(CAL_TASKS_IDLE, u32, ped.wrapping_add(PED_TASKS))
            };
            proceed = (answered_idle & 0xff == 0) as u8;
        } else {
            let left = rdf(this + CHAT_TIMER);
            if left > 0.0 {
                let step = f32::from_bits(rd32(lf_checker_rt::relocated(0x11735bc)));
                let next = sub(left, step);
                wrf(this + CHAT_TIMER, next);
                if next <= 0.0 {
                    proceed = 1;
                }
            }
        }
        // Partner-availability pair, then the retry when it refuses.
        let scenario_global = rd32(lf_checker_rt::relocated(0x12b4138));
        let seen = ped_query(ped, scenario_global, false);
        let ok: u32 =
            lf_checker_rt::callee_cdecl!(CAL_PARTNER_TEST, u32, seen);
        if ok & 0xff == 0 {
            let seen2 = ped_query(ped, scenario_global, true);
            let _retry: u32 =
                lf_checker_rt::callee_cdecl!(CAL_PARTNER_TEST2, u32, seen2);
            wr32(this + CHAT_TIMER, TIMER_RETRY);
            return rd32(this + SUB_TASK);
        }
        if proceed == 0 {
            return rd32(this + SUB_TASK);
        }
        let fresh: u32 = lf_checker_rt::callee_thiscall!(CAL_MAKE_SUB, u32, this, ped);
        if fresh != 0 {
            let sub = rd32(this + SUB_TASK);
            let type_slot = rd32(rd32(sub) + SUB_VT_TYPE);
            let is_chat: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(type_slot as usize);
            if is_chat(sub) == TYPE_CHAT {
                lf_checker_rt::callee_thiscall!(CAL_ATTACH_SUB, u32, sub, fresh);
                return rd32(this + SUB_TASK);
            }
        }
        // Finish checks.
        if rd8(this + FINISH_FLAG) == 0 {
            return rd32(this + SUB_TASK);
        }
        if rd32(this + AUX_ID) == AUX_A {
            let _r: u32 =
                lf_checker_rt::callee_thiscall!(CAL_GROUP_OP, u32, ped.wrapping_add(PED_GROUP));
        }
        if rd32(this + AUX_ID) == AUX_B {
            let _r: u32 =
                lf_checker_rt::callee_thiscall!(CAL_GROUP_OP, u32, ped.wrapping_add(PED_GROUP));
        }
        // Yield epilogue.
        if rd8(this + STATUS) & (STATUS_STICKY as u8) != 0 {
            return 0;
        }
        if self_subop(this, ped, 1) & 0xff != 0 {
            wr32(this + STATUS, rd32(this + STATUS) | STATUS_DONE);
            return 0;
        }
        let base: u32 =
            lf_checker_rt::callee_thiscall!(CAL_BASE_SUBOP, u32, this, ped, 2, 0);
        if base & 0xff != 0 {
            return 0;
        }
        rd32(this + SUB_TASK)
    }
});
