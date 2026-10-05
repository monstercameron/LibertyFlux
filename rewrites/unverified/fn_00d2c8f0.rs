// original: 0x00D2C8F0 CTaskComplexUseDropDownOnRoute::vf20

/// Timer-gated route-drop-down task update with a two-phase sub-task check.
///
/// `this` is the task object, `ped` the pedestrian (thiscall, one stack
/// word, callee pops 4).
///
/// Gate: when byte `+0x54` is set, and after latching the game timer into
/// `+0x4c` when byte `+0x55` is set (clearing it), the task proceeds only if
/// `start + duration` (`+0x4c`, `+0x50`) has reached the game timer, compared
/// as SIGNED 32-bit. Unless flag bit 1 at `+0xc` is set, virtual slot 5 of
/// the task is polled as `wants(ped, 1, 0)`; a zero low byte fails the gate,
/// otherwise bit 2 is set at `+0xc`. Then, when the signed word at `+0x58`
/// is not negative, virtual slot 31 of the ped is invoked with
/// `(&this+0x60, -10.0, 1)`. The gate path returns 0.
///
/// Otherwise the object at `+0x8` (the sub-task) is examined. When the
/// pointer at `+0x44` is non-null, four floats (`+0x20..+0x28`, `+0x40`) are
/// staged; if the word at `+0x20` of the `+0x44` object is zero it is first
/// initialised by two direct calls, then a cdecl/3 helper fills four output
/// floats. When the sub-task reports type `0x3a6` from virtual slot 3, the
/// outputs are stored at `+0xc0..+0xcc` and the staged `+0x40` float at
/// `+0xd8`. When it instead reports `0x11d`, a first long block runs: a
/// direct helper maps `(sub, ped)` to an object which must report `0x384`,
/// then a second direct helper builds three scaled floats
/// (`out[i] = staged3[i] * 2.0 + helper3[i]`, in that operand order) that a
/// virtual call on the object receives.
///
/// Join: whenever the sub-task reports `0x11d` (repolled, same answer within
/// a trial), a second long block runs the same object lookup and, on
/// `0x384`, a further virtual call taking the first block's three floats,
/// returning an object whose `+0x8` float minus the `+0x38` float of the
/// object at `ped+0x20` must EXCEED 1.0 (ordered comparison: NaN fails);
/// then virtual slot 5 of the task is polled again via a direct helper and
/// a non-zero low byte returns 0 instead. Falls through to returning the
/// `+0x8` object.
///
/// Edge cases: all staging moves are bitwise copies; the timer comparison is
/// signed; the threshold test is an ordered `>` (NaN takes the early exit);
/// float arithmetic order is the original's (scale-then-shift per lane).
lf_checker_rt::export!(thiscall, rw_00D2C8F0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TIMER: u32 = 0x0117_35B4;
        const SCALE: u32 = 0x00FE_8A24; // 2.0f
        const LIMIT: u32 = 0x00FE_88E8; // 1.0f
        const TASK_FLAGS: u32 = 0x0c;
        const POLL_DONE_BIT: u32 = 0x1;
        const POLL_OK_BIT: u32 = 0x2;
        const SUB_TASK: u32 = 0x08;
        const DROP_DATA: u32 = 0x44;
        const TIMER_START: u32 = 0x4c;
        const TIMER_DUR: u32 = 0x50;
        const TIMER_ARMED: u32 = 0x54;
        const TIMER_LATCH: u32 = 0x55;
        const PROGRESS: u32 = 0x58;
        const ANCHOR: u32 = 0x60;
        const POS_X: u32 = 0x20;
        const POS_Y: u32 = 0x24;
        const POS_Z: u32 = 0x28;
        const POS_W: u32 = 0x40;
        const TYPE_POS: u32 = 0x3a6;
        const TYPE_ROUTE: u32 = 0x11d;
        const TYPE_TARGET: u32 = 0x384;
        const INIT_FLAG: u32 = 0x20;
        const INIT_OBJ: u32 = 0x10;
        const OUT_X: u32 = 0xc0;
        const OUT_Y: u32 = 0xc4;
        const OUT_Z: u32 = 0xc8;
        const OUT_W: u32 = 0xcc;
        const OUT_EXTRA: u32 = 0xd8;
        const SPEED: u32 = 0xC120_0000; // -10.0f
        const VT_POLL: u32 = 0x14;
        const VT_PED_DROP: u32 = 0x7c;
        const VT_TYPE: u32 = 0x0c;
        const VT_APPLY: u32 = 0x24;
        const VT_MEASURE: u32 = 0x1c;
        const OBJ_MID: u32 = 0x14;
        const PED_AUX: u32 = 0x20;
        const AUX_REF: u32 = 0x38;
        const RES_VALUE: u32 = 0x08;

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
        unsafe fn timer() -> u32 {
            unsafe { lf_checker_rt::global::<u32>(TIMER).read() }
        }
        #[inline(always)]
        unsafe fn gscale() -> f32 {
            unsafe { lf_checker_rt::global::<f32>(SCALE).read() }
        }
        #[inline(always)]
        unsafe fn glimit() -> f32 {
            unsafe { lf_checker_rt::global::<f32>(LIMIT).read() }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn task_type(obj: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt + VT_TYPE));
                f(obj)
            }
        }

        let mut gated = false;
        if rd8(this + TIMER_ARMED) != 0 {
            if rd8(this + TIMER_LATCH) != 0 {
                wr32(this + TIMER_START, timer());
                wr8(this + TIMER_LATCH, 0);
            }
            let end = rd32(this + TIMER_DUR).wrapping_add(rd32(this + TIMER_START));
            if (end as i32) <= (timer() as i32) {
                gated = true;
            }
        }
        if gated {
            if (rd32(this + TASK_FLAGS) & POLL_DONE_BIT) == 0 {
                let vt = rd32(this);
                let poll: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(rd32(vt + VT_POLL)) };
                if (poll(this, ped, 1, 0) & 0xFF) == 0 {
                    gated = false;
                } else {
                    wr32(this + TASK_FLAGS, rd32(this + TASK_FLAGS) | POLL_OK_BIT);
                }
            }
        }
        if gated {
            if (rd32(this + PROGRESS) as i32) >= 0 {
                let vt = rd32(ped);
                let drop: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(rd32(vt + VT_PED_DROP)) };
                drop(ped, this + ANCHOR, SPEED, 1);
            }
            return 0;
        }

        let sub = rd32(this + SUB_TASK);
        let mut scaled = [0u32; 3];
        let data = rd32(this + DROP_DATA);
        if data != 0 {
            let mut out = [rd32(this + POS_X), rd32(this + POS_Y), rd32(this + POS_Z), 0u32];
            let mut extra = rd32(this + POS_W);
            if rd32(data + INIT_FLAG) == 0 {
                lf_checker_rt::callee_thiscall!(4, u32, data);
                lf_checker_rt::callee_thiscall!(5, u32, data + INIT_OBJ, rd32(data + INIT_FLAG));
            }
            lf_checker_rt::callee_cdecl!(
                6, u32,
                out.as_mut_ptr() as u32,
                core::ptr::addr_of_mut!(extra) as u32,
                rd32(data + INIT_FLAG)
            );
            let t = task_type(sub);
            if t == TYPE_POS {
                wr32(sub + OUT_X, out[0]);
                wr32(sub + OUT_Y, out[1]);
                wr32(sub + OUT_Z, out[2]);
                wr32(sub + OUT_W, out[3]);
                wr32(sub + OUT_EXTRA, extra);
            } else if t == TYPE_ROUTE {
                let obj = lf_checker_rt::callee_thiscall!(7, u32, sub, ped);
                if obj != 0 && task_type(obj) == TYPE_TARGET {
                    let mut wframe = [0u32; 7];
                    lf_checker_rt::callee_thiscall!(
                        9, u32,
                        wframe.as_mut_ptr() as u32,
                        extra
                    );
                    let k = gscale();
                    scaled[0] = add(mul(f32::from_bits(wframe[4]), k), f32::from_bits(out[0])).to_bits();
                    scaled[1] = add(mul(f32::from_bits(wframe[5]), k), f32::from_bits(out[1])).to_bits();
                    scaled[2] = add(mul(f32::from_bits(wframe[6]), k), f32::from_bits(out[2])).to_bits();
                    let mid = rd32(obj + OBJ_MID);
                    let apply: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        unsafe { core::mem::transmute(rd32(mid + VT_APPLY)) };
                    apply(obj + OBJ_MID, ped, scaled.as_ptr() as u32, 0);
                }
            }
        }
        if task_type(sub) == TYPE_ROUTE {
            let obj = lf_checker_rt::callee_thiscall!(7, u32, sub, ped);
            if obj != 0 && task_type(obj) == TYPE_TARGET {
                let mid = rd32(obj + OBJ_MID);
                let measure: extern "thiscall" fn(u32, u32) -> u32 =
                    unsafe { core::mem::transmute(rd32(mid + VT_MEASURE)) };
                let r = measure(obj + OBJ_MID, scaled.as_ptr() as u32);
                let aux = rd32(ped + PED_AUX);
                let d = sub(f32::from_bits(rd32(r + RES_VALUE)), f32::from_bits(rd32(aux + AUX_REF)));
                if d > glimit() {
                    if (lf_checker_rt::callee_thiscall!(12, u32, this, ped, 1, 0) & 0xFF) != 0 {
                        return 0;
                    }
                }
            }
        }
        sub
    }
});
