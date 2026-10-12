//! Differential cases for the scenario's main update tick.
//!
//! The lifted tick and its verified 32-bit rewrite run on the same generated trials.
//! Each trial fixes the scenario's inputs, the pedestrian's records, and what every
//! callee answers. The comparison covers the returned word, every call in order with
//! its arguments (the rewrite's callees are stubs on this side, the host's methods on
//! the lifted side), and the words each side leaves behind in the scenario, the
//! pedestrian's intelligence record and the leg anchor.
//!
//! The heading callee answers a double on the lifted side; the rewrite can only read
//! the low word of that answer, so the trials give it only the three answers the
//! rewrite maps (1.1, 1.2 and 0.0). This is a narrowing, stated in the registry.
//! 32-bit target only.

// Test-only harness: the rewrite takes raw addresses, so the harness gives it the
// addresses of its own buffers and stubs. The lifted crate has no unsafe.
#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::{Mutex, MutexGuard, OnceLock};

    use lf_peds_diff::rewrites::fn_00c7bc70;

    #[path = "../mutants/tick_mutants.rs"]
    mod tick_mutants;
    use tick_mutants::Mut;
    use lf_peds_diff::rt::{set_callee, set_global};
    use lf_peds_tasks::move_between_points::{
        MoveBetweenPointsScenario as Scen, RouteFirst, RouteSecond, TickHost, WorldQuery,
    };

    // ---- names and tokens ----

    /// Tokens the lifted host answers with. The same numbers are the words the trial
    /// stores in the rewrite's memory, so the log names them identically on both sides.
    const LEG_TOK: u32 = 0xE000_0001;
    const WORLD_TOK: u32 = 0xE000_0002;
    const WORLD_ANCHOR_TOK: u32 = 0xE000_0003;
    const PED_TOK: u32 = 0xE000_0010;
    const BASE_TOK: u32 = 0xE000_0011;
    const GROUP_TOK: u32 = 0xE000_0012;
    const CONFIG_TOK: u32 = 0xE000_0013;
    const ANCHOR_TOK: u32 = 0xE000_0014;
    const WORLD_QUERY_TOK: u32 = 0xE000_0100;
    const CALL_TOK: u32 = 0xE000_0200;
    const SUB_TOK: u32 = 0xE000_0300;
    const ANCHOR_SUB_TOK: u32 = 0xE000_0301;

    /// The names of tokens and of the scenario's own words, shared by both sides.
    const NAMES: [(u32, &str); 9] = [
        (PED_TOK, "ped"),
        (BASE_TOK, "pos"),
        (GROUP_TOK, "group"),
        (CONFIG_TOK, "config"),
        (ANCHOR_TOK, "anchor"),
        (LEG_TOK, "leg"),
        (WORLD_TOK, "world"),
        (WORLD_ANCHOR_TOK, "world_anchor"),
        (ANCHOR_SUB_TOK, "anchor_sub"),
    ];

    /// The shared name of a token on the lifted side (the 32-bit side maps the same
    /// numbers through the same table).
    fn tok(v: u32) -> Arg {
        match NAMES.iter().find(|(t, _)| *t == v) {
            Some((_, n)) => Arg::Sym(n),
            None => Arg::Val(v),
        }
    }

    // ---- call log ----

    /// One argument in the call log: a named object, or a plain word.
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Arg {
        Sym(&'static str),
        Val(u32),
    }

    /// One call in the log: the callee's name and its arguments in push order.
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Call {
        name: &'static str,
        args: Vec<Arg>,
    }

    /// What a case leaves behind: the returned word, the calls, and the words it wrote.
    #[derive(Debug, PartialEq, Eq)]
    struct Outcome {
        ret: u64,
        calls: Vec<Call>,
        effects: Vec<Arg>,
    }

    static LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());
    static SYMS: Mutex<Vec<(u32, &'static str)>> = Mutex::new(Vec::new());
    static SERIAL: Mutex<()> = Mutex::new(());

    fn serial() -> MutexGuard<'static, ()> {
        SERIAL.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn sym_of(v: u32) -> Arg {
        let syms = SYMS.lock().unwrap();
        match syms.iter().find(|(a, _)| *a == v) {
            Some((_, name)) => Arg::Sym(name),
            None => Arg::Val(v),
        }
    }

    fn set_syms(list: &[(u32, &'static str)]) {
        let mut syms = SYMS.lock().unwrap();
        syms.clear();
        syms.extend_from_slice(list);
        for (t, n) in NAMES {
            syms.push((t, n));
        }
    }

    fn rec(name: &'static str, args: Vec<Arg>) {
        LOG.lock().unwrap().push(Call { name, args });
    }

    fn take_log() -> Vec<Call> {
        std::mem::take(&mut *LOG.lock().unwrap())
    }

    // ---- trial inputs ----

    /// Everything one trial fixes: the scenario's own words, the pedestrian's and the
    /// shared words, and what each callee answers.
    #[derive(Clone, Copy, Debug)]
    struct Case {
        aux: u32,
        state: u32,
        active: u8,
        pending: u32,
        status: u32,
        wait: u32,
        payload: u32,
        step: u32,
        span: u32,
        draw: u32,
        route0_ok: u32,
        route1_ok: u32,
        route1_aux: u32,
        dest0: [u32; 4],
        base: [u32; 3],
        seat: u32,
        intel_anchor: u32,
        commit_leg: u32,
        leg_ok: u32,
        planned: u32,
        world_q: [u32; 7],
        world_anchor: u32,
        dock_ret: u32,
        board_ret: u32,
        seat_ret: u32,
        leg_len: u32,
        leg_vec: [u32; 4],
        head_pick: u32,
        sub_op1: u32,
        sub_op2: u32,
        weight_narrow: u32,
        weight_wide: u32,
        fare_lo: u32,
        fare_hi: u32,
        fare2_lo: u32,
        fare2_hi: u32,
        mask: u32,
        approach_ret: u32,
        anchor_aux: u32,
        task_sub: u32,
    }

    /// The three heading answers the rewrite can map back, as the double's low word
    /// and as the double itself.
    const HEAD_LOW: [u32; 3] = [0x9999_999A, 0x3333_3333, 0x0000_0000];
    const HEAD_VALUE: [f64; 3] = [1.1, 1.2, 0.0];

    /// Small deterministic generator.
    fn next(seed: &mut u64) -> u32 {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (*seed >> 33) as u32
    }

    fn pick(seed: &mut u64, options: &[u32]) -> u32 {
        options[(next(seed) as usize) % options.len()]
    }

    fn gen_case(seed: &mut u64) -> Case {
        let f = |x: f32| x.to_bits();
        let far = [f(10.0), f(2.0), f(3.0), 0x1234];
        let near = [f(1.5), f(2.0), f(3.0), 0x5678];
        let nan = [f(f32::NAN), f(2.0), f(3.0), 0x9ABC];
        let dest_pick = pick(seed, &[0, 1, 2, 3]);
        // The edge case puts the squared distance exactly on the threshold.
        let edge = [f(3.0), f(2.0), f(3.0), 0x2222];
        let dest = match dest_pick {
            0 => far,
            1 => near,
            2 => nan,
            _ => edge,
        };
        let leg_vecs = [
            [0x0000_0000, 0x3F80_0000, 0x4000_0000, 0x4040_0000],
            [0x0000_0000, 0xBF80_0000, 0x4080_0000, 0x40A0_0000],
        ];
        let lv = leg_vecs[(next(seed) as usize) % 2];
        let world_q = [0, 1, 2, 3, 4, 5, 6].map(|i| {
            if next(seed) % 2 == 0 { 0 } else { WORLD_QUERY_TOK + i as u32 }
        });
        Case {
            aux: pick(seed, &[0x1D, 0x1E, 0x05]),
            state: pick(seed, &[0, 0, 1, 1, 2]),
            active: pick(seed, &[0, 1]) as u8,
            pending: pick(seed, &[0, 3]),
            status: pick(seed, &[0, 1, 2, 3]),
            wait: pick(seed, &[f(0.25), f(6.0)]),
            payload: pick(seed, &[0, LEG_TOK]),
            step: pick(seed, &[f(0.5), f(2.0)]),
            span: pick(seed, &[f(10.0), f(0.0), f(3.0)]),
            draw: pick(seed, &[0, 0x8000_0000, 0xFFFF_FFFF, 12345]),
            route0_ok: pick(seed, &[0, 1]),
            route1_ok: pick(seed, &[0, 1]),
            route1_aux: pick(seed, &[0x88, 0]),
            dest0: dest,
            base: [f(1.0), f(2.0), f(3.0)],
            seat: pick(seed, &[0, 0x55]),
            intel_anchor: pick(seed, &[0, 0x66]),
            commit_leg: pick(seed, &[0, LEG_TOK]),
            leg_ok: pick(seed, &[0, 1]),
            planned: pick(seed, &[0, 1]),
            world_q,
            world_anchor: WORLD_ANCHOR_TOK,
            dock_ret: pick(seed, &[0, CALL_TOK]),
            board_ret: pick(seed, &[0, CALL_TOK + 1]),
            seat_ret: pick(seed, &[0, CALL_TOK + 2]),
            leg_len: pick(seed, &[0, 4]),
            leg_vec: lv,
            head_pick: pick(seed, &[0, 1, 2]),
            sub_op1: pick(seed, &[0, 1, 0x100, 0xFF]),
            sub_op2: pick(seed, &[0, 1, 0x100, 0xFF]),
            weight_narrow: pick(seed, &[0x3F00_0000, 0x3F80_0000]),
            weight_wide: pick(seed, &[0x3F00_0000, 0x3F80_0000]),
            fare_lo: pick(seed, &[0x1234, 0xFFFF_FFFF]),
            fare_hi: pick(seed, &[0x5678, 0]),
            fare2_lo: pick(seed, &[0x9ABC, 1]),
            fare2_hi: pick(seed, &[0xDEF0, 0]),
            mask: 0x8000_0000,
            approach_ret: pick(seed, &[0, CALL_TOK + 3]),
            anchor_aux: pick(seed, &[0x1D, 0x05]),
            task_sub: SUB_TOK,
        }
    }

    // ---- fixtures ----

    /// A scenario record: 64 bytes, 8-byte aligned.
    #[repr(C, align(8))]
    #[derive(Clone, Copy)]
    struct Words16([u32; 16]);

    /// Shared buffers the rewrite reads through scripted globals, fixed for the run.
    struct Globals {
        step: u32,
        weight_narrow: u32,
        weight_wide: u32,
        fare_hi: u32,
        fare_lo: u32,
        fare2_hi: u32,
        fare2_lo: u32,
        mask: u32,
        world: u32,
        chat_imm: u32,
    }

    static GLOBALS: OnceLock<Globals> = OnceLock::new();

    fn leak_word() -> u32 {
        let b: &'static mut [u32; 1] = Box::leak(Box::new([0u32]));
        b.as_ptr() as usize as u32
    }

    fn globals() -> &'static Globals {
        GLOBALS.get_or_init(|| {
            let g = Globals {
                step: leak_word(),
                weight_narrow: leak_word(),
                weight_wide: leak_word(),
                fare_hi: leak_word(),
                fare_lo: leak_word(),
                fare2_hi: leak_word(),
                fare2_lo: leak_word(),
                mask: leak_word(),
                world: leak_word(),
                chat_imm: leak_word(),
            };
            set_global(0x0117_35BC, g.step);
            set_global(0x00FE_8AD8, g.weight_narrow);
            set_global(0x00FE_8B38, g.weight_wide);
            set_global(0x00EE_1EB4, g.fare_hi);
            set_global(0x00EE_1EB0, g.fare_lo);
            set_global(0x00ED_7E70, g.fare2_hi);
            set_global(0x00ED_7E68, g.fare2_lo);
            set_global(0x00FE_8FA0, g.mask);
            set_global(0x0167_E2A0, g.world);
            set_global(0x00ED_4A40, g.chat_imm);
            g
        })
    }

    fn put(addr: u32, v: u32) {
        // SAFETY: `addr` is a word of a buffer this harness leaked or owns for the run.
        unsafe { (addr as *mut u32).write_unaligned(v) }
    }

    fn get(addr: u32) -> u32 {
        // SAFETY: `addr` is a word of a buffer this harness leaked or owns for the run.
        unsafe { (addr as *const u32).read_unaligned() }
    }

    /// The per-trial 32-bit records of the rewrite: scenario, pedestrian, placement,
    /// intelligence, group, configuration, leg anchor, and the vtables they point at.
    struct Records {
        task: Box<Words16>,
        task_vt: Box<[u32; 8]>,
        ped: Box<[u32; 0x2C0]>,
        placement: Box<[u32; 16]>,
        intel: Box<[u32; 0xB8]>,
        group: Box<[u32; 0x12]>,
        group_vt: Box<[u32; 0x20]>,
        config: Box<[u32; 0x12]>,
        anchor: Box<Words16>,
        anchor_vt: Box<[u32; 0x20]>,
    }

    static ANCHOR_ADDR: AtomicU32 = AtomicU32::new(0);
    static CUR: Mutex<Option<Case>> = Mutex::new(None);

    fn cur() -> Case {
        CUR.lock().unwrap().expect("a case is set")
    }

    // ---- 32-bit stubs ----

    /// The scenario's own sub-operation, through its vtable slot 5.
    extern "thiscall" fn sub_op_stub(task: u32, ped: u32, kind: u32, zero: u32) -> u32 {
        rec("sub_operation", vec![sym_of(task), sym_of(ped), Arg::Val(kind), Arg::Val(zero)]);
        let c = cur();
        if kind == 1 { c.sub_op1 } else { c.sub_op2 }
    }

    /// The group's seat call, through its vtable slot 17.
    extern "thiscall" fn seat_stub(group: u32, zero: u32) -> u32 {
        rec("group_seat", vec![sym_of(group), Arg::Val(zero)]);
        cur().seat
    }

    /// The leg service of the anchor, through its vtable slot 21: writes the four leg words.
    extern "thiscall" fn leg_service_stub(anchor: u32, out: u32, zero: u32) -> u32 {
        rec("leg_service", vec![sym_of(anchor), Arg::Sym("leg_vec"), Arg::Val(zero)]);
        let c = cur();
        for (i, w) in c.leg_vec.iter().enumerate() {
            put(out + 4 * i as u32, *w);
        }
        c.leg_len
    }

    extern "cdecl" fn config_stub(aux: u32) -> u32 {
        rec("config", vec![Arg::Val(aux)]);
        CONFIG_ADDR.load(Ordering::SeqCst)
    }

    extern "cdecl" fn leg_ok_stub(leg: u32) -> u32 {
        rec("leg_ok", vec![sym_of(leg)]);
        cur().leg_ok
    }

    extern "cdecl" fn schedule_stub(slot: u32, s2: u32, s1: u32, s0: u32, zero: u32) -> u32 {
        rec("schedule", vec![sym_of(slot), Arg::Sym("sched2"), Arg::Sym("sched1"), Arg::Sym("sched0"), Arg::Val(zero)]);
        let _ = (s2, s1, s0);
        cur().planned
    }

    extern "cdecl" fn schedule_use_stub(slot: u32) -> u32 {
        rec("schedule_use", vec![sym_of(slot)]);
        0
    }

    /// The first route call: writes the anchor and the spare word 12 bytes past each
    /// frame out-word.
    extern "thiscall" fn route_first_stub(
        aux: u32,
        base: u32,
        weight: u32,
        one_a: u32,
        aux2: u32,
        out0: u32,
        out1: u32,
        one_b: u32,
        ped: u32,
        one_c: u32,
    ) -> u32 {
        rec(
            "route_first",
            vec![
                sym_of(aux),
                sym_of(base),
                Arg::Val(weight),
                Arg::Val(one_a),
                sym_of(aux2),
                Arg::Sym("out0"),
                Arg::Sym("out1"),
                Arg::Val(one_b),
                sym_of(ped),
                Arg::Val(one_c),
            ],
        );
        let c = cur();
        put(out0 + 12, ANCHOR_ADDR.load(Ordering::SeqCst));
        put(out1 + 12, 0x77);
        c.route0_ok
    }

    extern "thiscall" fn dest_first_stub(ecx: u32, dest: u32, zero: u32) -> u32 {
        rec("dest_first", vec![Arg::Val(ecx), Arg::Sym("dest0"), Arg::Val(zero)]);
        for (i, w) in cur().dest0.iter().enumerate() {
            put(dest + 4 * i as u32, *w);
        }
        0
    }

    extern "cdecl" fn commit_route_stub(base: u32, pt: u32, value: u32) -> u32 {
        rec("commit_route", vec![sym_of(base), sym_of(pt), Arg::Val(value)]);
        cur().commit_leg
    }

    /// The second route call: writes the routed anchor and aux word.
    extern "thiscall" fn route_second_stub(
        aux: u32,
        point: u32,
        weight: u32,
        one_a: u32,
        aux2: u32,
        out0: u32,
        out1: u32,
        one_b: u32,
        ped: u32,
        zero: u32,
    ) -> u32 {
        rec(
            "route_second",
            vec![
                sym_of(aux),
                sym_of(point),
                Arg::Val(weight),
                Arg::Val(one_a),
                sym_of(aux2),
                Arg::Sym("out0"),
                Arg::Sym("out1"),
                Arg::Val(one_b),
                sym_of(ped),
                Arg::Val(zero),
            ],
        );
        let c = cur();
        put(out0, ANCHOR_ADDR.load(Ordering::SeqCst));
        put(out1, c.route1_aux);
        c.route1_ok
    }

    extern "thiscall" fn refine_leg_stub(anchor: u32, dest: u32, aux: u32) -> u32 {
        rec("refine_leg", vec![sym_of(anchor), Arg::Sym("dest1"), sym_of(aux)]);
        let _ = dest;
        0
    }

    extern "thiscall" fn commit_a_stub(task: u32, aux: u32) -> u32 {
        rec("commit_a", vec![sym_of(task), sym_of(aux)]);
        0
    }

    extern "thiscall" fn commit_b_stub(task: u32, anchor: u32) -> u32 {
        rec("commit_b", vec![sym_of(task), sym_of(anchor)]);
        0
    }

    fn world_answer(i: usize, world: u32) -> u32 {
        rec(WORLD_NAMES[i], vec![sym_of(world)]);
        cur().world_q[i]
    }

    const WORLD_NAMES: [&str; 7] = [
        "world_a", "world_b", "world_c", "world_d", "world_e", "world_f", "world_g",
    ];

    extern "thiscall" fn world_a_stub(w: u32) -> u32 {
        world_answer(0, w)
    }
    extern "thiscall" fn world_b_stub(w: u32) -> u32 {
        world_answer(1, w)
    }
    extern "thiscall" fn world_c_stub(w: u32) -> u32 {
        world_answer(2, w)
    }
    extern "thiscall" fn world_d_stub(w: u32) -> u32 {
        world_answer(3, w)
    }
    extern "thiscall" fn world_e_stub(w: u32) -> u32 {
        world_answer(4, w)
    }
    extern "thiscall" fn world_f_stub(w: u32) -> u32 {
        world_answer(5, w)
    }
    extern "thiscall" fn world_g_stub(w: u32) -> u32 {
        world_answer(6, w)
    }

    extern "thiscall" fn anchor_stub(world_a: u32) -> u32 {
        rec("anchor_of", vec![Arg::Val(world_a)]);
        WORLD_ANCHOR_TOK
    }

    extern "thiscall" fn dock_stub(ecx: u32, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) -> u32 {
        rec(
            "dock",
            vec![sym_of(ecx), Arg::Val(a), Arg::Val(b), Arg::Val(c), Arg::Val(d), Arg::Val(e), Arg::Val(f)],
        );
        cur().dock_ret
    }

    extern "thiscall" fn board_stub(
        ecx: u32,
        a: u32,
        gate: u32,
        lo: u32,
        hi: u32,
        f: u32,
        g: u32,
        h: u32,
        i: u32,
        j: u32,
        k: u32,
    ) -> u32 {
        rec(
            "board",
            vec![
                sym_of(ecx),
                Arg::Val(a),
                Arg::Sym("gate"),
                Arg::Val(lo),
                Arg::Val(hi),
                Arg::Val(f),
                Arg::Val(g),
                Arg::Val(h),
                Arg::Val(i),
                Arg::Val(j),
                Arg::Val(k),
            ],
        );
        let _ = gate;
        cur().board_ret
    }

    extern "thiscall" fn seat_info_stub(ecx: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
        rec("seat_info", vec![sym_of(ecx), Arg::Val(a), Arg::Val(b), Arg::Val(c), Arg::Val(d)]);
        cur().seat_ret
    }

    extern "thiscall" fn attach_stub(anchor: u32, boarded: u32) -> u32 {
        rec("attach", vec![sym_of(anchor), Arg::Val(boarded)]);
        0
    }

    extern "thiscall" fn leg_length_stub(len: u32) -> u32 {
        rec("leg_length", vec![Arg::Val(len)]);
        0
    }

    extern "cdecl" fn heading_stub() -> u32 {
        rec("heading", vec![]);
        HEAD_LOW[cur().head_pick as usize]
    }

    extern "thiscall" fn approach_stub(world: u32, bits: u32, lo: u32, hi: u32) -> u32 {
        rec("approach", vec![sym_of(world), Arg::Val(bits), Arg::Val(lo), Arg::Val(hi)]);
        cur().approach_ret
    }

    extern "thiscall" fn chat_ping_stub(ped: u32, imm: u32, weight: u32, z1: u32, z2: u32) -> u32 {
        rec("chat_ping", vec![sym_of(ped), sym_of(imm), Arg::Val(weight), Arg::Val(z1), Arg::Val(z2)]);
        0
    }

    extern "cdecl" fn random_stub() -> u32 {
        rec("random", vec![]);
        cur().draw
    }

    static CONFIG_ADDR: AtomicU32 = AtomicU32::new(0);

    // ---- the host the lifted tick runs against ----

    /// The lifted host. It answers from the same case and records the same calls.
    struct Host {
        log: Rc<RefCell<Vec<Call>>>,
        case: Case,
        flag: u32,
        anchor_leg: u32,
        anchor_state: u32,
    }

    impl Host {
        fn push(&self, name: &'static str, args: Vec<Arg>) {
            self.log.borrow_mut().push(Call { name, args });
        }

        fn world_q(&mut self, i: usize, world: u32) -> u32 {
            self.push(WORLD_NAMES[i], vec![tok(world)]);
            self.case.world_q[i]
        }
    }

    impl TickHost for Host {
        fn set_scenario_flag(&mut self, _ped: u32) {
            self.flag = 1;
        }

        fn sub_operation(&mut self, _task: &mut Scen, ped: u32, kind: u32) -> u32 {
            self.push("sub_operation", vec![Arg::Sym("task"), tok(ped), Arg::Val(kind), Arg::Val(0)]);
            if kind == 1 { self.case.sub_op1 } else { self.case.sub_op2 }
        }

        fn config(&mut self, aux_id: u32) -> u32 {
            self.push("config", vec![Arg::Val(aux_id)]);
            CONFIG_TOK
        }

        fn config_span(&mut self, _config: u32) -> f32 {
            f32::from_bits(self.case.span)
        }

        fn random(&mut self) -> u32 {
            self.push("random", vec![]);
            self.case.draw
        }

        fn frame_step(&mut self) -> f32 {
            f32::from_bits(self.case.step)
        }

        fn weight_bits(&mut self, narrow: bool) -> u32 {
            if narrow { self.case.weight_narrow } else { self.case.weight_wide }
        }

        fn placement_base(&mut self, _ped: u32) -> u32 {
            BASE_TOK
        }

        fn position(&mut self, _base: u32) -> [f32; 3] {
            self.case.base.map(f32::from_bits)
        }

        fn route_first(&mut self, base: u32, weight: u32, ped: u32) -> RouteFirst {
            self.push(
                "route_first",
                vec![
                    Arg::Sym("task.aux"),
                    tok(base),
                    Arg::Val(weight),
                    Arg::Val(1),
                    Arg::Sym("task.aux"),
                    Arg::Sym("out0"),
                    Arg::Sym("out1"),
                    Arg::Val(1),
                    tok(ped),
                    Arg::Val(1),
                ],
            );
            RouteFirst { ok: self.case.route0_ok, anchor: ANCHOR_TOK, spare: 0x77 }
        }

        fn refine_first(&mut self) -> [u32; 4] {
            self.push("dest_first", vec![Arg::Val(0), Arg::Sym("dest0"), Arg::Val(0)]);
            self.case.dest0
        }

        fn ped_group(&mut self, _ped: u32) -> u32 {
            GROUP_TOK
        }

        fn group_seat(&mut self, group: u32) -> u32 {
            self.push("group_seat", vec![tok(group), Arg::Val(0)]);
            self.case.seat
        }

        fn intel_anchor(&mut self, _ped: u32) -> u32 {
            self.case.intel_anchor
        }

        fn commit_route(&mut self, base: u32, value: u32) -> u32 {
            self.push("commit_route", vec![tok(base), Arg::Sym("point"), Arg::Val(value)]);
            self.case.commit_leg
        }

        fn set_anchor_leg(&mut self, _anchor: u32, leg: u32) {
            self.anchor_leg = leg;
        }

        fn set_anchor_state(&mut self, _anchor: u32, state: u32) {
            self.anchor_state = state;
        }

        fn leg_ok(&mut self, leg: u32) -> u32 {
            self.push("leg_ok", vec![tok(leg)]);
            self.case.leg_ok
        }

        fn schedule(&mut self, leg: u32) -> u32 {
            self.push("schedule", vec![tok(leg), Arg::Sym("sched2"), Arg::Sym("sched1"), Arg::Sym("sched0"), Arg::Val(0)]);
            self.case.planned
        }

        fn schedule_use(&mut self, leg: u32) {
            self.push("schedule_use", vec![tok(leg)]);
        }

        fn route_second(&mut self, ped: u32, weight: u32) -> RouteSecond {
            self.push(
                "route_second",
                vec![
                    Arg::Sym("task.aux"),
                    Arg::Sym("point"),
                    Arg::Val(weight),
                    Arg::Val(1),
                    Arg::Sym("task.aux"),
                    Arg::Sym("out0"),
                    Arg::Sym("out1"),
                    Arg::Val(1),
                    tok(ped),
                    Arg::Val(0),
                ],
            );
            RouteSecond { ok: self.case.route1_ok, anchor: ANCHOR_TOK, aux: self.case.route1_aux }
        }

        fn refine_leg(&mut self, anchor: u32, aux: u32) {
            self.push("refine_leg", vec![tok(anchor), Arg::Sym("dest1"), tok(aux)]);
        }

        fn commit_a(&mut self, _task: &mut Scen, aux: u32) {
            self.push("commit_a", vec![Arg::Sym("task"), tok(aux)]);
        }

        fn commit_b(&mut self, _task: &mut Scen, anchor: u32) {
            self.push("commit_b", vec![Arg::Sym("task"), tok(anchor)]);
        }

        fn world(&mut self) -> u32 {
            WORLD_TOK
        }

        fn world_query(&mut self, query: WorldQuery, world: u32) -> u32 {
            let i = match query {
                WorldQuery::A => 0,
                WorldQuery::B => 1,
                WorldQuery::C => 2,
                WorldQuery::D => 3,
                WorldQuery::E => 4,
                WorldQuery::F => 5,
                WorldQuery::G => 6,
            };
            self.world_q(i, world)
        }

        fn anchor_of(&mut self, world_a: u32) -> u32 {
            self.push("anchor_of", vec![Arg::Val(world_a)]);
            WORLD_ANCHOR_TOK
        }

        fn dock(&mut self, world: u32, args: [u32; 6]) -> u32 {
            let mut v = vec![tok(world)];
            v.extend(args.iter().map(|a| Arg::Val(*a)));
            self.push("dock", v);
            self.case.dock_ret
        }

        fn move_fare(&mut self) -> (u32, u32) {
            (self.case.fare_lo, self.case.fare_hi)
        }

        fn board(&mut self, world: u32, fare_lo: u32, fare_hi: u32) -> u32 {
            self.push(
                "board",
                vec![
                    tok(world),
                    Arg::Val(2),
                    Arg::Sym("gate"),
                    Arg::Val(fare_lo),
                    Arg::Val(fare_hi),
                    Arg::Val(0xFFFF_FFFF),
                    Arg::Val(1),
                    Arg::Val(0),
                    Arg::Val(0),
                    Arg::Val(0),
                    Arg::Val(1),
                ],
            );
            self.case.board_ret
        }

        fn seat_info(&mut self, object: u32, a: u32, b: u32) -> u32 {
            self.push("seat_info", vec![tok(object), Arg::Val(a), Arg::Val(b), Arg::Val(0), Arg::Val(0)]);
            self.case.seat_ret
        }

        fn attach(&mut self, anchor: u32, boarded: u32) {
            self.push("attach", vec![tok(anchor), Arg::Val(boarded)]);
        }

        fn leg_service(&mut self, anchor: u32) -> (u32, [u32; 4]) {
            self.push("leg_service", vec![tok(anchor), Arg::Sym("leg_vec"), Arg::Val(0)]);
            (self.case.leg_len, self.case.leg_vec)
        }

        fn leg_length(&mut self, len: u32) {
            self.push("leg_length", vec![Arg::Val(len)]);
        }

        fn sign_mask(&mut self) -> u32 {
            self.case.mask
        }

        fn heading(&mut self, _leg_vector: [f32; 3]) -> f64 {
            self.push("heading", vec![]);
            HEAD_VALUE[self.case.head_pick as usize]
        }

        fn approach_fare(&mut self) -> (u32, u32) {
            (self.case.fare2_lo, self.case.fare2_hi)
        }

        fn approach(&mut self, world: u32, approach_bits: u32, fare_lo: u32, fare_hi: u32) -> u32 {
            self.push("approach", vec![tok(world), Arg::Val(approach_bits), Arg::Val(fare_lo), Arg::Val(fare_hi)]);
            self.case.approach_ret
        }

        fn anchor_aux(&mut self, _anchor: u32) -> u32 {
            self.case.anchor_aux
        }

        fn anchor_sub(&mut self, _anchor: u32) -> u32 {
            ANCHOR_SUB_TOK
        }

        fn chat_ping(&mut self, ped: u32, weight_bits: u32) {
            self.push("chat_ping", vec![tok(ped), Arg::Sym("chat_imm"), Arg::Val(weight_bits), Arg::Val(0), Arg::Val(0)]);
        }
    }

    // ---- running the two sides ----

    /// Writes a case into the 32-bit records and runs the rewrite on them.
    fn run_32(c: Case) -> Outcome {
        let g = globals();
        put(g.step, c.step);
        put(g.weight_narrow, c.weight_narrow);
        put(g.weight_wide, c.weight_wide);
        put(g.fare_hi, c.fare_hi);
        put(g.fare_lo, c.fare_lo);
        put(g.fare2_hi, c.fare2_hi);
        put(g.fare2_lo, c.fare2_lo);
        put(g.mask, c.mask);
        put(g.world, WORLD_TOK);
        *CUR.lock().unwrap() = Some(c);

        let mut r = Records {
            task: Box::new(Words16([0; 16])),
            task_vt: Box::new([0; 8]),
            ped: Box::new([0; 0x2C0]),
            placement: Box::new([0; 16]),
            intel: Box::new([0; 0xB8]),
            group: Box::new([0; 0x12]),
            group_vt: Box::new([0; 0x20]),
            config: Box::new([0; 0x12]),
            anchor: Box::new(Words16([0; 16])),
            anchor_vt: Box::new([0; 0x20]),
        };
        r.task_vt[5] = sub_op_stub as *const () as usize as u32;
        r.group_vt[17] = seat_stub as *const () as usize as u32;
        r.anchor_vt[21] = leg_service_stub as *const () as usize as u32;
        let task = r.task.0.as_mut_ptr() as usize as u32;
        let ped = r.ped.as_mut_ptr() as usize as u32;
        let placement = r.placement.as_mut_ptr() as usize as u32;
        let intel = r.intel.as_mut_ptr() as usize as u32;
        let group = r.group.as_mut_ptr() as usize as u32;
        let config = r.config.as_mut_ptr() as usize as u32;
        let anchor = r.anchor.0.as_mut_ptr() as usize as u32;
        r.task.0[0] = r.task_vt.as_ptr() as usize as u32;
        r.group[0] = r.group_vt.as_ptr() as usize as u32;
        r.anchor.0[0] = r.anchor_vt.as_ptr() as usize as u32;
        ANCHOR_ADDR.store(anchor, Ordering::SeqCst);
        CONFIG_ADDR.store(config, Ordering::SeqCst);

        // Scenario words.
        r.task.0[0x08 / 4] = c.task_sub;
        r.task.0[0x0C / 4] = c.status;
        r.task.0[0x14 / 4] = c.aux;
        r.task.0[0x1C / 4] = c.pending;
        // The activity byte is the low byte of the word at 0x20; the other three stay zero.
        r.task.0[0x20 / 4] = u32::from(c.active);
        r.task.0[0x24 / 4] = c.wait;
        r.task.0[0x28 / 4] = c.state;
        r.task.0[0x2C / 4] = c.payload;
        // Anchor: aux and sub-task words; leg and state start at zero.
        r.anchor.0[0x14 / 4] = c.anchor_aux;
        r.anchor.0[0x08 / 4] = ANCHOR_SUB_TOK;
        // Pedestrian: placement, intelligence and group pointers.
        r.ped[0x20 / 4] = placement;
        r.ped[0x224 / 4] = intel;
        r.ped[0xA80 / 4] = group;
        for (i, w) in c.base.iter().enumerate() {
            r.placement[0x30 / 4 + i] = *w;
        }
        r.intel[0x284 / 4] = c.intel_anchor;
        r.config[0x44 / 4] = c.span;

        set_syms(&[
            (task, "task"),
            (task + 0x14, "task.aux"),
            (task + 0x30, "point"),
            (ped, "ped"),
            (placement + 0x30, "pos"),
            (group, "group"),
            (config, "config"),
            (anchor, "anchor"),
            (g.chat_imm, "chat_imm"),
        ]);
        set_callee(2, config_stub as *const () as usize as u32);
        set_callee(3, leg_ok_stub as *const () as usize as u32);
        set_callee(4, schedule_stub as *const () as usize as u32);
        set_callee(5, schedule_use_stub as *const () as usize as u32);
        set_callee(6, route_second_stub as *const () as usize as u32);
        set_callee(7, route_first_stub as *const () as usize as u32);
        set_callee(8, refine_leg_stub as *const () as usize as u32);
        set_callee(9, dest_first_stub as *const () as usize as u32);
        set_callee(10, commit_a_stub as *const () as usize as u32);
        set_callee(11, commit_b_stub as *const () as usize as u32);
        set_callee(12, world_a_stub as *const () as usize as u32);
        set_callee(13, world_b_stub as *const () as usize as u32);
        set_callee(14, world_c_stub as *const () as usize as u32);
        set_callee(15, world_d_stub as *const () as usize as u32);
        set_callee(16, world_e_stub as *const () as usize as u32);
        set_callee(17, world_f_stub as *const () as usize as u32);
        set_callee(18, world_g_stub as *const () as usize as u32);
        set_callee(19, anchor_stub as *const () as usize as u32);
        set_callee(20, dock_stub as *const () as usize as u32);
        set_callee(21, board_stub as *const () as usize as u32);
        set_callee(22, seat_info_stub as *const () as usize as u32);
        set_callee(23, attach_stub as *const () as usize as u32);
        set_callee(24, leg_length_stub as *const () as usize as u32);
        set_callee(25, heading_stub as *const () as usize as u32);
        set_callee(26, approach_stub as *const () as usize as u32);
        set_callee(27, commit_route_stub as *const () as usize as u32);
        set_callee(28, chat_ping_stub as *const () as usize as u32);
        set_callee(29, random_stub as *const () as usize as u32);

        let _ = take_log();
        let ret = fn_00c7bc70::rw_00c7bc70(task, ped);
        let calls = take_log();

        let effects = vec![
            Arg::Val(get(task + 0x0C)),
            Arg::Val(get(task + 0x24)),
            Arg::Val(get(task + 0x28)),
            sym_of(get(task + 0x2C)),
            Arg::Val(get(task + 0x30)),
            Arg::Val(get(task + 0x34)),
            Arg::Val(get(task + 0x38)),
            Arg::Val(get(task + 0x3C)),
            Arg::Val(u32::from(r.intel[0x2DC / 4].to_le_bytes()[0])),
            sym_of(get(anchor + 0x2C)),
            Arg::Val(get(anchor + 0x28)),
        ];
        Outcome { ret: u64::from(ret), calls, effects }
    }

    /// Runs the lifted tick on the same case.
    fn run_lifted(c: Case) -> Outcome {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut host = Host { log: log.clone(), case: c, flag: 0, anchor_leg: 0, anchor_state: 0 };
        let mut s = Scen {
            aux_id: c.aux,
            state: c.state,
            active: c.active,
            pending_guard: c.pending,
            status: c.status,
            payload: c.payload,
            sub_task: c.task_sub,
            wait_timer: f32::from_bits(c.wait),
            ..Scen::default()
        };
        let ret = s.tick(&mut host, PED_TOK);
        let calls = std::mem::take(&mut *log.borrow_mut());
        let effects = vec![
            Arg::Val(s.status),
            Arg::Val(s.wait_timer.to_bits()),
            Arg::Val(s.state),
            tok(s.payload),
            Arg::Val(s.point[0]),
            Arg::Val(s.point[1]),
            Arg::Val(s.point[2]),
            Arg::Val(s.point[3]),
            Arg::Val(host.flag),
            tok(host.anchor_leg),
            Arg::Val(host.anchor_state),
        ];
        Outcome { ret: u64::from(ret), calls, effects }
    }

    /// Runs the wrong version K on the same case.
    fn run_mutant<const K: u32>(c: Case) -> Outcome {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut host = Host { log: log.clone(), case: c, flag: 0, anchor_leg: 0, anchor_state: 0 };
        let mut m = Mut::<K>(Scen {
            aux_id: c.aux,
            state: c.state,
            active: c.active,
            pending_guard: c.pending,
            status: c.status,
            payload: c.payload,
            sub_task: c.task_sub,
            wait_timer: f32::from_bits(c.wait),
            ..Scen::default()
        });
        let ret = m.tick(&mut host, PED_TOK);
        let s = m.0;
        let calls = std::mem::take(&mut *log.borrow_mut());
        let effects = vec![
            Arg::Val(s.status),
            Arg::Val(s.wait_timer.to_bits()),
            Arg::Val(s.state),
            tok(s.payload),
            Arg::Val(s.point[0]),
            Arg::Val(s.point[1]),
            Arg::Val(s.point[2]),
            Arg::Val(s.point[3]),
            Arg::Val(host.flag),
            tok(host.anchor_leg),
            Arg::Val(host.anchor_state),
        ];
        Outcome { ret: u64::from(ret), calls, effects }
    }

    fn caught_by<const K: u32>() -> usize {
        cases().into_iter().filter(|c| run_32(*c) != run_mutant::<K>(*c)).count()
    }

    #[test]
    fn tick_wrong_versions_are_caught() {
        let _guard = serial();
        let _ = globals();
        let counts = [
            caught_by::<1>(),
            caught_by::<2>(),
            caught_by::<3>(),
            caught_by::<4>(),
            caught_by::<5>(),
            caught_by::<6>(),
            caught_by::<7>(),
            caught_by::<8>(),
            caught_by::<9>(),
            caught_by::<10>(),
        ];
        for (i, n) in counts.iter().enumerate() {
            eprintln!("tick wrong version {}: caught in {} of 600 trials", i + 1, n);
        }
        for (i, n) in counts.iter().enumerate() {
            assert!(*n > 0, "wrong tick version {} was not caught", i + 1);
        }
    }

    fn cases() -> Vec<Case> {
        let mut seed = 0x5EED_0007_u64;
        (0..600).map(|_| gen_case(&mut seed)).collect()
    }

    #[test]
    fn tick_matches_rewrite() {
        let _guard = serial();
        let _ = globals();
        let (mut completed_commit, mut heading_paths, mut chat, mut state_one) = (0, 0, 0, 0);
        for c in cases() {
            let want = run_32(c);
            let got = run_lifted(c);
            if want.calls.iter().any(|x| x.name == "commit_route") { completed_commit += 1; }
            if want.calls.iter().any(|x| x.name == "heading") { heading_paths += 1; }
            if want.calls.iter().any(|x| x.name == "chat_ping") { chat += 1; }
            if c.state == 1 { state_one += 1; }
            assert_eq!(got, want, "tick case {c:?}");
        }
        eprintln!(
            "tick: 600 trials; {completed_commit} reached the state-zero commit, {heading_paths} reached the heading, {chat} pinged, {state_one} started in state one"
        );
        assert!(completed_commit > 0 && heading_paths > 0 && chat > 0);
    }
}
