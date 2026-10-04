// original: 0x00DA51F0 CTaskComplexShockingEventGoto::vf20

/// Periodic update with a timer pre-gate: while the deadline has not
/// passed, run the virtual state check against the subtask and, when it
/// passes, mark the subtask, forward to the dispatch helper and report 0;
/// when the deadline passed, or the gate below fires, or the check
/// refuses, build the fallback subtask and return the current one.
///
/// The timer runs when `+0x7c` is set: a set `+0x7d` stamps `+0x74` with
/// the tick global and clears itself then, and the (wrapping) sum of
/// `+0x78` and `+0x74` at or below the tick skips the liveness gate. The
/// gate is the shared flag/mode/float-length predicate; a live probe that
/// fires takes the fallback path. The check is virtual slot 0x14 of the
/// subtask at `+0x08`, called with (arg, 1, 0) unless its bit 0 skips it;
/// a passed check sets its bit 1. The fallback passes (table, 0.05, 0, 0)
/// with the caller argument as object. Original: thiscall, one stack
/// word, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da51f0(this: u32, arg: u32) -> u32 {
    unsafe {
        const PROBE: u32 = 1;
        const CHECK: u32 = 2;
        const DISPATCH: u32 = 3;
        const FALLBACK: u32 = 4;
        const THRESH_SLOT: u32 = 0x00FE876C;
        const TICK_SLOT: u32 = 0x011735B4;
        const FALLBACK_TABLE: u32 = 0x00EEF598;
        const FALLBACK_BLEND: u32 = 0x3D4CCCCD;
        const CHECK_SLOT: u32 = 0x14;

        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let sub = ((this + 0x08) as *const u32).read();
        let mut skip_gate = false;
        if ((this + 0x7C) as *const u8).read() != 0 {
            if ((this + 0x7D) as *const u8).read() != 0 {
                let tick = (lf_checker_rt::relocated(TICK_SLOT) as *const u32).read();
                ((this + 0x74) as *mut u32).write(tick);
                ((this + 0x7D) as *mut u8).write(0);
            }
            let start = ((this + 0x74) as *const u32).read();
            let span = ((this + 0x78) as *const u32).read();
            let tick = (lf_checker_rt::relocated(TICK_SLOT) as *const u32).read();
            if start.wrapping_add(span) <= tick {
                skip_gate = true;
            }
        }
        if !skip_gate {
            let flag = ((this + 0x46) as *const u8).read();
            let mode = ((this + 0x48) as *const u32).read();
            let live = if flag != 0 {
                mode != 0
            } else if mode != 0 {
                false
            } else {
                let x = ((this + 0x30) as *const f32).read();
                let y = ((this + 0x34) as *const f32).read();
                let z = ((this + 0x38) as *const f32).read();
                let sq = add(add(mul(x, x), mul(y, y)), mul(z, z));
                let thresh = (lf_checker_rt::relocated(THRESH_SLOT) as *const f32).read();
                sq > thresh
            };
            if live {
                let ans: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, this, arg);
                if (ans & 0xFF) != 0 {
                    let table = lf_checker_rt::relocated(FALLBACK_TABLE);
                    lf_checker_rt::callee_thiscall!(FALLBACK, u32, arg, table, FALLBACK_BLEND, 0, 0);
                    return sub;
                }
            }
        }
        if ((sub + 0x0C) as *const u8).read() & 1 == 0 {
            let table = (sub as *const u32).read();
            let slot = ((table + CHECK_SLOT) as *const u32).read();
            let check: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            let ok = check(sub, arg, 1, 0);
            if (ok & 0xFF) == 0 {
                let table = lf_checker_rt::relocated(FALLBACK_TABLE);
                lf_checker_rt::callee_thiscall!(FALLBACK, u32, arg, table, FALLBACK_BLEND, 0, 0);
                return sub;
            }
            let marks = (sub + 0x0C) as *mut u32;
            marks.write(marks.read() | 2);
        }
        lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, arg);
        0
    }
});
