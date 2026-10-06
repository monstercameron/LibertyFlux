//! Differential case: the fist-shake clone slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full task blob, and the two pool calls
//! (slot and arguments in order) against the lift's trait calls, both
//! sides given the same scripted answers. A deliberately wrong lift must
//! be caught at least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{FistLink, FistPool, ShakeFist, TaskMgr, UninitTask};
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::{set_callee, set_manager};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{F_HELD, F_LINK, Rng, U32_EDGE, addr, fist_blob};

    // Alloc stub (slot 1) and its recording.
    static ALLOC_MGR: AtomicU32 = AtomicU32::new(0);
    static ALLOC_ANS: AtomicU32 = AtomicU32::new(0);
    static ALLOC_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn alloc_stub(mgr: u32) -> u32 {
        ALLOC_MGR.store(mgr, Ordering::SeqCst);
        ALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
        ALLOC_ANS.load(Ordering::SeqCst)
    }

    // Construct stub (slot 2) and its recording.
    static CONSTRUCT_BLOCK: AtomicU32 = AtomicU32::new(0);
    static CONSTRUCT_LINK: AtomicU32 = AtomicU32::new(0);
    static CONSTRUCT_ANS: AtomicU32 = AtomicU32::new(0);
    static CONSTRUCT_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn construct_stub(block: u32, link: u32) -> u32 {
        CONSTRUCT_BLOCK.store(block, Ordering::SeqCst);
        CONSTRUCT_LINK.store(link, Ordering::SeqCst);
        CONSTRUCT_COUNT.fetch_add(1, Ordering::SeqCst);
        CONSTRUCT_ANS.load(Ordering::SeqCst)
    }

    fn alloc_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = alloc_stub;
        f as usize as u32
    }

    fn construct_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = construct_stub;
        f as usize as u32
    }

    struct Fake {
        alloc_answer: u32,
        construct_answer: u32,
        allocs: Vec<u32>,
        constructs: Vec<(u32, Option<Handle32<FistLink>>)>,
    }

    impl FistPool for Fake {
        fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
            self.allocs.push(Handle32::raw_or_zero(manager));
            Handle32::new(self.alloc_answer)
        }

        fn construct(
            &mut self,
            block: Handle32<UninitTask>,
            link: Option<Handle32<FistLink>>,
        ) -> Option<Handle32<ShakeFist>> {
            self.constructs.push((block.get(), link));
            Handle32::new(self.construct_answer)
        }
    }

    /// Forgets the member word: constructs from a null link.
    fn wrong_clone(
        held: Option<Handle32<lf_peds_tasks::tasks::FistHeld>>,
        manager: Option<Handle32<TaskMgr>>,
        pool: &mut Fake,
    ) -> Option<Handle32<ShakeFist>> {
        let task = ShakeFist::new(held, None);
        task.clone_task(manager, pool)
    }

    #[test]
    fn fist_clone_matches() {
        set_callee(1, alloc_addr());
        set_callee(2, construct_addr());
        let mut rng = Rng(0xF157);
        let mut caught = 0;
        let mut cases = 0;
        let managers = {
            let mut v = U32_EDGE.to_vec();
            for _ in 0..8 {
                v.push(rng.u32());
            }
            v
        };
        // Fresh blocks: null plus nonzero cookies the stubs never dereference.
        let blocks = [0u32, 1, 0x1000, 0xFFFF_FFFF, rng.u32() | 1];
        let mut inputs = Vec::new();
        for &manager in &managers {
            for &fresh in &blocks {
                for &ans in &U32_EDGE {
                    inputs.push((manager, fresh, ans, rng.u32(), rng.u32()));
                }
            }
        }
        for (manager, fresh, ans, held, link) in inputs {
            let task_blob = fist_blob();
            let mut blob = *task_blob;
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[F_HELD] = held;
            blob[F_LINK] = link;
            let boxed = Box::new(blob);
            let before = *boxed;
            set_manager(manager);
            ALLOC_ANS.store(fresh, Ordering::SeqCst);
            CONSTRUCT_ANS.store(ans, Ordering::SeqCst);
            ALLOC_COUNT.store(0, Ordering::SeqCst);
            CONSTRUCT_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00D4DEE0::rw_00d4dee0(addr(&boxed[0])) };
            assert_eq!(ALLOC_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(ALLOC_MGR.load(Ordering::SeqCst), manager);
            let want_constructs = u32::from(fresh != 0);
            assert_eq!(CONSTRUCT_COUNT.load(Ordering::SeqCst), want_constructs);
            if fresh != 0 {
                assert_eq!(CONSTRUCT_BLOCK.load(Ordering::SeqCst), fresh);
                assert_eq!(CONSTRUCT_LINK.load(Ordering::SeqCst), link);
            }
            let lift = ShakeFist::new(Handle32::new(held), Handle32::new(link));
            let mut fake = Fake {
                alloc_answer: fresh,
                construct_answer: ans,
                allocs: Vec::new(),
                constructs: Vec::new(),
            };
            let lift_ret = lift.clone_task(Handle32::new(manager), &mut fake);
            assert_eq!(got, Handle32::raw_or_zero(lift_ret));
            assert_eq!(fake.allocs, vec![manager]);
            assert_eq!(fake.constructs.len() as u32, want_constructs);
            if fresh != 0 {
                assert_eq!(fake.constructs[0], (fresh, Handle32::new(link)));
            }
            // The source blob survives whole.
            assert_eq!(*boxed, before);
            let mut wfake = Fake {
                alloc_answer: fresh,
                construct_answer: ans,
                allocs: Vec::new(),
                constructs: Vec::new(),
            };
            wrong_clone(Handle32::new(held), Handle32::new(manager), &mut wfake);
            if fresh != 0
                && wfake.constructs.len() == 1
                && Handle32::raw_or_zero(wfake.constructs[0].1)
                    != CONSTRUCT_LINK.load(Ordering::SeqCst)
            {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong fist clone never caught ({cases} cases)");
    }
}
