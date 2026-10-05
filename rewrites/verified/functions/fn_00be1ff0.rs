// original: 0x00be1ff0 CTaskComplexSitIdle::vf20

/// Sit-idle task update: run the seated sub-behaviour, or pick a new one.
///
/// `this` is the complex task, `ped` the pedestrian it drives. The task keeps
/// its running subtask at `+0x08` and two state bytes at `+0x3C`/`+0x3D`. The
/// pedestrian carries a vehicle-or-prop pointer at `+0x2C4`, a seat-state word
/// at `+0x368` and an animation block at `+0x2B0`.
///
/// Fast path: the task's gate hook (virtual slot `+0x54`) accepts the
/// pedestrian and the subtask is running or starts (virtual slot `+0x14`,
/// started flag bit `0x02` in the subtask's status byte at `+0x0C`). While
/// the pedestrian is seated (seat state `3`) the update looks its seat up,
/// attaches the seat prop, applies the sit pose and clears the seat state.
/// Returns 0.
///
/// Slow path (gate or start refused): when both state bytes are set and the
/// subtask's type (virtual slot `+0x0C`) is `0x123`, a new task is requested
/// from the task pool (global pointer) and configured; otherwise, when no
/// state byte forces a restart and the type is `0x119`, a different pool task
/// is requested and configured with a blend factor of `-1.0`. If neither
/// applies the running subtask is returned unchanged.
///
/// Original: 0x00be1ff0 (thiscall, one stack word, returns a task pointer
/// or 0).
lf_checker_rt::export!(thiscall, rw_00be1ff0(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 0x08;
        const SUB_STATUS: u32 = 0x0c;
        const SUB_STARTED: u32 = 0x02;
        const SUB_ALIVE: u8 = 0x01;
        const STATE_A: u32 = 0x3c;
        const STATE_B: u32 = 0x3d;
        const PED_SEAT_LINK: u32 = 0x2c4;
        const PED_SEAT_STATE: u32 = 0x368;
        const PED_ANIM: u32 = 0x2b0;
        const PED_PROP: u32 = 0x20;
        const SEATED: u32 = 3;
        const SIT_POSE: u32 = 0x11;
        const TYPE_SIT: u32 = 0x123;
        const TYPE_IDLE: u32 = 0x119;
        const VT_GATE: u32 = 0x54;
        const VT_START: u32 = 0x14;
        const VT_TYPE: u32 = 0x0c;
        const TASK_POOL: u32 = 0x167e2a0;
        const BLEND_MINUS_ONE: u32 = 0xbf80_0000;

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

        /// Virtual gate hook on the task itself.
        unsafe fn gate(task: u32, ped: u32) -> u8 {
            unsafe {
                let slot = rd32(rd32(task) + VT_GATE);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                (f(task, ped) & 0xff) as u8
            }
        }
        /// Virtual start hook on the subtask.
        unsafe fn start_sub(sub: u32, ped: u32) -> u8 {
            unsafe {
                let slot = rd32(rd32(sub) + VT_START);
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                (f(sub, ped, 2, 0) & 0xff) as u8
            }
        }
        /// Virtual type query on the subtask.
        unsafe fn sub_type(sub: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(sub) + VT_TYPE);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(sub)
            }
        }
        /// Start the subtask unless already alive; mark it started.
        /// Returns false when the start hook refuses (caller returns `sub`).
        unsafe fn ensure_started(sub: u32, ped: u32) -> bool {
            unsafe {
                if rd8(sub + SUB_STATUS) & SUB_ALIVE == 0 {
                    if start_sub(sub, ped) == 0 {
                        return false;
                    }
                    wr32(sub + SUB_STATUS, rd32(sub + SUB_STATUS) | SUB_STARTED);
                }
                true
            }
        }

        if gate(this, ped) != 0 {
            let sub = rd32(this + SUBTASK);
            if ensure_started(sub, ped) {
                if rd32(ped + PED_SEAT_LINK) != 0 && rd32(ped + PED_SEAT_STATE) == SEATED {
                    let seat = lf_checker_rt::callee_thiscall!(1, u32, ped + PED_ANIM, ped, 1);
                    if seat != 0 {
                        let prop = rd32(ped + PED_PROP).wrapping_add(0x10);
                        lf_checker_rt::callee_thiscall!(2, u32, seat, prop);
                    }
                    lf_checker_rt::callee_thiscall!(3, u32, ped + PED_ANIM, SIT_POSE, 0, 0, 0);
                    wr32(ped + PED_SEAT_STATE, 0);
                }
                return 0;
            }
            return slow_path(this, ped);
        }
        slow_path(this, ped)
    }
});

/// Slow path of [`rw_00be1ff0`]: pick a replacement sub-behaviour.
#[inline(never)]
unsafe fn slow_path(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 0x08;
        const SUB_STATUS: u32 = 0x0c;
        const SUB_STARTED: u32 = 0x02;
        const SUB_ALIVE: u8 = 0x01;
        const STATE_A: u32 = 0x3c;
        const STATE_B: u32 = 0x3d;
        const TYPE_SIT: u32 = 0x123;
        const TYPE_IDLE: u32 = 0x119;
        const VT_START: u32 = 0x14;
        const VT_TYPE: u32 = 0x0c;
        const TASK_POOL: u32 = 0x167e2a0;
        const BLEND_MINUS_ONE: u32 = 0xbf80_0000;

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
        unsafe fn start_sub(sub: u32, ped: u32) -> u8 {
            unsafe {
                let slot = rd32(rd32(sub) + VT_START);
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                (f(sub, ped, 2, 0) & 0xff) as u8
            }
        }
        unsafe fn sub_type(sub: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(sub) + VT_TYPE);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(sub)
            }
        }

        let sub = rd32(this + SUBTASK);
        let state_a = rd8(this + STATE_A) != 0;
        let state_b = rd8(this + STATE_B) != 0;
        if state_a && state_b {
            // Both states set: the sit branch, else the subtask is kept.
            if sub_type(sub) != TYPE_SIT {
                return sub;
            }
        } else if sub_type(sub) != TYPE_IDLE {
            // No forced restart: the idle branch, else the subtask is kept.
            return sub;
        }
        if rd8(sub + SUB_STATUS) & SUB_ALIVE == 0 {
            if start_sub(sub, ped) == 0 {
                return sub;
            }
            wr32(sub + SUB_STATUS, rd32(sub + SUB_STATUS) | SUB_STARTED);
        }
        let pool: u32 = rd32(lf_checker_rt::relocated(TASK_POOL));
        let task = lf_checker_rt::callee_thiscall!(4, u32, pool);
        if task == 0 {
            return 0;
        }
        if state_a && state_b {
            lf_checker_rt::callee_thiscall!(5, u32, task, 0, 0)
        } else {
            lf_checker_rt::callee_thiscall!(6, u32, task, 0, BLEND_MINUS_ONE, 0, 0, 0, 0x0c)
        }
    }
}
