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
    use lf_camgame_diff::rewrites::fn_00b09cf0::rw_00b09cf0;
    use lf_camgame_diff::rewrites::fn_00b09da0::rw_00b09da0;
    use lf_camgame_diff::rt::{self, Call, Effect, SNAPSHOT, StubKind};
    use lf_core::Handle32;
    use lf_world::cam_game::{
        ACTIVE_MARKER, CCamGame, CamGameWorld, CamOwnerTag, CamTag, DEFAULT_ZOOM, IDLE_MARKER,
        PoolTag, WorkerTag,
    };

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
}
