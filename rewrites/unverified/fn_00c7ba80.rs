// original: 0x00c7ba80 CTaskComplexMobileMakeCall::vf20
//
// Outgoing-call scenario tick: decide the sub-task the pedestrian runs this
// frame. `this` is the scenario task, `ped` the pedestrian.
// Returns the task to run (the current sub-task at `+0x08`) or null when the
// scenario yields (status bit 1 set at `+0x0c`).
//
// Behaviour. Unless status bit 1 is already set, run the dial-progress
// branch: when the "dialling" byte (`+0x49`) is set, count the dial timer
// (`+0x44`) down by the frame step from the game's data and, once it runs
// out, poll the ped's task state and remember whether it answered idle;
// otherwise count the same timer down and remember whether it expired. Then
// the partner-availability pair shared with the chat scenario (ped vtable
// slot `PED_VT_SCENARIO` with the scenario-global pointer, urgent retry with
// flag 8 resetting the timer to `TIMER_RETRY`; the retry also clears the
// proceed flag). Then the type gate: continue only when the current sub-task
// has the call type id (`TYPE_CALL`) and its child the dial-tone id
// (`TYPE_DIALTONE`); with a target selected (`+0x40`) measure the range to
// the ped's placement and hand it to the ped. If the proceed flag survived,
// build the next call sub-task and attach it to the saved sub-task when both
// exist. Otherwise fall through to the finish check (flag byte `+0x48`) and
// the shared yield epilogue (status bit 0 or the task's own sub-operation,
// vtable slot `SELF_VT_SUBOP`, kinds 1 then 2, accepting the ped).
//
// Reads game data through unrelocated absolute addresses (the frame-step
// float and the scenario-global pointer), which the checker cannot serve on
// this machine; the values are read through `relocated()` so the rewrite
// stays correct wherever the image is mapped. Original: thiscall, one stack
// word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00c7ba80(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUB_TASK: u32 = 0x08;
        const STATUS: u32 = 0x0c;
        const STATUS_STICKY: u32 = 0x01;
        const STATUS_DONE: u32 = 0x02;
        const TARGET: u32 = 0x40;
        const DIAL_TIMER: u32 = 0x44;
        const FINISH_FLAG: u32 = 0x48;
        const DIALLING: u32 = 0x49;
        const TIMER_RETRY: u32 = 0x3c23d70a; // 0.01f
        const PED_PLACEMENT: u32 = 0x20;
        const PED_TASKS: u32 = 0x570;
        const SELF_VT_SUBOP: u32 = 0x14;
        const PED_VT_SCENARIO: u32 = 0x12c;
        const SUB_VT_TYPE: u32 = 0x0c;
        const TYPE_CALL: u32 = 0x11d;
        const TYPE_DIALTONE: u32 = 0x11a;
        const URGENT: u32 = 8;
        const CAL_TASKS_IDLE: u32 = 3;
        const CAL_PARTNER_TEST: u32 = 4;
        const CAL_PARTNER_TEST2: u32 = 5;
        const CAL_RANGE: u32 = 8;
        const CAL_USE_RANGE: u32 = 9;
        const CAL_MAKE_SUB: u32 = 10;
        const CAL_ATTACH_SUB: u32 = 11;
        const CAL_BASE_SUBOP: u32 = 12;

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
        /// Task type id through a task object's vtable.
        #[inline(always)]
        unsafe fn type_id(task: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(task) + SUB_VT_TYPE);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(task)
            }
        }

        // Shared gate (this scenario has no intelligence prologue).
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
        // Dial-progress branch.
        let mut proceed: u8 = 0;
        let step = f32::from_bits(rd32(lf_checker_rt::relocated(0x11735bc)));
        if rd8(this + DIALLING) != 0 {
            let next = sub(rdf(this + DIAL_TIMER), step);
            wrf(this + DIAL_TIMER, next);
            if next <= 0.0 {
                let answered_idle: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_TASKS_IDLE,
                    u32,
                    ped.wrapping_add(PED_TASKS)
                );
                proceed = (answered_idle & 0xff == 0) as u8;
            }
        } else {
            let left = rdf(this + DIAL_TIMER);
            if left > 0.0 {
                let next = sub(left, step);
                wrf(this + DIAL_TIMER, next);
                if next <= 0.0 {
                    proceed = 1;
                }
            }
        }
        // Partner-availability pair, then the retry when it refuses. The
        // retry clears the proceed flag.
        let scenario_global = rd32(lf_checker_rt::relocated(0x12b4138));
        let seen = ped_query(ped, scenario_global, false);
        let ok: u32 =
            lf_checker_rt::callee_cdecl!(CAL_PARTNER_TEST, u32, seen);
        if ok & 0xff == 0 {
            let seen2 = ped_query(ped, scenario_global, true);
            let _retry: u32 =
                lf_checker_rt::callee_cdecl!(CAL_PARTNER_TEST2, u32, seen2);
            proceed = 0;
            wr32(this + DIAL_TIMER, TIMER_RETRY);
        }
        // Type gate: call sub-task with a dial-tone child, else skip ahead.
        let mut saved_sub: u32 = 0;
        if type_id(rd32(this + SUB_TASK)) == TYPE_CALL {
            saved_sub = rd32(this + SUB_TASK);
            let child = rd32(saved_sub + 0x14);
            if child != 0
                && type_id(child) == TYPE_DIALTONE
                && rd32(this + TARGET) != 0
            {
                let target = rd32(this + TARGET);
                let anchor = rd32(target + 0x20);
                let (base, other) = if anchor == 0 {
                    (target.wrapping_add(0x10), target.wrapping_add(0x10))
                } else {
                    (anchor.wrapping_add(0x30), anchor.wrapping_add(0x30))
                };
                let placement = rd32(ped + PED_PLACEMENT);
                let range: f32 = lf_checker_rt::callee_cdecl!(
                    CAL_RANGE,
                    f32,
                    rd32(base),
                    rd32(other.wrapping_add(4)),
                    rdf(placement.wrapping_add(0x30)).to_bits(),
                    rdf(placement.wrapping_add(0x34)).to_bits()
                );
                lf_checker_rt::callee_thiscall!(
                    CAL_USE_RANGE,
                    u32,
                    ped,
                    range.to_bits()
                );
            }
        }
        if proceed == 0 {
            return rd32(this + SUB_TASK);
        }
        let fresh: u32 = lf_checker_rt::callee_thiscall!(CAL_MAKE_SUB, u32, this, ped);
        if fresh == 0 || saved_sub == 0 {
            if rd8(this + FINISH_FLAG) == 0 {
                return rd32(this + SUB_TASK);
            }
        } else {
            lf_checker_rt::callee_thiscall!(CAL_ATTACH_SUB, u32, saved_sub, fresh);
            return rd32(this + SUB_TASK);
        }
        // Finish check and yield epilogue.
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
