// original: 0x00ca8c40 CEventHandler::vf51
/// Answer a fire-threat event: validate the handler's scope object through
/// two virtual checks, fetch a three-word descriptor through a third, then
/// dispatch on the event kind.
///
/// `handler` points to the event handler (`+0x04` holds its ped, `+0x0c`
/// receives the new task). `event` points to the event record: `+0x10` is
/// the kind selector, `+0x20..0x28` a position, `+0x34` a radius float,
/// `+0x38` a spot word. The second and third stack arguments are not read.
///
/// Guards: a scope lookup off the ped must return non-null (else return
/// zero with nothing stored); virtual slot `+0x28` on it must approve (its
/// low byte set, else return its value with nothing stored); virtual slot
/// `+0x30` yields an inner object whose virtual slot `+0x1c` fills the
/// three-word descriptor. Kind 0x202 allocates a pool slot and constructs
/// a walk-round-fire task from the spot word, the event position, the
/// radius and the descriptor; kinds 0x38c and 0x38e build a smart-flee
/// task from the event position and two fixed constants instead. Kind
/// 0xc8 stores zero and returns the kind itself; anything else returns the
/// kind with nothing stored. A null pool slot stores zero and returns
/// zero. All kind comparisons are signed.
///
/// Original: 0x00ca8c40 (thiscall, three stack words; the second and third
/// are not read).
lf_checker_rt::export!(thiscall, rw_00ca8c40(handler: u32, event: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        const HANDLER_PED: u32 = 0x04;
        const HANDLER_TASK: u32 = 0x0c;
        const EVT_KIND: u32 = 0x10;
        const EVT_POS: u32 = 0x20;
        const EVT_RADIUS: u32 = 0x34;
        const EVT_SPOT: u32 = 0x38;
        const PED_SCOPE: u32 = 0x224;
        const SCOPE_OFF: u32 = 0x44;
        const VT_CHECK: u32 = 0x28;
        const VT_INNER: u32 = 0x30;
        const VT_FILL: u32 = 0x1c;
        const KIND_WALK: u32 = 0x202;
        const KIND_ZERO: u32 = 0xc8;
        const KIND_FLEE_LO: u32 = 0x38c;
        const KIND_FLEE_HI: u32 = 0x38e;
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const FLEE_RADIUS: u32 = 0x00eef920;
        const FLEE_PARAM: u32 = 0x00eef924;
        const SCOPE_LOOKUP: u32 = 1;
        const V_CHECK: u32 = 2;
        const V_INNER: u32 = 3;
        const V_FILL: u32 = 4;
        const ALLOC: u32 = 5;
        const WALK_ROUND: u32 = 6;
        const SMART_FLEE: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let own = rd32(handler + HANDLER_PED);
        let radius = rd32(event + EVT_RADIUS);
        let scope: u32 =
            lf_checker_rt::callee_thiscall!(SCOPE_LOOKUP, u32, rd32(own + PED_SCOPE) + SCOPE_OFF);
        if scope == 0 {
            return 0;
        }
        let vtab = rd32(scope);
        let check: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtab + VT_CHECK) as usize);
        let approved = check(scope);
        if (approved as u8) == 0 {
            return approved;
        }
        let inner_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtab + VT_INNER) as usize);
        let inner = inner_of(scope);
        let mut desc = [0u32; 3];
        let fill: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(inner) + VT_FILL) as usize);
        fill(inner, desc.as_mut_ptr() as u32);

        let spot = rd32(event + EVT_SPOT);
        let kind = rd32(event + EVT_KIND);
        if (kind as i32) > KIND_FLEE_LO as i32 {
            if kind != KIND_FLEE_HI {
                return kind;
            }
        } else if kind == KIND_FLEE_LO {
            // Falls through to the flee path below.
        } else if kind == KIND_ZERO {
            ((handler + HANDLER_TASK) as *mut u32).write_unaligned(0);
            return kind;
        } else if kind != KIND_WALK {
            return kind;
        } else {
            let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
            let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
            if slot == 0 {
                ((handler + HANDLER_TASK) as *mut u32).write_unaligned(0);
                return 0;
            }
            let task: u32 = lf_checker_rt::callee_thiscall!(
                WALK_ROUND,
                u32,
                slot,
                spot,
                event + EVT_POS,
                radius,
                desc.as_mut_ptr() as u32
            );
            ((handler + HANDLER_TASK) as *mut u32).write_unaligned(task);
            return task;
        }
        // Smart-flee path (kinds 0x38c and 0x38e).
        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if slot == 0 {
            ((handler + HANDLER_TASK) as *mut u32).write_unaligned(0);
            return 0;
        }
        let task: u32 = lf_checker_rt::callee_thiscall!(
            SMART_FLEE,
            u32,
            slot,
            event + EVT_POS,
            0,
            rd32(lf_checker_rt::relocated(FLEE_RADIUS)),
            rd32(lf_checker_rt::relocated(FLEE_PARAM)),
            0
        );
        ((handler + HANDLER_TASK) as *mut u32).write_unaligned(task);
        task
    }
});
