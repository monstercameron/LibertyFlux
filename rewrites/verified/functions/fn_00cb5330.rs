// original: 0x00cb5330 ped_task_dispatch_extended (proposed)

/// Dispatch a ped task request by code, with float, random-pick and
/// follow-up creation paths plus a default cleanup path.
///
/// `this` is the requesting object: a selector dword at `+0x2c`, a cached
/// pick at `+0x28`, a follow-up handle at `+0x30`, floats at `+0x20` and
/// `+0x24`, dwords at `+0x34`/`+0x38` and a flag byte at `+0x3c`. `code`
/// selects the path; `ctx` is a context the float and follow-up paths read
/// (a float at `+0xaa0`, a pointer at `+0x20`).
///
/// Paths: `0x386` adds pi to the context float, converts the callee's double
/// answer to float and creates a tuned task; `0x11a` branches on the
/// selector (0 builds a follow-up handle through a three-argument call and
/// creates a default task, 2 reuses or randomly repicks the cached value and
/// creates a task from it, anything else falls through to the default);
/// `0x3ae` creates a ten-argument task and tags the result; `0x516` and any
/// other code release the follow-up handle when set and return 0, as does a
/// null task manager. A null manager on the `0x3ae` path faults on a null
/// write, exactly like the original.
///
/// The gate call fetches the task manager from a global; every creation call
/// takes it in ECX. The float operation order is the original's.
///
/// Original: 0x00cb5330 (thiscall, two stack words; true size 500 bytes, the
/// shared return-0 epilogue lies just past the listed 495), returns the
/// creation call's result, or 0 on the default and manager-missing paths.
lf_checker_rt::export!(thiscall, rw_00cb5330(this: u32, code: u32, ctx: u32) -> u32 {
    unsafe {
        const SEL: u32 = 0x2c;
        const PICK: u32 = 0x28;
        const HANDLE: u32 = 0x30;
        const F20: u32 = 0x20;
        const F24: u32 = 0x24;
        const DW34: u32 = 0x34;
        const DW38: u32 = 0x38;
        const FLAGS: u32 = 0x3c;
        const CTX_ROW: u32 = 0x20;
        const CTX_F: u32 = 0xaa0;
        const GATE_CALLEE: u32 = 1;
        const DBL_CALLEE: u32 = 2;
        const TUNE_CALLEE: u32 = 3;
        const RND_CALLEE: u32 = 4;
        const MAKE_CALLEE: u32 = 5;
        const FOLLOW_CALLEE: u32 = 6;
        const BIG_CALLEE: u32 = 7;
        const REL_CALLEE: u32 = 8;
        const TASK_MGR_GLOBAL: u32 = 0x167e2a0;
        const SEED_GLOBAL: u32 = 0x11735b4;
        const PI: f32 = 3.1415927410125732;
        const ONE: f32 = 1.0;
        const SMALL: f32 = 0.02;
        const EIGHT: f32 = 8.0;
        const THREE: f32 = 3.0;
        const FOUR_BITS: u32 = 0x40800000;

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
        unsafe fn gate() -> u32 {
            unsafe {
                let g = lf_checker_rt::global::<u32>(TASK_MGR_GLOBAL).read();
                lf_checker_rt::callee_thiscall!(GATE_CALLEE, u32, g)
            }
        }

        let mut c = code;
        loop {
            if c == 0x3ae {
                let mgr = gate();
                if mgr == 0 {
                    // Null write the original also makes; fault parity holds.
                    (0x5c as *mut u32).write_unaligned(FOUR_BITS);
                    return 0;
                }
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    BIG_CALLEE, u32, mgr,
                    rdf(this + F20).to_bits(), this.wrapping_add(0x40),
                    ONE.to_bits(), THREE.to_bits(),
                    0xffffffff, 1, 0, 0, 0, 1
                );
                ((r + 0x5c) as *mut u32).write_unaligned(FOUR_BITS);
                return r;
            }
            if c > 0x3ae {
                if c == 0x516 {
                    break;
                }
                c = 0x516;
                continue;
            }
            if c == 0x11a {
                let sel = rd32(this + SEL);
                if sel == 0 {
                    let p = rd32(ctx + CTX_ROW).wrapping_add(0x30);
                    let r: u32 = lf_checker_rt::callee_cdecl!(
                        FOLLOW_CALLEE, u32, p, rdf(this + F24).to_bits(), ctx
                    );
                    ((this + HANDLE) as *mut u32).write_unaligned(r);
                    let mgr = gate();
                    if mgr == 0 {
                        return 0;
                    }
                    return lf_checker_rt::callee_thiscall!(
                        MAKE_CALLEE, u32, mgr, 0xfa0, 0, 0, EIGHT.to_bits()
                    );
                }
                if sel == 2 {
                    let pick = rd32(this + PICK);
                    let v = if pick == 0xffffffff {
                        ((this + FLAGS) as *mut u8).write(0);
                        lf_checker_rt::callee_cdecl!(RND_CALLEE, u32, 0x1f40, 0x3e80)
                    } else {
                        if rd8(this + FLAGS) == 0 {
                            let seed = lf_checker_rt::global::<u32>(SEED_GLOBAL).read();
                            ((this + DW34) as *mut u32).write_unaligned(seed);
                            ((this + DW38) as *mut u32).write_unaligned(pick);
                            ((this + FLAGS) as *mut u8).write(1);
                        }
                        pick
                    };
                    let mgr = gate();
                    if mgr == 0 {
                        return 0;
                    }
                    return lf_checker_rt::callee_thiscall!(
                        MAKE_CALLEE, u32, mgr, v, 0, 0, EIGHT.to_bits()
                    );
                }
                c = 0x516;
                continue;
            }
            if c == 0x386 {
                let x = core::hint::black_box(rdf(ctx + CTX_F))
                    + core::hint::black_box(PI);
                let d: f64 = lf_checker_rt::callee_cdecl!(DBL_CALLEE, f64, x.to_bits());
                let s = d as f32;
                let mgr = gate();
                if mgr == 0 {
                    return 0;
                }
                return lf_checker_rt::callee_thiscall!(
                    TUNE_CALLEE, u32, mgr,
                    s.to_bits(), ONE.to_bits(), SMALL.to_bits()
                );
            }
            c = 0x516;
        }
        let h = rd32(this + HANDLE);
        if h != 0 {
            lf_checker_rt::callee_cdecl!(REL_CALLEE, u32, h);
        }
        0
    }
});
