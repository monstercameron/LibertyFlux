// original: 0x00C7E200 CTaskComplexStationaryScenario::vf19 STAGE A (partial)

/// Stationary-scenario task update, stage A: everything except the
/// heading-angle regions.
///
/// `this` is the scenario task, `ped` the ped it drives. The full update
/// gates on the task's phase word at `+0x14` (phase `0x39` aims the
/// scenario, phase `0x46` poses it), refreshes the cached pose while the
/// flag at `+0x58` is clear, then either builds follow-up tasks when the
/// ped has moved away or settles the scenario. Every one of those regions
/// calls the game's arc-tangent helper, which takes its two doubles in
/// the vector registers and returns a double the same way; the checker
/// cannot script such a callee, so this stage pins the inputs away from
/// all five of its call sites (see the contract) and traps if one is
/// reached. What IS proven: the entry gating, the cached-pose setup, the
/// moved-away distance test against the global at `G_LIM`, the full
/// task-building dance through the task-system singleton (callees B-H),
/// and the settle tail, including the vtable install at `G_VTAB` and the
/// low-byte-masked call argument.
///
/// Proven paths in detail: setup copies the cached pose words
/// (`+0x30/0x34/0x38/0x50`) to the frame; the distance test squares the
/// ground deltas between the cached pose and the ped matrix's position
/// (`[ped+0x20]+0x30/0x34`) and compares against `G_LIM` (unordered takes
/// the settle side). The build path stamps
/// `+0x54`, resolves the worker task (callee B) and chains two groups of
/// scenario/chain calls (callees C-G, the buffer arguments pointing at
/// the frame's pose words with a zero fourth word) before returning the
/// worker. The settle path either installs a fresh task object (callee H
/// plus the vtable and zero writes) when both status bytes (`+0x24`,
/// `+0x65`) are clear, or issues one final scenario call whose fourth
/// argument carries only the low byte of `+0x59`. A null singleton at any
/// step ends the tick with 0 or with the worker so far, exactly as the
/// original.
///
/// The traps (`panic!`) mark the four pinned-away regions: phase `0x39`,
/// phase `0x46`, the pose-refresh block and the object-transform block.
/// They must never fire under the stage-A contract; reaching one fails
/// the trial loudly rather than passing vacuously.
///
/// Original: 0x00C7E200 (thiscall, one stack word, returns eax).
lf_checker_rt::export!(thiscall, rw_00c7e200(this: u32, ped: u32) -> u32 {
    unsafe {
        const PHASE: u32 = 0x14;
        const PHASE_AIM: u32 = 0x39;
        const PHASE_POSE: u32 = 0x46;
        const PHASE_TAIL: u32 = 0x35;
        const DIRTY: u32 = 0x58;
        const SUBJECT: u32 = 0x1C;
        const PED_MATRIX: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const STAMP54: u32 = 0x54;
        const STAMP54_V: u32 = 1;
        const STATUS_A: u32 = 0x24;
        const STATUS_B: u32 = 0x65;
        const STATUS_C: u32 = 0x59;
        const G_LIM: u32 = 0x00FE8778;
        const G_E1C: u32 = 0x00FE870C;
        const G_C: u32 = 0x00EE1EB4;
        const G_D: u32 = 0x00EE1EB0;
        const G_VTAB: u32 = 0x00EB391C;
        const NEG_ONE_BITS: u32 = 0xBF800000;
        const EIGHT_BITS: u32 = 0x41000000;
        const PI_BITS: u32 = 0x40490FDB;
        const A_E676: u32 = 1;
        const A_E692: u32 = 2;
        const A_E6A9: u32 = 3;
        const A_E6D7: u32 = 4;
        const A_E749: u32 = 5;
        const A_E760: u32 = 6;
        const A_E78E: u32 = 7;
        const A_E817: u32 = 8;
        const A_E87C: u32 = 9;
        const C_WORKER: u32 = 11;
        const C_SCEN: u32 = 12;
        const C_BUILD: u32 = 13;
        const C_CHAIN: u32 = 14;
        const C_FIN: u32 = 15;
        const C_CHAT: u32 = 16;
        const C_INSTALL: u32 = 17;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn rdb(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wrb(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        unsafe fn sys_of(id: u32) -> u32 {
            unsafe {
                const G_TASK_SYS: u32 = 0x0167E2A0;
                lf_checker_rt::callee_thiscall!(
                    id,
                    u32,
                    lf_checker_rt::global::<u32>(G_TASK_SYS).read()
                )
            }
        }

        let phase = rd32(this.wrapping_add(PHASE));
        if phase == PHASE_AIM {
            panic!("stage A trap: aim phase needs the xmm-double callee");
        }
        if phase == PHASE_POSE {
            panic!("stage A trap: pose phase needs the xmm-double callee");
        }
        if rdb(this.wrapping_add(DIRTY)) == 0 {
            panic!("stage A trap: pose-refresh block needs the xmm-double callee");
        }
        // E4E3 setup: the cached pose words land in the frame slots the
        // build calls below read back.
        let s30 = rd32(this.wrapping_add(0x30));
        let s34 = rd32(this.wrapping_add(0x34));
        let s38 = rd32(this.wrapping_add(0x38));
        let s50 = rd32(this.wrapping_add(0x50));
        if rd32(this.wrapping_add(SUBJECT)) != 0 {
            panic!("stage A trap: object-transform block is unreachable (faults or needs the xmm-double callee)");
        }
        // E643 distance test: the vector registers still hold the cached
        // pose words from the setup above (the direct jump here skips the
        // frame reloads further down).
        let pedm = rd32(ped.wrapping_add(PED_MATRIX));
        let dy = sub(f32::from_bits(s34), rdf(pedm.wrapping_add(POS_Y)));
        let dx = sub(f32::from_bits(s30), rdf(pedm.wrapping_add(POS_X)));
        let dist2 = add(mul(dy, dy), mul(dx, dx));
        if !(dist2 > rdf(lf_checker_rt::relocated(G_LIM))) {
            // E805 settle tail.
            if rdb(this.wrapping_add(STATUS_A)) != 0 || rdb(this.wrapping_add(STATUS_B)) != 0
            {
                let e1c = if phase == PHASE_TAIL {
                    rd32(lf_checker_rt::relocated(G_E1C))
                } else {
                    0
                };
                let c = rdb(this.wrapping_add(STATUS_C));
                wrb(this.wrapping_add(STATUS_C), 0);
                let sys = sys_of(A_E87C);
                if sys == 0 {
                    return 0;
                }
                let b65 = rd32(this.wrapping_add(STATUS_B)) & 0xFF;
                let a_clear = if rdb(this.wrapping_add(STATUS_A)) == 0 { 1 } else { 0 };
                return lf_checker_rt::callee_thiscall!(
                    C_SCEN, u32, sys, 0, e1c, b65, c as u32, a_clear, 0xC
                );
            }
            let sys = sys_of(A_E817);
            if sys == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(C_INSTALL, u32, sys);
            wr32(sys, lf_checker_rt::relocated(G_VTAB));
            wr32(sys.wrapping_add(0x14), 0);
            wrb(sys.wrapping_add(0x18), 0);
            wr32(sys.wrapping_add(0x1C), 0);
            return sys;
        }
        // E669 build path.
        wr32(this.wrapping_add(STAMP54), STAMP54_V);
        let sys = sys_of(A_E676);
        let worker: u32 = if sys == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(C_WORKER, u32, sys)
        };
        let buf = [s30, s34, s38, 0u32];
        let sys2 = sys_of(A_E692);
        let chained: u32 = if sys2 == 0 {
            0
        } else {
            let sys3 = sys_of(A_E6A9);
            let first: u32 = if sys3 == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    C_SCEN, u32, sys3, 0, NEG_ONE_BITS, 0, 0, 1, 0xFFFFFFFF
                )
            };
            let sys4 = sys_of(A_E6D7);
            let second: u32 = if sys4 == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    C_BUILD,
                    u32,
                    sys4,
                    2,
                    buf.as_ptr() as u32,
                    rd32(lf_checker_rt::relocated(G_D)),
                    rd32(lf_checker_rt::relocated(G_C)),
                    0xFFFFFFFF,
                    1,
                    0,
                    0,
                    0,
                    1
                )
            };
            lf_checker_rt::callee_thiscall!(C_CHAIN, u32, sys2, second, first, 0, 0)
        };
        lf_checker_rt::callee_thiscall!(C_FIN, u32, worker, chained);
        let sys5 = sys_of(A_E749);
        if sys5 == 0 {
            lf_checker_rt::callee_thiscall!(C_FIN, u32, worker, 0);
            return worker;
        }
        let sys6 = sys_of(A_E760);
        let third: u32 = if sys6 == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(
                C_SCEN, u32, sys6, 0, NEG_ONE_BITS, 0, 0, 1, 0xFFFFFFFF
            )
        };
        let sys7 = sys_of(A_E78E);
        let fourth: u32 = if sys7 == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(
                C_CHAT,
                u32,
                sys7,
                buf.as_ptr() as u32,
                s50,
                EIGHT_BITS,
                PI_BITS,
                0x7D0
            )
        };
        let chained2: u32 =
            lf_checker_rt::callee_thiscall!(C_CHAIN, u32, sys5, fourth, third, 0, 0);
        lf_checker_rt::callee_thiscall!(C_FIN, u32, worker, chained2);
        worker
    }
});
