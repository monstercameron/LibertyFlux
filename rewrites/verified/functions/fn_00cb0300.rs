// original: 0x00cb0300 CTaskComplexGoToPointAnyMeans::vf20

/// Decide the next step of the go-to-point task: refresh a timed wait, or,
/// once the pedestrian is far enough from the goal, re-pick its move task.
///
/// `this` is the task object; `ped` points at the pedestrian (`+PED_POS_OBJ`
/// holds the position block, `+PED_QUERY` feeds the locator callee). Returns
/// the sub-task at `+SUBTASK` on every quiet path, or the swap callee's
/// answer after a re-pick.
///
/// Behaviour. Let `kind(x)` be the virtual type query at vtable slot `+0xc`
/// and `T` the global tick counter:
/// - Unless `kind(subtask) == KIND_ROUTE` with a live `+INNER` child of kind
///   `KIND_SWAP`, return the sub-task.
/// - Timer stage. If `+STAMP != NEVER` and `+ARMED` is clear, latch
///   `+DEADLINE = T`, `+SPAN = TIMEOUT`, `+ARMED = 1`. If `+ARMED` is then
///   set: re-latch `+DEADLINE = T` when `+RELATCH` is set (and clear it);
///   when `+SPAN + +DEADLINE <= T` (signed) call the swap callee
///   (id 1, code `CODE_WAIT`), re-latch the deadline and span, and return
///   the swap's answer.
/// - Distance stage. With `d` the goal-minus-pedestrian vector (`+GOAL` on
///   the task, `+PX/+PY/+PZ` on the position block) and `d2` its squared
///   length accumulated `((dx*dx + dy*dy) + dz*dz)`, return the sub-task
///   unless `d2 > NEAR2` (a `jbe`, so NaN also returns).
/// - Re-pick stage. Ask the locator callee (id 2, object at
///   `[ped+PED_QUERY]+QUERY_BIAS`) for a candidate; return the sub-task when
///   it answers null or the already-stored `+PICK`. Gate on the validity
///   callee (id 3); release the old pick through id 4 when nonzero, store
///   the candidate, call the swap callee (id 1, code `CODE_MOVE`), release
///   through id 5, and return the swap's answer.
///
/// Original: 0x00cb0300 (thiscall, one stack word). Single-precision SSE in
/// the original's operand order (pinned through `black_box`); the timer
/// comparison is signed (`setle`), so a deadline near `0x7fffffff` wraps
/// into a re-fire.
fn decide_00cb0300<const INVERT_DIST: bool>(this: u32, ped: u32) -> u32 {
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
        const GOAL: u32 = 0x20;
        const PICK: u32 = 0x3c;
        const STAMP: u32 = 0x40;
        const DEADLINE: u32 = 0x44;
        const SPAN: u32 = 0x48;
        const ARMED: u32 = 0x4c;
        const RELATCH: u32 = 0x4d;
        const INNER: u32 = 0x14;
        const PED_POS_OBJ: u32 = 0x20;
        const PX: u32 = 0x30;
        const PED_QUERY: u32 = 0x224;
        const QUERY_BIAS: u32 = 0xf4;
        const KIND_ROUTE: u32 = 0x11d;
        const KIND_SWAP: u32 = 0x3ae;
        const NEVER: u32 = 0xffff_ffff;
        const TIMEOUT: u32 = 0xbb8;
        const CODE_WAIT: u32 = 0x340;
        const CODE_MOVE: u32 = 0x2de;
        const TIMER_GVA: u32 = 0x0117_35b4;
        const NEAR2_GVA: u32 = 0x00fe_8c74;
        let task = rd32(this + SUBTASK);
        let mut slot = task;
        if kind_of(task) != KIND_ROUTE {
            return slot;
        }
        let sub = rd32(task + INNER);
        if sub == 0 {
            return slot;
        }
        if kind_of(sub) != KIND_SWAP {
            return slot;
        }
        let tick = lf_checker_rt::global::<u32>(TIMER_GVA).read_unaligned();
        if rd32(this + STAMP) != NEVER && rd8(this + ARMED) == 0 {
            wr32(this + DEADLINE, tick);
            wr32(this + SPAN, TIMEOUT);
            wr8(this + ARMED, 1);
        }
        if rd8(this + ARMED) != 0 {
            if rd8(this + RELATCH) != 0 {
                wr32(this + DEADLINE, tick);
                wr8(this + RELATCH, 0);
            }
            let end = rd32(this + SPAN).wrapping_add(rd32(this + DEADLINE));
            if (end as i32) <= (tick as i32) {
                let r: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, CODE_WAIT, ped);
                let tick2 = lf_checker_rt::global::<u32>(TIMER_GVA).read_unaligned();
                wr32(this + DEADLINE, tick2);
                wr32(this + SPAN, TIMEOUT);
                wr8(this + ARMED, 1);
                return r;
            }
        }
        let pos = rd32(ped + PED_POS_OBJ);
        let dx = fsub(
            ((pos + PX) as *const f32).read_unaligned(),
            ((this + GOAL) as *const f32).read_unaligned(),
        );
        let dy = fsub(
            ((pos + PX + 4) as *const f32).read_unaligned(),
            ((this + GOAL + 4) as *const f32).read_unaligned(),
        );
        let dz = fsub(
            ((pos + PX + 8) as *const f32).read_unaligned(),
            ((this + GOAL + 8) as *const f32).read_unaligned(),
        );
        let d2 = fadd(fadd(fmul(dx, dx), fmul(dy, dy)), fmul(dz, dz));
        let near2 = lf_checker_rt::global::<f32>(NEAR2_GVA).read_unaligned();
        let near = !(d2 > near2);
        let near = if INVERT_DIST { !near } else { near };
        if near {
            return slot;
        }
        let q = rd32(ped + PED_QUERY).wrapping_add(QUERY_BIAS);
        let cand: u32 = lf_checker_rt::callee_thiscall!(2, u32, q);
        if cand == 0 {
            return slot;
        }
        if cand == rd32(this + PICK) {
            return slot;
        }
        if lf_checker_rt::callee_cdecl!(3, u32, cand, ped, 1) as u8 == 0 {
            return slot;
        }
        let cur = rd32(this + PICK);
        if cur != 0 {
            lf_checker_rt::callee_thiscall!(4, u32, cur, this + PICK);
        }
        wr32(this + PICK, cand);
        let s: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, CODE_MOVE, ped);
        let cur2 = rd32(this + PICK);
        lf_checker_rt::callee_thiscall!(5, u32, cur2, this + PICK);
        slot = s;
        slot
    }
}

lf_checker_rt::export!(thiscall, rw_00cb0300(this: u32, ped: u32) -> u32 {
    decide_00cb0300::<false>(this, ped)
});
