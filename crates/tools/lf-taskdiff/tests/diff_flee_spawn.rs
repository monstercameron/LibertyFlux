//! Differential case: the flee spawner slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the untouched task blob, and the alloc and
//! constructor calls against the lift's trait calls, both sides given
//! the same scripted answers. The vector form's spilled position is
//! snapped through the passed pointer and compared bitwise. A
//! deliberately wrong lift must be caught at least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{FleeEntity, FleeSpawn, FleeTask, SubTask, TaskMgr, UninitTask};
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::{set_callee, set_manager2, set_threshold};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        FL_FLAG_BYTE, FL_KIND, FL_MODE, FL_POS, FL_STATE_BYTE, Rng, addr, blob_byte, flee_blob,
        set_blob_byte,
    };

    // Alloc stub (slot 1) and its recording.
    static ALLOC_MGR: AtomicU32 = AtomicU32::new(0);
    static ALLOC_ANS: AtomicU32 = AtomicU32::new(0);
    static ALLOC_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn alloc_stub(mgr: u32) -> u32 {
        ALLOC_MGR.store(mgr, Ordering::SeqCst);
        ALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
        ALLOC_ANS.load(Ordering::SeqCst)
    }

    // Entity-constructor stub (slot 2) and its recording.
    static EA_BLOCK: AtomicU32 = AtomicU32::new(0);
    static EA_MODE: AtomicU32 = AtomicU32::new(0);
    static EA_ONE_A: AtomicU32 = AtomicU32::new(0);
    static EA_ONE_B: AtomicU32 = AtomicU32::new(0);
    static EA_KIND: AtomicU32 = AtomicU32::new(0);
    static EA_ANS: AtomicU32 = AtomicU32::new(0);
    static EA_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn entity_stub(
        block: u32,
        mode: u32,
        one_a: u32,
        one_b: u32,
        kind: u32,
    ) -> u32 {
        EA_BLOCK.store(block, Ordering::SeqCst);
        EA_MODE.store(mode, Ordering::SeqCst);
        EA_ONE_A.store(one_a, Ordering::SeqCst);
        EA_ONE_B.store(one_b, Ordering::SeqCst);
        EA_KIND.store(kind, Ordering::SeqCst);
        EA_COUNT.fetch_add(1, Ordering::SeqCst);
        EA_ANS.load(Ordering::SeqCst)
    }

    // Vector-constructor stub (slot 3): snaps the three spilled words
    // through the passed pointer and records the rest.
    static VB_BLOCK: AtomicU32 = AtomicU32::new(0);
    static VB_WORDS: [AtomicU32; 3] = [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];
    static VB_ONE_A: AtomicU32 = AtomicU32::new(0);
    static VB_ONE_B: AtomicU32 = AtomicU32::new(0);
    static VB_KIND: AtomicU32 = AtomicU32::new(0);
    static VB_ANS: AtomicU32 = AtomicU32::new(0);
    static VB_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn vector_stub(
        block: u32,
        spill: u32,
        one_a: u32,
        one_b: u32,
        kind: u32,
    ) -> u32 {
        VB_BLOCK.store(block, Ordering::SeqCst);
        unsafe {
            VB_WORDS[0].store((spill as *const u32).read(), Ordering::SeqCst);
            VB_WORDS[1].store(((spill + 4) as *const u32).read(), Ordering::SeqCst);
            VB_WORDS[2].store(((spill + 8) as *const u32).read(), Ordering::SeqCst);
        }
        VB_ONE_A.store(one_a, Ordering::SeqCst);
        VB_ONE_B.store(one_b, Ordering::SeqCst);
        VB_KIND.store(kind, Ordering::SeqCst);
        VB_COUNT.fetch_add(1, Ordering::SeqCst);
        VB_ANS.load(Ordering::SeqCst)
    }

    fn alloc_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = alloc_stub;
        f as usize as u32
    }

    fn entity_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 = entity_stub;
        f as usize as u32
    }

    fn vector_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 = vector_stub;
        f as usize as u32
    }

    struct Fake {
        alloc_answer: u32,
        entity_answer: u32,
        vector_answer: u32,
        allocs: Vec<u32>,
        entities: Vec<(u32, u32, u32)>,
        vectors: Vec<(u32, [u32; 3], u32)>,
    }

    impl FleeSpawn for Fake {
        fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
            self.allocs.push(Handle32::raw_or_zero(manager));
            Handle32::new(self.alloc_answer)
        }

        fn construct_entity(
            &mut self,
            block: Handle32<UninitTask>,
            entity: Handle32<FleeEntity>,
            kind: u32,
        ) -> Option<Handle32<SubTask>> {
            self.entities.push((block.get(), entity.get(), kind));
            Handle32::new(self.entity_answer)
        }

        fn construct_vector(
            &mut self,
            block: Handle32<UninitTask>,
            pos: [f32; 3],
            kind: u32,
        ) -> Option<Handle32<SubTask>> {
            self.vectors.push((
                block.get(),
                [pos[0].to_bits(), pos[1].to_bits(), pos[2].to_bits()],
                kind,
            ));
            Handle32::new(self.vector_answer)
        }
    }

    /// Pinned-order helpers for the inverted gate below.
    fn mul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }

    fn add(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }

    /// Rests exactly where the gate fires (the inverted liveness test).
    fn wrong_spawn(
        task: &FleeTask,
        threshold: f32,
        manager: Option<Handle32<TaskMgr>>,
        spawn: &mut Fake,
    ) -> Option<Handle32<SubTask>> {
        let pos = task.pos();
        let fire = if task.flag() {
            task.mode().is_some()
        } else if task.mode().is_some() {
            false
        } else {
            let sq = add(
                add(mul(pos[0], pos[0]), mul(pos[1], pos[1])),
                mul(pos[2], pos[2]),
            );
            sq > threshold
        };
        if fire {
            return None;
        }
        let block = spawn.alloc(manager)?;
        match task.mode() {
            Some(entity) => spawn.construct_entity(block, entity, task.kind()),
            None => spawn.construct_vector(block, task.pos(), task.kind()),
        }
    }

    #[test]
    fn flee_spawn_matches() {
        set_callee(1, alloc_addr());
        set_callee(2, entity_addr());
        set_callee(3, vector_addr());
        let mut rng = Rng(0xF1EA);
        let mut caught = 0;
        let mut cases = 0;
        // Positions: exact small lengths, edges, NaNs and random bits.
        let bare = 0.0f32.to_bits();
        let one = 1.0f32.to_bits();
        let three = 3.0f32.to_bits();
        let four = 4.0f32.to_bits();
        let neg = (-2.5f32).to_bits();
        let big = 1.0e20f32.to_bits();
        let tiny = f32::MIN_POSITIVE.to_bits();
        let inf = f32::INFINITY.to_bits();
        let nan = f32::NAN.to_bits();
        let basics = [bare, one, three, four, neg, big, tiny, inf, nan];
        // Thresholds around the in-game value and the float edges.
        let small = 0.05f32.to_bits();
        let zero = 0.0f32.to_bits();
        let mid = 25.0f32.to_bits();
        let huge = f32::MAX.to_bits();
        let neg_inf = f32::NEG_INFINITY.to_bits();
        let nan_t = f32::NAN.to_bits();
        let thresholds = [small, zero, mid, huge, neg_inf, nan_t, rng.u32()];
        let flags = [0u8, 1, 0xFF];
        let modes = [0u32, 1, 0xE0, 0xFFFF_FFFF, rng.u32() | 1];
        let managers = [0u32, 1, 0x9E57, rng.u32()];
        let blocks = [0u32, 1, 0x1000, rng.u32() | 1];
        let mut inputs = Vec::new();
        for &flag in &flags {
            for &mode in &modes {
                for &px in &basics {
                    for &py in &[bare, one, four, nan, rng.u32()] {
                        for &pz in &[bare, one, rng.u32()] {
                            for &threshold in &thresholds {
                                inputs.push((flag, mode, [px, py, pz], threshold));
                            }
                        }
                    }
                }
            }
        }
        // Stride the cross product so the binary stays quick while every
        // axis keeps its edges (the stride is coprime to the axis lengths).
        let mut picked = Vec::new();
        for (i, input) in inputs.iter().enumerate() {
            if i % 11 == 0 {
                picked.push(*input);
            }
        }
        for _ in 0..300 {
            picked.push(inputs[(rng.next() % inputs.len() as u64) as usize]);
        }
        for (flag, mode, pos_bits, threshold_bits) in picked {
            let manager = managers[(rng.next() % managers.len() as u64) as usize];
            let fresh = blocks[(rng.next() % blocks.len() as u64) as usize];
            let built_e = rng.u32();
            let built_v = rng.u32();
            let kind = rng.u32();
            let mut blob = *flee_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[FL_KIND] = kind;
            blob[FL_POS] = pos_bits[0];
            blob[FL_POS + 1] = pos_bits[1];
            blob[FL_POS + 2] = pos_bits[2];
            set_blob_byte(&mut blob, FL_FLAG_BYTE, flag);
            blob[FL_MODE] = mode;
            set_blob_byte(&mut blob, FL_STATE_BYTE, rng.u32() as u8);
            let boxed = Box::new(blob);
            let before = *boxed;
            set_manager2(manager);
            set_threshold(threshold_bits);
            ALLOC_ANS.store(fresh, Ordering::SeqCst);
            EA_ANS.store(built_e, Ordering::SeqCst);
            VB_ANS.store(built_v, Ordering::SeqCst);
            ALLOC_COUNT.store(0, Ordering::SeqCst);
            EA_COUNT.store(0, Ordering::SeqCst);
            VB_COUNT.store(0, Ordering::SeqCst);
            // The ignored stack word carries garbage, proving it unread.
            let got = unsafe { fn_00DA6530::rw_00da6530(addr(&boxed[0]), 0xB0B) };
            let fired = ALLOC_COUNT.load(Ordering::SeqCst) == 1;
            assert!(ALLOC_COUNT.load(Ordering::SeqCst) <= 1);
            if fired {
                assert_eq!(ALLOC_MGR.load(Ordering::SeqCst), manager);
            }
            let want_entity = fired && fresh != 0 && mode != 0;
            let want_vector = fired && fresh != 0 && mode == 0;
            assert_eq!(EA_COUNT.load(Ordering::SeqCst), u32::from(want_entity));
            assert_eq!(VB_COUNT.load(Ordering::SeqCst), u32::from(want_vector));
            if want_entity {
                assert_eq!(EA_BLOCK.load(Ordering::SeqCst), fresh);
                assert_eq!(EA_MODE.load(Ordering::SeqCst), mode);
                assert_eq!(EA_ONE_A.load(Ordering::SeqCst), 1);
                assert_eq!(EA_ONE_B.load(Ordering::SeqCst), 1);
                assert_eq!(EA_KIND.load(Ordering::SeqCst), kind);
                assert_eq!(got, built_e);
            }
            if want_vector {
                assert_eq!(VB_BLOCK.load(Ordering::SeqCst), fresh);
                assert_eq!(VB_WORDS[0].load(Ordering::SeqCst), pos_bits[0]);
                assert_eq!(VB_WORDS[1].load(Ordering::SeqCst), pos_bits[1]);
                assert_eq!(VB_WORDS[2].load(Ordering::SeqCst), pos_bits[2]);
                assert_eq!(VB_ONE_A.load(Ordering::SeqCst), 1);
                assert_eq!(VB_ONE_B.load(Ordering::SeqCst), 1);
                assert_eq!(VB_KIND.load(Ordering::SeqCst), kind);
                assert_eq!(got, built_v);
            }
            if !want_entity && !want_vector {
                assert_eq!(got, 0);
            }
            // The spawner writes nothing to the task.
            assert_eq!(*boxed, before);
            let pos = [
                f32::from_bits(pos_bits[0]),
                f32::from_bits(pos_bits[1]),
                f32::from_bits(pos_bits[2]),
            ];
            let lift = FleeTask::new(
                None,
                0,
                kind,
                pos,
                flag != 0,
                Handle32::new(mode),
                blob_byte(&boxed[..], FL_STATE_BYTE),
            );
            let mut fake = Fake {
                alloc_answer: fresh,
                entity_answer: built_e,
                vector_answer: built_v,
                allocs: Vec::new(),
                entities: Vec::new(),
                vectors: Vec::new(),
            };
            let lift_ret = lift.spawn(
                f32::from_bits(threshold_bits),
                Handle32::new(manager),
                &mut fake,
            );
            assert_eq!(got, Handle32::raw_or_zero(lift_ret));
            assert_eq!(fake.allocs.len() as u32, u32::from(fired));
            if fired {
                assert_eq!(fake.allocs[0], manager);
            }
            assert_eq!(fake.entities.len() as u32, u32::from(want_entity));
            assert_eq!(fake.vectors.len() as u32, u32::from(want_vector));
            if want_entity {
                assert_eq!(fake.entities[0], (fresh, mode, kind));
            }
            if want_vector {
                assert_eq!(fake.vectors[0], (fresh, pos_bits, kind));
            }
            let mut wfake = Fake {
                alloc_answer: fresh,
                entity_answer: built_e,
                vector_answer: built_v,
                allocs: Vec::new(),
                entities: Vec::new(),
                vectors: Vec::new(),
            };
            let w_ret = wrong_spawn(
                &lift,
                f32::from_bits(threshold_bits),
                Handle32::new(manager),
                &mut wfake,
            );
            if Handle32::raw_or_zero(w_ret) != got || wfake.allocs.len() as u32 != u32::from(fired)
            {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong flee spawn never caught ({cases} cases)");
    }
}
