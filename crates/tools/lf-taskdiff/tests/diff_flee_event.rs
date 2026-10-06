//! Differential case: the flee event handler against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the untouched task/ped/entity/table blobs,
//! the built blobs' marks words, the refreshed clock, and every helper
//! call against the lift's trait calls, both sides given the same
//! scripted answers. The ped-type table is planted with decoy entries
//! so a wrong slot read shows; the shared random and allocator slots
//! are pinned by path; constants and tables are pinned; position words
//! are snapped through the scratch pointers. A deliberately wrong lift
//! must be caught at least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{
        EventChild, FleeAnswer, FleeEntity, FleeEvent, FleePed, FleeProbe, FleeTarget, FleeTask,
        ReactPed, Reaction, ReactionTask, TaskMgr, UninitTask,
    };
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::{
        clock, set_callee, set_clock, set_flee_rate, set_manager, set_seed_arg, set_table_base,
        set_tick,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        B_MARKSW, B_MARKSW_B, E_KINDW, E_STATUS_BYTE, FL_FLAG_BYTE, FL_KIND, FL_MODE, FL_POS,
        FL_STATE_BYTE, P_AUX_BYTE, P_FLAGS_BYTE, P_PROBEW, P_SEED_OFF, P_TARGETW, P_TYPE_OFF, Rng,
        TABLE_LEN, TABLE_NEG, addr, built_b_blob, built_blob, entity_blob, entry_blob,
        event_ped_blob, flee_blob, set_blob_byte, set_blob_u16,
    };

    /// The seed table word the proof pins.
    const EVENT_SEED_TABLE: u32 = 0x00EEF5FC;
    /// The spawn table word the proof pins.
    const SPAWN_TABLE: u32 = 0x00EEF608;
    /// The seed call's constant one word (as float bits).
    const SEED_ONE: u32 = 0x3F80_0000;
    /// Task B's constant span word.
    const TASK_B_SPAN: u32 = 0x4479_C000;
    /// The clock refresh step past the base.
    const CLOCK_STEP: u32 = 0x4E20;
    /// The entity kind mask and the wanted kind bits.
    const KIND_MASK: u32 = 0x3C0;
    const WANT_KIND: u32 = 0xC0;
    /// The mark bit and the flag bit set on built tasks.
    const SET_BIT: u32 = 8;
    const FLAG_BIT: u32 = 0x10;
    /// The mask clearing the wide-range bit for low kinds.
    const WIDE_MASK: u32 = 0xFFFB_FFFF;
    /// The seed-path kind bounds (compared signed).
    const SEED_LO: i32 = 0x15;
    const SEED_HI: i32 = 0x1B;
    /// The spawn-path kind set.
    const SPAWN_FIRST: u32 = 0x1B;
    const SPAWN_LAST: u32 = 0x1D;

    // Random stubs (slots 1 and 4) sharing one scripted answer: at most
    // one fires per run (the seed range and the spawn set are disjoint),
    // so the proof pins the slot by the taken path.
    static RAND1_ANS: AtomicU32 = AtomicU32::new(0);
    static RAND1_COUNT: AtomicU32 = AtomicU32::new(0);
    static RAND2_ANS: AtomicU32 = AtomicU32::new(0);
    static RAND2_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "cdecl" fn rand1_stub() -> u32 {
        RAND1_COUNT.fetch_add(1, Ordering::SeqCst);
        RAND1_ANS.load(Ordering::SeqCst)
    }

    extern "cdecl" fn rand2_stub() -> u32 {
        RAND2_COUNT.fetch_add(1, Ordering::SeqCst);
        RAND2_ANS.load(Ordering::SeqCst)
    }

    // Seed stub (slot 2): object plus ten words, all recorded.
    static SEED_ARGS: [AtomicU32; 11] = [
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
    ];
    static SEED_COUNT: AtomicU32 = AtomicU32::new(0);

    #[allow(clippy::too_many_arguments)]
    extern "thiscall" fn seed_stub(
        obj: u32,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
        g: u32,
        h: u32,
        i: u32,
        j: u32,
    ) -> u32 {
        SEED_ARGS[0].store(obj, Ordering::SeqCst);
        SEED_ARGS[1].store(a, Ordering::SeqCst);
        SEED_ARGS[2].store(b, Ordering::SeqCst);
        SEED_ARGS[3].store(c, Ordering::SeqCst);
        SEED_ARGS[4].store(d, Ordering::SeqCst);
        SEED_ARGS[5].store(e, Ordering::SeqCst);
        SEED_ARGS[6].store(f, Ordering::SeqCst);
        SEED_ARGS[7].store(g, Ordering::SeqCst);
        SEED_ARGS[8].store(h, Ordering::SeqCst);
        SEED_ARGS[9].store(i, Ordering::SeqCst);
        SEED_ARGS[10].store(j, Ordering::SeqCst);
        SEED_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    // Check stub (slot 3).
    static CHECK_PROBE: AtomicU32 = AtomicU32::new(0);
    static CHECK_ENTITY: AtomicU32 = AtomicU32::new(0);
    static CHECK_ANS: AtomicU32 = AtomicU32::new(0);
    static CHECK_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn check_stub(probe: u32, entity: u32) -> u32 {
        CHECK_PROBE.store(probe, Ordering::SeqCst);
        CHECK_ENTITY.store(entity, Ordering::SeqCst);
        CHECK_COUNT.fetch_add(1, Ordering::SeqCst);
        CHECK_ANS.load(Ordering::SeqCst)
    }

    // Allocator stubs (slots 5, 7, 9, 11): at most one fires per run,
    // so the proof pins the slot by the taken path.
    static MGR1_MGR: AtomicU32 = AtomicU32::new(0);
    static MGR1_ANS: AtomicU32 = AtomicU32::new(0);
    static MGR1_COUNT: AtomicU32 = AtomicU32::new(0);
    static MGR2_MGR: AtomicU32 = AtomicU32::new(0);
    static MGR2_ANS: AtomicU32 = AtomicU32::new(0);
    static MGR2_COUNT: AtomicU32 = AtomicU32::new(0);
    static MGR3_MGR: AtomicU32 = AtomicU32::new(0);
    static MGR3_ANS: AtomicU32 = AtomicU32::new(0);
    static MGR3_COUNT: AtomicU32 = AtomicU32::new(0);
    static MGR4_MGR: AtomicU32 = AtomicU32::new(0);
    static MGR4_ANS: AtomicU32 = AtomicU32::new(0);
    static MGR4_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn mgr1_stub(mgr: u32) -> u32 {
        MGR1_MGR.store(mgr, Ordering::SeqCst);
        MGR1_COUNT.fetch_add(1, Ordering::SeqCst);
        MGR1_ANS.load(Ordering::SeqCst)
    }

    extern "thiscall" fn mgr2_stub(mgr: u32) -> u32 {
        MGR2_MGR.store(mgr, Ordering::SeqCst);
        MGR2_COUNT.fetch_add(1, Ordering::SeqCst);
        MGR2_ANS.load(Ordering::SeqCst)
    }

    extern "thiscall" fn mgr3_stub(mgr: u32) -> u32 {
        MGR3_MGR.store(mgr, Ordering::SeqCst);
        MGR3_COUNT.fetch_add(1, Ordering::SeqCst);
        MGR3_ANS.load(Ordering::SeqCst)
    }

    extern "thiscall" fn mgr4_stub(mgr: u32) -> u32 {
        MGR4_MGR.store(mgr, Ordering::SeqCst);
        MGR4_COUNT.fetch_add(1, Ordering::SeqCst);
        MGR4_ANS.load(Ordering::SeqCst)
    }

    // Spawn stub (slot 6).
    static SPAWN_ARGS: [AtomicU32; 4] = [
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
    ];
    static SPAWN_ANS: AtomicU32 = AtomicU32::new(0);
    static SPAWN_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn spawn_stub(block: u32, table: u32, one: u32, entity: u32) -> u32 {
        SPAWN_ARGS[0].store(block, Ordering::SeqCst);
        SPAWN_ARGS[1].store(table, Ordering::SeqCst);
        SPAWN_ARGS[2].store(one, Ordering::SeqCst);
        SPAWN_ARGS[3].store(entity, Ordering::SeqCst);
        SPAWN_COUNT.fetch_add(1, Ordering::SeqCst);
        SPAWN_ANS.load(Ordering::SeqCst)
    }

    // Task-A stub (slot 8).
    static BUILD_A_ARGS: [AtomicU32; 3] = [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];
    static BUILD_A_ANS: AtomicU32 = AtomicU32::new(0);
    static BUILD_A_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn build_a_stub(block: u32, entity: u32, zero: u32) -> u32 {
        BUILD_A_ARGS[0].store(block, Ordering::SeqCst);
        BUILD_A_ARGS[1].store(entity, Ordering::SeqCst);
        BUILD_A_ARGS[2].store(zero, Ordering::SeqCst);
        BUILD_A_COUNT.fetch_add(1, Ordering::SeqCst);
        BUILD_A_ANS.load(Ordering::SeqCst)
    }

    // Task-B stub (slot 10): snaps the three position words plus the
    // trailing zero through the scratch pointer.
    static BUILD_B_ARGS: [AtomicU32; 6] = [
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
    ];
    static BUILD_B_WORDS: [AtomicU32; 4] = [
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
    ];
    static BUILD_B_ANS: AtomicU32 = AtomicU32::new(0);
    static BUILD_B_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn build_b_stub(
        block: u32,
        buf: u32,
        z0: u32,
        span: u32,
        rate: u32,
        z1: u32,
    ) -> u32 {
        BUILD_B_ARGS[0].store(block, Ordering::SeqCst);
        BUILD_B_ARGS[1].store(buf, Ordering::SeqCst);
        BUILD_B_ARGS[2].store(z0, Ordering::SeqCst);
        BUILD_B_ARGS[3].store(span, Ordering::SeqCst);
        BUILD_B_ARGS[4].store(rate, Ordering::SeqCst);
        BUILD_B_ARGS[5].store(z1, Ordering::SeqCst);
        unsafe {
            for (i, cell) in BUILD_B_WORDS.iter().enumerate() {
                cell.store(
                    ((buf + i as u32 * 4) as *const u32).read(),
                    Ordering::SeqCst,
                );
            }
        }
        BUILD_B_COUNT.fetch_add(1, Ordering::SeqCst);
        BUILD_B_ANS.load(Ordering::SeqCst)
    }

    // Fallback stub (slot 12): snaps the same scratch shape.
    static BUILD_C_ARGS: [AtomicU32; 3] = [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];
    static BUILD_C_WORDS: [AtomicU32; 4] = [
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
    ];
    static BUILD_C_ANS: AtomicU32 = AtomicU32::new(0);
    static BUILD_C_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn build_c_stub(block: u32, target: u32, buf: u32) -> u32 {
        BUILD_C_ARGS[0].store(block, Ordering::SeqCst);
        BUILD_C_ARGS[1].store(target, Ordering::SeqCst);
        BUILD_C_ARGS[2].store(buf, Ordering::SeqCst);
        unsafe {
            for (i, cell) in BUILD_C_WORDS.iter().enumerate() {
                cell.store(
                    ((buf + i as u32 * 4) as *const u32).read(),
                    Ordering::SeqCst,
                );
            }
        }
        BUILD_C_COUNT.fetch_add(1, Ordering::SeqCst);
        BUILD_C_ANS.load(Ordering::SeqCst)
    }

    fn register_all() {
        let f1: extern "cdecl" fn() -> u32 = rand1_stub;
        set_callee(1, f1 as usize as u32);
        let fs: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            seed_stub;
        set_callee(2, fs as usize as u32);
        let fc: extern "thiscall" fn(u32, u32) -> u32 = check_stub;
        set_callee(3, fc as usize as u32);
        let f4: extern "cdecl" fn() -> u32 = rand2_stub;
        set_callee(4, f4 as usize as u32);
        let m1: extern "thiscall" fn(u32) -> u32 = mgr1_stub;
        set_callee(5, m1 as usize as u32);
        let sp: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = spawn_stub;
        set_callee(6, sp as usize as u32);
        let m2: extern "thiscall" fn(u32) -> u32 = mgr2_stub;
        set_callee(7, m2 as usize as u32);
        let ba: extern "thiscall" fn(u32, u32, u32) -> u32 = build_a_stub;
        set_callee(8, ba as usize as u32);
        let m3: extern "thiscall" fn(u32) -> u32 = mgr3_stub;
        set_callee(9, m3 as usize as u32);
        let bb: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 = build_b_stub;
        set_callee(10, bb as usize as u32);
        let m4: extern "thiscall" fn(u32) -> u32 = mgr4_stub;
        set_callee(11, m4 as usize as u32);
        let bc: extern "thiscall" fn(u32, u32, u32) -> u32 = build_c_stub;
        set_callee(12, bc as usize as u32);
    }

    struct Fake {
        rand_answer: u32,
        kind_answer: u32,
        check_answer: u32,
        alloc_answer: u32,
        spawn_answer: u32,
        build_a_answer: Option<Reaction>,
        build_b_answer: Option<Reaction>,
        build_c_answer: u32,
        rands: u32,
        seeds: Vec<(u32, u32)>,
        kinds: Vec<u32>,
        checks: Vec<(u32, u32)>,
        allocs: Vec<u32>,
        spawns: Vec<(u32, u32)>,
        builds_a: Vec<(u32, u32)>,
        builds_b: Vec<(u32, [u32; 3], u32)>,
        builds_c: Vec<(u32, u32, [u32; 3])>,
    }

    impl FleeEvent for Fake {
        fn rand_word(&mut self) -> u32 {
            self.rands += 1;
            self.rand_answer
        }

        fn seed(&mut self, ped: Handle32<FleePed>, seed_arg: u32) {
            self.seeds.push((ped.get(), seed_arg));
        }

        fn entity_kind(&mut self, entity: Handle32<FleeEntity>) -> u32 {
            self.kinds.push(entity.get());
            self.kind_answer
        }

        fn check(
            &mut self,
            probe: Option<Handle32<FleeProbe>>,
            entity: Handle32<FleeEntity>,
        ) -> u32 {
            self.checks
                .push((Handle32::raw_or_zero(probe), entity.get()));
            self.check_answer
        }

        fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
            self.allocs.push(Handle32::raw_or_zero(manager));
            Handle32::new(self.alloc_answer)
        }

        fn spawn(
            &mut self,
            block: Handle32<UninitTask>,
            entity: Handle32<FleeEntity>,
        ) -> Option<Handle32<EventChild>> {
            self.spawns.push((block.get(), entity.get()));
            Handle32::new(self.spawn_answer)
        }

        fn build_a(
            &mut self,
            block: Handle32<UninitTask>,
            entity: Handle32<FleeEntity>,
        ) -> Option<Reaction> {
            self.builds_a.push((block.get(), entity.get()));
            self.build_a_answer
        }

        fn build_b(
            &mut self,
            block: Handle32<UninitTask>,
            pos: [f32; 3],
            rate: u32,
        ) -> Option<Reaction> {
            self.builds_b.push((
                block.get(),
                [pos[0].to_bits(), pos[1].to_bits(), pos[2].to_bits()],
                rate,
            ));
            self.build_b_answer
        }

        fn build_c(
            &mut self,
            block: Handle32<UninitTask>,
            target: Handle32<FleeTarget>,
            pos: [f32; 3],
        ) -> Option<Handle32<EventChild>> {
            self.builds_c.push((
                block.get(),
                target.get(),
                [pos[0].to_bits(), pos[1].to_bits(), pos[2].to_bits()],
            ));
            Handle32::new(self.build_c_answer)
        }
    }

    /// One differential case's inputs.
    struct Case {
        flag: u8,
        mode_live: bool,
        pos: [u32; 3],
        kind: u32,
        state_byte: u8,
        type_id: i16,
        status: u8,
        probe_word: u32,
        flags: u8,
        aux: u8,
        target_live: bool,
        kind_word: u32,
        check_ans: u32,
        rand: u32,
        seed_arg: u32,
        clock: u32,
        clock_base: u32,
        rate: u32,
        manager: u32,
        o1: u32,
        o4: u32,
        spawn_ans: u32,
        c_ans: u32,
        marks_a: u32,
        marks_b: u32,
        /// The predicted observable-calls outcome, when predictable.
        predict_calls: Option<bool>,
    }

    /// The flag-bits computation, restated to pin the built marks.
    fn expect_flag_bits(marks: u32, state: u8) -> u32 {
        let set = marks | SET_BIT;
        (set & !FLAG_BIT) | ((u32::from(state) & 1) << 4)
    }

    /// Runs one case on both sides; returns whether the wrong lift was caught.
    fn run_case(case: &Case, rng: &mut Rng) -> bool {
        let fresh = case.status < 3;
        let kind_i = case.kind as i32;
        // The task blob.
        let mut blob = *flee_blob();
        for w in blob.iter_mut() {
            *w = rng.u32();
        }
        blob[FL_KIND] = case.kind;
        blob[FL_POS] = case.pos[0];
        blob[FL_POS + 1] = case.pos[1];
        blob[FL_POS + 2] = case.pos[2];
        set_blob_byte(&mut blob, FL_FLAG_BYTE, case.flag);
        set_blob_byte(&mut blob, FL_STATE_BYTE, case.state_byte);
        let mut entity = entity_blob();
        for w in entity.iter_mut() {
            *w = rng.u32();
        }
        entity[E_KINDW] = case.kind_word;
        let entity_boxed = entity;
        let entity_before = *entity_boxed;
        let entity_addr = addr(&entity_boxed[0]);
        blob[FL_MODE] = if case.mode_live { entity_addr } else { 0 };
        // The ped blob.
        let mut ped = event_ped_blob();
        for w in ped.iter_mut() {
            *w = rng.u32();
        }
        set_blob_u16(&mut ped[..], P_TYPE_OFF, case.type_id as u16);
        ped[P_PROBEW] = case.probe_word;
        set_blob_byte(&mut ped[..], P_FLAGS_BYTE, case.flags);
        set_blob_byte(&mut ped[..], P_AUX_BYTE, case.aux);
        let mut target = *entity_blob();
        for w in target.iter_mut() {
            *w = rng.u32();
        }
        let target_boxed = Box::new(target);
        let target_addr = addr(&target_boxed[0]);
        ped[P_TARGETW] = if case.target_live { target_addr } else { 0 };
        let ped_boxed = ped;
        let ped_before = *ped_boxed;
        let ped_addr = addr(&ped_boxed[0]);
        let boxed = Box::new(blob);
        let before = *boxed;
        // The type table: decoys everywhere, the live entry at the slot.
        let mut live_entry = entry_blob();
        for w in live_entry.iter_mut() {
            *w = rng.u32();
        }
        set_blob_byte(&mut live_entry[..], E_STATUS_BYTE, case.status);
        let live_boxed = live_entry;
        let live_addr = addr(&live_boxed[0]);
        let mut decoy_entry = entry_blob();
        for w in decoy_entry.iter_mut() {
            *w = rng.u32();
        }
        set_blob_byte(
            &mut decoy_entry[..],
            E_STATUS_BYTE,
            if fresh { 9 } else { 0 },
        );
        let decoy_boxed = decoy_entry;
        let decoy_addr = addr(&decoy_boxed[0]);
        let mut table = vec![decoy_addr; TABLE_LEN];
        let slot = (TABLE_NEG as i32 + i32::from(case.type_id)) as usize;
        table[slot] = live_addr;
        // The built blobs the build stubs answer.
        let mut built_a = built_blob();
        for w in built_a.iter_mut() {
            *w = rng.u32();
        }
        built_a[B_MARKSW] = case.marks_a;
        let built_a_before = *built_a;
        let built_a_boxed = built_a;
        let built_a_addr = addr(&built_a_boxed[0]);
        let mut built_b = built_b_blob();
        for w in built_b.iter_mut() {
            *w = rng.u32();
        }
        built_b[B_MARKSW_B] = case.marks_b;
        let built_b_before = *built_b;
        let built_b_boxed = built_b;
        let built_b_addr = addr(&built_b_boxed[0]);
        // Script the globals and the stubs.
        set_table_base(addr(&table[0]).wrapping_add(TABLE_NEG as u32 * 4));
        set_seed_arg(case.seed_arg);
        set_clock(case.clock);
        set_tick(case.clock_base);
        set_flee_rate(case.rate);
        set_manager(case.manager);
        RAND1_ANS.store(case.rand, Ordering::SeqCst);
        RAND2_ANS.store(case.rand, Ordering::SeqCst);
        CHECK_ANS.store(case.check_ans, Ordering::SeqCst);
        MGR1_ANS.store(case.o1, Ordering::SeqCst);
        MGR2_ANS.store(rng.u32() | 1, Ordering::SeqCst);
        let o2 = MGR2_ANS.load(Ordering::SeqCst);
        MGR3_ANS.store(rng.u32() | 1, Ordering::SeqCst);
        let o3 = MGR3_ANS.load(Ordering::SeqCst);
        MGR4_ANS.store(case.o4, Ordering::SeqCst);
        SPAWN_ANS.store(case.spawn_ans, Ordering::SeqCst);
        BUILD_A_ANS.store(built_a_addr, Ordering::SeqCst);
        BUILD_B_ANS.store(built_b_addr, Ordering::SeqCst);
        BUILD_C_ANS.store(case.c_ans, Ordering::SeqCst);
        for count in [
            &RAND1_COUNT,
            &RAND2_COUNT,
            &SEED_COUNT,
            &CHECK_COUNT,
            &MGR1_COUNT,
            &MGR2_COUNT,
            &MGR3_COUNT,
            &MGR4_COUNT,
            &SPAWN_COUNT,
            &BUILD_A_COUNT,
            &BUILD_B_COUNT,
            &BUILD_C_COUNT,
        ] {
            count.store(0, Ordering::SeqCst);
        }
        let got = unsafe { fn_00DA58E0::rw_00da58e0(addr(&boxed[0]), ped_addr) };
        let clock_after = clock();
        // The observed path, read off the stub counts.
        let c = |cell: &AtomicU32| cell.load(Ordering::SeqCst);
        let passed = c(&RAND1_COUNT)
            + c(&SEED_COUNT)
            + c(&CHECK_COUNT)
            + c(&RAND2_COUNT)
            + c(&MGR1_COUNT)
            + c(&MGR2_COUNT)
            + c(&MGR3_COUNT)
            + c(&MGR4_COUNT)
            > 0;
        if let Some(predict) = case.predict_calls {
            assert_eq!(passed, predict, "call prediction missed");
        }
        let in_seed_range = fresh && (SEED_LO..SEED_HI).contains(&kind_i);
        if passed {
            assert_eq!(c(&RAND1_COUNT), u32::from(in_seed_range));
        } else {
            assert_eq!(c(&RAND1_COUNT), 0);
        }
        if c(&SEED_COUNT) == 1 {
            assert!(fresh && kind_i >= SEED_LO);
            assert_eq!(
                SEED_ARGS[0].load(Ordering::SeqCst),
                ped_addr.wrapping_add(P_SEED_OFF)
            );
            assert_eq!(SEED_ARGS[1].load(Ordering::SeqCst), EVENT_SEED_TABLE);
            assert_eq!(SEED_ARGS[2].load(Ordering::SeqCst), 0);
            assert_eq!(SEED_ARGS[3].load(Ordering::SeqCst), 0);
            assert_eq!(SEED_ARGS[4].load(Ordering::SeqCst), case.seed_arg);
            assert_eq!(SEED_ARGS[5].load(Ordering::SeqCst), 0xFFFF_FFFF);
            assert_eq!(SEED_ARGS[6].load(Ordering::SeqCst), 0);
            assert_eq!(SEED_ARGS[7].load(Ordering::SeqCst), 0);
            assert_eq!(SEED_ARGS[8].load(Ordering::SeqCst), SEED_ONE);
            assert_eq!(SEED_ARGS[9].load(Ordering::SeqCst), 0);
            assert_eq!(SEED_ARGS[10].load(Ordering::SeqCst), 0);
        } else {
            assert_eq!(c(&SEED_COUNT), 0);
        }
        let want_check = passed && case.mode_live && case.kind_word & KIND_MASK == WANT_KIND;
        assert_eq!(c(&CHECK_COUNT), u32::from(want_check));
        if want_check {
            assert_eq!(CHECK_PROBE.load(Ordering::SeqCst), case.probe_word);
            assert_eq!(CHECK_ENTITY.load(Ordering::SeqCst), entity_addr);
        }
        let stage_two = want_check && case.check_ans & 0xFF == 0;
        let want_r2 = stage_two
            && (SPAWN_FIRST..=SPAWN_LAST).contains(&case.kind)
            && case.flags & 4 == 0
            && case.clock < case.clock_base
            && case.aux & 4 != 0;
        assert_eq!(c(&RAND2_COUNT), u32::from(want_r2));
        assert!(c(&MGR1_COUNT) <= 1);
        if c(&MGR1_COUNT) == 1 {
            assert!(want_r2);
            assert_eq!(MGR1_MGR.load(Ordering::SeqCst), case.manager);
        }
        let spawn_taken = c(&MGR1_COUNT) == 1;
        assert_eq!(
            clock_after,
            if spawn_taken {
                case.clock_base.wrapping_add(CLOCK_STEP)
            } else {
                case.clock
            }
        );
        let want_spawn = spawn_taken && case.o1 != 0;
        assert_eq!(c(&SPAWN_COUNT), u32::from(want_spawn));
        if want_spawn {
            assert_eq!(SPAWN_ARGS[0].load(Ordering::SeqCst), case.o1);
            assert_eq!(SPAWN_ARGS[1].load(Ordering::SeqCst), SPAWN_TABLE);
            assert_eq!(SPAWN_ARGS[2].load(Ordering::SeqCst), 1);
            assert_eq!(SPAWN_ARGS[3].load(Ordering::SeqCst), entity_addr);
        }
        let want_a = stage_two && !spawn_taken;
        assert_eq!(c(&MGR2_COUNT), u32::from(want_a));
        if want_a {
            assert_eq!(MGR2_MGR.load(Ordering::SeqCst), case.manager);
        }
        assert_eq!(c(&BUILD_A_COUNT), u32::from(want_a));
        if want_a {
            assert_eq!(BUILD_A_ARGS[0].load(Ordering::SeqCst), o2);
            assert_eq!(BUILD_A_ARGS[1].load(Ordering::SeqCst), entity_addr);
            assert_eq!(BUILD_A_ARGS[2].load(Ordering::SeqCst), 0);
        }
        let fallback = passed && !stage_two;
        let want_c_alloc = fallback && case.flags & 4 != 0 && case.target_live;
        assert_eq!(c(&MGR4_COUNT), u32::from(want_c_alloc));
        if want_c_alloc {
            assert_eq!(MGR4_MGR.load(Ordering::SeqCst), case.manager);
        }
        let want_c = want_c_alloc && case.o4 != 0;
        assert_eq!(c(&BUILD_C_COUNT), u32::from(want_c));
        if want_c {
            assert_eq!(BUILD_C_ARGS[0].load(Ordering::SeqCst), case.o4);
            assert_eq!(BUILD_C_ARGS[1].load(Ordering::SeqCst), target_addr);
            assert_eq!(BUILD_C_WORDS[0].load(Ordering::SeqCst), case.pos[0]);
            assert_eq!(BUILD_C_WORDS[1].load(Ordering::SeqCst), case.pos[1]);
            assert_eq!(BUILD_C_WORDS[2].load(Ordering::SeqCst), case.pos[2]);
            assert_eq!(BUILD_C_WORDS[3].load(Ordering::SeqCst), 0);
        }
        let want_b = fallback && case.flags & 4 == 0;
        assert_eq!(c(&MGR3_COUNT), u32::from(want_b));
        if want_b {
            assert_eq!(MGR3_MGR.load(Ordering::SeqCst), case.manager);
        }
        assert_eq!(c(&BUILD_B_COUNT), u32::from(want_b));
        if want_b {
            assert_eq!(BUILD_B_ARGS[0].load(Ordering::SeqCst), o3);
            assert_eq!(BUILD_B_ARGS[2].load(Ordering::SeqCst), 0);
            assert_eq!(BUILD_B_ARGS[3].load(Ordering::SeqCst), TASK_B_SPAN);
            assert_eq!(BUILD_B_ARGS[4].load(Ordering::SeqCst), case.rate);
            assert_eq!(BUILD_B_ARGS[5].load(Ordering::SeqCst), 0);
            assert_eq!(BUILD_B_WORDS[0].load(Ordering::SeqCst), case.pos[0]);
            assert_eq!(BUILD_B_WORDS[1].load(Ordering::SeqCst), case.pos[1]);
            assert_eq!(BUILD_B_WORDS[2].load(Ordering::SeqCst), case.pos[2]);
            assert_eq!(BUILD_B_WORDS[3].load(Ordering::SeqCst), 0);
        }
        let expect_got = if want_spawn {
            case.spawn_ans
        } else if spawn_taken {
            0
        } else if want_a {
            built_a_addr
        } else if want_c {
            case.c_ans
        } else if want_c_alloc {
            0
        } else if want_b {
            built_b_addr
        } else {
            0
        };
        assert_eq!(got, expect_got);
        // Blobs: the task, ped, entity and table survive; the built
        // blobs gain their flag bits.
        assert_eq!(*boxed, before);
        assert_eq!(*ped_boxed, ped_before);
        assert_eq!(*entity_boxed, entity_before);
        let mut expect_a = built_a_before;
        if want_a {
            let mut marks = expect_flag_bits(case.marks_a, case.state_byte);
            if kind_i < SEED_HI {
                marks &= WIDE_MASK;
            }
            expect_a[B_MARKSW] = marks;
        }
        assert_eq!(*built_a_boxed, expect_a);
        let mut expect_b = built_b_before;
        if want_b {
            expect_b[B_MARKSW_B] = expect_flag_bits(case.marks_b, case.state_byte);
        }
        assert_eq!(*built_b_boxed, expect_b);
        // The lift runs the same inputs through its trait calls.
        let pos = [
            f32::from_bits(case.pos[0]),
            f32::from_bits(case.pos[1]),
            f32::from_bits(case.pos[2]),
        ];
        let lift = FleeTask::new(
            None,
            0,
            case.kind,
            pos,
            case.flag != 0,
            Handle32::new(before[FL_MODE]),
            case.state_byte,
        );
        let ped_handle = Handle32::<FleePed>::new(ped_addr).expect("ped is live");
        let snap = ReactPed::new(
            ped_handle,
            Some(case.status),
            Handle32::new(case.probe_word),
            case.flags,
            case.aux,
            Handle32::new(ped_before[P_TARGETW]),
        );
        let mut fake = Fake {
            rand_answer: case.rand,
            kind_answer: case.kind_word,
            check_answer: case.check_ans,
            alloc_answer: if spawn_taken {
                case.o1
            } else if want_a {
                o2
            } else if want_c_alloc {
                case.o4
            } else if want_b {
                o3
            } else {
                0xDEAD
            },
            spawn_answer: case.spawn_ans,
            build_a_answer: Some(Reaction::new(
                Handle32::<ReactionTask>::new(built_a_addr).expect("built A is live"),
                case.marks_a,
            )),
            build_b_answer: Some(Reaction::new(
                Handle32::<ReactionTask>::new(built_b_addr).expect("built B is live"),
                case.marks_b,
            )),
            build_c_answer: case.c_ans,
            rands: 0,
            seeds: Vec::new(),
            kinds: Vec::new(),
            checks: Vec::new(),
            allocs: Vec::new(),
            spawns: Vec::new(),
            builds_a: Vec::new(),
            builds_b: Vec::new(),
            builds_c: Vec::new(),
        };
        let mut lift_clock = case.clock;
        let lift_ret = lift.handle_event(
            Some(snap),
            case.seed_arg,
            &mut lift_clock,
            case.clock_base,
            case.rate,
            Handle32::new(case.manager),
            &mut fake,
        );
        let (lift_raw, lift_built) = match lift_ret {
            None => (0, None),
            Some(FleeAnswer::Spawned(h)) => (h.get(), None),
            Some(FleeAnswer::Built(r)) => (r.task().get(), Some(r.marks())),
            Some(FleeAnswer::Routed(h)) => (h.get(), None),
        };
        assert_eq!(got, lift_raw);
        assert_eq!(lift_clock, clock_after);
        assert_eq!(fake.rands, c(&RAND1_COUNT) + c(&RAND2_COUNT));
        assert_eq!(fake.seeds.len() as u32, c(&SEED_COUNT));
        if c(&SEED_COUNT) == 1 {
            assert_eq!(fake.seeds[0], (ped_addr, case.seed_arg));
        }
        // The kind check is a silent memory read on the rewrite side
        // but a trait call on the lift side, so its count comes from
        // the inputs: the gate fires without the floats exactly when
        // the flag is set with a live mode (the float path only runs
        // with a null mode, which never reaches the check).
        let want_kinds = case.flag != 0 && case.mode_live;
        assert_eq!(fake.kinds.len() as u32, u32::from(want_kinds));
        if want_kinds {
            assert_eq!(fake.kinds[0], entity_addr);
        }
        assert_eq!(fake.checks.len() as u32, u32::from(want_check));
        if want_check {
            assert_eq!(fake.checks[0], (case.probe_word, entity_addr));
        }
        let want_alloc = u32::from(spawn_taken)
            + u32::from(want_a)
            + u32::from(want_c_alloc)
            + u32::from(want_b);
        assert_eq!(fake.allocs.len() as u32, want_alloc);
        if want_alloc == 1 {
            assert_eq!(fake.allocs[0], case.manager);
        }
        assert_eq!(fake.spawns.len() as u32, u32::from(want_spawn));
        if want_spawn {
            assert_eq!(fake.spawns[0], (case.o1, entity_addr));
        }
        assert_eq!(fake.builds_a.len() as u32, u32::from(want_a));
        if want_a {
            assert_eq!(fake.builds_a[0], (o2, entity_addr));
            assert_eq!(lift_built, Some(built_a_boxed[B_MARKSW]));
        }
        assert_eq!(fake.builds_b.len() as u32, u32::from(want_b));
        if want_b {
            assert_eq!(fake.builds_b[0].0, o3);
            assert_eq!(fake.builds_b[0].1, case.pos);
            assert_eq!(fake.builds_b[0].2, case.rate);
            assert_eq!(lift_built, Some(built_b_boxed[B_MARKSW_B]));
        }
        assert_eq!(fake.builds_c.len() as u32, u32::from(want_c));
        if want_c {
            assert_eq!(fake.builds_c[0].0, case.o4);
            assert_eq!(fake.builds_c[0].1, target_addr);
            assert_eq!(fake.builds_c[0].2, case.pos);
        }
        match lift_ret {
            Some(FleeAnswer::Spawned(_)) => {
                assert!(want_spawn || case.spawn_ans == 0 && spawn_taken && case.o1 != 0)
            }
            Some(FleeAnswer::Built(_)) => assert!(want_a || want_b),
            Some(FleeAnswer::Routed(_)) => assert!(want_c),
            None => assert!(
                !(want_spawn && case.spawn_ans != 0)
                    && !want_a
                    && !want_b
                    && !(want_c && case.c_ans != 0)
            ),
        }
        // The wrong lift runs the neighbouring kind. Its allocator
        // answers a live block: it may take a build path where the
        // observed path answered null, and the comparison below (not a
        // panic) is what catches it.
        let mut wfake = Fake {
            rand_answer: case.rand,
            kind_answer: case.kind_word,
            check_answer: case.check_ans,
            alloc_answer: o2,
            spawn_answer: case.spawn_ans,
            build_a_answer: fake.build_a_answer,
            build_b_answer: fake.build_b_answer,
            build_c_answer: case.c_ans,
            rands: 0,
            seeds: Vec::new(),
            kinds: Vec::new(),
            checks: Vec::new(),
            allocs: Vec::new(),
            spawns: Vec::new(),
            builds_a: Vec::new(),
            builds_b: Vec::new(),
            builds_c: Vec::new(),
        };
        let shifted = FleeTask::new(
            None,
            0,
            case.kind.wrapping_add(1),
            pos,
            case.flag != 0,
            Handle32::new(before[FL_MODE]),
            case.state_byte,
        );
        let mut wrong_clock = case.clock;
        let w_ret = shifted.handle_event(
            Some(snap),
            case.seed_arg,
            &mut wrong_clock,
            case.clock_base,
            case.rate,
            Handle32::new(case.manager),
            &mut wfake,
        );
        let w_raw = match w_ret {
            None => 0,
            Some(FleeAnswer::Spawned(h)) => h.get(),
            Some(FleeAnswer::Built(r)) => r.task().get(),
            Some(FleeAnswer::Routed(h)) => h.get(),
        };
        w_raw != got
            || wfake.rands != fake.rands
            || wfake.seeds.len() != fake.seeds.len()
            || wfake.allocs.len() != fake.allocs.len()
            || wrong_clock != clock_after
    }

    #[test]
    fn flee_event_matches() {
        register_all();
        let mut rng = Rng(0xE7E);
        let mut caught = 0;
        let mut cases = 0;
        // Phase A: the gate sweep with a spawn-full downstream, so a
        // passed gate always shows calls. Clear-cut shapes carry a
        // prediction; boundary shapes only require agreement.
        let shapes: [([u32; 3], Option<bool>); 8] = [
            ([0, 0, 0], Some(false)),
            ([0x4120_0000, 0, 0], Some(true)),
            ([f32::NAN.to_bits(), 0, 0], Some(false)),
            ([f32::INFINITY.to_bits(), 0, 0], Some(true)),
            ([0x3E4C_CCCD, 0x3DCC_CCCD, 0], None),
            ([0x3F80_0000, 0x3F80_0000, 0x3F80_0000], Some(true)),
            ([0xC0A0_0000, 0, 0], Some(true)),
            ([rng.u32(), rng.u32(), rng.u32()], None),
        ];
        for &(flag, mode_live, gate_pred) in &[
            (0u8, false, None),
            (0u8, true, Some(false)),
            (1u8, false, Some(false)),
            (1u8, true, Some(true)),
            (0xFFu8, true, Some(true)),
        ] {
            for (pos, float_pred) in &shapes {
                let predict_calls = match (gate_pred, float_pred) {
                    (Some(false), _) => Some(false),
                    (Some(true), _) => Some(true),
                    (None, p) => *p,
                };
                let case = Case {
                    flag,
                    mode_live,
                    pos: *pos,
                    kind: 0x1B,
                    state_byte: 1,
                    type_id: 7,
                    status: 0,
                    probe_word: 0xBE66,
                    flags: 0,
                    aux: 4,
                    target_live: false,
                    kind_word: 0xC0,
                    check_ans: 0,
                    rand: 0,
                    seed_arg: 0xA11CE,
                    clock: 100,
                    clock_base: 200,
                    rate: 0x3F80_0000,
                    manager: 0x71,
                    o1: 0x9000,
                    o4: 0,
                    spawn_ans: 0xA000,
                    c_ans: 0,
                    marks_a: 0,
                    marks_b: 0,
                    predict_calls,
                };
                if run_case(&case, &mut rng) {
                    caught += 1;
                }
                cases += 1;
            }
        }
        // Phase B: the downstream sweep with a passed gate.
        let statuses = [0u8, 2, 3, 4, 0xFF, rng.u32() as u8];
        let kinds = [
            0u32,
            0x14,
            0x15,
            0x1A,
            0x1B,
            0x1C,
            0x1D,
            0x1E,
            0x7FFF_FFFF,
            0x8000_0000,
            0xFFFF_FFFE,
            0xFFFF_FFFF,
            rng.u32(),
        ];
        let rands = [0u32, 0x4000_0000, 0xFFFF_FFFF, 10813, 16384, rng.u32()];
        let kind_words = [0xC0u32, 0, 0x3C0, rng.u32()];
        let checks = [0u32, 1, 0x100];
        let flagsets = [0u8, 4, 0xFB, 0xFF];
        let auxsets = [0u8, 4, 0xFF];
        let clocks = [
            (100u32, 200u32),
            (200, 200),
            (300, 200),
            (0, 0),
            (0xFFFF_FFFF, 0),
            (0, 1),
        ];
        let tids = [-32768i16, -1, 0, 1, 21, 32767, rng.u32() as i16];
        let states = [0u8, 1, 0xFE, 0xFF];
        let mut inputs = Vec::new();
        for _ in 0..600 {
            inputs.push((
                statuses[(rng.next() % statuses.len() as u64) as usize],
                kinds[(rng.next() % kinds.len() as u64) as usize],
                rands[(rng.next() % rands.len() as u64) as usize],
                kind_words[(rng.next() % kind_words.len() as u64) as usize],
                checks[(rng.next() % checks.len() as u64) as usize],
                flagsets[(rng.next() % flagsets.len() as u64) as usize],
                auxsets[(rng.next() % auxsets.len() as u64) as usize],
                clocks[(rng.next() % clocks.len() as u64) as usize],
                tids[(rng.next() % tids.len() as u64) as usize],
                states[(rng.next() % states.len() as u64) as usize],
            ));
        }
        // Targeted corners: the spawn set, the seed bounds, the status
        // bound, null allocs on the safe paths, null answers.
        for &kind in &[0x1Bu32, 0x1C, 0x1D] {
            for &rand in &[0u32, 0x4000_0000] {
                inputs.push((0, kind, rand, 0xC0, 0, 0, 4, (100, 200), 7, 1));
            }
        }
        for &kind in &[0x1Au32, 0x1B, 0x8000_0000, 0xFFFF_FFFF] {
            inputs.push((0, kind, 0, 0xC0, 0, 0xFF, 0, (300, 200), 7, 0xFF));
        }
        for &status in &[2u8, 3] {
            for &kind in &[0x15u32, 0x1B] {
                inputs.push((status, kind, 0, 0, 1, 0, 0, (0, 0), -5, 0));
            }
        }
        for (status, kind, rand, kind_word, check_ans, flags, aux, (clock, base), tid, state) in
            inputs
        {
            let target_live = rng.next() & 1 == 0;
            // The gate always passes here; observable calls are exactly
            // predictable (the two random rolls are covered by the
            // draws that precede them).
            let fresh_b = status < 3;
            let kind_bi = kind as i32;
            let in_seed_range_b = fresh_b && (SEED_LO..SEED_HI).contains(&kind_bi);
            let uncond_seed_b = fresh_b && kind_bi >= SEED_HI;
            let check_fires_b = kind_word & KIND_MASK == WANT_KIND;
            let stage_two_b = check_fires_b && check_ans & 0xFF == 0;
            let r2_fires_b = stage_two_b
                && (SPAWN_FIRST..=SPAWN_LAST).contains(&kind)
                && flags & 4 == 0
                && clock < base
                && aux & 4 != 0;
            let fallback_b = !stage_two_b;
            let target_alloc_b = fallback_b && flags & 4 != 0 && target_live;
            let b_alloc_b = fallback_b && flags & 4 == 0;
            let expect_calls_b = in_seed_range_b
                || uncond_seed_b
                || check_fires_b
                || r2_fires_b
                || target_alloc_b
                || b_alloc_b;
            let case = Case {
                flag: 1,
                mode_live: true,
                pos: [rng.u32(), rng.u32(), rng.u32()],
                kind,
                state_byte: state,
                type_id: tid,
                status,
                probe_word: rng.u32(),
                flags,
                aux,
                target_live,
                kind_word,
                check_ans,
                rand,
                seed_arg: rng.u32(),
                clock,
                clock_base: base,
                rate: rng.u32(),
                manager: rng.u32(),
                o1: if rng.next() & 3 == 0 {
                    0
                } else {
                    rng.u32() | 1
                },
                o4: if rng.next() & 3 == 0 {
                    0
                } else {
                    rng.u32() | 1
                },
                spawn_ans: if rng.next() & 3 == 0 {
                    0
                } else {
                    rng.u32() | 1
                },
                c_ans: if rng.next() & 3 == 0 {
                    0
                } else {
                    rng.u32() | 1
                },
                marks_a: rng.u32(),
                marks_b: rng.u32(),
                predict_calls: Some(expect_calls_b),
            };
            if run_case(&case, &mut rng) {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong flee event never caught ({cases} cases)");
    }
}
