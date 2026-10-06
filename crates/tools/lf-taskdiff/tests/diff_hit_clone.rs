//! Differential case: the hit-response clone slot against its verified
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
    use lf_peds_tasks::tasks::{HitBase, HitPool, HitResponse, TaskMgr, UninitTask};
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::{set_callee, set_manager};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{H_KIND, Rng, U32_EDGE, addr, hit_blob};

    // Alloc stub (slot 1) and its recording.
    static ALLOC_MGR: AtomicU32 = AtomicU32::new(0);
    static ALLOC_ANS: AtomicU32 = AtomicU32::new(0);
    static ALLOC_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn alloc_stub(mgr: u32) -> u32 {
        ALLOC_MGR.store(mgr, Ordering::SeqCst);
        ALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
        ALLOC_ANS.load(Ordering::SeqCst)
    }

    // Initialiser stub (slot 2) and its recording.
    static INIT_BLOCK: AtomicU32 = AtomicU32::new(0);
    static INIT_KIND: AtomicU32 = AtomicU32::new(0);
    static INIT_ANS: AtomicU32 = AtomicU32::new(0);
    static INIT_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn init_stub(block: u32, kind: u32) -> u32 {
        INIT_BLOCK.store(block, Ordering::SeqCst);
        INIT_KIND.store(kind, Ordering::SeqCst);
        INIT_COUNT.fetch_add(1, Ordering::SeqCst);
        INIT_ANS.load(Ordering::SeqCst)
    }

    fn alloc_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = alloc_stub;
        f as usize as u32
    }

    fn init_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = init_stub;
        f as usize as u32
    }

    struct Fake {
        alloc_answer: u32,
        init_answer: u32,
        allocs: Vec<u32>,
        inits: Vec<(u32, u32)>,
    }

    impl HitPool for Fake {
        fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
            self.allocs.push(Handle32::raw_or_zero(manager));
            Handle32::new(self.alloc_answer)
        }

        fn construct(
            &mut self,
            block: Handle32<UninitTask>,
            kind: u32,
        ) -> Option<Handle32<HitResponse>> {
            self.inits.push((block.get(), kind));
            Handle32::new(self.init_answer)
        }
    }

    struct Base;

    impl HitBase for Base {
        fn construct_base(&mut self) {}
    }

    /// Initialises from the neighbouring kind.
    fn wrong_clone(
        kind: u32,
        manager: Option<Handle32<TaskMgr>>,
        pool: &mut Fake,
    ) -> Option<Handle32<HitResponse>> {
        let task = HitResponse::new(kind.wrapping_add(1), &mut Base);
        task.clone_task(manager, pool)
    }

    #[test]
    fn hit_clone_matches() {
        set_callee(1, alloc_addr());
        set_callee(2, init_addr());
        let mut rng = Rng(0x41CC);
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
        let kinds = {
            let mut v = U32_EDGE.to_vec();
            v.extend([4, 5]);
            for _ in 0..8 {
                v.push(rng.u32());
            }
            v
        };
        let mut inputs = Vec::new();
        for &manager in &managers {
            for &fresh in &blocks {
                for &ans in &U32_EDGE {
                    for &kind in &kinds {
                        inputs.push((manager, fresh, ans, kind));
                    }
                }
            }
        }
        for (manager, fresh, ans, kind) in inputs {
            let mut blob = *hit_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[H_KIND] = kind;
            let boxed = Box::new(blob);
            let before = *boxed;
            set_manager(manager);
            ALLOC_ANS.store(fresh, Ordering::SeqCst);
            INIT_ANS.store(ans, Ordering::SeqCst);
            ALLOC_COUNT.store(0, Ordering::SeqCst);
            INIT_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CCA8B0::rw_rs01_cca8b0(addr(&boxed[0]) as *const u8) };
            assert_eq!(ALLOC_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(ALLOC_MGR.load(Ordering::SeqCst), manager);
            let want_inits = u32::from(fresh != 0);
            assert_eq!(INIT_COUNT.load(Ordering::SeqCst), want_inits);
            if fresh != 0 {
                assert_eq!(INIT_BLOCK.load(Ordering::SeqCst), fresh);
                assert_eq!(INIT_KIND.load(Ordering::SeqCst), kind);
            }
            let lift = HitResponse::new(kind, &mut Base);
            let mut fake = Fake {
                alloc_answer: fresh,
                init_answer: ans,
                allocs: Vec::new(),
                inits: Vec::new(),
            };
            let lift_ret = lift.clone_task(Handle32::new(manager), &mut fake);
            assert_eq!(got, Handle32::raw_or_zero(lift_ret));
            assert_eq!(fake.allocs, vec![manager]);
            assert_eq!(fake.inits.len() as u32, want_inits);
            if fresh != 0 {
                assert_eq!(fake.inits[0], (fresh, kind));
            }
            // The source blob survives whole.
            assert_eq!(*boxed, before);
            let mut wfake = Fake {
                alloc_answer: fresh,
                init_answer: ans,
                allocs: Vec::new(),
                inits: Vec::new(),
            };
            wrong_clone(kind, Handle32::new(manager), &mut wfake);
            if fresh != 0 && wfake.inits.len() == 1 && wfake.inits[0].1 != kind {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong hit clone never caught ({cases} cases)");
    }
}
