//! Differential case: the response-build slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full handler blob (the event words
//! are unread), and the two factory calls (slot and arguments in
//! order) against the lift's trait calls, both sides given the same
//! scripted answers. A deliberately wrong lift must be caught at least
//! once. One binary per factory slot, so the stub registry is never
//! shared. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_evthandler_diff::rewrites::*;
    use lf_evthandler_diff::{set_callee1, set_callee2, set_manager};
    use lf_peds_tasks::event_handler::{
        ConvertRequest, EventHandler, FactoryHandle, FactoryState, Task, TaskFactory, TaskManager,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, U32_EDGE, addr, handler_blob};

    // Allocator stub (slot 1) and its recording.
    static ALLOC_MGR: AtomicU32 = AtomicU32::new(0);
    static ALLOC_ANS: AtomicU32 = AtomicU32::new(0);
    static ALLOC_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn alloc_stub(mgr: u32) -> u32 {
        ALLOC_MGR.store(mgr, Ordering::SeqCst);
        ALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
        ALLOC_ANS.load(Ordering::SeqCst)
    }

    // Builder stub (slot 2: object, fixed all-ones word) and its recording.
    static BUILD_OBJ: AtomicU32 = AtomicU32::new(0);
    static BUILD_WORD: AtomicU32 = AtomicU32::new(0);
    static BUILD_ANS: AtomicU32 = AtomicU32::new(0);
    static BUILD_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn build_stub(obj: u32, word: u32) -> u32 {
        BUILD_OBJ.store(obj, Ordering::SeqCst);
        BUILD_WORD.store(word, Ordering::SeqCst);
        BUILD_COUNT.fetch_add(1, Ordering::SeqCst);
        BUILD_ANS.load(Ordering::SeqCst)
    }

    fn alloc_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = alloc_stub;
        f as usize as u32
    }

    fn build_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = build_stub;
        f as usize as u32
    }

    struct Fake {
        lookup_answer: u32,
        convert_answer: u32,
        lookups: Vec<u32>,
        converts: Vec<(u32, ConvertRequest)>,
    }

    impl TaskFactory for Fake {
        fn lookup(
            &mut self,
            manager: Option<Handle32<TaskManager>>,
        ) -> Option<Handle32<FactoryHandle>> {
            self.lookups.push(Handle32::raw_or_zero(manager));
            Handle32::new(self.lookup_answer)
        }

        fn convert(
            &mut self,
            handle: Handle32<FactoryHandle>,
            request: ConvertRequest,
        ) -> Option<Handle32<Task>> {
            self.converts.push((handle.get(), request));
            Handle32::new(self.convert_answer)
        }
    }

    /// Stores the built response but answers the previous task (the
    /// bug: the return value lags the store by one call).
    fn wrong_answers_old(
        h: &mut EventHandler,
        f: &mut Fake,
        s: &FactoryState,
    ) -> Option<Handle32<Task>> {
        let old = h.pending();
        let _ = h.build_response(f, s);
        old
    }

    #[test]
    fn build_response_matches() {
        set_callee1(alloc_addr());
        set_callee2(build_addr());
        let mut rng = Rng(0xB011);
        let mut caught = 0;
        let mut cases = 0;
        let managers = {
            let mut v = U32_EDGE.to_vec();
            for _ in 0..4 {
                v.push(rng.u32());
            }
            v
        };
        let objects = [0u32, 1, 0x1234_5678, 0xFFFF_FFFF, rng.u32()];
        let mut inputs = Vec::new();
        // Phase A: factory answers crossed, previous task sampled.
        for &manager in &managers {
            for &obj in &objects {
                for &ans in &U32_EDGE {
                    inputs.push((manager, obj, ans, rng.u32()));
                }
            }
        }
        // Phase B: answer and previous task crossed on edges, so the
        // wrong lift (which answers the previous task) must be caught.
        for &ans in &U32_EDGE {
            for &pending in &U32_EDGE {
                inputs.push((rng.u32(), 0x1111_1111, ans, pending));
            }
        }
        for (manager, obj, ans, pending) in inputs {
            let mut hh = handler_blob();
            let owner = rng.u32();
            hh[H_VTABLE] = rng.u32();
            hh[H_OWNER] = owner;
            hh[H_PAD] = rng.u32();
            hh[H_PENDING] = pending;
            let h_before = *hh;
            set_manager(manager);
            ALLOC_ANS.store(obj, Ordering::SeqCst);
            BUILD_ANS.store(ans, Ordering::SeqCst);
            ALLOC_COUNT.store(0, Ordering::SeqCst);
            BUILD_COUNT.store(0, Ordering::SeqCst);
            // The slot ignores its stack words: sentinels prove it.
            let got = unsafe { fn_00CA7340::rw_00ca7340(addr(&hh[0]), 0xB0B, 0xC0C, 0xD0D) };
            assert_eq!(ALLOC_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(ALLOC_MGR.load(Ordering::SeqCst), manager);
            let want_builds = u32::from(obj != 0);
            assert_eq!(BUILD_COUNT.load(Ordering::SeqCst), want_builds);
            if obj != 0 {
                assert_eq!(BUILD_OBJ.load(Ordering::SeqCst), obj);
                assert_eq!(BUILD_WORD.load(Ordering::SeqCst), 0xFFFF_FFFF);
            }
            let mut lift = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let state = FactoryState::new(Handle32::new(manager));
            let mut fake = Fake {
                lookup_answer: obj,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let lift_ret = lift.build_response(&mut fake, &state);
            assert_eq!(got, Handle32::raw_or_zero(lift_ret));
            assert_eq!(fake.lookups, vec![manager]);
            assert_eq!(fake.converts.len() as u32, want_builds);
            if obj != 0 {
                assert_eq!(fake.converts[0], (obj, ConvertRequest::Build));
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hh, expect_h);
            // The wrong lift answers the previous task: caught
            // wherever the built response differs from it.
            let mut w = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let mut wfake = Fake {
                lookup_answer: obj,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let w_ret = wrong_answers_old(&mut w, &mut wfake, &state);
            if Handle32::raw_or_zero(w_ret) != got {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong build never caught ({cases} cases)");
    }
}
