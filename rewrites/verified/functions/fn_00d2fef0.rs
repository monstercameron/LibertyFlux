// original: 0x00d2fef0 CTaskComplexMoveFollowNavMeshRoute__vf5 (symbols)

/// Navmesh-route-follow task state update: poll subtasks, advance or re-plan.
///
/// `this` points to the task (`+0x4`/`+0x8` subtask slots, `+0x6c` pending
/// handle, `+0xd8` state flags, `+0xac..0xb5` plan cache). `a0` is a context
/// object (config at `+0x224`, motion state at `+0xa80`), `a1` a mode word
/// passed to the re-plan query, and `a2` the active route leg, whose virtual
/// status (slot `+4`) drives the update. Six callees: the leg status, subtask status
/// (slot `+0xc` on the `+0x4`/`+0x8` objects), a re-plan query (slot `+0x14`,
/// thiscall with context, context and leg), a leg notifier (direct thiscall),
/// and a handle releaser (direct cdecl).
///
/// A null leg, an unrecognized status, or a failed re-plan falls through to
/// the re-plan tail: unless the `+0x8` object is already flagged, the query
/// runs (a false answer returns 0); the pending handle is released and the
/// `0x4` flag cleared; with `0x200` set the shared tick is latched into the
/// plan cache; flags are normalized (`0x1000` cleared, `0x800` set); and with
/// `0x200000` set a nonzero motion speed sets `0x4000` on the motion state
/// (zero speed, including negative zero, skips it; NaN sets it). The tail
/// returns 1. On the polling path a `0x81` leg status notifies the leg; a
/// status of 6 checks the subtasks (a live `+0x4` subtask whose status is not
/// `0x3be`, or a missing one with `0x40000` clear, arms step two; with
/// `0x200` set and a live `+0x8` subtask whose status is not `0x3be`, the
/// `0x40000` flag is set and the leg's counter advances); a status of 5, or
/// of 1 after another status, re-checks (`0x200` must be clear, `0x100000`
/// set, `+0x8` live with status `0x384` to return early, else the re-plan
/// tail runs). All polling exits return 0. Float operations run in the
/// original's operand order.
///
/// Original: 0x00d2fef0 (thiscall: object in ECX, three stack words, callee
/// pops 0xc, boolean result in AL; the prologue loads EBP from the first
/// stack word, so the context is `a0`, not `a1`).
lf_checker_rt::export!(thiscall, rw_00d2fef0(this: u32, a0: u32, a1: u32, a2: u32) -> u8 {
    unsafe {
        const SUB_A: u32 = 0x04;
        const SUB_B: u32 = 0x08;
        const PENDING: u32 = 0x6c;
        const FLAGS: u32 = 0xd8;
        const CACHE_TICK: u32 = 0xac;
        const CACHE_N: u32 = 0xb0;
        const CACHE_F: u32 = 0xb4;
        const CFG: u32 = 0x224;
        const MOTION: u32 = 0xa80;
        const MOT_DX: u32 = 0x14;
        const MOT_DY: u32 = 0x18;
        const MOT_FLAGS: u32 = 0x50;
        const FLAG_ARMED: u32 = 0x200;
        const FLAG_ADV: u32 = 0x40000;
        const FLAG_PLAN: u32 = 0x100000;
        const FLAG_FAST: u32 = 0x200000;
        const VSTATUS: u32 = 4;
        const VSUB: u32 = 0x0c;
        const VQUERY: u32 = 0x14;
        const LEG_NOTIFY: u32 = 4;
        const LEG_RELEASE: u32 = 5;
        const TICK_FILE_VA: u32 = 0x011735b4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        fn leg_status(leg: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(leg) + VSTATUS);
                let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
                f(leg)
            }
        }
        #[inline(always)]
        fn sub_status(obj: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(obj) + VSUB);
                let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
                f(obj)
            }
        }

        // Re-plan tail, shared by the null leg, unknown statuses and failed
        // re-checks. Returns the tail result.
        #[inline(always)]
        unsafe fn tail(this: u32, a0: u32, a1: u32, leg: u32) -> u8 {
            unsafe {
                let sub = rd32(this + SUB_B);
                if ((sub + 0x0c) as *const u8).read() & 1 == 0 {
                    let slot = rd32(rd32(sub) + VQUERY);
                    let q: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    if q(sub, a0, a1, leg) & 0xff == 0 {
                        return 0;
                    }
                    let w = rd32(sub + 0x0c);
                    ((sub + 0x0c) as *mut u32).write_unaligned(w | 2);
                }
                let pend = rd32(this + PENDING);
                if pend != 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(LEG_RELEASE, u32, pend);
                    let f = rd32(this + FLAGS);
                    ((this + FLAGS) as *mut u32).write_unaligned(f & !4);
                    ((this + PENDING) as *mut u32).write_unaligned(0);
                }
                if rd32(this + FLAGS) & FLAG_ARMED != 0 {
                    let tick = lf_checker_rt::global::<u32>(TICK_FILE_VA).read_unaligned();
                    ((this + CACHE_TICK) as *mut u32).write_unaligned(tick);
                    ((this + CACHE_N) as *mut u32).write_unaligned(0);
                    ((this + CACHE_F) as *mut u8).write(1);
                }
                let e = (rd32(this + FLAGS) & !0x1000) | 0x800;
                ((this + FLAGS) as *mut u32).write_unaligned(e);
                if e & FLAG_FAST != 0 {
                    let mo = rd32(a0 + MOTION);
                    let dx = rdf(mo + MOT_DX);
                    let dy = rdf(mo + MOT_DY);
                    let sp = add(mul(dx, dx), mul(dy, dy)).sqrt();
                    if sp != 0.0 {
                        let mf = rd32(mo + MOT_FLAGS);
                        ((mo + MOT_FLAGS) as *mut u32).write_unaligned(mf | 0x4000);
                    }
                }
                1
            }
        }

        if a2 == 0 {
            return tail(this, a0, a1, a2);
        }
        if leg_status(a2) == 0x81 {
            let cfg = rd32(a0 + CFG);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                LEG_NOTIFY,
                u32,
                cfg,
                a2.wrapping_add(0x10)
            );
        }
        if leg_status(a2) == 6 {
            let armed = if rd32(this + FLAGS) & FLAG_ADV != 0 {
                false
            } else {
                let sa = rd32(this + SUB_A);
                if sa == 0 {
                    true
                } else {
                    sub_status(sa) != 0x3be
                }
            };
            if rd32(this + FLAGS) & FLAG_ARMED != 0
                && armed
                && {
                    let sb = rd32(this + SUB_B);
                    sb != 0 && sub_status(sb) != 0x3be
                }
            {
                let f = rd32(this + FLAGS);
                ((this + FLAGS) as *mut u32).write_unaligned(f | FLAG_ADV);
                let c = rd32(a2 + 4);
                ((a2 + 4) as *mut u32).write_unaligned(c.wrapping_add(1));
            }
            return 0;
        }
        if leg_status(a2) != 5 && leg_status(a2) != 1 {
            return tail(this, a0, a1, a2);
        }
        let e = rd32(this + FLAGS);
        if e & FLAG_ARMED != 0 || e & FLAG_PLAN == 0 {
            return tail(this, a0, a1, a2);
        }
        let sb = rd32(this + SUB_B);
        if sb == 0 {
            return tail(this, a0, a1, a2);
        }
        if sub_status(sb) == 0x384 {
            return 0;
        }
        tail(this, a0, a1, a2)
    }
});
