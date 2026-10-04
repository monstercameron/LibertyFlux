// original: 0x00cb0760 CTaskComplexMoveCrossRoadAtTrafficLights::vf20

/// Decide the next step of the cross-road-at-lights task: track the light
/// phase, and when the pedestrian is far from the crossing, roll a random
/// ={offset, speed}= pair and hand it to the crossing controller.
///
/// `this` is the task object; `ped` points at the pedestrian (position block
/// at `+POS_BLK`, light block at `+LIGHT_BLK`). Returns the sub-task at
/// `+SUBTASK` on every path; everything else is updated state and outgoing
/// calls.
///
/// Behaviour:
/// - Poll the sensor (id 1) on the position block and classify through id 2
///   (low byte only, zero-extended). Unless flag `PHASE_SEEN` of `+FLAGS`
///   is set and the class differs from `+PHASE`, set `PHASE_SEEN`.
///   (The two goal words copied to dead stack slots by the original are
///   reproduced as black-boxed locals; they are never read back.)
/// - Return the sub-task unless `+GATE` is nonzero, bit 0 of `+FLAGS` is set,
///   bit 2 is clear, and `kind(subtask) == KIND_CROSS` (id 9).
/// - When `+ARMED` is clear, latch `+DEADLINE = TICK`, `+SPAN = WAIT`,
///   `+ARMED = 1` and return the sub-task.
/// - When the timer callee (id 3 on `this+DEADLINE`) answers false, or the
///   light state `[LIGHT_BLK+LIGHT_AT]` equals `LIGHT_HOLD`, return.
/// - With `d` the goal-minus-position vector (`+GOAL` vs `+PX`), return the
///   refreshed task unless `d2 = ((dx*dx + dy*dy) + dz*dz)` exceeds `FAR2`.
/// - Unless the polled class is 1 or 2, or the probe callee (id 4 on the
///   handle `subtask+HANDLE`) answers 3 or more (signed), refresh.
/// - Roll twice from the random callee (id 5, same scripted value per trial):
///   the first roll masked to 15 bits must be below `ROLL_MAX`. Convert the
///   second roll from signed integer to float and scale:
///   `f = (((v * K1) * K2) + K3) + K4`. When the fetch callee (id 6) answers
///   true and `f <= FMAX`, use `FMAX` instead.
/// - Deliver `(fbits, 1)` through the handle's virtual slot `+4` (id 7).
///   Unless flag `NOTIFIED` of `+FLAGS` is set, also notify through id 8
///   `(ped, relocated(CODE_GVA), 0.5f, 0, 0)` and set `NOTIFIED`.
///   Two quirks: the fetch callee (id 6) pops nothing (the original cleans
///   its argument itself) and is entered with whatever the previous stubs
///   left in ecx (the original never reloads it), so its ecx is uncompared;
///   the notify callee's first word is a relocated image address, derived
///   through `relocated`, not the file VA.
/// - Refreshing means `+DEADLINE = TICK`, `+SPAN = WAIT`, `+ARMED = 1`, then
///   returning the sub-task.
///
/// Original: 0x00cb0760 (thiscall, one stack word). Single-precision SSE in
/// the original's operand order (pinned through `black_box`); the probe
/// comparison is signed, the roll gate unsigned.
fn decide_00cb0760<const KIND_BUMP: bool>(this: u32, ped: u32) -> u32 {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read_unaligned() }
    }
    unsafe fn wr8(a: u32, v: u8) {
        unsafe { (a as *mut u8).write_unaligned(v) }
    }
    fn kind_of(obj: u32) -> u32 {
        unsafe {
            let vtable = (obj as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(0x0c) as *const u32).read_unaligned();
            let query: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            query(obj)
        }
    }
    #[inline(always)]
    fn fsub(x: f32, y: f32) -> f32 {
        core::hint::black_box(x) - core::hint::black_box(y)
    }
    #[inline(always)]
    fn fmul(x: f32, y: f32) -> f32 {
        core::hint::black_box(x) * core::hint::black_box(y)
    }
    #[inline(always)]
    fn fadd(x: f32, y: f32) -> f32 {
        core::hint::black_box(x) + core::hint::black_box(y)
    }
    unsafe {
        const SUBTASK: u32 = 0x08;
        const GOAL: u32 = 0x30;
        const DEAD_X: u32 = 0x40;
        const GATE: u32 = 0x50;
        const DEADLINE: u32 = 0x54;
        const SPAN: u32 = 0x58;
        const WAIT: u32 = 0x3e8;
        const ARMED: u32 = 0x5c;
        const PHASE: u32 = 0x70;
        const FLAGS: u32 = 0x74;
        const PHASE_SEEN: u32 = 8;
        const NOTIFIED: u32 = 0x10;
        const HANDLE: u32 = 0x14;
        const POS_BLK: u32 = 0x20;
        const PX: u32 = 0x30;
        const LIGHT_BLK: u32 = 0x21c;
        const LIGHT_AT: u32 = 0x12c;
        const LIGHT_HOLD: u32 = 2;
        const KIND_CROSS: u32 = 0x3ae;
        const ROLL_MAX: u32 = 0x1388;
        const ROLL_MASK: u32 = 0x7fff;
        const PROBE_MAX: i32 = 3;
        const CODE_GVA: u32 = 0x00ed_7cd4;
        const HALF_BITS: u32 = 0x3f00_0000;
        const TICK_GVA: u32 = 0x0117_35b4;
        const FAR2_GVA: u32 = 0x00fe_8b28;
        const K1_GVA: u32 = 0x00fe_8684;
        const K2_GVA: u32 = 0x00fe_8830;
        const K3_GVA: u32 = 0x00fe_888c;
        const K4_GVA: u32 = 0x00fe_88e8;
        const FMAX_GVA: u32 = 0x00fe_8960;
        let kind_want = if KIND_BUMP { KIND_CROSS + 1 } else { KIND_CROSS };
        let obj = rd32(ped + POS_BLK);
        let mut dead0 = rd32(this + DEAD_X);
        let mut dead1 = rd32(this + DEAD_X + 4);
        core::hint::black_box((&mut dead0, &mut dead1));
        let a1 = lf_checker_rt::callee_cdecl!(1, u32, obj.wrapping_add(PX)) as u8 as u32;
        let a2: u32 = lf_checker_rt::callee_cdecl!(2, u32, a1);
        let mut flags = rd32(this + FLAGS);
        if flags & PHASE_SEEN == 0 && a2 != rd32(this + PHASE) {
            flags |= PHASE_SEEN;
            wr32(this + FLAGS, flags);
        }
        let task = rd32(this + SUBTASK);
        if rd32(this + GATE) == 0 {
            return task;
        }
        let fb = rd8(this + FLAGS);
        if fb & 1 == 0 {
            return task;
        }
        if fb & 4 != 0 {
            return task;
        }
        if kind_of(task) != kind_want {
            return task;
        }
        let tick = lf_checker_rt::global::<u32>(TICK_GVA).read_unaligned();
        if rd8(this + ARMED) == 0 {
            wr32(this + DEADLINE, tick);
            wr32(this + SPAN, WAIT);
            wr8(this + ARMED, 1);
            return task;
        }
        if lf_checker_rt::callee_thiscall!(3, u32, this + DEADLINE) as u8 == 0 {
            return task;
        }
        let light = rd32(ped + LIGHT_BLK);
        if rd32(light + LIGHT_AT) == LIGHT_HOLD {
            return task;
        }
        let dx = fsub(
            ((this + GOAL) as *const f32).read_unaligned(),
            ((obj + PX) as *const f32).read_unaligned(),
        );
        let dy = fsub(
            ((this + GOAL + 4) as *const f32).read_unaligned(),
            ((obj + PX + 4) as *const f32).read_unaligned(),
        );
        let dz = fsub(
            ((this + GOAL + 8) as *const f32).read_unaligned(),
            ((obj + PX + 8) as *const f32).read_unaligned(),
        );
        let d2 = fadd(fadd(fmul(dx, dx), fmul(dy, dy)), fmul(dz, dz));
        let far2 = lf_checker_rt::global::<f32>(FAR2_GVA).read_unaligned();
        fn refresh(this: u32, tick: u32) -> u32 {
            unsafe {
                const SUBTASK: u32 = 0x08;
                const DEADLINE: u32 = 0x54;
                const SPAN: u32 = 0x58;
                const WAIT: u32 = 0x3e8;
                const ARMED: u32 = 0x5c;
                wr32(this + DEADLINE, tick);
                wr32(this + SPAN, WAIT);
                wr8(this + ARMED, 1);
                rd32(this + SUBTASK)
            }
        }
        if !(d2 > far2) {
            return refresh(this, tick);
        }
        if a2 != 1 && a2 != 2 {
            return refresh(this, tick);
        }
        let h = task.wrapping_add(HANDLE);
        if lf_checker_rt::callee_thiscall!(4, u32, h) as i32 >= PROBE_MAX {
            return refresh(this, tick);
        }
        let v5a: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
        if (v5a & ROLL_MASK) >= ROLL_MAX {
            return refresh(this, tick);
        }
        let v5b: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
        let k1 = lf_checker_rt::global::<f32>(K1_GVA).read_unaligned();
        let k2 = lf_checker_rt::global::<f32>(K2_GVA).read_unaligned();
        let k3 = lf_checker_rt::global::<f32>(K3_GVA).read_unaligned();
        let k4 = lf_checker_rt::global::<f32>(K4_GVA).read_unaligned();
        let fmax = lf_checker_rt::global::<f32>(FMAX_GVA).read_unaligned();
        let mut f = fadd(
            fadd(fmul(fmul(v5b as i32 as f32, k1), k2), k3),
            k4,
        );
        if lf_checker_rt::callee_thiscall!(6, u32, h, ped) as u8 != 0 && !(f > fmax) {
            f = fmax;
        }
        let hv = rd32(h);
        let tgt = rd32(hv + 4);
        let deliver: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        deliver(h, f.to_bits(), 1);
        if rd8(this + FLAGS) & (NOTIFIED as u8) == 0 {
            let code = lf_checker_rt::relocated(CODE_GVA);
            lf_checker_rt::callee_thiscall!(8, u32, ped, code, HALF_BITS, 0, 0);
            wr32(this + FLAGS, rd32(this + FLAGS) | NOTIFIED);
        }
        refresh(this, tick)
    }
}

lf_checker_rt::export!(thiscall, rw_00cb0760(this: u32, ped: u32) -> u32 {
    decide_00cb0760::<false>(this, ped)
});


