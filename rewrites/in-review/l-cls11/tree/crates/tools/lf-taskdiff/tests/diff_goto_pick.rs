//! Differential case: the goto target picker against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full task blob (rolled wait, stamp,
//! wait copy and arming against the lift, everything else preserved),
//! and the eleven helper calls against the lift's trait calls, both
//! sides given the same scripted answers. The shared hash and allocator
//! slots are pinned through distinct answers and branch shapes; the
//! seed, goal and second-child constants are pinned; the first child's
//! speed-and-rate words are snapped through the passed pointer. A
//! deliberately wrong lift must be caught at least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{
        GotoChild, GotoPed, GotoPick, GotoTask, TaskMgr, UninitTask,
    };
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::{set_callee, set_manager, set_rate, set_tick};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        G_ARMED_BYTE, G_FLAG_BYTE, G_KIND, G_MODE, G_POS, G_RESTAMP_BYTE, G_SPEED, G_STAMP, G_SUB,
        G_WAIT, G_WAITCP, Rng, addr, blob_byte, goto_blob, ped_blob, set_blob_byte,
    };

    /// Byte offset of the member the goal finder takes.
    const MEMBER_OFF: u32 = 0x20;
    /// Byte offset of the seed object past the ped.
    const SEED_OFF: u32 = 0x570;
    /// The seed table word the proof pins.
    const SEED_TABLE: u32 = 0x00EEF58C;
    /// The second child's constant blend word.
    const CHILD2_BLEND: u32 = 0x3EA8_F5C3;
    /// The second child's constant span word.
    const CHILD2_SPAN: u32 = 0x14;
    /// The seed call's constant one word (as float bits).
    const SEED_ONE: u32 = 0x3F80_0000;

    // Seed stub (slot 1): object plus ten words, all recorded.
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

    // Hash stubs (slots 2 and 3) with separate answers.
    static HASH1_KIND: AtomicU32 = AtomicU32::new(0);
    static HASH1_ANS: AtomicU32 = AtomicU32::new(0);
    static HASH1_COUNT: AtomicU32 = AtomicU32::new(0);
    static HASH2_KIND: AtomicU32 = AtomicU32::new(0);
    static HASH2_ANS: AtomicU32 = AtomicU32::new(0);
    static HASH2_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "cdecl" fn hash1_stub(kind: u32) -> u32 {
        HASH1_KIND.store(kind, Ordering::SeqCst);
        HASH1_COUNT.fetch_add(1, Ordering::SeqCst);
        HASH1_ANS.load(Ordering::SeqCst)
    }

    extern "cdecl" fn hash2_stub(kind: u32) -> u32 {
        HASH2_KIND.store(kind, Ordering::SeqCst);
        HASH2_COUNT.fetch_add(1, Ordering::SeqCst);
        HASH2_ANS.load(Ordering::SeqCst)
    }

    // Random stub (slot 4).
    static RAND_ANS: AtomicU32 = AtomicU32::new(0);
    static RAND_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "cdecl" fn rand_stub() -> u32 {
        RAND_COUNT.fetch_add(1, Ordering::SeqCst);
        RAND_ANS.load(Ordering::SeqCst)
    }

    // Goal stub (slot 5).
    static GOAL_MEMBER: AtomicU32 = AtomicU32::new(0);
    static GOAL_SCRATCH: AtomicU32 = AtomicU32::new(0);
    static GOAL_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn goal_stub(member: u32, scratch: u32) -> u32 {
        GOAL_MEMBER.store(member, Ordering::SeqCst);
        GOAL_SCRATCH.store(scratch, Ordering::SeqCst);
        GOAL_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    // Allocator stubs (slots 6, 8, 10) with separate answers.
    static MGR1_MGR: AtomicU32 = AtomicU32::new(0);
    static MGR1_ANS: AtomicU32 = AtomicU32::new(0);
    static MGR1_COUNT: AtomicU32 = AtomicU32::new(0);
    static MGR2_MGR: AtomicU32 = AtomicU32::new(0);
    static MGR2_ANS: AtomicU32 = AtomicU32::new(0);
    static MGR2_COUNT: AtomicU32 = AtomicU32::new(0);
    static MGR3_MGR: AtomicU32 = AtomicU32::new(0);
    static MGR3_ANS: AtomicU32 = AtomicU32::new(0);
    static MGR3_COUNT: AtomicU32 = AtomicU32::new(0);

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

    // First-child stub (slot 7): snaps the two words behind the pointer.
    static C1_BLOCK: AtomicU32 = AtomicU32::new(0);
    static C1_SPEED: AtomicU32 = AtomicU32::new(0);
    static C1_RATE: AtomicU32 = AtomicU32::new(0);
    static C1_ANS: AtomicU32 = AtomicU32::new(0);
    static C1_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn child1_stub(block: u32, arg: u32) -> u32 {
        C1_BLOCK.store(block, Ordering::SeqCst);
        unsafe {
            C1_SPEED.store(((arg - 24) as *const u32).read(), Ordering::SeqCst);
            C1_RATE.store(((arg - 20) as *const u32).read(), Ordering::SeqCst);
        }
        C1_COUNT.fetch_add(1, Ordering::SeqCst);
        C1_ANS.load(Ordering::SeqCst)
    }

    // Second-child stub (slot 9): block plus six words, all recorded.
    static C2_ARGS: [AtomicU32; 7] = [
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
    ];
    static C2_ANS: AtomicU32 = AtomicU32::new(0);
    static C2_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn child2_stub(
        block: u32,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
    ) -> u32 {
        C2_ARGS[0].store(block, Ordering::SeqCst);
        C2_ARGS[1].store(a, Ordering::SeqCst);
        C2_ARGS[2].store(b, Ordering::SeqCst);
        C2_ARGS[3].store(c, Ordering::SeqCst);
        C2_ARGS[4].store(d, Ordering::SeqCst);
        C2_ARGS[5].store(e, Ordering::SeqCst);
        C2_ARGS[6].store(f, Ordering::SeqCst);
        C2_COUNT.fetch_add(1, Ordering::SeqCst);
        C2_ANS.load(Ordering::SeqCst)
    }

    // Combine stub (slot 11).
    static COMB_ARGS: [AtomicU32; 5] = [
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
    ];
    static COMB_ANS: AtomicU32 = AtomicU32::new(0);
    static COMB_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn combine_stub(block: u32, c1: u32, c2: u32, z1: u32, z2: u32) -> u32 {
        COMB_ARGS[0].store(block, Ordering::SeqCst);
        COMB_ARGS[1].store(c1, Ordering::SeqCst);
        COMB_ARGS[2].store(c2, Ordering::SeqCst);
        COMB_ARGS[3].store(z1, Ordering::SeqCst);
        COMB_ARGS[4].store(z2, Ordering::SeqCst);
        COMB_COUNT.fetch_add(1, Ordering::SeqCst);
        COMB_ANS.load(Ordering::SeqCst)
    }

    fn seed_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            seed_stub;
        f as usize as u32
    }

    fn hash1_addr() -> u32 {
        let f: extern "cdecl" fn(u32) -> u32 = hash1_stub;
        f as usize as u32
    }

    fn hash2_addr() -> u32 {
        let f: extern "cdecl" fn(u32) -> u32 = hash2_stub;
        f as usize as u32
    }

    fn rand_addr() -> u32 {
        let f: extern "cdecl" fn() -> u32 = rand_stub;
        f as usize as u32
    }

    fn goal_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = goal_stub;
        f as usize as u32
    }

    fn mgr1_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = mgr1_stub;
        f as usize as u32
    }

    fn mgr2_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = mgr2_stub;
        f as usize as u32
    }

    fn mgr3_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = mgr3_stub;
        f as usize as u32
    }

    fn child1_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = child1_stub;
        f as usize as u32
    }

    fn child2_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 = child2_stub;
        f as usize as u32
    }

    fn combine_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 = combine_stub;
        f as usize as u32
    }

    struct Fake {
        hash_answers: [u32; 2],
        hash_next: usize,
        rand_answer: u32,
        alloc_answers: [u32; 3],
        alloc_next: usize,
        child1_answer: u32,
        child2_answer: u32,
        combine_answer: u32,
        seeds: Vec<u32>,
        hashes: Vec<u32>,
        rands: u32,
        goals: Vec<u32>,
        allocs: Vec<u32>,
        firsts: Vec<(u32, u32, u32)>,
        seconds: Vec<u32>,
        combines: Vec<(u32, u32, u32)>,
    }

    impl GotoPick for Fake {
        fn seed(&mut self, ped: Option<Handle32<GotoPed>>) {
            self.seeds.push(Handle32::raw_or_zero(ped));
        }

        fn hash_kind(&mut self, kind: u32) -> u32 {
            self.hashes.push(kind);
            let ans = self.hash_answers[self.hash_next];
            self.hash_next += 1;
            ans
        }

        fn rand_word(&mut self) -> u32 {
            self.rands += 1;
            self.rand_answer
        }

        fn find_goal(&mut self, kind: u32) {
            self.goals.push(kind);
        }

        fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
            self.allocs.push(Handle32::raw_or_zero(manager));
            let ans = self.alloc_answers[self.alloc_next];
            self.alloc_next += 1;
            Handle32::new(ans)
        }

        fn build_first(
            &mut self,
            block: Handle32<UninitTask>,
            speed: f32,
            rate: f32,
        ) -> Option<Handle32<GotoChild>> {
            self.firsts.push((block.get(), speed.to_bits(), rate.to_bits()));
            Handle32::new(self.child1_answer)
        }

        fn build_second(&mut self, block: Handle32<UninitTask>) -> Option<Handle32<GotoChild>> {
            self.seconds.push(block.get());
            Handle32::new(self.child2_answer)
        }

        fn combine(
            &mut self,
            block: Handle32<UninitTask>,
            first: Option<Handle32<GotoChild>>,
            second: Option<Handle32<GotoChild>>,
        ) -> Option<Handle32<GotoChild>> {
            self.combines.push((
                block.get(),
                Handle32::raw_or_zero(first),
                Handle32::raw_or_zero(second),
            ));
            Handle32::new(self.combine_answer)
        }
    }

    fn new_fake(
        h1: u32,
        h2: u32,
        rand: u32,
        objs: [u32; 3],
        c1: u32,
        c2: u32,
        comb: u32,
    ) -> Fake {
        Fake {
            hash_answers: [h1, h2],
            hash_next: 0,
            rand_answer: rand,
            alloc_answers: objs,
            alloc_next: 0,
            child1_answer: c1,
            child2_answer: c2,
            combine_answer: comb,
            seeds: Vec::new(),
            hashes: Vec::new(),
            rands: 0,
            goals: Vec::new(),
            allocs: Vec::new(),
            firsts: Vec::new(),
            seconds: Vec::new(),
            combines: Vec::new(),
        }
    }

    /// A picker that hashes the neighbouring kind.
    fn wrong_pick(
        task: &mut GotoTask,
        ped: Option<Handle32<GotoPed>>,
        stamp: u32,
        rate: f32,
        manager: Option<Handle32<TaskMgr>>,
        pick: &mut Fake,
    ) -> Option<Handle32<GotoChild>> {
        let shifted = GotoTask::new(
            task.subtask(),
            task.kind().wrapping_add(1),
            task.pos(),
            task.flag(),
            task.mode(),
            task.wait(),
            task.stamp(),
            task.wait_copy(),
            task.armed(),
            task.restamp(),
            task.speed(),
        );
        let mut shifted = shifted;
        let ret = shifted.pick_target(ped, stamp, rate, manager, pick);
        *task = shifted;
        ret
    }

    #[test]
    fn goto_pick_matches() {
        set_callee(1, seed_addr());
        set_callee(2, hash1_addr());
        set_callee(3, hash2_addr());
        set_callee(4, rand_addr());
        set_callee(5, goal_addr());
        set_callee(6, mgr1_addr());
        set_callee(7, child1_addr());
        set_callee(8, mgr2_addr());
        set_callee(9, child2_addr());
        set_callee(10, mgr3_addr());
        set_callee(11, combine_addr());
        let mut rng = Rng(0x9C4);
        let mut caught = 0;
        let mut cases = 0;
        let kinds = [
            0u32,
            1,
            2,
            0x15,
            0x1B,
            0x7FFF_FFFF,
            0x8000_0000,
            0xFFFF_FFFF,
            rng.u32(),
            rng.u32(),
        ];
        // Hash pairs: equal, far apart both ways, edges, random.
        let hpairs = [
            (0u32, 0u32),
            (0, 0xFFFF_FFFF),
            (0xFFFF_FFFF, 0),
            (1, 0x8000_0000),
            (rng.u32(), rng.u32()),
            (rng.u32(), rng.u32()),
        ];
        // Random words: low-half edges plus full random.
        let rands = [0u32, 1, 0xFFFF, 0x1_0000, rng.u32(), rng.u32() | 0xFFFF];
        // Allocation triples as null patterns; live slots take cookies.
        let triples = [
            (false, false, false),
            (true, false, false),
            (false, true, false),
            (false, false, true),
            (true, true, false),
            (true, false, true),
            (false, true, true),
            (true, true, true),
        ];
        // Child answers: nulls and live cookies.
        let childpairs = [(0u32, 0u32), (1, 0), (0, 1), (1, 1)];
        let mut inputs = Vec::new();
        for &kind in &kinds {
            for &(h1, h2) in &hpairs {
                for &rnd in &rands {
                    for &triple in &triples {
                        for &pair in &childpairs {
                            inputs.push((kind, h1, h2, rnd, triple, pair));
                        }
                    }
                }
            }
        }
        for (case_no, &(kind, h1, h2, rnd, (t1, t2, t3), (p1, p2))) in
            inputs.iter().enumerate()
        {
            let speed_bits = match case_no % 6 {
                0 => 1.0f32.to_bits(),
                1 => 0.0f32.to_bits(),
                2 => (-3.25f32).to_bits(),
                3 => f32::MAX.to_bits(),
                4 => f32::NAN.to_bits(),
                _ => rng.u32(),
            };
            let rate_bits = match case_no % 4 {
                0 => 0.5f32.to_bits(),
                1 => f32::MIN_POSITIVE.to_bits(),
                2 => f32::NEG_INFINITY.to_bits(),
                _ => rng.u32(),
            };
            let stamp = rng.u32();
            let manager = rng.u32();
            let live_ped = case_no % 3 != 0;
            let o1 = if t1 { rng.u32() | 1 } else { 0 };
            let o2 = if t2 { rng.u32() | 1 } else { 0 };
            let o3 = if t3 { rng.u32() | 1 } else { 0 };
            let c1 = if p1 == 0 { 0 } else { rng.u32() | 1 };
            let c2 = if p2 == 0 { 0 } else { rng.u32() | 1 };
            let comb = if case_no % 5 == 0 { 0 } else { rng.u32() };
            let mut blob = *goto_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[G_KIND] = kind;
            blob[G_SPEED] = speed_bits;
            let ped = ped_blob();
            let ped_addr = if live_ped { addr(&ped[0]) } else { 0 };
            let boxed = Box::new(blob);
            let before = *boxed;
            set_tick(stamp);
            set_rate(rate_bits);
            set_manager(manager);
            HASH1_ANS.store(h1, Ordering::SeqCst);
            HASH2_ANS.store(h2, Ordering::SeqCst);
            RAND_ANS.store(rnd, Ordering::SeqCst);
            MGR1_ANS.store(o1, Ordering::SeqCst);
            MGR2_ANS.store(o2, Ordering::SeqCst);
            MGR3_ANS.store(o3, Ordering::SeqCst);
            C1_ANS.store(c1, Ordering::SeqCst);
            C2_ANS.store(c2, Ordering::SeqCst);
            COMB_ANS.store(comb, Ordering::SeqCst);
            SEED_COUNT.store(0, Ordering::SeqCst);
            HASH1_COUNT.store(0, Ordering::SeqCst);
            HASH2_COUNT.store(0, Ordering::SeqCst);
            RAND_COUNT.store(0, Ordering::SeqCst);
            GOAL_COUNT.store(0, Ordering::SeqCst);
            MGR1_COUNT.store(0, Ordering::SeqCst);
            MGR2_COUNT.store(0, Ordering::SeqCst);
            MGR3_COUNT.store(0, Ordering::SeqCst);
            C1_COUNT.store(0, Ordering::SeqCst);
            C2_COUNT.store(0, Ordering::SeqCst);
            COMB_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00DA5B90::rw_00da5b90(addr(&boxed[0]), ped_addr) };
            // The fixed calls happen exactly once with pinned arguments.
            assert_eq!(SEED_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(SEED_ARGS[0].load(Ordering::SeqCst), ped_addr.wrapping_add(SEED_OFF));
            assert_eq!(SEED_ARGS[1].load(Ordering::SeqCst), SEED_TABLE);
            assert_eq!(SEED_ARGS[2].load(Ordering::SeqCst), 0);
            assert_eq!(SEED_ARGS[3].load(Ordering::SeqCst), 0);
            assert_eq!(SEED_ARGS[4].load(Ordering::SeqCst), 0);
            assert_eq!(SEED_ARGS[5].load(Ordering::SeqCst), 0xFFFF_FFFF);
            assert_eq!(SEED_ARGS[6].load(Ordering::SeqCst), 0);
            assert_eq!(SEED_ARGS[7].load(Ordering::SeqCst), 0);
            assert_eq!(SEED_ARGS[8].load(Ordering::SeqCst), SEED_ONE);
            assert_eq!(SEED_ARGS[9].load(Ordering::SeqCst), 0);
            assert_eq!(SEED_ARGS[10].load(Ordering::SeqCst), 0);
            assert_eq!(HASH1_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(HASH1_KIND.load(Ordering::SeqCst), kind);
            assert_eq!(HASH2_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(HASH2_KIND.load(Ordering::SeqCst), kind);
            assert_eq!(RAND_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(GOAL_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(
                GOAL_MEMBER.load(Ordering::SeqCst),
                addr(&boxed[0]).wrapping_add(MEMBER_OFF)
            );
            assert_ne!(GOAL_SCRATCH.load(Ordering::SeqCst), 0);
            assert_eq!(MGR1_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(MGR1_MGR.load(Ordering::SeqCst), manager);
            assert_eq!(MGR2_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(MGR2_MGR.load(Ordering::SeqCst), manager);
            assert_eq!(MGR3_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(MGR3_MGR.load(Ordering::SeqCst), manager);
            let want_c1 = o1 != 0;
            let want_c2 = o2 != 0;
            let want_comb = o3 != 0;
            assert_eq!(C1_COUNT.load(Ordering::SeqCst), u32::from(want_c1));
            if want_c1 {
                assert_eq!(C1_BLOCK.load(Ordering::SeqCst), o1);
                assert_eq!(C1_SPEED.load(Ordering::SeqCst), speed_bits);
                assert_eq!(C1_RATE.load(Ordering::SeqCst), rate_bits);
            }
            assert_eq!(C2_COUNT.load(Ordering::SeqCst), u32::from(want_c2));
            if want_c2 {
                assert_eq!(C2_ARGS[0].load(Ordering::SeqCst), o2);
                assert_eq!(C2_ARGS[1].load(Ordering::SeqCst), 0);
                assert_eq!(C2_ARGS[2].load(Ordering::SeqCst), CHILD2_BLEND);
                assert_eq!(C2_ARGS[3].load(Ordering::SeqCst), 0);
                assert_eq!(C2_ARGS[4].load(Ordering::SeqCst), 0);
                assert_eq!(C2_ARGS[5].load(Ordering::SeqCst), 0);
                assert_eq!(C2_ARGS[6].load(Ordering::SeqCst), CHILD2_SPAN);
            }
            assert_eq!(COMB_COUNT.load(Ordering::SeqCst), u32::from(want_comb));
            let child1 = if want_c1 { c1 } else { 0 };
            let child2 = if want_c2 { c2 } else { 0 };
            if want_comb {
                assert_eq!(COMB_ARGS[0].load(Ordering::SeqCst), o3);
                assert_eq!(COMB_ARGS[1].load(Ordering::SeqCst), child1);
                assert_eq!(COMB_ARGS[2].load(Ordering::SeqCst), child2);
                assert_eq!(COMB_ARGS[3].load(Ordering::SeqCst), 0);
                assert_eq!(COMB_ARGS[4].load(Ordering::SeqCst), 0);
                assert_eq!(got, comb);
            } else {
                assert_eq!(got, 0);
            }
            // The lift runs the same inputs through its trait calls.
            let mut lift = GotoTask::new(
                Handle32::new(before[G_SUB]),
                kind,
                [
                    f32::from_bits(before[G_POS]),
                    f32::from_bits(before[G_POS + 1]),
                    f32::from_bits(before[G_POS + 2]),
                ],
                blob_byte(&before[..], G_FLAG_BYTE) != 0,
                Handle32::new(before[G_MODE]),
                before[G_WAIT],
                before[G_STAMP],
                before[G_WAITCP],
                blob_byte(&before[..], G_ARMED_BYTE) != 0,
                blob_byte(&before[..], G_RESTAMP_BYTE) != 0,
                f32::from_bits(speed_bits),
            );
            let mut fake = new_fake(h1, h2, rnd, [o1, o2, o3], c1, c2, comb);
            let lift_ret = lift.pick_target(
                Handle32::new(ped_addr),
                stamp,
                f32::from_bits(rate_bits),
                Handle32::new(manager),
                &mut fake,
            );
            assert_eq!(got, Handle32::raw_or_zero(lift_ret));
            assert_eq!(fake.seeds, vec![ped_addr]);
            assert_eq!(fake.hashes, vec![kind, kind]);
            assert_eq!(fake.rands, 1);
            assert_eq!(fake.goals, vec![kind]);
            assert_eq!(fake.allocs, vec![manager, manager, manager]);
            assert_eq!(fake.firsts.len() as u32, u32::from(want_c1));
            if want_c1 {
                assert_eq!(fake.firsts[0], (o1, speed_bits, rate_bits));
            }
            assert_eq!(fake.seconds.len() as u32, u32::from(want_c2));
            if want_c2 {
                assert_eq!(fake.seconds[0], o2);
            }
            assert_eq!(fake.combines.len() as u32, u32::from(want_comb));
            if want_comb {
                assert_eq!(fake.combines[0], (o3, child1, child2));
            }
            // The blob gains the wait, the stamp, the wait copy and the
            // arming; the restamp and every other word survive.
            let mut expect = before;
            expect[G_WAIT] = lift.wait();
            expect[G_STAMP] = stamp;
            expect[G_WAITCP] = lift.wait();
            set_blob_byte(&mut expect, G_ARMED_BYTE, 1);
            assert_eq!(*boxed, expect);
            assert_eq!(boxed[G_STAMP], lift.stamp());
            assert_eq!(boxed[G_WAITCP], lift.wait_copy());
            assert!(lift.armed());
            assert_eq!(boxed[G_WAIT], lift.wait());
            // The wrong lift hashes a different kind on every case.
            let mut wlift = GotoTask::new(
                Handle32::new(before[G_SUB]),
                kind,
                [
                    f32::from_bits(before[G_POS]),
                    f32::from_bits(before[G_POS + 1]),
                    f32::from_bits(before[G_POS + 2]),
                ],
                blob_byte(&before[..], G_FLAG_BYTE) != 0,
                Handle32::new(before[G_MODE]),
                before[G_WAIT],
                before[G_STAMP],
                before[G_WAITCP],
                blob_byte(&before[..], G_ARMED_BYTE) != 0,
                blob_byte(&before[..], G_RESTAMP_BYTE) != 0,
                f32::from_bits(speed_bits),
            );
            let mut wfake = new_fake(h1, h2, rnd, [o1, o2, o3], c1, c2, comb);
            wrong_pick(
                &mut wlift,
                Handle32::new(ped_addr),
                stamp,
                f32::from_bits(rate_bits),
                Handle32::new(manager),
                &mut wfake,
            );
            if wfake.hashes.len() == 2 && wfake.hashes[0] != kind {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong goto pick never caught ({cases} cases)");
    }
}
