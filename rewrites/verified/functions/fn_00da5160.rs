// original: 0x00DA5160 CTaskComplexShockingEventFlee::vf20

/// Periodic update: when the task is live, probe the target and keep the
/// current subtask if the probe fires; otherwise run the virtual state
/// check, mark the task, forward to the dispatch helper and report 0. A
/// refused check keeps the subtask too.
///
/// Liveness is the shared gate (flag/mode/float-length predicate, float
/// arithmetic in the original's order). The check is virtual slot 0x14 of
/// `this`, called with (arg, 1, 0) unless bit 0 of `+0x0c` skips it; a
/// passed check sets bit 1 of `+0x0c`. Returns the subtask at `+0x08` on
/// the keep paths, 0 after a dispatch. Original: thiscall, one stack
/// word, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da5160(this: u32, arg: u32) -> u32 {
    unsafe {
        const PROBE: u32 = 1;
        const CHECK: u32 = 2;
        const DISPATCH: u32 = 3;
        const THRESH_SLOT: u32 = 0x00FE876C;
        const CHECK_SLOT: u32 = 0x14;

        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

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
        let sub = ((this + 0x08) as *const u32).read();
        if live {
            let ans: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, this, arg);
            if (ans & 0xFF) != 0 {
                return sub;
            }
        }
        if ((this + 0x0C) as *const u8).read() & 1 == 0 {
            let table = (this as *const u32).read();
            let slot = ((table + CHECK_SLOT) as *const u32).read();
            let check: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            let ok = check(this, arg, 1, 0);
            if (ok & 0xFF) == 0 {
                return sub;
            }
            let marks = (this + 0x0C) as *mut u32;
            marks.write(marks.read() | 2);
        }
        lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, arg);
        0
    }
});
