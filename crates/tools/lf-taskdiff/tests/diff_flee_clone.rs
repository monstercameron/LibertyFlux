//! Differential case: the flee clone slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the untouched source blob, the clone blob
//! (the carried state byte against the lift, everything else preserved),
//! and the two pool calls against the lift's trait calls, both sides
//! given the same scripted answers. The copy constructor's member
//! address is pinned to the blob's member offset. A deliberately wrong
//! lift must be caught at least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{FleePool, FleeTask, TaskMgr, UninitTask};
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::{set_callee, set_manager};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        FL_KIND, FL_STATE_BYTE, Rng, U32_EDGE, addr, blob_byte, flee_blob, set_blob_byte,
    };

    /// Byte offset of the embedded member the copy constructor takes.
    const MEMBER_OFF: u32 = 0x20;

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
    static CONSTRUCT_MEMBER: AtomicU32 = AtomicU32::new(0);
    static CONSTRUCT_ANS: AtomicU32 = AtomicU32::new(0);
    static CONSTRUCT_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn construct_stub(block: u32, member: u32) -> u32 {
        CONSTRUCT_BLOCK.store(block, Ordering::SeqCst);
        CONSTRUCT_MEMBER.store(member, Ordering::SeqCst);
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
        constructs: Vec<(u32, u32, u8)>,
    }

    impl FleePool for Fake {
        fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
            self.allocs.push(Handle32::raw_or_zero(manager));
            Handle32::new(self.alloc_answer)
        }

        fn construct(
            &mut self,
            block: Handle32<UninitTask>,
            member: u32,
            state: u8,
        ) -> Option<Handle32<FleeTask>> {
            self.constructs.push((block.get(), member, state));
            Handle32::new(self.construct_answer)
        }
    }

    /// A clone that carries a bumped state byte.
    fn wrong_clone(
        task: &FleeTask,
        manager: Option<Handle32<TaskMgr>>,
        pool: &mut Fake,
    ) -> Option<Handle32<FleeTask>> {
        let block = pool.alloc(manager)?;
        pool.construct(block, task.kind(), task.state().wrapping_add(1))
    }

    #[test]
    fn flee_clone_matches() {
        set_callee(1, alloc_addr());
        set_callee(2, construct_addr());
        let mut rng = Rng(0xF1C0);
        let mut caught = 0;
        let mut cases = 0;
        let managers = {
            let mut v = U32_EDGE.to_vec();
            for _ in 0..4 {
                v.push(rng.u32());
            }
            v
        };
        // Fresh blocks: nonzero cookies the stubs never dereference (a
        // null block faults on the rewrite side, so the lift's panic for
        // it is pinned by a host test instead).
        let blocks = [1u32, 0x1000, rng.u32() | 1, rng.u32() | 1];
        let mut inputs = Vec::new();
        for &manager in &managers {
            for &fresh in &blocks {
                inputs.push((manager, fresh, rng.u32(), rng.u32() as u8));
            }
        }
        for (manager, fresh, kind, state) in inputs {
            let mut blob = *flee_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[FL_KIND] = kind;
            set_blob_byte(&mut blob, FL_STATE_BYTE, state);
            let boxed = Box::new(blob);
            let before = *boxed;
            // The clone blob: random words proving the rewrite stores
            // only the state byte into it.
            let mut clone = *flee_blob();
            for w in clone.iter_mut() {
                *w = rng.u32();
            }
            let clone_before = clone;
            let clone_boxed = Box::new(clone);
            let clone_addr = addr(&clone_boxed[0]);
            set_manager(manager);
            ALLOC_ANS.store(fresh, Ordering::SeqCst);
            CONSTRUCT_ANS.store(clone_addr, Ordering::SeqCst);
            ALLOC_COUNT.store(0, Ordering::SeqCst);
            CONSTRUCT_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00DA5080::rw_00da5080(addr(&boxed[0])) };
            assert_eq!(got, clone_addr);
            assert_eq!(ALLOC_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(ALLOC_MGR.load(Ordering::SeqCst), manager);
            assert_eq!(CONSTRUCT_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(CONSTRUCT_BLOCK.load(Ordering::SeqCst), fresh);
            assert_eq!(
                CONSTRUCT_MEMBER.load(Ordering::SeqCst),
                addr(&boxed[0]).wrapping_add(MEMBER_OFF)
            );
            // The source blob survives whole; the clone gains the state byte.
            assert_eq!(*boxed, before);
            let mut clone_expect = clone_before;
            set_blob_byte(&mut clone_expect, FL_STATE_BYTE, state);
            assert_eq!(*clone_boxed, clone_expect);
            let lift = FleeTask::new(None, 0, kind, [0.0, 0.0, 0.0], false, None, state);
            let mut fake = Fake {
                alloc_answer: fresh,
                construct_answer: clone_addr,
                allocs: Vec::new(),
                constructs: Vec::new(),
            };
            let lift_ret = lift.clone_task(Handle32::new(manager), &mut fake);
            assert_eq!(got, Handle32::raw_or_zero(lift_ret));
            assert_eq!(fake.allocs, vec![manager]);
            assert_eq!(fake.constructs, vec![(fresh, kind, state)]);
            assert_eq!(blob_byte(&clone_boxed[..], FL_STATE_BYTE), state);
            let mut wfake = Fake {
                alloc_answer: fresh,
                construct_answer: clone_addr,
                allocs: Vec::new(),
                constructs: Vec::new(),
            };
            wrong_clone(&lift, Handle32::new(manager), &mut wfake);
            if wfake.constructs.len() == 1 && wfake.constructs[0].2 != state {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong flee clone never caught ({cases} cases)");
    }
}
