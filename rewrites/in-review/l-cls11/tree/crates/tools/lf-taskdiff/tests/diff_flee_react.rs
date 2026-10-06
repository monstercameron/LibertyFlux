//! Differential case: the flee reaction update against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the untouched source/entity/ped/subtask
//! blobs, the built blob's marks word, and every helper call against
//! the lift's trait calls, both sides given the same scripted answers.
//! The probe's ped-relative word, the build's trailing zero and the
//! dispatch task address are pinned. A deliberately wrong lift must be
//! caught at least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{
        FleeEntity, FleePed, FleeReact, FleeTask, Reaction, SubTask, TaskMgr, UninitTask,
    };
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::{set_callee, set_manager};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        B_MARKSW, E_KINDW, FL_FLAG_BYTE, FL_KIND, FL_MODE, FL_POS, FL_STATE_BYTE, FL_SUB, P_PROBEW,
        Rng, addr, big_ped_blob, blob_byte, built_blob, entity_blob, fake_table, flee_blob,
        sub_blob,
    };

    /// The subtask type the update builds for.
    const WANT_TYPE: u32 = 0x16E;
    /// The entity kind mask and the wanted kind bits.
    const KIND_MASK: u32 = 0x3C0;
    const WANT_KIND: u32 = 0xC0;
    /// The mark bit set on the built reaction.
    const MARK_BIT: u32 = 8;
    /// The mask clearing the wide-range bit for low kinds.
    const WIDE_MASK: u32 = 0xFFFB_FFFF;
    /// Kinds below this clear the wide-range bit.
    const FLAG_LIMIT: u32 = 0x1B;

    // Subtask type stub (planted in the fake table at word 3) and its recording.
    static TYPE_SUB: AtomicU32 = AtomicU32::new(0);
    static TYPE_ANS: AtomicU32 = AtomicU32::new(0);
    static TYPE_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn type_stub(sub: u32) -> u32 {
        TYPE_SUB.store(sub, Ordering::SeqCst);
        TYPE_COUNT.fetch_add(1, Ordering::SeqCst);
        TYPE_ANS.load(Ordering::SeqCst)
    }

    // Probe stub (slot 2) and its recording.
    static PROBE_OBJ: AtomicU32 = AtomicU32::new(0);
    static PROBE_ENTITY: AtomicU32 = AtomicU32::new(0);
    static PROBE_ANS: AtomicU32 = AtomicU32::new(0);
    static PROBE_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn probe_stub(obj: u32, entity: u32) -> u32 {
        PROBE_OBJ.store(obj, Ordering::SeqCst);
        PROBE_ENTITY.store(entity, Ordering::SeqCst);
        PROBE_COUNT.fetch_add(1, Ordering::SeqCst);
        PROBE_ANS.load(Ordering::SeqCst)
    }

    // Alloc stub (slot 3) and its recording.
    static ALLOC_MGR: AtomicU32 = AtomicU32::new(0);
    static ALLOC_ANS: AtomicU32 = AtomicU32::new(0);
    static ALLOC_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn alloc_stub(mgr: u32) -> u32 {
        ALLOC_MGR.store(mgr, Ordering::SeqCst);
        ALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
        ALLOC_ANS.load(Ordering::SeqCst)
    }

    // Build stub (slot 4) and its recording.
    static BUILD_BLOCK: AtomicU32 = AtomicU32::new(0);
    static BUILD_ENTITY: AtomicU32 = AtomicU32::new(0);
    static BUILD_ZERO: AtomicU32 = AtomicU32::new(0);
    static BUILD_ANS: AtomicU32 = AtomicU32::new(0);
    static BUILD_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn build_stub(block: u32, entity: u32, zero: u32) -> u32 {
        BUILD_BLOCK.store(block, Ordering::SeqCst);
        BUILD_ENTITY.store(entity, Ordering::SeqCst);
        BUILD_ZERO.store(zero, Ordering::SeqCst);
        BUILD_COUNT.fetch_add(1, Ordering::SeqCst);
        BUILD_ANS.load(Ordering::SeqCst)
    }

    // Dispatch stub (slot 5) and its recording.
    static DISPATCH_THIS: AtomicU32 = AtomicU32::new(0);
    static DISPATCH_ARG: AtomicU32 = AtomicU32::new(0);
    static DISPATCH_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn dispatch_stub(this: u32, arg: u32) -> u32 {
        DISPATCH_THIS.store(this, Ordering::SeqCst);
        DISPATCH_ARG.store(arg, Ordering::SeqCst);
        DISPATCH_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    fn type_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = type_stub;
        f as usize as u32
    }

    fn probe_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = probe_stub;
        f as usize as u32
    }

    fn alloc_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = alloc_stub;
        f as usize as u32
    }

    fn build_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 = build_stub;
        f as usize as u32
    }

    fn dispatch_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = dispatch_stub;
        f as usize as u32
    }

    struct Fake {
        type_answer: u32,
        kind_answer: u32,
        probe_answer: u32,
        alloc_answer: u32,
        build_answer: Option<Reaction>,
        types: Vec<u32>,
        kinds: Vec<u32>,
        probes: Vec<(u32, u32)>,
        allocs: Vec<u32>,
        builds: Vec<(u32, u32)>,
        dispatches: Vec<u32>,
    }

    impl FleeReact for Fake {
        fn subtask_type(&mut self, sub: Handle32<SubTask>) -> u32 {
            self.types.push(sub.get());
            self.type_answer
        }

        fn entity_kind(&mut self, entity: Handle32<FleeEntity>) -> u32 {
            self.kinds.push(entity.get());
            self.kind_answer
        }

        fn probe(
            &mut self,
            ped: Option<Handle32<FleePed>>,
            entity: Handle32<FleeEntity>,
        ) -> u32 {
            self.probes.push((Handle32::raw_or_zero(ped), entity.get()));
            self.probe_answer
        }

        fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
            self.allocs.push(Handle32::raw_or_zero(manager));
            Handle32::new(self.alloc_answer)
        }

        fn build(
            &mut self,
            block: Handle32<UninitTask>,
            entity: Handle32<FleeEntity>,
        ) -> Option<Reaction> {
            self.builds.push((block.get(), entity.get()));
            self.build_answer
        }

        fn dispatch(&mut self, ped: Option<Handle32<FleePed>>) {
            self.dispatches.push(Handle32::raw_or_zero(ped));
        }
    }

    /// A reaction update with the flag limit off by one.
    fn wrong_react(
        task: &FleeTask,
        ped: Option<Handle32<FleePed>>,
        manager: Option<Handle32<TaskMgr>>,
        react: &mut Fake,
    ) -> Option<Reaction> {
        let sub = task.subtask().expect("wrong lift keeps a subtask");
        if react.subtask_type(sub) != WANT_TYPE {
            react.dispatch(ped);
            return None;
        }
        let Some(entity) = task.mode() else {
            react.dispatch(ped);
            return None;
        };
        if react.entity_kind(entity) & KIND_MASK != WANT_KIND {
            react.dispatch(ped);
            return None;
        }
        if react.probe(ped, entity) & 0xFF != 0 {
            react.dispatch(ped);
            return None;
        }
        let block = react.alloc(manager).expect("wrong lift keeps a block");
        let built = react.build(block, entity).expect("wrong lift keeps a build");
        let mut marks = built.marks() | MARK_BIT;
        if task.kind() < FLAG_LIMIT + 1 {
            marks &= WIDE_MASK;
        }
        Some(Reaction::new(built.task(), marks))
    }

    #[test]
    fn flee_react_matches() {
        set_callee(2, probe_addr());
        set_callee(3, alloc_addr());
        set_callee(4, build_addr());
        set_callee(5, dispatch_addr());
        let mut rng = Rng(0xEAC7);
        let mut caught = 0;
        let mut cases = 0;
        let type_answers = [WANT_TYPE, 0, 1, WANT_TYPE - 1, WANT_TYPE + 1, rng.u32()];
        let kind_words = [0xC0u32, 0, 0x3C0, 0xC1, 0x80, rng.u32()];
        let probe_answers = [0u32, 1, 0xFF, 0x100, rng.u32()];
        let kinds = [0u32, 1, FLAG_LIMIT - 1, FLAG_LIMIT, FLAG_LIMIT + 1, 0x8000_0000, rng.u32()];
        let mut inputs = Vec::new();
        for &type_ans in &type_answers {
            for &live_mode in &[false, true] {
                for &kind_word in &kind_words {
                    for &probe_ans in &probe_answers {
                        for &kind in &kinds {
                            inputs.push((type_ans, live_mode, kind_word, probe_ans, kind));
                        }
                    }
                }
            }
        }
        for (type_ans, live_mode, kind_word, probe_ans, kind) in inputs {
            let manager = rng.u32();
            let fresh = rng.u32() | 1;
            let probe_word = rng.u32();
            let init_marks = rng.u32();
            let mut blob = *flee_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[FL_KIND] = kind;
            // A live subtask with the type stub in its table.
            let table = fake_table(8, 3, type_addr());
            let mut sub = sub_blob();
            sub[0] = addr(&table[0]);
            let sub_boxed = sub;
            let sub_before = *sub_boxed;
            let sub_addr = addr(&sub_boxed[0]);
            blob[FL_SUB] = sub_addr;
            // The mode entity, live or null.
            let mut entity = entity_blob();
            for w in entity.iter_mut() {
                *w = rng.u32();
            }
            entity[E_KINDW] = kind_word;
            let entity_boxed = entity;
            let entity_before = *entity_boxed;
            let entity_addr = addr(&entity_boxed[0]);
            blob[FL_MODE] = if live_mode { entity_addr } else { 0 };
            // A live ped carrying the probe word.
            let mut ped = big_ped_blob();
            for w in ped.iter_mut() {
                *w = rng.u32();
            }
            ped[P_PROBEW] = probe_word;
            let ped_boxed = ped;
            let ped_before = *ped_boxed;
            let ped_addr = addr(&ped_boxed[0]);
            let boxed = Box::new(blob);
            let before = *boxed;
            // The built blob the build stub answers.
            let mut built = built_blob();
            for w in built.iter_mut() {
                *w = rng.u32();
            }
            built[B_MARKSW] = init_marks;
            let built_before = *built;
            let built_boxed = built;
            let built_addr = addr(&built_boxed[0]);
            set_manager(manager);
            TYPE_ANS.store(type_ans, Ordering::SeqCst);
            PROBE_ANS.store(probe_ans, Ordering::SeqCst);
            ALLOC_ANS.store(fresh, Ordering::SeqCst);
            BUILD_ANS.store(built_addr, Ordering::SeqCst);
            TYPE_COUNT.store(0, Ordering::SeqCst);
            PROBE_COUNT.store(0, Ordering::SeqCst);
            ALLOC_COUNT.store(0, Ordering::SeqCst);
            BUILD_COUNT.store(0, Ordering::SeqCst);
            DISPATCH_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00DA6080::rw_00da6080(addr(&boxed[0]), ped_addr) };
            // The expected path from the scripted answers.
            let past_type = type_ans == WANT_TYPE;
            let past_mode = past_type && live_mode;
            let past_kind = past_mode && kind_word & KIND_MASK == WANT_KIND;
            let built_it = past_kind && probe_ans & 0xFF == 0;
            assert_eq!(TYPE_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(TYPE_SUB.load(Ordering::SeqCst), sub_addr);
            assert_eq!(PROBE_COUNT.load(Ordering::SeqCst), u32::from(past_kind));
            if past_kind {
                assert_eq!(PROBE_OBJ.load(Ordering::SeqCst), probe_word);
                assert_eq!(PROBE_ENTITY.load(Ordering::SeqCst), entity_addr);
            }
            assert_eq!(ALLOC_COUNT.load(Ordering::SeqCst), u32::from(built_it));
            if built_it {
                assert_eq!(ALLOC_MGR.load(Ordering::SeqCst), manager);
            }
            assert_eq!(BUILD_COUNT.load(Ordering::SeqCst), u32::from(built_it));
            if built_it {
                assert_eq!(BUILD_BLOCK.load(Ordering::SeqCst), fresh);
                assert_eq!(BUILD_ENTITY.load(Ordering::SeqCst), entity_addr);
                assert_eq!(BUILD_ZERO.load(Ordering::SeqCst), 0);
            }
            assert_eq!(DISPATCH_COUNT.load(Ordering::SeqCst), u32::from(!built_it));
            if !built_it {
                assert_eq!(DISPATCH_THIS.load(Ordering::SeqCst), addr(&boxed[0]));
                assert_eq!(DISPATCH_ARG.load(Ordering::SeqCst), ped_addr);
                assert_eq!(got, 0);
            } else {
                assert_eq!(got, built_addr);
            }
            // Source, entity, ped and subtask blobs survive whole.
            assert_eq!(*boxed, before);
            assert_eq!(*entity_boxed, entity_before);
            assert_eq!(*ped_boxed, ped_before);
            assert_eq!(*sub_boxed, sub_before);
            let mut built_expect = built_before;
            if built_it {
                let mut marks = init_marks | MARK_BIT;
                if kind < FLAG_LIMIT {
                    marks &= WIDE_MASK;
                }
                built_expect[B_MARKSW] = marks;
            }
            assert_eq!(*built_boxed, built_expect);
            let lift = FleeTask::new(
                Handle32::new(sub_addr),
                0,
                kind,
                [
                    f32::from_bits(before[FL_POS]),
                    f32::from_bits(before[FL_POS + 1]),
                    f32::from_bits(before[FL_POS + 2]),
                ],
                blob_byte(&before[..], FL_FLAG_BYTE) != 0,
                Handle32::new(before[FL_MODE]),
                blob_byte(&before[..], FL_STATE_BYTE),
            );
            let mut fake = Fake {
                type_answer: type_ans,
                kind_answer: kind_word,
                probe_answer: probe_ans,
                alloc_answer: fresh,
                build_answer: Some(Reaction::new(
                    Handle32::new(built_addr).expect("built blob is live"),
                    init_marks,
                )),
                types: Vec::new(),
                kinds: Vec::new(),
                probes: Vec::new(),
                allocs: Vec::new(),
                builds: Vec::new(),
                dispatches: Vec::new(),
            };
            let lift_ret = lift.react(
                Handle32::new(ped_addr),
                Handle32::new(manager),
                &mut fake,
            );
            assert_eq!(got, Handle32::raw_or_zero(lift_ret.map(Reaction::task)));
            assert_eq!(fake.types, vec![sub_addr]);
            assert_eq!(fake.kinds.len() as u32, u32::from(past_mode));
            if past_mode {
                assert_eq!(fake.kinds[0], entity_addr);
            }
            assert_eq!(fake.probes.len() as u32, u32::from(past_kind));
            if past_kind {
                assert_eq!(fake.probes[0], (ped_addr, entity_addr));
            }
            assert_eq!(fake.allocs.len() as u32, u32::from(built_it));
            if built_it {
                assert_eq!(fake.allocs[0], manager);
            }
            assert_eq!(fake.builds.len() as u32, u32::from(built_it));
            if built_it {
                assert_eq!(fake.builds[0], (fresh, entity_addr));
            }
            assert_eq!(fake.dispatches.len() as u32, u32::from(!built_it));
            if !built_it {
                assert_eq!(fake.dispatches[0], ped_addr);
            }
            if let Some(reaction) = lift_ret {
                assert_eq!(reaction.task().get(), built_addr);
                assert_eq!(reaction.marks(), built_boxed[B_MARKSW]);
            }
            let mut wfake = Fake {
                type_answer: type_ans,
                kind_answer: kind_word,
                probe_answer: probe_ans,
                alloc_answer: fresh,
                build_answer: Some(Reaction::new(
                    Handle32::new(built_addr).expect("built blob is live"),
                    init_marks,
                )),
                types: Vec::new(),
                kinds: Vec::new(),
                probes: Vec::new(),
                allocs: Vec::new(),
                builds: Vec::new(),
                dispatches: Vec::new(),
            };
            let w_ret = wrong_react(
                &lift,
                Handle32::new(ped_addr),
                Handle32::new(manager),
                &mut wfake,
            );
            let w_marks = w_ret.map(Reaction::marks).unwrap_or(0);
            if built_it && w_marks != built_boxed[B_MARKSW] {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong flee react never caught ({cases} cases)");
    }
}
