//! Differential cases: the lifted game camera methods against their
//! verified 32-bit rewrites.
//!
//! Each lifted method runs against its verified rewrite on the same
//! generated camera images. The cases compare the return (where the
//! original returns more than the constant answer), every byte of the
//! camera image the rewrite leaves behind, and every call in order with its
//! arguments. Each method also runs a deliberately wrong version, which must
//! be caught on at least one trial. 32-bit target only.

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_camgame_diff::rewrites::fn_00b05d30::rw_00b05d30;
    use lf_camgame_diff::rewrites::fn_00b07760::rw_00b07760;
    use lf_camgame_diff::rewrites::fn_00b09cf0::rw_00b09cf0;
    use lf_camgame_diff::rewrites::fn_00b09da0::rw_00b09da0;
    use lf_camgame_diff::rt::{self, Call, Effect, SNAPSHOT, StubKind};
    use lf_core::Handle32;
    use lf_world::cam_game::{
        ACTIVE_MARKER, CCamGame, CamGameWorld, CamGlobal, CamOwnerTag, CamTag, DEFAULT_ZOOM,
        GateBlock, GateTag, IDLE_MARKER, PoolTag, SingletonField, SingletonTag, TailTag, TargetTag,
        WorkerTag,
    };
    use std::collections::HashMap;

    /// Size of a camera image: enough for every offset the rewrites touch.
    const OBJ: usize = 0x200;

    /// Trials per case. Each case is bounded: the loops below run this many
    /// times and stop.
    const TRIALS: u32 = 2000;

    /// Small deterministic generator (splitmix64).
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }

        fn u32(&mut self) -> u32 {
            (self.next() >> 32) as u32
        }

        fn u8(&mut self) -> u8 {
            self.next() as u8
        }

        fn below(&mut self, n: u32) -> u32 {
            self.u32() % n
        }
    }

    /// Edge flag bytes: none, each bit alone, pairs, all and all but one.
    /// Split into groups so that no run of table entries is long.
    const FLAG_EDGE: [u8; 12] = [
        // none, then the low bits alone
        0x00, 0x01, 0x02, 0x04, 0x08,
        // the high bits alone, then pairs
        0x40, 0x80, 0x09,
        // pairs with the high bits, all, and all but the top bit
        0x41, 0xC0, 0xFF, 0x7F,
    ];

    /// Edge words for values the callees write.
    const WORD_EDGE: [u32; 8] = [
        0, 1, 0x80, 0xFF, 0x7FFF_FFFF, 0x8000_0000, 0x0000_0040, 0xFFFF_FFFF,
    ];

    fn read_word(bytes: &[u8], off: usize, width: u8) -> u32 {
        if width == 1 {
            u32::from(bytes[off])
        } else {
            u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap())
        }
    }

    fn write_word(bytes: &mut [u8], off: usize, width: u8, v: u32) {
        if width == 1 {
            bytes[off] = v as u8;
        } else {
            bytes[off..off + 4].copy_from_slice(&v.to_le_bytes());
        }
    }

    /// The camera word at `off`, as the rewrites see it (widened).
    fn get_field(cam: &CCamGame, off: u32) -> u32 {
        match off {
            0x74 => cam.blend,
            0x114 => cam.owner.map_or(0, Handle32::get),
            0x140 => cam.mode,
            0x144 => u32::from(cam.mode_flag),
            0x148 => cam.default_word,
            0x14C => cam.timer,
            0x150 => u32::from(cam.timer_flag),
            0x154 => cam.word_154,
            0x1BC => cam.zoom_bits,
            0x1C4 => u32::from(cam.flags_1c4),
            0x1C5 => u32::from(cam.flags_1c5),
            0x1C8 => cam.pending_1c8,
            0x1CC => cam.pending_1cc,
            0x1D0 => cam.pending_1d0,
            0x1D4 => cam.word_1d4,
            0x1D8 => u32::from(cam.flag_1d8),
            0x1DC => cam.word_1dc,
            0x1E0 => cam.word_1e0,
            0x1E4 => cam.word_1e4,
            0x1E8 => cam.word_1e8,
            _ => panic!("offset {off:#x} is not a modelled camera word"),
        }
    }

    fn set_field(cam: &mut CCamGame, off: u32, v: u32) {
        match off {
            0x74 => cam.blend = v,
            0x114 => cam.owner = Handle32::new(v),
            0x140 => cam.mode = v,
            0x144 => cam.mode_flag = v as u8,
            0x148 => cam.default_word = v,
            0x14C => cam.timer = v,
            0x150 => cam.timer_flag = v as u8,
            0x154 => cam.word_154 = v,
            0x1BC => cam.zoom_bits = v,
            0x1C4 => cam.flags_1c4 = v as u8,
            0x1C5 => cam.flags_1c5 = v as u8,
            0x1C8 => cam.pending_1c8 = v,
            0x1CC => cam.pending_1cc = v,
            0x1D0 => cam.pending_1d0 = v,
            0x1D4 => cam.word_1d4 = v,
            0x1D8 => cam.flag_1d8 = v as u8,
            0x1DC => cam.word_1dc = v,
            0x1E0 => cam.word_1e0 = v,
            0x1E4 => cam.word_1e4 = v,
            0x1E8 => cam.word_1e8 = v,
            _ => panic!("offset {off:#x} is not a modelled camera word"),
        }
    }

    /// The camera words the callees record, in the order of [`SNAPSHOT`].
    fn snap_cam(cam: &CCamGame) -> Vec<u32> {
        SNAPSHOT.iter().map(|&(off, _)| get_field(cam, off)).collect()
    }

    /// Reads a camera image into the lifted state. Every modelled word is
    /// read; the bytes the lift does not model stay in the image.
    fn decode(bytes: &[u8]) -> CCamGame {
        let mut cam = CCamGame::default();
        for &(off, width) in &SNAPSHOT {
            set_field(&mut cam, off, read_word(bytes, off as usize, width));
        }
        cam
    }

    /// Writes the lifted state back into the image, over the modelled words.
    fn encode(cam: &CCamGame, bytes: &mut [u8]) {
        for &(off, width) in &SNAPSHOT {
            write_word(bytes, off as usize, width, get_field(cam, off));
        }
    }

    /// Applies the trial's scripted writes to the lifted state, as the
    /// effect stubs apply them to the image.
    fn apply_effects(cam: &mut CCamGame, effects: &[Effect]) {
        for e in effects {
            set_field(cam, e.off, e.value);
        }
    }

    /// The lifted side's collaborators: records each call with the words the
    /// rewrite's stub records, and answers from the trial's values.
    #[derive(Default)]
    struct Fake {
        log: Vec<Call>,
        manager: Option<Handle32<PoolTag>>,
        default_word: u32,
        worker: Option<Handle32<WorkerTag>>,
        effects: Vec<Effect>,
        // The gate step's world: its answers and the byte images it reads
        // and writes, keyed by the address the rewrite sees.
        inhibit: u8,
        gate_a: Option<u32>,
        gate_b: Option<u32>,
        targets: [Option<u32>; 2],
        words: [u16; 2],
        singleton: bool,
        tail: Option<u32>,
        images: Vec<(u32, Vec<u8>)>,
        matched: Option<u8>,
        done: Option<u8>,
        // The blend step's world: its globals and the singleton's fields.
        ugl: HashMap<CamGlobal, u32>,
        sing: Option<u32>,
        sing_active: u8,
        sing_mode: u32,
        gate_one: u32,
        gate_two: u32,
    }

    impl Fake {
        fn image(&self, addr: u32) -> &[u8] {
            &self
                .images
                .iter()
                .find(|(a, _)| *a == addr)
                .expect("the case installed this image")
                .1
        }

        fn image_mut(&mut self, addr: u32) -> &mut Vec<u8> {
            &mut self
                .images
                .iter_mut()
                .find(|(a, _)| *a == addr)
                .expect("the case installed this image")
                .1
        }
    }

    /// Index of a block's answers: the first block is 0, the second 1.
    fn block_index(block: GateBlock) -> usize {
        match block {
            GateBlock::First => 0,
            GateBlock::Second => 1,
        }
    }

    impl CamGameWorld for Fake {
        fn destroy_body(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            let mut args = vec![this.get()];
            args.extend(snap_cam(cam));
            self.log.push(("destroy_body".to_string(), args));
            apply_effects(cam, &self.effects);
        }

        fn pool_manager(&mut self) -> Option<Handle32<PoolTag>> {
            self.manager
        }

        fn pool_release(&mut self, manager: Option<Handle32<PoolTag>>, this: Handle32<CamTag>) {
            let m = manager.map_or(0, Handle32::get);
            self.log
                .push(("pool_release".to_string(), vec![m, this.get()]));
        }

        fn teardown(&mut self, _cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.log.push(("teardown".to_string(), vec![this.get()]));
        }

        fn create_worker(
            &mut self,
            owner: Option<Handle32<CamOwnerTag>>,
            mode: u32,
            extra: u32,
            this: Handle32<CamTag>,
        ) -> Option<Handle32<WorkerTag>> {
            let o = owner.map_or(0, Handle32::get);
            self.log.push((
                "create_worker".to_string(),
                vec![o, mode, extra, this.get()],
            ));
            self.worker
        }

        fn chain_set_flags(&mut self, worker: Option<Handle32<WorkerTag>>) {
            let w = worker.map_or(0, Handle32::get);
            self.log.push(("chain_set_flags".to_string(), vec![w]));
        }

        fn release_worker(&mut self, worker: Option<Handle32<WorkerTag>>) {
            let w = worker.map_or(0, Handle32::get);
            self.log.push(("release_worker".to_string(), vec![w]));
        }

        fn reset_state_block(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            let mut args = vec![this.get()];
            args.extend(snap_cam(cam));
            self.log.push(("reset_state_block".to_string(), args));
            apply_effects(cam, &self.effects);
        }

        fn shared_default_word(&mut self) -> u32 {
            self.default_word
        }

        fn inhibit_flag(&mut self) -> u8 {
            self.inhibit
        }

        fn gate_status(&mut self, block: GateBlock) -> Option<Handle32<GateTag>> {
            self.log.push(("gate_status".to_string(), vec![0, 0]));
            let addr = match block {
                GateBlock::First => self.gate_a,
                GateBlock::Second => self.gate_b,
            };
            addr.and_then(Handle32::new)
        }

        fn gate_byte(&mut self, gate: Handle32<GateTag>, offset: u32) -> u8 {
            self.image(gate.get())[offset as usize]
        }

        fn find_target(
            &mut self,
            _cam: &mut CCamGame,
            this: Handle32<CamTag>,
            block: GateBlock,
        ) -> Option<Handle32<TargetTag>> {
            self.log
                .push(("find_target".to_string(), vec![this.get(), 0x16, 0]));
            self.targets[block_index(block)].and_then(Handle32::new)
        }

        fn gate_word(&mut self, block: GateBlock) -> u16 {
            self.words[block_index(block)]
        }

        fn notify_sink(
            &mut self,
            _cam: &mut CCamGame,
            this: Handle32<CamTag>,
            word: u16,
            _block: GateBlock,
        ) {
            let sub = this.get().wrapping_add(0x10);
            self.log.push((
                "notify_sink".to_string(),
                vec![this.get(), sub, u32::from(word), 1, 1],
            ));
        }

        fn sync_target(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            let mut args = vec![this.get()];
            args.extend(snap_cam(cam));
            self.log.push(("sync_target".to_string(), args));
            apply_effects(cam, &self.effects);
        }

        fn singleton_present(&mut self) -> bool {
            self.log.push(("singleton_present".to_string(), vec![]));
            self.singleton
        }

        fn middle_update(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>, step: u32) {
            let mut args = vec![this.get()];
            args.extend(snap_cam(cam));
            args.push(step);
            self.log.push(("middle_update".to_string(), args));
            apply_effects(cam, &self.effects);
        }

        fn tail(&mut self, _cam: &mut CCamGame, this: Handle32<CamTag>) -> Option<Handle32<TailTag>> {
            self.log
                .push(("tail".to_string(), vec![this.get(), 2, 0]));
            self.tail.and_then(Handle32::new)
        }

        fn tail_state(&mut self, tail: Handle32<TailTag>) -> u8 {
            self.image(tail.get())[0x22E]
        }

        fn tail_flags(&mut self, tail: Handle32<TailTag>) -> u8 {
            self.image(tail.get())[0x18C]
        }

        fn set_tail_flags(&mut self, tail: Handle32<TailTag>, flags: u8) {
            self.image_mut(tail.get())[0x18C] = flags;
        }

        fn set_tail_state(&mut self, tail: Handle32<TailTag>, state: u8) {
            self.image_mut(tail.get())[0x22E] = state;
        }

        fn set_matched(&mut self, matched: u8) {
            self.matched = Some(matched);
        }

        fn clear_done(&mut self) {
            self.done = Some(0);
        }

        fn update_1(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.log_update("update_1", cam, this, None);
        }

        fn update_2(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.log_update("update_2", cam, this, None);
        }

        fn update_3(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.log_update("update_3", cam, this, None);
        }

        fn update_4(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.log_update("update_4", cam, this, None);
        }

        fn watch_init(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.log_update("watch_init", cam, this, None);
        }

        fn singleton(&mut self) -> Option<Handle32<SingletonTag>> {
            self.log.push(("singleton".to_string(), vec![]));
            self.sing.and_then(Handle32::new)
        }

        fn singleton_field(&mut self, _singleton: Handle32<SingletonTag>, field: SingletonField) -> u32 {
            match field {
                SingletonField::ActiveByte => u32::from(self.sing_active),
                SingletonField::ModeWord => self.sing_mode,
            }
        }

        fn blend_gate_one(&mut self) -> u32 {
            self.log.push(("blend_gate_one".to_string(), vec![]));
            self.gate_one
        }

        fn blend_gate_two(&mut self) -> u32 {
            self.log.push(("blend_gate_two".to_string(), vec![]));
            self.gate_two
        }

        fn read_global(&mut self, global: CamGlobal) -> u32 {
            self.ugl.get(&global).copied().unwrap_or(0)
        }

        fn write_global(&mut self, global: CamGlobal, bits: u32) {
            let masked = bits & global_mask(global);
            self.ugl.insert(global, masked);
        }

        fn update_9(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.log_update("update_9", cam, this, None);
        }

        fn update_10(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.log_update("update_10", cam, this, None);
        }

        fn update_11(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.log_update("update_11", cam, this, None);
        }
    }

    /// The bytes a global occupies: the mask of its value.
    fn global_mask(global: CamGlobal) -> u32 {
        match global {
            CamGlobal::InhibitA
            | CamGlobal::InhibitB
            | CamGlobal::Done1
            | CamGlobal::Done2
            | CamGlobal::Fallback => 0xFF,
            _ => 0xFFFF_FFFF,
        }
    }

    /// The file address and width of each global the blend step uses.
    const UPDATE_GLOBALS: [(CamGlobal, u32, u8); 14] = [
        (CamGlobal::Ceiling, 0x00FE_88E8, 4),
        (CamGlobal::InhibitA, 0x0116_0C39, 1),
        (CamGlobal::InhibitB, 0x017F_5EB3, 1),
        (CamGlobal::Step1, 0x0104_0070, 4),
        (CamGlobal::Accumulator1Step, 0x0104_0074, 4),
        (CamGlobal::Step2, 0x0104_0078, 4),
        (CamGlobal::Accumulator2Step, 0x0104_007C, 4),
        (CamGlobal::Cap, 0x0104_0080, 4),
        (CamGlobal::Accumulator1, 0x0161_54A4, 4),
        (CamGlobal::Count, 0x0161_54A8, 4),
        (CamGlobal::Done1, 0x0161_54A0, 1),
        (CamGlobal::Accumulator2, 0x0161_54AC, 4),
        (CamGlobal::Done2, 0x0161_54A1, 1),
        (CamGlobal::Fallback, 0x012B_D193, 1),
    ];

    impl Fake {
        /// Logs an update callee with the words the rewrite's stub records:
        /// the camera's identity and its snapshot, then any extra word.
        fn log_update(&mut self, name: &str, cam: &mut CCamGame, this: Handle32<CamTag>, extra: Option<u32>) {
            let mut args = vec![this.get()];
            args.extend(snap_cam(cam));
            args.extend(extra);
            self.log.push((name.to_string(), args));
            apply_effects(cam, &self.effects);
        }
    }

    /// Random camera image: every word random, the flag bytes cycling
    /// through edge values, the owner zero one time in eight.
    fn random_object(rng: &mut Rng, trial: u32) -> Vec<u8> {
        let mut bytes: Vec<u8> = (0..OBJ).map(|_| rng.u8()).collect();
        // The table pointer (first word) is never written by any rewrite:
        // keep it as random as the rest.
        let f4 = FLAG_EDGE[(trial as usize) % FLAG_EDGE.len()];
        let f5 = FLAG_EDGE[(trial as usize / 3) % FLAG_EDGE.len()];
        bytes[0x1C4] = f4;
        bytes[0x1C5] = f5;
        let owner = if rng.below(8) == 0 {
            0
        } else {
            (rng.u32() | 1).max(1)
        };
        write_word(&mut bytes, 0x114, 4, owner);
        bytes
    }

    /// Random scripted writes: up to three, at camera words, with edge
    /// values one time in four.
    fn random_effects(rng: &mut Rng) -> Vec<Effect> {
        let count = rng.below(4);
        (0..count)
            .map(|_| {
                let (off, width) = SNAPSHOT[rng.below(SNAPSHOT.len() as u32) as usize];
                let raw = if rng.below(4) == 0 {
                    WORD_EDGE[rng.below(WORD_EDGE.len() as u32) as usize]
                } else {
                    rng.u32()
                };
                let value = if width == 1 { raw & 0xFF } else { raw };
                Effect { off, width, value }
            })
            .collect()
    }

    /// Compares the rewrite's calls with the lifted calls: same length, and
    /// each rewrite name maps to the lifted name, with equal words.
    fn calls_agree(rw: &[Call], lift: &[Call], map: &[(&str, &str)]) -> bool {
        if rw.len() != lift.len() {
            return false;
        }
        rw.iter().zip(lift).all(|((rn, ra), (ln, la))| {
            map.iter()
                .any(|(r, l)| *r == rn.as_str() && *l == ln.as_str())
                && ra == la
        })
    }

    /// A deliberately wrong version of a lifted method. The last argument is
    /// the method's one scalar input (the destructor's flags word; zero for
    /// the others).
    type Mutant = fn(&mut CCamGame, &mut Fake, Handle32<CamTag>, u32);

    // ---------------------------------------------------------------------
    // Deleting destructor (slot 0).

    /// Wrong version: tests bit 1 where the original tests bit 0.
    fn mutant_vf0_bit1(cam: &mut CCamGame, world: &mut Fake, this: Handle32<CamTag>, flags: u32) {
        world.destroy_body(cam, this);
        if flags & 2 != 0 {
            let manager = world.pool_manager();
            world.pool_release(manager, this);
        }
    }

    /// Runs the destructor cases; returns the number of trials that disagree
    /// (zero for the lifted method).
    fn vf0_run(mutant: Option<Mutant>) -> u32 {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x0C_AA_0001);
        let mut mismatches = 0;
        let flags_edge = [0u32, 1, 2, 3, 0xFFFF_FFFE, 0xFFFF_FFFF, 0x8000_0001, 0x10];
        for trial in 0..TRIALS {
            let before = random_object(&mut rng, trial);
            let effects = random_effects(&mut rng);
            let manager = if rng.below(6) == 0 { 0 } else { rng.u32() | 1 };
            let flags = if trial % 2 == 0 {
                flags_edge[(trial as usize / 2) % flags_edge.len()]
            } else {
                rng.u32()
            };
            rt::set_case(
                &[
                    (1, StubKind::Thiscall1Effect, vec![]),
                    (2, StubKind::Thiscall2, vec![0]),
                ],
                &effects,
                manager,
                0,
            );
            let mut rw = before.clone();
            let addr = rw.as_mut_ptr() as usize as u32;
            let ret = rw_00b05d30(addr, flags);
            let rw_calls = rt::take_calls();
            assert_eq!(ret, addr, "the original returns the object it was given");
            let expected_calls = if flags & 1 != 0 { 2 } else { 1 };
            assert_eq!(rw_calls.len(), expected_calls, "the original's destructor calls");

            let mut lift = before.clone();
            let mut cam = decode(&before);
            let mut fake = Fake {
                manager: Handle32::new(manager),
                effects: effects.clone(),
                ..Fake::default()
            };
            let this = Handle32::new(addr).expect("non-zero identity");
            match mutant {
                None => cam.deleting_destructor(&mut fake, this, flags),
                Some(m) => m(&mut cam, &mut fake, this, flags),
            }
            encode(&cam, &mut lift);
            let map = [("#1", "destroy_body"), ("#2", "pool_release")];
            let calls_ok = calls_agree(&rw_calls, &fake.log, &map);
            if lift != rw || !calls_ok {
                mismatches += 1;
            }
        }
        eprintln!("destructor trials disagreeing: {mismatches} of {TRIALS}");
        mismatches
    }

    #[test]
    fn vf0_lifted_matches_rewrite() {
        assert_eq!(vf0_run(None), 0, "lifted deleting destructor disagrees");
    }

    #[test]
    fn vf0_wrong_version_is_caught() {
        assert!(
            vf0_run(Some(mutant_vf0_bit1)) > 0,
            "the bit-1 version was not caught"
        );
    }

    // ---------------------------------------------------------------------
    // Restart (slot 1).

    /// Wrong version: skips the worker's release.
    fn mutant_vf1_no_release(
        cam: &mut CCamGame,
        world: &mut Fake,
        this: Handle32<CamTag>,
        _arg: u32,
    ) {
        world.teardown(cam, this);
        let owner = cam.owner;
        let worker = world.create_worker(owner, 1, 0, this);
        world.chain_set_flags(worker);
        cam.flags_1c4 &= !ACTIVE_MARKER;
        cam.pending_1c8 = 0;
        cam.pending_1cc = 0;
        cam.pending_1d0 = 0;
    }

    /// Wrong version: passes the worker's mode and extra words swapped.
    fn mutant_vf1_swapped(cam: &mut CCamGame, world: &mut Fake, this: Handle32<CamTag>, _arg: u32) {
        world.teardown(cam, this);
        let owner = cam.owner;
        let worker = world.create_worker(owner, 0, 1, this);
        world.chain_set_flags(worker);
        world.release_worker(worker);
        cam.flags_1c4 &= !ACTIVE_MARKER;
        cam.pending_1c8 = 0;
        cam.pending_1cc = 0;
        cam.pending_1d0 = 0;
    }

    fn vf1_run(mutant: Option<Mutant>) -> u32 {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x0C_AA_0002);
        let mut mismatches = 0;
        for trial in 0..TRIALS {
            // The camera's table: slot six (byte offset 0x18) is the
            // dispatched teardown; the worker's table: slot one (byte offset
            // 4) is its release.
            let mut camera_table = vec![0u32; 0x10].into_boxed_slice();
            camera_table[0x18 / 4] = rt::teardown_stub_addr();
            let mut worker_table = vec![0u32; 0x4].into_boxed_slice();
            worker_table[1] = rt::release_worker_stub_addr();
            let mut worker_obj = vec![0u32; 0x4].into_boxed_slice();
            worker_obj[0] = worker_table.as_ptr() as usize as u32;
            let worker_addr = worker_obj.as_ptr() as usize as u32;

            let mut before = random_object(&mut rng, trial);
            let table_addr = camera_table.as_ptr() as usize as u32;
            write_word(&mut before, 0, 4, table_addr);
            // Every trial uses the same camera and worker objects, so the
            // rewrite's reads of the tables see the stubs installed above.
            rt::set_case(
                &[
                    (2, StubKind::Thiscall4, vec![worker_addr]),
                    (3, StubKind::Thiscall1, vec![]),
                ],
                &[],
                0,
                0,
            );
            let mut rw = before.clone();
            let addr = rw.as_mut_ptr() as usize as u32;
            let ret = rw_00b09da0(addr);
            let rw_calls = rt::take_calls();
            assert_eq!(ret & 0xFF, 1, "the original always answers 1");
            assert_eq!(rw_calls.len(), 4, "the original's restart calls");

            let mut lift = before.clone();
            let mut cam = decode(&before);
            let worker = Handle32::new(worker_addr);
            let mut fake = Fake {
                worker,
                ..Fake::default()
            };
            let this = Handle32::new(addr).expect("non-zero identity");
            match mutant {
                None => cam.restart(&mut fake, this),
                Some(m) => m(&mut cam, &mut fake, this, 0),
            }
            encode(&cam, &mut lift);
            let map = [
                ("teardown", "teardown"),
                ("#2", "create_worker"),
                ("#3", "chain_set_flags"),
                ("release_worker", "release_worker"),
            ];
            let calls_ok = calls_agree(&rw_calls, &fake.log, &map);
            if lift != rw || !calls_ok {
                mismatches += 1;
            }
        }
        eprintln!("restart trials disagreeing: {mismatches} of {TRIALS}");
        mismatches
    }

    #[test]
    fn vf1_lifted_matches_rewrite() {
        assert_eq!(vf1_run(None), 0, "lifted restart disagrees");
    }

    #[test]
    fn vf1_wrong_versions_are_caught() {
        assert!(
            vf1_run(Some(mutant_vf1_no_release)) > 0,
            "the version without the release was not caught"
        );
        assert!(
            vf1_run(Some(mutant_vf1_swapped)) > 0,
            "the swapped-argument version was not caught"
        );
    }

    // ---------------------------------------------------------------------
    // Idle reset (slot 6).

    /// Wrong version: takes the first flag byte before the reset step runs,
    /// instead of reading it again after.
    fn mutant_vf6_stale_flags(
        cam: &mut CCamGame,
        world: &mut Fake,
        this: Handle32<CamTag>,
        _arg: u32,
    ) {
        cam.flags_1c5 &= 0xf1;
        cam.word_1e4 = 0;
        cam.word_1e8 = 0;
        cam.word_1e0 = 0;
        cam.mode_flag = 0;
        cam.mode = 3;
        cam.timer_flag = 0;
        cam.flags_1c4 &= 0xf8;
        cam.default_word = world.shared_default_word();
        cam.timer = 0x7530;
        let stale = cam.flags_1c4;
        world.reset_state_block(cam, this);
        cam.flags_1c5 &= 0xfe;
        cam.flags_1c4 = (stale & 0xc7) | IDLE_MARKER;
        cam.zoom_bits = DEFAULT_ZOOM.to_bits();
        cam.flag_1d8 = 0;
        cam.word_1d4 = 0;
        cam.word_154 = 0;
        cam.word_1dc = 0;
    }

    /// Wrong version: sets the active marker where the original sets the
    /// idle marker.
    fn mutant_vf6_active(cam: &mut CCamGame, world: &mut Fake, this: Handle32<CamTag>, _arg: u32) {
        cam.flags_1c5 &= 0xf1;
        cam.word_1e4 = 0;
        cam.word_1e8 = 0;
        cam.word_1e0 = 0;
        cam.mode_flag = 0;
        cam.mode = 3;
        cam.timer_flag = 0;
        cam.flags_1c4 &= 0xf8;
        cam.default_word = world.shared_default_word();
        cam.timer = 0x7530;
        world.reset_state_block(cam, this);
        let flags = cam.flags_1c4;
        cam.flags_1c5 &= 0xfe;
        cam.flags_1c4 = (flags & 0xc7) | ACTIVE_MARKER;
        cam.zoom_bits = DEFAULT_ZOOM.to_bits();
        cam.flag_1d8 = 0;
        cam.word_1d4 = 0;
        cam.word_154 = 0;
        cam.word_1dc = 0;
    }

    fn vf6_run(mutant: Option<Mutant>) -> u32 {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x0C_AA_0003);
        let mut mismatches = 0;
        for trial in 0..TRIALS {
            let before = random_object(&mut rng, trial);
            let mut effects = random_effects(&mut rng);
            // Half the trials also rewrite the first flag byte in the reset
            // step, so the re-read of that byte is exercised on every case.
            if trial % 2 == 0 {
                effects.push(Effect {
                    off: 0x1C4,
                    width: 1,
                    value: u32::from(rng.u8()),
                });
            }
            let default = if trial % 5 == 0 { 0 } else { rng.u32() };
            rt::set_case(&[(1, StubKind::Thiscall1Effect, vec![])], &effects, 0, default);
            let mut rw = before.clone();
            let addr = rw.as_mut_ptr() as usize as u32;
            let ret = rw_00b09cf0(addr);
            let rw_calls = rt::take_calls();
            assert_eq!(ret & 0xFF, 1, "the original always answers 1");
            assert_eq!(rw_calls.len(), 1, "the original's reset call");

            let mut lift = before.clone();
            let mut cam = decode(&before);
            let mut fake = Fake {
                default_word: default,
                effects: effects.clone(),
                ..Fake::default()
            };
            let this = Handle32::new(addr).expect("non-zero identity");
            match mutant {
                None => cam.reset_to_idle(&mut fake, this),
                Some(m) => m(&mut cam, &mut fake, this, 0),
            }
            encode(&cam, &mut lift);
            let map = [("#1", "reset_state_block")];
            let calls_ok = calls_agree(&rw_calls, &fake.log, &map);
            if lift != rw || !calls_ok {
                mismatches += 1;
            }
        }
        eprintln!("idle reset trials disagreeing: {mismatches} of {TRIALS}");
        mismatches
    }

    #[test]
    fn vf6_lifted_matches_rewrite() {
        assert_eq!(vf6_run(None), 0, "lifted idle reset disagrees");
    }

    #[test]
    fn vf6_wrong_versions_are_caught() {
        assert!(
            vf6_run(Some(mutant_vf6_stale_flags)) > 0,
            "the stale-flag version was not caught"
        );
        assert!(
            vf6_run(Some(mutant_vf6_active)) > 0,
            "the active-marker version was not caught"
        );
    }
    // ---------------------------------------------------------------------
    // Camera event step (slot 4).

    /// Size of a gate status object: enough for every offset the step reads.
    const GATE_LEN: usize = 0x32A0;

    /// Size of the tail object: enough for every offset the tail step touches.
    const TAIL_LEN: usize = 0x240;

    /// Draws a folded value that makes the pair high (above the threshold).
    fn xor_high(rng: &mut Rng) -> u8 {
        0x80 | (rng.u8() & 0x7F)
    }

    /// Draws a folded value that keeps the pair low (at or below the threshold).
    fn xor_low(rng: &mut Rng) -> u8 {
        rng.u8() & 0x7F
    }

    /// A gate status object: random bytes, with the selector nonzero half the
    /// time and each folded pair high or low, with the boundary value now and
    /// then, so both verdict paths and both pair tests are taken.
    fn gen_gate(rng: &mut Rng) -> Vec<u8> {
        let mut g: Vec<u8> = (0..GATE_LEN).map(|_| rng.u8()).collect();
        g[0x328D] = if rng.below(2) == 0 {
            0
        } else {
            (rng.below(255) + 1) as u8
        };
        let base = rng.u8();
        g[0x269C] = base;
        g[0x269E] = base ^ if rng.below(2) == 0 { xor_high(rng) } else { xor_low(rng) };
        g[0x269F] = base ^ if rng.below(2) == 0 { xor_high(rng) } else { xor_low(rng) };
        if rng.below(8) == 0 {
            g[0x269F] = base ^ 0x7F;
        }
        if rng.below(8) == 0 {
            g[0x269E] = base ^ 0x80;
        }
        let c0 = rng.u8();
        g[0x29BC] = c0;
        g[0x29BE] = c0 ^ if rng.below(2) == 0 { xor_high(rng) } else { xor_low(rng) };
        if rng.below(8) == 0 {
            g[0x29BE] = c0 ^ 0x7F;
        }
        g
    }

    /// A tail object: random bytes; the state is the matched state a third of
    /// the time, so both matched values are taken.
    fn gen_tail(rng: &mut Rng) -> Vec<u8> {
        let mut t: Vec<u8> = (0..TAIL_LEN).map(|_| rng.u8()).collect();
        if rng.below(3) == 0 {
            t[0x22E] = 3;
        }
        t
    }

    /// Renames the rewrite's numbered calls to the lifted names, and adds the
    /// step number the middle chain's callees are identified by.
    fn normalize_gate(calls: Vec<Call>) -> Vec<Call> {
        calls
            .into_iter()
            .map(|(name, mut args)| {
                let id = name.strip_prefix('#').and_then(|s| s.parse::<u32>().ok());
                let lifted = match id {
                    Some(1 | 13) => "gate_status",
                    Some(2 | 14) => "find_target",
                    Some(3 | 15) => "notify_sink",
                    Some(4) => "sync_target",
                    Some(5) => "singleton_present",
                    Some(k @ 6..=12) => {
                        args.push(k);
                        "middle_update"
                    }
                    Some(16) => "tail",
                    _ => return (name, args),
                };
                (lifted.to_string(), args)
            })
            .collect()
    }

    /// Deliberately wrong versions of the camera event step, built by wrapping
    /// the world: each changes one answer or one call the lifted step makes.
    #[derive(Debug, Clone, Copy)]
    enum GateMutant {
        /// The second block passes the first block's global word.
        SwapWords,
        /// The inhibit flag is read as clear.
        IgnoreInhibit,
        /// The middle chain runs its seven steps in reverse order.
        ReverseChain,
        /// The tail step never clears the busy bit.
        TailKeepsBusy,
        /// The selector byte is read inverted.
        InvertSelector,
    }

    struct Mutated {
        inner: Fake,
        kind: GateMutant,
    }

    impl CamGameWorld for Mutated {
        fn destroy_body(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.inner.destroy_body(cam, this);
        }
        fn pool_manager(&mut self) -> Option<Handle32<PoolTag>> {
            self.inner.pool_manager()
        }
        fn pool_release(&mut self, manager: Option<Handle32<PoolTag>>, this: Handle32<CamTag>) {
            self.inner.pool_release(manager, this);
        }
        fn teardown(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.inner.teardown(cam, this);
        }
        fn create_worker(
            &mut self,
            owner: Option<Handle32<CamOwnerTag>>,
            mode: u32,
            extra: u32,
            this: Handle32<CamTag>,
        ) -> Option<Handle32<WorkerTag>> {
            self.inner.create_worker(owner, mode, extra, this)
        }
        fn chain_set_flags(&mut self, worker: Option<Handle32<WorkerTag>>) {
            self.inner.chain_set_flags(worker);
        }
        fn release_worker(&mut self, worker: Option<Handle32<WorkerTag>>) {
            self.inner.release_worker(worker);
        }
        fn reset_state_block(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.inner.reset_state_block(cam, this);
        }
        fn shared_default_word(&mut self) -> u32 {
            self.inner.shared_default_word()
        }
        fn inhibit_flag(&mut self) -> u8 {
            match self.kind {
                GateMutant::IgnoreInhibit => 0,
                _ => self.inner.inhibit_flag(),
            }
        }
        fn gate_status(&mut self, block: GateBlock) -> Option<Handle32<GateTag>> {
            self.inner.gate_status(block)
        }
        fn gate_byte(&mut self, gate: Handle32<GateTag>, offset: u32) -> u8 {
            let v = self.inner.gate_byte(gate, offset);
            match self.kind {
                GateMutant::InvertSelector if offset == 0x328D => u8::from(v == 0),
                _ => v,
            }
        }
        fn find_target(
            &mut self,
            cam: &mut CCamGame,
            this: Handle32<CamTag>,
            block: GateBlock,
        ) -> Option<Handle32<TargetTag>> {
            self.inner.find_target(cam, this, block)
        }
        fn gate_word(&mut self, block: GateBlock) -> u16 {
            match (self.kind, block) {
                (GateMutant::SwapWords, GateBlock::Second) => self.inner.gate_word(GateBlock::First),
                _ => self.inner.gate_word(block),
            }
        }
        fn notify_sink(
            &mut self,
            cam: &mut CCamGame,
            this: Handle32<CamTag>,
            word: u16,
            block: GateBlock,
        ) {
            self.inner.notify_sink(cam, this, word, block);
        }
        fn sync_target(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) {
            self.inner.sync_target(cam, this);
        }
        fn singleton_present(&mut self) -> bool {
            self.inner.singleton_present()
        }
        fn middle_update(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>, step: u32) {
            let step = match self.kind {
                GateMutant::ReverseChain => 18 - step,
                _ => step,
            };
            self.inner.middle_update(cam, this, step);
        }
        fn tail(&mut self, cam: &mut CCamGame, this: Handle32<CamTag>) -> Option<Handle32<TailTag>> {
            self.inner.tail(cam, this)
        }
        fn tail_state(&mut self, tail: Handle32<TailTag>) -> u8 {
            self.inner.tail_state(tail)
        }
        fn tail_flags(&mut self, tail: Handle32<TailTag>) -> u8 {
            self.inner.tail_flags(tail)
        }
        fn set_tail_flags(&mut self, tail: Handle32<TailTag>, flags: u8) {
            if !matches!(self.kind, GateMutant::TailKeepsBusy) {
                self.inner.set_tail_flags(tail, flags);
            }
        }
        fn set_tail_state(&mut self, tail: Handle32<TailTag>, state: u8) {
            self.inner.set_tail_state(tail, state);
        }
        fn set_matched(&mut self, matched: u8) {
            self.inner.set_matched(matched);
        }
        fn clear_done(&mut self) {
            self.inner.clear_done();
        }
    }

    /// Runs the camera event step cases. Returns the trials that disagree
    /// (zero for the lifted step).
    fn gate_run(mutant: Option<GateMutant>) -> u32 {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x0C_AA_0004);
        let mut mismatches = 0;
        for trial in 0..TRIALS {
            let gate_a_buf = gen_gate(&mut rng);
            let gate_b_buf = gen_gate(&mut rng);
            let tail_rw = gen_tail(&mut rng);
            let gate_a_addr = gate_a_buf.as_ptr() as usize as u32;
            let gate_b_addr = gate_b_buf.as_ptr() as usize as u32;
            let tail_addr = tail_rw.as_ptr() as usize as u32;

            let zero_gate_a = rng.below(8) == 0;
            let zero_gate_b = rng.below(8) == 0;
            let gate_a = if zero_gate_a { 0 } else { gate_a_addr };
            let gate_b = if zero_gate_b { 0 } else { gate_b_addr };
            let targets = [
                if rng.below(3) == 0 { 0 } else { 0x5000_0001 + trial },
                if rng.below(3) == 0 { 0 } else { 0x6000_0001 + trial },
            ];
            let singleton = rng.below(2) == 1;
            let tail = if rng.below(8) == 0 { 0 } else { tail_addr };
            let words = [rng.u32() as u16, rng.u32() as u16];
            let inhibit = if rng.below(2) == 0 { 0 } else { 1 };
            let matched_init = rng.u8();
            let done_init = rng.u8();
            let effects = random_effects(&mut rng);

            let cam_rw = random_object(&mut rng, trial);
            let cam_addr = cam_rw.as_ptr() as usize as u32;
            let cam_before = cam_rw.clone();
            // The lifted side starts from the images as they were before the
            // original ran; the original writes the tail object.
            let tail_before = tail_rw.clone();

            rt::set_case(
                &[
                    (1, StubKind::Cdecl2, vec![gate_a]),
                    (2, StubKind::Thiscall3, vec![targets[0]]),
                    (3, StubKind::Thiscall5, vec![]),
                    (4, StubKind::Thiscall1Effect, vec![]),
                    (5, StubKind::Cdecl0, vec![u32::from(singleton)]),
                    (6, StubKind::Thiscall1Effect, vec![]),
                    (7, StubKind::Thiscall1Effect, vec![]),
                    (8, StubKind::Thiscall1Effect, vec![]),
                    (9, StubKind::Thiscall1Effect, vec![]),
                    (10, StubKind::Thiscall1Effect, vec![]),
                    (11, StubKind::Thiscall1Effect, vec![]),
                    (12, StubKind::Thiscall1Effect, vec![]),
                    (13, StubKind::Cdecl2, vec![gate_b]),
                    (14, StubKind::Thiscall3, vec![targets[1]]),
                    (15, StubKind::Thiscall5, vec![]),
                    (16, StubKind::Thiscall3, vec![tail]),
                ],
                &effects,
                0,
                0,
            );
            rt::set_gate_globals(inhibit, words[0], words[1]);
            rt::set_gate_outputs(matched_init, done_init);
            let ret = rw_00b07760(cam_addr);
            let rw_calls = normalize_gate(rt::take_calls());
            assert_eq!(ret & 0xFF, 1, "the original always answers 1");
            let matched_rw = rt::gate_matched();
            let done_rw = rt::gate_done();

            let mut fake = Fake {
                inhibit,
                gate_a: (!zero_gate_a).then_some(gate_a_addr),
                gate_b: (!zero_gate_b).then_some(gate_b_addr),
                targets: [
                    (targets[0] != 0).then_some(targets[0]),
                    (targets[1] != 0).then_some(targets[1]),
                ],
                words,
                singleton,
                tail: (tail != 0).then_some(tail_addr),
                images: vec![
                    (gate_a_addr, gate_a_buf.clone()),
                    (gate_b_addr, gate_b_buf.clone()),
                    (tail_addr, tail_before),
                ],
                effects: effects.clone(),
                ..Fake::default()
            };
            let mut cam = decode(&cam_before);
            let this = Handle32::new(cam_addr).expect("non-zero identity");
            match mutant {
                None => cam.gate_update(&mut fake, this),
                Some(kind) => {
                    let mut m = Mutated { inner: fake, kind };
                    cam.gate_update(&mut m, this);
                    fake = m.inner;
                }
            }
            let mut cam_lift = cam_before.clone();
            encode(&cam, &mut cam_lift);

            let matched_expected = fake.matched.unwrap_or(matched_init);
            let tail_bytes_ok = tail == 0 || fake.images[2].1 == tail_rw;
            let ok = cam_lift == cam_rw
                && rw_calls == fake.log
                && matched_rw == matched_expected
                && done_rw == 0
                && fake.done == Some(0)
                && tail_bytes_ok;
            if !ok {
                mismatches += 1;
            }
        }
        eprintln!("camera event step trials disagreeing: {mismatches} of {TRIALS}");
        mismatches
    }

    #[test]
    fn vf4_lifted_matches_rewrite() {
        assert_eq!(gate_run(None), 0, "lifted camera event step disagrees");
    }

    #[test]
    fn vf4_wrong_versions_are_caught() {
        for kind in [
            GateMutant::SwapWords,
            GateMutant::IgnoreInhibit,
            GateMutant::ReverseChain,
            GateMutant::TailKeepsBusy,
            GateMutant::InvertSelector,
        ] {
            assert!(gate_run(Some(kind)) > 0, "{kind:?} was not caught");
        }
    }
__UPDATE_TESTS__
