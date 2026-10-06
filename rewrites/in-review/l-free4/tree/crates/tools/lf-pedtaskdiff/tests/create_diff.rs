//! Differential cases, part 2: task creation.
//!
//! Each case builds real 32-bit objects, runs the rewrite and the lifted
//! method on the same inputs, and compares returns and every effect
//! (written bytes and callee call logs). Each method has deliberately
//! wrong lifts that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_core::Handle32;
    use lf_peds_tasks::ped_task::{
        ArgLookup, BuildManagers, ChainClone, ChainCloner, ChainEntry, ChainOwner, CloneProduct,
        FoundEntry, Kind11Task, PedMgr, PedTaskOutcome, TaskBuildCtx, FLAG_DONE, FLAG_EXTRA,
        KIND_11, NONE, PRIORITY,
    };
    use lf_pedtaskdiff::rewrites::*;
    use lf_pedtaskdiff::{set_callee, set_relocated};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{LOOKUP_VA, PEDMGR_VA, Rng, get_u32, heap_addr, lock, put_u32};

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_core::Handle32;
        use lf_peds_tasks::ped_task::{
            BuildManagers, ChainClone, ChainCloner, Kind11Task, PedTaskOutcome, TaskBuildCtx,
            FLAG_EXTRA, KIND_11,
        };

        /// The builder that ignores a kept task and always builds.
        pub fn build_always<C: TaskBuildCtx, const N: usize>(
            mgrs: &BuildManagers,
            ctx: &mut C,
            handle: u32,
            arg1: u32,
            words: [u32; N],
        ) -> Option<Handle32<Kind11Task>> {
            if handle != 0 {
                match ctx.ped_task(mgrs.ped_mgr, handle) {
                    PedTaskOutcome::NullPed => panic!("null ped lookup faults in the original"),
                    PedTaskOutcome::Build | PedTaskOutcome::Keep(_) => {}
                }
            }
            let found = ctx.resolve_arg(mgrs.lookup, arg1);
            ctx.build_kind11(handle, found, &words, KIND_11)
        }

        /// The cloner that forgets the extra flag bits on maker calls.
        pub fn clone_raw_flags<C: ChainClone, const SET_DONE: bool>(
            cloner: &ChainCloner,
            ctx: &mut C,
            key: u32,
        ) {
            let mut node = ctx.find_first(key);
            while let Some(found) = node {
                let e = &found.entry;
                // Wrong: raw flags instead of `flags | FLAG_EXTRA`.
                let flags = e.flags;
                let product = if e.aux == 0 {
                    ctx.make_full(cloner.owner, e.y, e.x, flags, e.aux, super::PRIORITY, super::NONE)
                } else if e.y == super::NONE {
                    ctx.make_alt(cloner.owner, e.alt_a, e.alt_b, flags, e.aux, super::PRIORITY)
                } else if e.x == super::NONE {
                    ctx.make_alt(cloner.owner, e.alt_a, e.alt_b, flags, e.aux, super::PRIORITY)
                } else {
                    ctx.make_full(cloner.owner, e.y, e.x, flags, e.aux, super::PRIORITY, super::NONE)
                };
                if let Some(p) = product {
                    ctx.set_first(p, e.f1);
                    ctx.store_second(p, e.f2);
                    ctx.set_third(p, e.f3);
                    if SET_DONE {
                        ctx.mark_done(p);
                    }
                }
                node = ctx.find_next(key);
            }
        }

        /// The cloner with the done-flag write flipped.
        pub fn clone_flipped_done<C: ChainClone, const SET_DONE: bool>(
            cloner: &ChainCloner,
            ctx: &mut C,
            key: u32,
        ) {
            let mut node = ctx.find_first(key);
            while let Some(found) = node {
                let e = &found.entry;
                let flags = e.flags | FLAG_EXTRA;
                let product = if e.aux == 0 {
                    ctx.make_full(cloner.owner, e.y, e.x, flags, e.aux, super::PRIORITY, super::NONE)
                } else if e.y == super::NONE {
                    ctx.make_alt(cloner.owner, e.alt_a, e.alt_b, flags, e.aux, super::PRIORITY)
                } else if e.x == super::NONE {
                    ctx.make_alt(cloner.owner, e.alt_a, e.alt_b, flags, e.aux, super::PRIORITY)
                } else {
                    ctx.make_full(cloner.owner, e.y, e.x, flags, e.aux, super::PRIORITY, super::NONE)
                };
                if let Some(p) = product {
                    ctx.set_first(p, e.f1);
                    ctx.store_second(p, e.f2);
                    ctx.set_third(p, e.f3);
                    // Wrong: flipped.
                    if !SET_DONE {
                        ctx.mark_done(p);
                    }
                }
                node = ctx.find_next(key);
            }
        }
    }

    // Recording stubs for the rewrite side.

    static MGR_ADDR: Mutex<u32> = Mutex::new(0);
    static LOOKUP_ADDR: Mutex<u32> = Mutex::new(0);
    static LOOKUP_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static LOOKUP_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "thiscall" fn lookup_stub(this_arg: u32, key: u32) -> u32 {
        let mgr = *MGR_ADDR.lock().unwrap();
        let lookup = *LOOKUP_ADDR.lock().unwrap();
        assert!(
            this_arg == mgr || this_arg == lookup,
            "lookup this {this_arg:#x} not a planted manager"
        );
        LOOKUP_LOG.lock().unwrap().push((this_arg, key));
        LOOKUP_SCRIPT.lock().unwrap().pop_front().unwrap()
    }

    static BUILD_LOG: Mutex<Vec<Vec<u32>>> = Mutex::new(Vec::new());
    static BUILD_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "cdecl" fn build4_stub(h: u32, f: u32, w2: u32, kind: u32) -> u32 {
        assert_eq!(kind, KIND_11, "build kind");
        BUILD_LOG.lock().unwrap().push(vec![h, f, w2, kind]);
        BUILD_SCRIPT.lock().unwrap().pop_front().unwrap()
    }
    extern "cdecl" fn build5_stub(h: u32, f: u32, w2: u32, w3: u32, kind: u32) -> u32 {
        assert_eq!(kind, KIND_11, "build kind");
        BUILD_LOG.lock().unwrap().push(vec![h, f, w2, w3, kind]);
        BUILD_SCRIPT.lock().unwrap().pop_front().unwrap()
    }

    /// One stub call, in order, mirroring the lift's clone calls minus
    /// the direct stores (proven against the product images instead).
    #[derive(Debug, Clone, PartialEq)]
    enum StubEvent {
        FindFirst(u32),
        FindNext(u32),
        MakeFull(u32, u32, u32, u32, u32, u32, u32),
        MakeAlt(u32, u32, u32, u32, u32, u32),
        SetFirst(u32, u32),
        SetThird(u32, u32),
    }

    static STUB_SEQ: Mutex<Vec<StubEvent>> = Mutex::new(Vec::new());
    static FIND_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static FIND_FIRST_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    static FIND_NEXT_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "thiscall" fn find_first_stub(key: u32) -> u32 {
        FIND_LOG.lock().unwrap().push((1, key));
        STUB_SEQ.lock().unwrap().push(StubEvent::FindFirst(key));
        FIND_FIRST_SCRIPT.lock().unwrap().pop_front().unwrap()
    }
    extern "thiscall" fn find_next_stub(key: u32) -> u32 {
        FIND_LOG.lock().unwrap().push((6, key));
        STUB_SEQ.lock().unwrap().push(StubEvent::FindNext(key));
        FIND_NEXT_SCRIPT.lock().unwrap().pop_front().unwrap()
    }

    static OWNER_ADDR: Mutex<u32> = Mutex::new(0);
    static MAKE_LOG: Mutex<Vec<(u32, Vec<u32>)>> = Mutex::new(Vec::new());
    static MAKE_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "thiscall" fn make_full_stub(
        owner: u32,
        y: u32,
        x: u32,
        flags: u32,
        aux: u32,
        prio: u32,
        none: u32,
    ) -> u32 {
        assert_eq!(owner, *OWNER_ADDR.lock().unwrap(), "maker owner");
        assert_eq!(prio, PRIORITY, "maker priority");
        assert_eq!(none, NONE, "maker none");
        MAKE_LOG
            .lock()
            .unwrap()
            .push((2, vec![owner, y, x, flags, aux, prio, none]));
        STUB_SEQ
            .lock()
            .unwrap()
            .push(StubEvent::MakeFull(owner, y, x, flags, aux, prio, none));
        MAKE_SCRIPT.lock().unwrap().pop_front().unwrap()
    }
    extern "thiscall" fn make_alt_stub(
        owner: u32,
        a: u32,
        b: u32,
        flags: u32,
        aux: u32,
        prio: u32,
    ) -> u32 {
        assert_eq!(owner, *OWNER_ADDR.lock().unwrap(), "maker owner");
        assert_eq!(prio, PRIORITY, "maker priority");
        MAKE_LOG
            .lock()
            .unwrap()
            .push((3, vec![owner, a, b, flags, aux, prio]));
        STUB_SEQ
            .lock()
            .unwrap()
            .push(StubEvent::MakeAlt(owner, a, b, flags, aux, prio));
        MAKE_SCRIPT.lock().unwrap().pop_front().unwrap()
    }

    static SET_LOG: Mutex<Vec<(u32, u32, u32)>> = Mutex::new(Vec::new());
    extern "thiscall" fn set_f1_stub(product: u32, f1: u32) -> u32 {
        SET_LOG.lock().unwrap().push((4, product, f1));
        STUB_SEQ.lock().unwrap().push(StubEvent::SetFirst(product, f1));
        0
    }
    extern "thiscall" fn set_f3_stub(product: u32, f3: u32) -> u32 {
        SET_LOG.lock().unwrap().push((5, product, f3));
        STUB_SEQ.lock().unwrap().push(StubEvent::SetThird(product, f3));
        0
    }

    fn clear_logs() {
        LOOKUP_LOG.lock().unwrap().clear();
        LOOKUP_SCRIPT.lock().unwrap().clear();
        BUILD_LOG.lock().unwrap().clear();
        BUILD_SCRIPT.lock().unwrap().clear();
        FIND_LOG.lock().unwrap().clear();
        FIND_FIRST_SCRIPT.lock().unwrap().clear();
        FIND_NEXT_SCRIPT.lock().unwrap().clear();
        MAKE_LOG.lock().unwrap().clear();
        MAKE_SCRIPT.lock().unwrap().clear();
        SET_LOG.lock().unwrap().clear();
        STUB_SEQ.lock().unwrap().clear();
    }

    // Lift-side fakes.

    #[derive(Debug, PartialEq)]
    enum BuildCall {
        Ped(u32, u32),
        Resolve(u32, u32),
        Build(u32, u32, Vec<u32>, u32),
    }

    struct FakeBuild {
        calls: Vec<BuildCall>,
        ped: VecDeque<PedTaskOutcome>,
        found: VecDeque<u32>,
        built: VecDeque<Option<Handle32<Kind11Task>>>,
    }

    impl TaskBuildCtx for FakeBuild {
        fn ped_task(&mut self, mgr: Option<Handle32<PedMgr>>, handle: u32) -> PedTaskOutcome {
            self.calls.push(BuildCall::Ped(Handle32::raw_or_zero(mgr), handle));
            self.ped.pop_front().unwrap()
        }
        fn resolve_arg(&mut self, lookup: Option<Handle32<ArgLookup>>, arg1: u32) -> u32 {
            self.calls
                .push(BuildCall::Resolve(Handle32::raw_or_zero(lookup), arg1));
            self.found.pop_front().unwrap()
        }
        fn build_kind11(
            &mut self,
            handle: u32,
            found: u32,
            words: &[u32],
            kind: u32,
        ) -> Option<Handle32<Kind11Task>> {
            self.calls.push(BuildCall::Build(handle, found, words.to_vec(), kind));
            self.built.pop_front().unwrap()
        }
    }

    #[derive(Debug, PartialEq)]
    enum CloneCall {
        FindFirst(u32),
        FindNext(u32),
        MakeFull(u32, u32, u32, u32, u32, u32, u32),
        MakeAlt(u32, u32, u32, u32, u32, u32),
        SetFirst(u32, u32),
        StoreSecond(u32, u32),
        SetThird(u32, u32),
        MarkDone(u32),
    }

    struct FakeClone {
        calls: Vec<CloneCall>,
        finds: VecDeque<Option<FoundEntry>>,
        products: VecDeque<Option<Handle32<CloneProduct>>>,
    }

    impl FakeClone {
        fn next_find(&mut self, first: bool, key: u32) -> Option<FoundEntry> {
            self.calls.push(if first {
                CloneCall::FindFirst(key)
            } else {
                CloneCall::FindNext(key)
            });
            self.finds.pop_front().unwrap()
        }
    }

    impl ChainClone for FakeClone {
        fn find_first(&mut self, key: u32) -> Option<FoundEntry> {
            self.next_find(true, key)
        }
        fn find_next(&mut self, key: u32) -> Option<FoundEntry> {
            self.next_find(false, key)
        }
        fn make_full(
            &mut self,
            owner: Option<Handle32<ChainOwner>>,
            y: u32,
            x: u32,
            flags: u32,
            aux: u32,
            priority: u32,
            none: u32,
        ) -> Option<Handle32<CloneProduct>> {
            self.calls.push(CloneCall::MakeFull(
                Handle32::raw_or_zero(owner),
                y,
                x,
                flags,
                aux,
                priority,
                none,
            ));
            self.products.pop_front().unwrap()
        }
        fn make_alt(
            &mut self,
            owner: Option<Handle32<ChainOwner>>,
            a: u32,
            b: u32,
            flags: u32,
            aux: u32,
            priority: u32,
        ) -> Option<Handle32<CloneProduct>> {
            self.calls.push(CloneCall::MakeAlt(
                Handle32::raw_or_zero(owner),
                a,
                b,
                flags,
                aux,
                priority,
            ));
            self.products.pop_front().unwrap()
        }
        fn set_first(&mut self, product: Handle32<CloneProduct>, f1: u32) {
            self.calls.push(CloneCall::SetFirst(product.get(), f1));
        }
        fn store_second(&mut self, product: Handle32<CloneProduct>, f2: u32) {
            self.calls.push(CloneCall::StoreSecond(product.get(), f2));
        }
        fn set_third(&mut self, product: Handle32<CloneProduct>, f3: u32) {
            self.calls.push(CloneCall::SetThird(product.get(), f3));
        }
        fn mark_done(&mut self, product: Handle32<CloneProduct>) {
            self.calls.push(CloneCall::MarkDone(product.get()));
        }
    }

    /// One build case's script.
    struct BuildCase {
        handle: u32,
        arg1: u32,
        w2: u32,
        w3: u32,
        task_addr: u32,
        task_flag: u8,
        found: u32,
        built: u32,
    }

    fn run_build<const N: usize>(
        mgr: u32,
        lookup: u32,
        bc: &BuildCase,
        case: u32,
    ) -> (u32, Vec<(u32, u32)>, Vec<Vec<u32>>) {
        clear_logs();

        // Ped image with random fill: task pointer at +0x6C.
        let mut ped_img = Box::new([0u8; 0x80]);
        for (i, b) in ped_img.iter_mut().enumerate() {
            *b = (bc.handle.wrapping_add(i as u32 * 31) & 0xFF) as u8;
        }
        put_u32(&mut ped_img[..], 0x6C, bc.task_addr);
        let ped_addr = heap_addr(&ped_img);
        // Task image with random fill: flag byte at +0x0E.
        let mut task_img = Box::new([0u8; 0x20]);
        for (i, b) in task_img.iter_mut().enumerate() {
            *b = (bc.task_flag.wrapping_add(i as u8 * 17)) as u8;
        }
        task_img[0x0E] = bc.task_flag;
        let real_task = heap_addr(&task_img);
        if bc.task_addr != 0 {
            put_u32(&mut ped_img[..], 0x6C, real_task);
        }

        LOOKUP_SCRIPT.lock().unwrap().push_back(ped_addr);
        LOOKUP_SCRIPT.lock().unwrap().push_back(bc.found);
        BUILD_SCRIPT.lock().unwrap().push_back(bc.built);

        let rw_answer = if N == 1 {
            unsafe { fn_00BC00C0::rw_00bc00c0(bc.handle, bc.arg1, bc.w2) }
        } else {
            unsafe { fn_00BC0110::rw_00bc0110(bc.handle, bc.arg1, bc.w2, bc.w3) }
        };

        let outcome = if bc.task_addr == 0 || bc.task_flag == 0 {
            PedTaskOutcome::Build
        } else {
            PedTaskOutcome::Keep(Handle32::new(real_task).unwrap())
        };
        let mgrs = BuildManagers::new(Handle32::new(mgr), Handle32::new(lookup));
        let mut fake = FakeBuild {
            calls: Vec::new(),
            ped: VecDeque::from([outcome]),
            found: VecDeque::from([bc.found]),
            built: VecDeque::from([Handle32::new(bc.built)]),
        };
        let got = if N == 1 {
            mgrs.build_task(&mut fake, bc.handle, bc.arg1, [bc.w2])
        } else {
            mgrs.build_task(&mut fake, bc.handle, bc.arg1, [bc.w2, bc.w3])
        };
        assert_eq!(Handle32::raw_or_zero(got), rw_answer, "case {case}: answer");

        // Rewrite log (grouped by stub) against lift log (in order).
        let rw_lookup = LOOKUP_LOG.lock().unwrap().clone();
        let rw_build = BUILD_LOG.lock().unwrap().clone();
        let want = want_calls::<N>(mgr, &rw_lookup, &rw_build);
        assert_eq!(fake.calls, want, "case {case}: calls");
        (rw_answer, rw_lookup, rw_build)
    }

    /// Lifts stub logs into lift-shaped calls for comparison.
    fn want_calls<const N: usize>(
        mgr: u32,
        rw_lookup: &[(u32, u32)],
        rw_build: &[Vec<u32>],
    ) -> Vec<BuildCall> {
        let mut want = Vec::new();
        for (this_arg, key) in rw_lookup.iter() {
            if *this_arg == mgr {
                want.push(BuildCall::Ped(*this_arg, *key));
            } else {
                want.push(BuildCall::Resolve(*this_arg, *key));
            }
        }
        for words in rw_build.iter() {
            if N == 1 {
                want.push(BuildCall::Build(words[0], words[1], vec![words[2]], words[3]));
            } else {
                want.push(BuildCall::Build(
                    words[0],
                    words[1],
                    vec![words[2], words[3]],
                    words[4],
                ));
            }
        }
        want
    }

    #[test]
    fn build_matches_rewrite() {
        let _guard = lock();
        set_callee(1, lookup_stub as usize as u32);
        // Managers planted once: identities shared by every case and by
        // the mutant runs, so call logs compare directly.
        let mgr_img = Box::new([0x51u8; 16]);
        let lookup_img = Box::new([0x52u8; 16]);
        let mgr = heap_addr(&mgr_img);
        let lookup = heap_addr(&lookup_img);
        *MGR_ADDR.lock().unwrap() = mgr;
        *LOOKUP_ADDR.lock().unwrap() = lookup;
        let mgr_word = Box::new(mgr);
        let lookup_word = Box::new(lookup);
        set_relocated(PEDMGR_VA, heap_addr(&mgr_word));
        set_relocated(LOOKUP_VA, heap_addr(&lookup_word));

        let mut rng = Rng(0xB011_D5);
        let mut compared = 0u32;
        let mut caught4 = 0u32;
        let mut caught5 = 0u32;
        for case in 0..120 {
            let shape = case % 5;
            let handle = if shape == 0 { 0 } else { rng.edge_word() | 1 };
            let task_null = shape == 1;
            let flag_set = shape == 3;
            let bc = BuildCase {
                handle,
                arg1: rng.edge_word(),
                w2: rng.edge_word(),
                w3: rng.edge_word(),
                task_addr: if task_null || shape == 0 { 0 } else { 1 },
                task_flag: if flag_set { (rng.u32() as u8) | 1 } else { 0 },
                found: rng.edge_word(),
                built: if case % 7 == 0 { 0 } else { rng.edge_word() | 0x1000 },
            };
            // Instance one: the four-word build.
            set_callee(2, build4_stub as usize as u32);
            let (rw4, rw_lookup4, rw_build4) = run_build::<1>(mgr, lookup, &bc, case);
            compared += 1;
            // Instance two: the five-word build.
            set_callee(2, build5_stub as usize as u32);
            let (rw5, rw_lookup5, rw_build5) = run_build::<2>(mgr, lookup, &bc, case);
            compared += 1;

            // The wrong lift ignores kept tasks: compare its answers
            // and calls against the rewrite's, as for the real lift.
            let task_img = Box::new([0u8; 0x20]);
            let real_task = heap_addr(&task_img);
            let outcome = if bc.task_addr == 0 || bc.task_flag == 0 {
                PedTaskOutcome::Build
            } else {
                PedTaskOutcome::Keep(Handle32::new(real_task).unwrap())
            };
            let mgrs = BuildManagers::new(Handle32::new(mgr), Handle32::new(lookup));
            for (n, rw_answer, rw_lookup, rw_build, caught) in [
                (1, rw4, &rw_lookup4, &rw_build4, &mut caught4),
                (2, rw5, &rw_lookup5, &rw_build5, &mut caught5),
            ] {
                let mut fake = FakeBuild {
                    calls: Vec::new(),
                    ped: VecDeque::from([outcome]),
                    found: VecDeque::from([bc.found]),
                    built: VecDeque::from([Handle32::new(bc.built)]),
                };
                let bad = if n == 1 {
                    wrong::build_always(&mgrs, &mut fake, bc.handle, bc.arg1, [bc.w2])
                } else {
                    wrong::build_always(&mgrs, &mut fake, bc.handle, bc.arg1, [bc.w2, bc.w3])
                };
                let want = if n == 1 {
                    want_calls::<1>(mgr, rw_lookup, rw_build)
                } else {
                    want_calls::<2>(mgr, rw_lookup, rw_build)
                };
                if Handle32::raw_or_zero(bad) != rw_answer || fake.calls != want {
                    *caught += 1;
                }
            }
        }
        assert_eq!(compared, 240);
        assert!(caught4 > 0, "build4 mutant was never caught");
        assert!(caught5 > 0, "build5 mutant was never caught");
    }

    /// One clone case's script: entries and per-entry null products.
    struct CloneScript {
        entries: Vec<ChainEntry>,
        null_product: Vec<bool>,
    }

    /// Plants one entry image: words at their documented offsets over a
    /// deterministic fill, so a wrong offset reads back garbage.
    fn plant_entry(e: &ChainEntry, seed: u32) -> Box<[u8; 0x60]> {
        let mut img = Box::new([0u8; 0x60]);
        for (i, b) in img.iter_mut().enumerate() {
            *b = seed.wrapping_add(i as u32 * 37).to_le_bytes()[0];
        }
        put_u32(&mut img[..], 0x04, e.flags);
        put_u32(&mut img[..], 0x08, e.aux);
        put_u32(&mut img[..], 0x0C, e.x);
        put_u32(&mut img[..], 0x10, e.y);
        put_u32(&mut img[..], 0x14, e.alt_a);
        put_u32(&mut img[..], 0x18, e.alt_b);
        put_u32(&mut img[..], 0x4C, e.f1);
        put_u32(&mut img[..], 0x54, e.f2);
        put_u32(&mut img[..], 0x58, e.f3);
        img
    }

    fn plant_product(seed: u32) -> (Box<[u8; 0x60]>, u32) {
        let mut img = Box::new([0u8; 0x60]);
        for (i, b) in img.iter_mut().enumerate() {
            *b = seed.wrapping_add(i as u32 * 53).to_le_bytes()[1];
        }
        let orig04 = get_u32(&img[..], 0x04);
        (img, orig04)
    }

    fn run_clone<const SET_DONE: bool>(
        owner: u32,
        script: &CloneScript,
        key: u32,
        case: u32,
    ) -> (Vec<StubEvent>, Vec<CloneCall>, Vec<(u32, u32, u32)>) {
        // Returns (stub events, lift calls, per-product (addr, orig04, f2)).
        clear_logs();
        let mut this_img = Box::new([0u8; 0x80]);
        put_u32(&mut this_img[..], 0x78, owner);
        let this_addr = heap_addr(&this_img);

        let mut node_imgs = Vec::new();
        let mut node_addrs = Vec::new();
        for (i, e) in script.entries.iter().enumerate() {
            let img = plant_entry(e, 0xE000 + case * 16 + i as u32);
            node_addrs.push(heap_addr(&img));
            node_imgs.push(img);
        }
        let mut prod_imgs = Vec::new();
        let mut prod_addrs = Vec::new();
        let mut prod_orig = Vec::new();
        for (i, null) in script.null_product.iter().enumerate() {
            if *null {
                prod_addrs.push(0);
                prod_orig.push(0);
            } else {
                let (img, orig) = plant_product(0x9000 + case * 16 + i as u32);
                prod_addrs.push(heap_addr(&img));
                prod_orig.push(orig);
                prod_imgs.push(img);
            }
        }

        if node_addrs.is_empty() {
            FIND_FIRST_SCRIPT.lock().unwrap().push_back(0);
        } else {
            FIND_FIRST_SCRIPT.lock().unwrap().push_back(node_addrs[0]);
            for a in node_addrs.iter().skip(1) {
                FIND_NEXT_SCRIPT.lock().unwrap().push_back(*a);
            }
            FIND_NEXT_SCRIPT.lock().unwrap().push_back(0);
        }
        for a in prod_addrs.iter() {
            MAKE_SCRIPT.lock().unwrap().push_back(*a);
        }

        let rw_answer = if SET_DONE {
            unsafe { fn_00B4E640::rw_00b4e640(this_addr, key) }
        } else {
            unsafe { fn_00B4E750::rw_00b4e750(this_addr, key) }
        };
        assert_eq!(rw_answer, 0, "case {case}: answer");

        // Lift side: handles wrap the planted addresses.
        let mut finds = VecDeque::new();
        for (e, a) in script.entries.iter().zip(node_addrs.iter()) {
            finds.push_back(Some(FoundEntry {
                node: Handle32::new(*a).unwrap(),
                entry: *e,
            }));
        }
        finds.push_back(None);
        let mut products = VecDeque::new();
        for a in prod_addrs.iter() {
            products.push_back(Handle32::new(*a));
        }
        let cloner = ChainCloner::new(Handle32::new(owner));
        let mut fake = FakeClone { calls: Vec::new(), finds, products };
        cloner.clone_chain::<FakeClone, SET_DONE>(&mut fake, key);

        // Stub events against lift calls (minus the direct stores).
        let stub_seq = STUB_SEQ.lock().unwrap().clone();
        let mut lift_seq = Vec::new();
        for c in fake.calls.iter() {
            match c {
                CloneCall::FindFirst(k) => lift_seq.push(StubEvent::FindFirst(*k)),
                CloneCall::FindNext(k) => lift_seq.push(StubEvent::FindNext(*k)),
                CloneCall::MakeFull(o, y, x, f, a, p, n) => {
                    lift_seq.push(StubEvent::MakeFull(*o, *y, *x, *f, *a, *p, *n));
                }
                CloneCall::MakeAlt(o, a, b, f, au, p) => {
                    lift_seq.push(StubEvent::MakeAlt(*o, *a, *b, *f, *au, *p));
                }
                CloneCall::SetFirst(p, v) => lift_seq.push(StubEvent::SetFirst(*p, *v)),
                CloneCall::SetThird(p, v) => lift_seq.push(StubEvent::SetThird(*p, *v)),
                CloneCall::StoreSecond(..) | CloneCall::MarkDone(..) => {}
            }
        }
        assert_eq!(lift_seq, stub_seq, "case {case}: call order and arguments");

        // Direct stores against the product images.
        let mut prods = Vec::new();
        for (i, e) in script.entries.iter().enumerate() {
            let p = prod_addrs[i];
            if p == 0 {
                continue;
            }
            let img = prod_imgs.iter().find(|im| heap_addr(im) == p).unwrap();
            let w54 = get_u32(&img[..], 0x54);
            let w04 = get_u32(&img[..], 0x04);
            assert!(
                fake.calls.contains(&CloneCall::StoreSecond(p, e.f2)),
                "case {case}: store_second logged"
            );
            assert_eq!(w54, e.f2, "case {case}: product f2 store");
            if SET_DONE {
                assert!(
                    fake.calls.contains(&CloneCall::MarkDone(p)),
                    "case {case}: mark_done logged"
                );
                assert_eq!(w04, prod_orig[i] | FLAG_DONE, "case {case}: done or-ing");
            } else {
                assert_eq!(w04, prod_orig[i], "case {case}: flag word untouched");
            }
            prods.push((p, prod_orig[i], e.f2));
        }
        if !SET_DONE {
            assert!(
                !fake.calls.iter().any(|c| matches!(c, CloneCall::MarkDone(_))),
                "case {case}: never marks"
            );
        }
        (stub_seq, fake.calls, prods)
    }

    fn entry_of(
        flags: u32,
        aux: u32,
        x: u32,
        y: u32,
        alt_a: u32,
        alt_b: u32,
        f1: u32,
        f2: u32,
        f3: u32,
    ) -> ChainEntry {
        ChainEntry { flags, aux, x, y, alt_a, alt_b, f1, f2, f3 }
    }

    #[test]
    fn clone_matches_rewrite() {
        let _guard = lock();
        set_callee(1, find_first_stub as usize as u32);
        set_callee(2, make_full_stub as usize as u32);
        set_callee(3, make_alt_stub as usize as u32);
        set_callee(4, set_f1_stub as usize as u32);
        set_callee(5, set_f3_stub as usize as u32);
        set_callee(6, find_next_stub as usize as u32);
        let owner_img = Box::new([0x77u8; 16]);
        let owner = heap_addr(&owner_img);
        *OWNER_ADDR.lock().unwrap() = owner;

        let mut rng = Rng(0xC10E_5);
        let mut compared = 0u32;
        let mut caught_raw_a = 0u32;
        let mut caught_done_a = 0u32;
        let mut caught_raw_b = 0u32;
        let mut caught_done_b = 0u32;
        for case in 0..96 {
            let shape = case % 8;
            let key = rng.edge_word();
            let script = match shape {
                0 => CloneScript { entries: vec![], null_product: vec![] },
                1 => CloneScript {
                    entries: vec![entry_of(1, 0, 11, 12, 13, 14, 21, 22, 23)],
                    null_product: vec![false],
                },
                2 => CloneScript {
                    entries: vec![entry_of(2, 7, 11, NONE, 13, 14, 21, 22, 23)],
                    null_product: vec![false],
                },
                3 => CloneScript {
                    entries: vec![entry_of(3, 7, NONE, 12, 13, 14, 21, 22, 23)],
                    null_product: vec![false],
                },
                4 => CloneScript {
                    entries: vec![entry_of(4, 7, 11, 12, 13, 14, 21, 22, 23)],
                    null_product: vec![false],
                },
                5 => CloneScript {
                    entries: vec![
                        entry_of(5, 0, 11, 12, 13, 14, 21, 22, 23),
                        entry_of(6, 9, NONE, NONE, 15, 16, 24, 25, 26),
                    ],
                    null_product: vec![true, false],
                },
                6 => CloneScript {
                    entries: vec![
                        entry_of(1, 0, 11, 12, 13, 14, 21, 22, 23),
                        entry_of(2, 7, 11, NONE, 13, 14, 24, 25, 26),
                        entry_of(4, 7, 11, 12, 13, 14, 27, 28, 29),
                    ],
                    null_product: vec![false, false, false],
                },
                _ => {
                    let n = rng.below(4);
                    let mut entries = Vec::new();
                    let mut null_product = Vec::new();
                    for _ in 0..n {
                        // Half the flags keep the extra bit clear so the
                        // dropped-bits mutant cannot hide.
                        let mut flags = rng.edge_word();
                        if rng.below(2) == 0 {
                            flags &= !FLAG_EXTRA;
                        }
                        entries.push(entry_of(
                            flags,
                            rng.edge_word(),
                            rng.edge_word(),
                            rng.edge_word(),
                            rng.edge_word(),
                            rng.edge_word(),
                            rng.u32(),
                            rng.u32(),
                            rng.u32(),
                        ));
                        null_product.push(rng.below(3) == 0);
                    }
                    // At least one NONE selector sometimes, for the alt path.
                    if n > 0 && rng.below(2) == 0 {
                        let i = rng.below(n) as usize;
                        if rng.below(2) == 0 {
                            entries[i].y = NONE;
                        } else {
                            entries[i].x = NONE;
                        }
                        entries[i].aux |= 1;
                    }
                    CloneScript { entries, null_product }
                }
            };

            let (stub_a, _, prods_a) = run_clone::<true>(owner, &script, key, case);
            compared += 1;
            let (stub_b, _, prods_b) = run_clone::<false>(owner, &script, key, case);
            compared += 1;

            // Mutants run on fresh fakes with placeholder node and
            // product handles (never dereferenced): maker arguments
            // never include the node, and find calls log only the key.
            // The raw-flags mutant is caught by maker flag words, the
            // flipped-done mutant by its mark set against the image.
            for (set_done, stub_seq, prods, caught_raw, caught_done) in [
                (true, &stub_a, &prods_a, &mut caught_raw_a, &mut caught_done_a),
                (false, &stub_b, &prods_b, &mut caught_raw_b, &mut caught_done_b),
            ] {
                let mut finds = VecDeque::new();
                for e in script.entries.iter() {
                    finds.push_back(Some(FoundEntry {
                        node: Handle32::new(0x4000_0000 + rng.u32() % 0x1000).unwrap(),
                        entry: *e,
                    }));
                }
                finds.push_back(None);
                let mut products = VecDeque::new();
                for null in script.null_product.iter() {
                    products.push_back(if *null { None } else { Handle32::new(0x5000_0000) });
                }
                let cloner = ChainCloner::new(Handle32::new(owner));
                // Raw-flags mutant: maker flag words must differ from
                // the stubs' wherever the extra bit was clear.
                let mut fake = FakeClone {
                    calls: Vec::new(),
                    finds: finds.clone(),
                    products: products.clone(),
                };
                if set_done {
                    wrong::clone_raw_flags::<FakeClone, true>(&cloner, &mut fake, key);
                } else {
                    wrong::clone_raw_flags::<FakeClone, false>(&cloner, &mut fake, key);
                }
                let mut stub_flags = Vec::new();
                for ev in stub_seq.iter() {
                    match ev {
                        StubEvent::MakeFull(_, _, _, f, _, _, _) => stub_flags.push(*f),
                        StubEvent::MakeAlt(_, _, _, f, _, _) => stub_flags.push(*f),
                        _ => {}
                    }
                }
                let mut mut_flags = Vec::new();
                for c in fake.calls.iter() {
                    match c {
                        CloneCall::MakeFull(_, _, _, f, _, _, _) => mut_flags.push(*f),
                        CloneCall::MakeAlt(_, _, _, f, _, _) => mut_flags.push(*f),
                        _ => {}
                    }
                }
                if stub_flags != mut_flags {
                    *caught_raw += 1;
                }
                // Flipped-done mutant: its mark set must equal the
                // image-derived set; it never does when products exist.
                let mut fake = FakeClone { calls: Vec::new(), finds, products };
                if set_done {
                    wrong::clone_flipped_done::<FakeClone, true>(&cloner, &mut fake, key);
                } else {
                    wrong::clone_flipped_done::<FakeClone, false>(&cloner, &mut fake, key);
                }
                let mut mut_marked: Vec<u32> = fake
                    .calls
                    .iter()
                    .filter_map(|c| match c {
                        CloneCall::MarkDone(p) => Some(*p),
                        _ => None,
                    })
                    .collect();
                mut_marked.sort_unstable();
                // Image-derived truth: instance A marks every product,
                // instance B marks none.
                let image_marked = !prods.is_empty() && set_done;
                let mutant_marked = !mut_marked.is_empty();
                if image_marked != mutant_marked {
                    *caught_done += 1;
                }
            }
        }
        assert_eq!(compared, 192);
        assert!(caught_raw_a > 0, "clone-A raw-flags mutant was never caught");
        assert!(caught_done_a > 0, "clone-A flipped-done mutant was never caught");
        assert!(caught_raw_b > 0, "clone-B raw-flags mutant was never caught");
        assert!(caught_done_b > 0, "clone-B flipped-done mutant was never caught");
    }
}