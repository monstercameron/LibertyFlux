// original: 0x00b73d30 task_ctor_guarded_branch (proposed)

/// Build a guarded follow-up task through two dispatch switches.
///
/// `this` is the task object (state at `+STATE`, host at `+HOST`, probe
/// flag written at `+PROBED`, created task at `+TASK`); `arg` is the
/// host-side record (callback at `+CALLBACK`, level word at `+LEVEL`,
/// mode byte at `+WIDE`, scale gate at `+NO_SCALE`).
///
/// Behaviour: callee 0 probes the host (its low-byte answer is stored at
/// `+PROBED`); when it succeeds, a bias flag is set from the host's inner
/// value at `+INNER` (above one half for most states, below minus one
/// half for states 5-6). A float slot starts at `K_SLOT`; when the wide
/// mode byte is set, the game-data gate equals 1, the host's mode word
/// at `+MODE` equals 1 and callee 1 answers non-zero, it reloads from
/// `K_SLOT2` (game data). Two predicate bits are then derived: `dl`
/// (host present, bit 5 of the table word at `+BFLAG`, state 6 or 8)
/// and, when the first switch is skipped, `cl` (`dl`, the kind word at
/// `+KIND` equalling `KIND_GO`, or the mode word equalling 2).
///
/// The request code starts at `0x14C`. When neither the wide byte, the
/// kind word nor `dl` is set, and the probe or the bias succeeded,
/// callee 2 runs and its answer (unless -1) selects the first switch
/// (states 5-6: `0x161` plus the bias; states 7-8: `0x163`/`0x160` by the
/// bias). Otherwise the second switch runs: per state a `cl`-set code
/// (`0x14C`/`0x14E`/`0x14D`/`0x14F`), a wide-byte code that also reloads
/// the slot from `K_WIDE` (`0x171`/`0x171`/`0x173`/`0x173`), and for
/// state 5 a fallback `0x150` when the subtype byte at `+SUB` or the
/// game-data byte is non-zero. The code lives in one slot shared with
/// both callee calls: each call overwrites it, and a switch default keeps
/// whatever the last call wrote (`0x14C` only when no call ran yet).
///
/// Callee 3 runs with the code by reference (it overwrites the slot) and
/// its answer becomes the task; callee 4 creates from (task, overwritten
/// slot, 8.0, -1). Unless the scale gate is set, the duration stored at
/// `+DURATION` is the slot minus the scaled level word
/// (`level*K_A*K_B`, in that operand order), else the slot itself.
/// Callee 5 attaches a non-null task (the null path is dead: both
/// duration stores fault first). The stack cookie is verified through
/// callee 6, which preserves the registers; the rewrite passes the cookie
/// value directly since its own stack layout differs.
///
/// Returns callee 5's answer.
///
/// Original: 0x00b73d30 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b73d30(this: u32, arg: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x24;
        const HOST: u32 = 0x20;
        const TASK: u32 = 0x18;
        const PROBED: u32 = 0x1c;
        const SUB: u32 = 0x1d;
        const MODEL: u32 = 0x2e;
        const MODE: u32 = 0x1304;
        const INNER: u32 = 0x20;
        const KIND: u32 = 0xc4;
        const BFLAG: u32 = 0x94;
        const TABLE: u32 = 0x01295cd8;
        const GATE1: u32 = 0x011d6fd4;
        const GATE2: u32 = 0x01670ce4;
        const COOKIE: u32 = 0x01057fb4;
        const CALLBACK: u32 = 0x78;
        const LEVEL: u32 = 0x2c;
        const WIDE: u32 = 0x211;
        const NO_SCALE: u32 = 0x219;
        const DURATION: u32 = 0x54;
        const K_HALF: u32 = 0x00fe8830;
        const K_NEG_HALF: u32 = 0x00fe8d7c;
        const K_SLOT: u32 = 0x00fe88e8;
        const K_SLOT2: u32 = 0x01046b6c;
        const K_WIDE: u32 = 0x00fe8a24;
        const K_A: u32 = 0x00fe8684;
        const K_B: u32 = 0x00fe879c;
        const VTBL: u32 = 0x00b6fe10;
        const KIND_GO: u32 = 0x57;
        const EIGHT: u32 = 0x4100_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn second_switch(
            state: u32,
            cl: bool,
            wide: bool,
            sub: u8,
            gate2: u8,
            k_wide: f32,
            slot_a: &mut f32,
            slot: u32,
        ) -> u32 {
            unsafe {
                match state {
                    5 => {
                        if cl {
                            0x14c
                        } else if wide {
                            *slot_a = k_wide;
                            0x171
                        } else if sub != 0 || gate2 != 0 {
                            0x150
                        } else {
                            0x14c
                        }
                    }
                    6 => {
                        if cl {
                            0x14e
                        } else if wide {
                            *slot_a = k_wide;
                            0x171
                        } else {
                            0x14e
                        }
                    }
                    7 => {
                        if cl {
                            0x14d
                        } else if wide {
                            *slot_a = k_wide;
                            0x173
                        } else {
                            0x14d
                        }
                    }
                    8 => {
                        if cl {
                            0x14f
                        } else if wide {
                            *slot_a = k_wide;
                            0x173
                        } else {
                            0x14f
                        }
                    }
                    _ => slot,
                }
            }
        }

        let host = rd32(this + HOST);
        let mut dummy = 0u32;
        let probe: u32 =
            lf_checker_rt::callee_thiscall!(0, u32, &mut dummy as *mut u32 as u32, host);
        let state = rd32(this + STATE);
        let ok = probe & 0xff != 0;
        wr8(this + PROBED, ok as u8);
        let mut bl = 0u32;
        if ok {
            let m = rdf(rd32(host + INNER) + 8);
            if state == 5 || state == 6 {
                if rdf(lf_checker_rt::relocated(K_NEG_HALF)) > m {
                    bl = 1;
                }
            } else if m > rdf(lf_checker_rt::relocated(K_HALF)) {
                bl = 1;
            }
        }
        let mut slot_a = rdf(lf_checker_rt::relocated(K_SLOT));
        let wide = rd8(arg + WIDE) != 0;
        if rd8(arg + NO_SCALE) != 0
            && rd32(lf_checker_rt::relocated(GATE1)) == 1
            && host != 0
            && rd32(host + MODE) == 1
        {
            let go: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
            if go & 0xff != 0 {
                slot_a = rdf(lf_checker_rt::relocated(K_SLOT2));
            }
        }
        let idx = rd16(host + MODEL) as u16 as i16 as i32;
        let entry = rd32(
            lf_checker_rt::relocated(TABLE).wrapping_add((idx as u32).wrapping_mul(4)),
        );
        let kind = rd32(entry + KIND);
        let dl = host != 0
            && (rd32(entry + BFLAG) >> 5) & 1 == 1
            && (state == 6 || state == 8);
        // One slot is shared by both callee 2/3 calls and both switches:
        // the stub overwrites it on each call, and a switch default keeps
        // whatever the last stub wrote.
        let mut slot: u32 = 0x14c;
        let first = !(wide || kind == KIND_GO || dl) && (ok || bl != 0);
        if first {
            let r2: u32 = lf_checker_rt::callee_cdecl!(
                2,
                u32,
                kind,
                &mut slot as *mut u32 as u32,
                arg,
                host,
                0,
                0,
                0,
                1
            );
            if r2 != 0xffff_ffff {
                match state {
                    5 | 6 => slot = 0x161 + (bl != 0) as u32,
                    7 | 8 => {
                        slot = if bl != 0 { 0x163 } else { 0x160 };
                    }
                    _ => {}
                }
            } else {
                // Falls through to the second switch below.
                let cl = dl || kind == KIND_GO || rd32(host + MODE) == 2;
                slot = second_switch(
                    state, cl, wide, rd8(this + SUB),
                    rd8(lf_checker_rt::relocated(GATE2)),
                    rdf(lf_checker_rt::relocated(K_WIDE)),
                    &mut slot_a, slot,
                );
            }
        } else {
            let cl = dl || kind == KIND_GO || rd32(host + MODE) == 2;
            slot = second_switch(
                state, cl, wide, rd8(this + SUB),
                rd8(lf_checker_rt::relocated(GATE2)),
                rdf(lf_checker_rt::relocated(K_WIDE)),
                &mut slot_a, slot,
            );
        }
        let task: u32 = lf_checker_rt::callee_cdecl!(
            3,
            u32,
            kind,
            &mut slot as *mut u32 as u32,
            arg,
            host,
            0,
            0,
            0,
            1
        );
        let task2: u32 = lf_checker_rt::callee_thiscall!(
            4,
            u32,
            rd32(arg + CALLBACK),
            task,
            slot,
            EIGHT,
            0xffff_ffff
        );
        wr32(this + TASK, task2);
        if rd8(arg + NO_SCALE) == 0 {
            let level = rd16(arg + LEVEL) as i32 as f32;
            let t = mul(
                mul(level, rdf(lf_checker_rt::relocated(K_A))),
                rdf(lf_checker_rt::relocated(K_B)),
            );
            wrf(task2 + DURATION, sub(slot_a, t));
        } else {
            wrf(task2 + DURATION, slot_a);
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(
            5,
            u32,
            task2,
            1,
            lf_checker_rt::relocated(VTBL),
            this
        );
        lf_checker_rt::callee_thiscall!(6, u32, rd32(lf_checker_rt::relocated(COOKIE)));
        r
    }
});
