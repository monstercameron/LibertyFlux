//! Differential case: the type-gated slot against its verified rewrite
//! on the same generated inputs.
//!
//! Compares the return value, the full handler and event blobs, and the
//! two factory calls (slot and arguments in order) against the lift's
//! trait calls, both sides given the same scripted answers. A
//! deliberately wrong lift must be caught at least once. One binary per
//! factory slot, so the stub registry is never shared. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_evthandler_diff::rewrites::*;
    use lf_evthandler_diff::{set_callee1, set_callee2, set_manager};
    use lf_peds_tasks::event_handler::{
        ConvertRequest, EventHandler, FactoryHandle, FactoryState, GatedAnswer, Task, TaskFactory,
        TaskManager, KIND_CLEAR, KIND_GATED_CONVERT,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        E_KIND, H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, U32_EDGE, addr, event_blob, handler_blob,
    };

    // Lookup stub (slot 1) and its recording.
    static LOOKUP_MGR: AtomicU32 = AtomicU32::new(0);
    static LOOKUP_ANS: AtomicU32 = AtomicU32::new(0);
    static LOOKUP_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn lookup_stub(mgr: u32) -> u32 {
        LOOKUP_MGR.store(mgr, Ordering::SeqCst);
        LOOKUP_COUNT.fetch_add(1, Ordering::SeqCst);
        LOOKUP_ANS.load(Ordering::SeqCst)
    }

    // Convert stub (slot 2, no request word) and its recording.
    static CONVERT_H: AtomicU32 = AtomicU32::new(0);
    static CONVERT_ANS: AtomicU32 = AtomicU32::new(0);
    static CONVERT_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn convert_stub(h: u32) -> u32 {
        CONVERT_H.store(h, Ordering::SeqCst);
        CONVERT_COUNT.fetch_add(1, Ordering::SeqCst);
        CONVERT_ANS.load(Ordering::SeqCst)
    }

    fn lookup_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = lookup_stub;
        f as usize as u32
    }

    fn convert_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = convert_stub;
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

    fn gated_word(answer: GatedAnswer, kind: u32) -> u32 {
        match answer {
            GatedAnswer::Cleared | GatedAnswer::Ignored => kind,
            GatedAnswer::Converted(t) => Handle32::raw_or_zero(t),
        }
    }

    /// Confuses the two gated kinds: converts the clear kind too.
    fn wrong_gated(
        h: &mut EventHandler,
        kind: u32,
        f: &mut Fake,
        s: &FactoryState,
    ) -> GatedAnswer {
        let k = if kind == KIND_CLEAR {
            KIND_GATED_CONVERT
        } else {
            kind
        };
        h.answer_gated_type(k, f, s)
    }

    #[test]
    fn gated_type_matches() {
        set_callee1(lookup_addr());
        set_callee2(convert_addr());
        let mut rng = Rng(0x6A7E);
        let mut caught = 0;
        let mut cases = 0;
        let mut kinds = U32_EDGE.to_vec();
        // The clear kind, the convert kind, and their neighbours.
        kinds.extend([0xC7, 0xC8, 0xC9]);
        kinds.extend([0x25B, 0x25C, 0x25D]);
        for _ in 0..32 {
            kinds.push(rng.u32());
        }
        let mut inputs = Vec::new();
        for &kind in &kinds {
            for &manager in &[0u32, 1, rng.u32()] {
                for &h in &[0u32, 0xABCD, rng.u32() | 1] {
                    for &ans in &[0u32, 0xC8, rng.u32()] {
                        inputs.push((kind, manager, h, ans, rng.u32(), rng.u32()));
                    }
                }
            }
        }
        for (kind, manager, h, ans, owner, pending) in inputs {
            let hblob = handler_blob_box(&mut rng, owner, pending);
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            e[E_KIND] = kind;
            let h_before = *hblob;
            let e_before = *e;
            set_manager(manager);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            CONVERT_ANS.store(ans, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            CONVERT_COUNT.store(0, Ordering::SeqCst);
            let got =
                unsafe { fn_00CA5E60::rw_00ca5e60(addr(&hblob[0]), addr(&e[0]), 0xB0B, 0xC0C) };
            let converts = kind != KIND_CLEAR && kind == KIND_GATED_CONVERT;
            assert_eq!(
                LOOKUP_COUNT.load(Ordering::SeqCst),
                u32::from(converts),
                "kind {kind:#x}"
            );
            if converts {
                assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
                assert_eq!(CONVERT_COUNT.load(Ordering::SeqCst), u32::from(h != 0));
                if h != 0 {
                    assert_eq!(CONVERT_H.load(Ordering::SeqCst), h);
                }
            } else {
                assert_eq!(CONVERT_COUNT.load(Ordering::SeqCst), 0);
            }
            let mut lift = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let state = FactoryState::new(Handle32::new(manager));
            let mut fake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let lift_ret = lift.answer_gated_type(kind, &mut fake, &state);
            assert_eq!(got, gated_word(lift_ret, kind), "kind {kind:#x}");
            assert_eq!(fake.lookups.len() as u32, u32::from(converts));
            if converts {
                assert_eq!(fake.lookups[0], manager);
                assert_eq!(fake.converts.len() as u32, u32::from(h != 0));
                if h != 0 {
                    assert_eq!(fake.converts[0], (h, ConvertRequest::Plain));
                }
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hblob, expect_h, "kind {kind:#x}");
            assert_eq!(*e, e_before, "kind {kind:#x}");
            let mut w = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let mut wfake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let w_ret = wrong_gated(&mut w, kind, &mut wfake, &state);
            let stub_lookups = LOOKUP_COUNT.load(Ordering::SeqCst);
            let stub_converts = CONVERT_COUNT.load(Ordering::SeqCst);
            if gated_word(w_ret, kind) != got
                || w.pending() != lift.pending()
                || wfake.lookups.len() as u32 != stub_lookups
                || wfake.converts.len() as u32 != stub_converts
            {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong gated never caught ({cases} cases)");
    }

    fn handler_blob_box(rng: &mut Rng, owner: u32, pending: u32) -> Box<[u32; 4]> {
        let mut h = handler_blob();
        h[H_VTABLE] = rng.u32();
        h[H_OWNER] = owner;
        h[H_PAD] = rng.u32();
        h[H_PENDING] = pending;
        h
    }
}
