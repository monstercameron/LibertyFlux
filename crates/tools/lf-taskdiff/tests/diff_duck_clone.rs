//! Differential case: the duck clone slot against its verified rewrite
//! on the same generated inputs.
//!
//! Compares the return value, the full task blob, and the two pool calls
//! (slot and arguments in order) against the lift's trait calls, both
//! sides given the same scripted answers. The level word is compared
//! sign-extended. A deliberately wrong lift must be caught at least
//! once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{DuckPool, DuckSpec, DuckTask, TaskMgr, UninitTask};
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::{set_callee, set_manager};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        D_LEVEL, D_SPAN, D_TAG_BYTE, Rng, U32_EDGE, addr, blob_byte, duck_blob, set_blob_byte,
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

    // Construct stub (slot 2) and its recording.
    static CONSTRUCT_BLOCK: AtomicU32 = AtomicU32::new(0);
    static CONSTRUCT_TAG: AtomicU32 = AtomicU32::new(0);
    static CONSTRUCT_SPAN: AtomicU32 = AtomicU32::new(0);
    static CONSTRUCT_LEVEL: AtomicU32 = AtomicU32::new(0);
    static CONSTRUCT_ANS: AtomicU32 = AtomicU32::new(0);
    static CONSTRUCT_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn construct_stub(block: u32, tag: u32, span: u32, level: u32) -> u32 {
        CONSTRUCT_BLOCK.store(block, Ordering::SeqCst);
        CONSTRUCT_TAG.store(tag, Ordering::SeqCst);
        CONSTRUCT_SPAN.store(span, Ordering::SeqCst);
        CONSTRUCT_LEVEL.store(level, Ordering::SeqCst);
        CONSTRUCT_COUNT.fetch_add(1, Ordering::SeqCst);
        CONSTRUCT_ANS.load(Ordering::SeqCst)
    }

    fn alloc_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = alloc_stub;
        f as usize as u32
    }

    fn construct_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = construct_stub;
        f as usize as u32
    }

    struct Fake {
        alloc_answer: u32,
        construct_answer: u32,
        allocs: Vec<u32>,
        constructs: Vec<(u32, DuckSpec)>,
    }

    impl DuckPool for Fake {
        fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
            self.allocs.push(Handle32::raw_or_zero(manager));
            Handle32::new(self.alloc_answer)
        }

        fn construct(
            &mut self,
            block: Handle32<UninitTask>,
            spec: DuckSpec,
        ) -> Option<Handle32<DuckTask>> {
            self.constructs.push((block.get(), spec));
            Handle32::new(self.construct_answer)
        }
    }

    /// Builds the clone from the neighbouring level.
    fn wrong_clone(
        task: &DuckTask,
        manager: Option<Handle32<TaskMgr>>,
        pool: &mut Fake,
    ) -> Option<Handle32<DuckTask>> {
        let shifted = DuckTask::new(
            task.marks(),
            task.start(),
            task.span(),
            task.level().wrapping_add(1),
            task.done(),
            task.flagged(),
            task.tag(),
        );
        shifted.clone_task(manager, pool)
    }

    #[test]
    fn duck_clone_matches() {
        set_callee(1, alloc_addr());
        set_callee(2, construct_addr());
        let mut rng = Rng(0xD0CC);
        let mut caught = 0;
        let mut cases = 0;
        let managers = {
            let mut v = U32_EDGE.to_vec();
            for _ in 0..4 {
                v.push(rng.u32());
            }
            v
        };
        // Fresh blocks: null plus nonzero cookies the stubs never dereference.
        let blocks = [0u32, 1, 0x1000, rng.u32() | 1];
        // Levels: sign-extension edges plus random halves.
        let levels = {
            let mut v = vec![0u16, 1, 0x7FFF, 0x8000, 0xFFFF, 0xFFFE];
            for _ in 0..8 {
                v.push((rng.u32() & 0xFFFF) as u16);
            }
            v
        };
        let mut inputs = Vec::new();
        for &manager in &managers {
            for &fresh in &blocks {
                for &ans in &U32_EDGE {
                    for &level in &levels {
                        inputs.push((manager, fresh, ans, level, rng.u32(), rng.u32()));
                    }
                }
            }
        }
        for (manager, fresh, ans, level, span, tag) in inputs {
            let mut blob = *duck_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[D_SPAN] = span;
            blob[D_LEVEL] = (blob[D_LEVEL] & 0xFFFF_0000) | u32::from(level);
            set_blob_byte(&mut blob, D_TAG_BYTE, (tag & 0xFF) as u8);
            let boxed = Box::new(blob);
            let before = *boxed;
            set_manager(manager);
            ALLOC_ANS.store(fresh, Ordering::SeqCst);
            CONSTRUCT_ANS.store(ans, Ordering::SeqCst);
            ALLOC_COUNT.store(0, Ordering::SeqCst);
            CONSTRUCT_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00D4DE90::rw_00d4de90(addr(&boxed[0])) };
            assert_eq!(ALLOC_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(ALLOC_MGR.load(Ordering::SeqCst), manager);
            let want_constructs = u32::from(fresh != 0);
            assert_eq!(CONSTRUCT_COUNT.load(Ordering::SeqCst), want_constructs);
            let tag_byte = blob_byte(&boxed[..], D_TAG_BYTE);
            let level16 = (boxed[D_LEVEL] & 0xFFFF) as u16 as i16;
            if fresh != 0 {
                assert_eq!(CONSTRUCT_BLOCK.load(Ordering::SeqCst), fresh);
                assert_eq!(CONSTRUCT_TAG.load(Ordering::SeqCst), u32::from(tag_byte));
                assert_eq!(CONSTRUCT_SPAN.load(Ordering::SeqCst), span);
                assert_eq!(
                    CONSTRUCT_LEVEL.load(Ordering::SeqCst),
                    (level16 as i32) as u32
                );
            }
            let lift = DuckTask::new(0, 0, span, level16, false, false, tag_byte);
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
                assert_eq!(
                    fake.constructs[0],
                    (
                        fresh,
                        DuckSpec {
                            tag: tag_byte,
                            span,
                            level: level16,
                        }
                    )
                );
            }
            // The source blob survives whole.
            assert_eq!(*boxed, before);
            let mut wfake = Fake {
                alloc_answer: fresh,
                construct_answer: ans,
                allocs: Vec::new(),
                constructs: Vec::new(),
            };
            wrong_clone(&lift, Handle32::new(manager), &mut wfake);
            if fresh != 0
                && wfake.constructs.len() == 1
                && (wfake.constructs[0].1.level as i32) as u32
                    != CONSTRUCT_LEVEL.load(Ordering::SeqCst)
            {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong duck clone never caught ({cases} cases)");
    }
}
