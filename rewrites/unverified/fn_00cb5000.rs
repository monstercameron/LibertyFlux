// original: 0x00cb5000 ped_task_dispatch_by_code (proposed)

/// Dispatch a ped task request by an integer code, choosing one of several
/// task-creation calls against the task manager.
///
/// `this` is the requesting object: dword at `+0x08` (a sub-object whose
/// virtual slot at `+0x0c` reports a task id), floats at `+0x18`, `+0x20`,
/// `+0x24`, `+0x28`, `+0x30`, `+0x38`, a dword at `+0x34` and a flag byte at
/// `+0x3c` (bit 1 gates the aimed path, bit 0 feeds the creation call, bit 3
/// requests an extra flag on the result). `code` selects the path, `ctx` is a
/// context the aimed path reads (pointer at `+0x20` to a row of floats at
/// `+0x10`/`+0x14`/`+0x18` and `+0x30`/`+0x34`/`+0x38`, and a float at
/// `+0xe4`).
///
/// Paths: `0x11a` with the flag bit set, a non-null sub-object and a reported
/// id of `0x384` computes a dot product of the offset between the object's
/// position and the context row against the row's direction, scales the
/// context weight by half, divides by the dot when it is positive and the
/// weight is not negative, clamps the quotient to [1.0, 8.0], and creates the
/// aimed task, falling back to a default-speed creation when the geometry is
/// degenerate; `0x11a` otherwise creates a default task; `0xca` creates a
/// one-argument task; `0x384` creates a task from the object's state, stores
/// the object's dword into the result, optionally sets a flag on it, and
/// finishes through a follow-up call whose last argument is 1e8 when the
/// object's float lies in [2.0, 3.0) and 10.0 otherwise; any other code, a
/// code above `0x384`, or a null task manager returns 0.
///
/// The gate call fetches the task manager from a global; every creation call
/// takes it in ECX. NaN inputs follow the original's unordered-compare
/// behavior: a NaN dot or weight takes the degenerate path, a NaN quotient
/// passes the clamp through, and a NaN range value takes the 10.0 branch.
/// The float operation order is the original's.
///
/// Original: 0x00cb5000 (thiscall, two stack words), returns the creation
/// call's result, or the follow-up path's created task.
lf_checker_rt::export!(thiscall, rw_00cb5000(this: u32, code: u32, ctx: u32) -> u32 {
    unsafe {
        const SUB_OBJ: u32 = 0x08;
        const VTASK_ID: u32 = 0x0c;
        const WANT_ID: u32 = 0x384;
        const POS_X: u32 = 0x20;
        const POS_Y: u32 = 0x24;
        const POS_Z: u32 = 0x28;
        const F18: u32 = 0x18;
        const F30: u32 = 0x30;
        const F38: u32 = 0x38;
        const DW34: u32 = 0x34;
        const FLAGS: u32 = 0x3c;
        const CTX_ROW: u32 = 0x20;
        const CTX_W: u32 = 0xe4;
        const RES_DW: u32 = 0xa4;
        const RES_FLAGS: u32 = 0xc4;
        const GATE_CALLEE: u32 = 1;
        const AIMED_CALLEE: u32 = 2;
        const SIMPLE_CALLEE: u32 = 3;
        const MAKE_CALLEE: u32 = 4;
        const FINISH_CALLEE: u32 = 5;
        const VTASK_CALLEE: u32 = 6;
        const TASK_MGR_GLOBAL: u32 = 0x167e2a0;
        const HALF: f32 = 0.5;
        const ONE: f32 = 1.0;
        const TWO: f32 = 2.0;
        const THREE: f32 = 3.0;
        const EIGHT: f32 = 8.0;
        const TEN: f32 = 10.0;
        const BIG: f32 = 100000000.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn gate() -> u32 {
            unsafe {
                let g = lf_checker_rt::global::<u32>(TASK_MGR_GLOBAL).read();
                lf_checker_rt::callee_thiscall!(GATE_CALLEE, u32, g)
            }
        }

        if code > WANT_ID {
            return 0;
        }
        if code == WANT_ID {
            // Creation path.
            let mgr = gate();
            let task = if mgr == 0 {
                0
            } else {
                let flag = (rd8(this + FLAGS) & 1) as u32;
                let f30 = rdf(this + F30);
                let f18 = rdf(this + F18);
                lf_checker_rt::callee_thiscall!(
                    MAKE_CALLEE, u32, mgr,
                    f18.to_bits(), this.wrapping_add(POS_X), f30.to_bits(), 0, flag
                )
            };
            // A null manager faults here on the write to 0xa4, as does the
            // original; fault parity is part of the proof.
            ((task + RES_DW) as *mut u32).write_unaligned(rd32(this + DW34));
            if rd8(this + FLAGS) & 8 != 0 {
                let p = (task + RES_FLAGS) as *mut u32;
                p.write_unaligned(p.read_unaligned() | 0x40);
            }
            let f18 = rdf(this + F18);
            // Small branch unless f18 is ordered in [2.0, 3.0).
            let big = f18 >= TWO && THREE > f18;
            let last = if big { BIG } else { TEN };
            lf_checker_rt::callee_thiscall!(
                FINISH_CALLEE, u32, this,
                task, ctx, rdf(this + F38).to_bits(), last.to_bits()
            );
            return task;
        }
        if code == 0xca {
            let mgr = gate();
            if mgr == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(SIMPLE_CALLEE, u32, mgr, 1);
        }
        if code != 0x11a {
            return 0;
        }
        // Aimed path: needs the flag bit, a sub-object, and id 0x384.
        let mut aimed = rd8(this + FLAGS) & 2 != 0;
        if aimed {
            let sub = rd32(this + SUB_OBJ);
            if sub == 0 {
                aimed = false;
            } else {
                let hook: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(sub) + VTASK_ID) as usize);
                if hook(sub) != WANT_ID {
                    aimed = false;
                }
            }
        }
        if aimed {
            let row = rd32(ctx + CTX_ROW);
            let d0 = sub(rdf(this + POS_Y), rdf(row + 0x34));
            let d2 = sub(rdf(this + POS_X), rdf(row + 0x30));
            let d1 = sub(rdf(this + POS_Z), rdf(row + 0x38));
            let mut dot = mul(rdf(row + 0x14), d0);
            dot = add(dot, mul(rdf(row + 0x10), d2));
            dot = add(dot, mul(rdf(row + 0x18), d1));
            let w = mul(rdf(ctx + CTX_W), HALF);
            // Degenerate unless dot > 0 and w >= 0, both ordered.
            let ok = dot > 0.0 && w >= 0.0;
            let speed = if ok {
                let q = core::hint::black_box(w) / core::hint::black_box(dot);
                if ONE > q {
                    ONE
                } else if q > EIGHT {
                    EIGHT
                } else {
                    q
                }
            } else {
                EIGHT
            };
            let mgr = gate();
            if mgr == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(
                AIMED_CALLEE, u32, mgr, 0x7d0, 0, 1, speed.to_bits()
            );
        }
        let mgr = gate();
        if mgr == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(AIMED_CALLEE, u32, mgr, 1, 0, 0, EIGHT.to_bits())
    }
});
