// original: 0x00cf8670 CTaskComplexClimbLadder::vf19

/// Decide whether a ped may start climbing a ladder, and dispatch the next task.
///
/// `this` is the climb-ladder task object, `ped` the ped attempting the climb
/// (thiscall: `this` in ECX, `ped` the one stack word; callee pops 4).
///
/// Behaviour: bump the shared one-byte start counter at `START_COUNT` (0
/// becomes 1, any other value is kept). Take the cached subtask at
/// `+SUBTASK`; when it is null and the object reports none at `+HAS_SUBTASK`,
/// ask the acquire callee for one with (`ped`, `this+STATE`, has-flag) and
/// fail when it answers null. Then run the reachability check callee with
/// (subtask, state word, `this+0x20`, `this+0x30`, `this+0x74`) and fail when
/// its low byte is 0. Then compare heights: `gap = [this+HEIGHT] -
/// [[ped+0x20]+HEIGHT]` fails above 1.0; otherwise the nearer of
/// `|[target-1] - [this+HEIGHT_REF]|` and `|[target-1] - [this+HEIGHT]|`
/// picks state 1 (flag 1) or state 4 (flag 0) written to `+TASK_STATE` and
/// `+LADDER_FLAG`. Then run the begin callee with (`ped`, subtask, state,
/// `this+0x20`, `this+0x30`, `this+0x40`, `this+0x70`, state-id, `this+0x50`,
/// 1) and fail when its low byte is 0. Success dispatches the task id
/// `TASK_CLIMB` (0x3ae) through the task-dispatch callee; every failure path
/// dispatches `TASK_ABORT` (0xcb) instead. Returns whatever the dispatch
/// callee answered (left in EAX by the original).
///
/// Float order is the original's: `gap = base - target`, then
/// `x2 = target - 1`, `x1 = |x2 - ref|`, `x2 = |x2 - base|`, all through
/// black-boxed operands. The `above 1.0` test is a plain `>` (unordered
/// never takes it); the nearer-side test is `!(x2 > x1)` so unordered
/// (NaN) takes the state-1 side exactly like the original's `jbe`.
/// The absolute value masks the low lane with `ABS_MASK`, the 16 bytes the
/// original ANDs from its constant table.
lf_checker_rt::export!(thiscall, rw_00cf8670(this: u32, ped: u32) -> u32 {
    unsafe {
        const START_COUNT: u32 = 0x171de21;
        const SUBTASK: u32 = 0x90;
        const HAS_SUBTASK: u32 = 0x7c;
        const STATE: u32 = 0x94;
        const TASK_STATE: u32 = 0x14;
        const LADDER_FLAG: u32 = 0x76;
        const HEIGHT: u32 = 0x38;
        const HEIGHT_REF: u32 = 0x28;
        const PED_TARGET: u32 = 0x20;
        const ONE: f32 = 1.0;
        const ABS_MASK: u32 = 0x7fff_ffff;
        const TASK_CLIMB: u32 = 0x3ae;
        const TASK_ABORT: u32 = 0xcb;
        const ACQUIRE: u32 = 1;
        const REACH_CHECK: u32 = 2;
        const BEGIN: u32 = 3;
        const DISPATCH: u32 = 4;

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
        fn abs_lane(x: f32) -> f32 {
            f32::from_bits(x.to_bits() & ABS_MASK)
        }

        let counter = lf_checker_rt::global::<u8>(START_COUNT);
        let seen: u8 = counter.read();
        counter.write(if seen == 0 { 1 } else { seen });

        let mut subtask = rd32(this + SUBTASK);
        if subtask == 0 {
            let has: u32 = if rd32(this + HAS_SUBTASK) != 0 { 1 } else { 0 };
            subtask = lf_checker_rt::callee_cdecl!(ACQUIRE, u32, ped, this + STATE, has);
            if subtask == 0 {
                return lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, TASK_ABORT, ped);
            }
        }
        let reach = lf_checker_rt::callee_cdecl!(
            REACH_CHECK, u32, subtask, rd32(this + STATE),
            this + 0x20, this + 0x30, this + 0x74
        );
        if (reach & 0xff) == 0 {
            return lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, TASK_ABORT, ped);
        }
        let target = rd32(ped + PED_TARGET);
        let base_h = rdf(this + HEIGHT);
        let gap = sub(base_h, rdf(target + HEIGHT));
        if gap > ONE {
            return lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, TASK_ABORT, ped);
        }
        let x2 = sub(rdf(target + HEIGHT), ONE);
        let near_ref = abs_lane(sub(x2, rdf(this + HEIGHT_REF)));
        let near_base = abs_lane(sub(x2, base_h));
        if !(near_base > near_ref) {
            ((this + TASK_STATE) as *mut u32).write_unaligned(1);
            ((this + LADDER_FLAG) as *mut u8).write(1);
        } else {
            ((this + TASK_STATE) as *mut u32).write_unaligned(4);
            ((this + LADDER_FLAG) as *mut u8).write(0);
        }
        let go = lf_checker_rt::callee_cdecl!(
            BEGIN, u32, ped, subtask, rd32(this + STATE),
            this + 0x20, this + 0x30, this + 0x40, this + 0x70,
            rd32(this + TASK_STATE), this + 0x50, 1
        );
        if (go & 0xff) == 0 {
            return lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, TASK_ABORT, ped);
        }
        lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, TASK_CLIMB, ped)
    }
});
