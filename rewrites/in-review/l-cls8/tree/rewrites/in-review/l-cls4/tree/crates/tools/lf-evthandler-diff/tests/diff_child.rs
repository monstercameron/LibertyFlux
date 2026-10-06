//! Differential case: the child slot against its verified rewrite on
//! the same generated inputs.
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
        ConvertRequest, EventChild, EventHandler, EventRef, FactoryAnswer, FactoryHandle,
        FactoryState, Task, TaskFactory, TaskManager,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, addr, event_blob, handler_blob};

    /// Event blob word holding the child (byte offset 0xC).
    const E_CHILD: usize = 3;

    // Lookup stub (slot 1) and its recording.
    static LOOKUP_MGR: AtomicU32 = AtomicU32::new(0);
    static LOOKUP_ANS: AtomicU32 = AtomicU32::new(0);
    static LOOKUP_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn lookup_stub(mgr: u32) -> u32 {
        LOOKUP_MGR.store(mgr, Ordering::SeqCst);
        LOOKUP_COUNT.fetch_add(1, Ordering::SeqCst);
        LOOKUP_ANS.load(Ordering::SeqCst)
    }

    // Convert stub (slot 2, one request word) and its recording.
    static CONVERT_H: AtomicU32 = AtomicU32::new(0);
    static CONVERT_CHILD: AtomicU32 = AtomicU32::new(0);
    static CONVERT_ANS: AtomicU32 = AtomicU32::new(0);
    static CONVERT_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn convert_stub(h: u32, child: u32) -> u32 {
        CONVERT_H.store(h, Ordering::SeqCst);
        CONVERT_CHILD.store(child, Ordering::SeqCst);
        CONVERT_COUNT.fetch_add(1, Ordering::SeqCst);
        CONVERT_ANS.load(Ordering::SeqCst)
    }

    fn lookup_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = lookup_stub;
        f as usize as u32
    }

    fn convert_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = convert_stub;
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

    fn answer_word(answer: FactoryAnswer) -> u32 {
        match answer {
            FactoryAnswer::Passthrough(e) => e.get(),
            FactoryAnswer::Converted(t) => Handle32::raw_or_zero(t),
        }
    }

    /// Always passes through, never converts.
    fn wrong_passthrough(
        h: &mut EventHandler,
        event: Handle32<EventRef>,
        child: Option<Handle32<EventChild>>,
        f: &mut Fake,
        s: &FactoryState,
    ) -> FactoryAnswer {
        let _ = (h, child, f, s);
        FactoryAnswer::Passthrough(event)
    }

    #[test]
    fn child_or_passthrough_matches() {
        set_callee1(lookup_addr());
        set_callee2(convert_addr());
        let mut rng = Rng(0xC41D);
        let mut caught = 0;
        let mut cases = 0;
        let mut children = vec![0u32, 1, 0x8000_0000, 0xFFFF_FFFF];
        for _ in 0..8 {
            children.push(rng.u32());
        }
        let mut inputs = Vec::new();
        for &child in &children {
            for &manager in &[0u32, 1, rng.u32()] {
                for &h in &[0u32, 0xBEEF, rng.u32() | 1] {
                    for &ans in &[0u32, rng.u32()] {
                        inputs.push((child, manager, h, ans, rng.u32(), rng.u32()));
                    }
                }
            }
        }
        for (child, manager, h, ans, owner, pending) in inputs {
            let mut hh = handler_blob();
            hh[H_VTABLE] = rng.u32();
            hh[H_OWNER] = owner;
            hh[H_PAD] = rng.u32();
            hh[H_PENDING] = pending;
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            e[E_CHILD] = child;
            let h_before = *hh;
            let e_before = *e;
            let ev = addr(&e[0]);
            set_manager(manager);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            CONVERT_ANS.store(ans, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            CONVERT_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA4F70::rw_00ca4f70(addr(&hh[0]), ev, 0xB0B, 0xC0C) };
            let converts = child != 0;
            assert_eq!(
                LOOKUP_COUNT.load(Ordering::SeqCst),
                u32::from(converts),
                "child {child:#x}"
            );
            if converts {
                assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
                assert_eq!(CONVERT_COUNT.load(Ordering::SeqCst), u32::from(h != 0));
                if h != 0 {
                    assert_eq!(CONVERT_H.load(Ordering::SeqCst), h);
                    assert_eq!(CONVERT_CHILD.load(Ordering::SeqCst), child);
                }
            } else {
                assert_eq!(got, ev, "pass-through must answer the event");
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
            let event = Handle32::new(ev).expect("test event address is live");
            let lift_ret =
                lift.answer_child(event, Handle32::new(child), &mut fake, &state);
            assert_eq!(got, answer_word(lift_ret), "child {child:#x}");
            assert_eq!(fake.lookups.len() as u32, u32::from(converts));
            if converts {
                assert_eq!(fake.lookups[0], manager);
                assert_eq!(fake.converts.len() as u32, u32::from(h != 0));
                if h != 0 {
                    let want = Handle32::new(child).expect("convert path has a child");
                    assert_eq!(fake.converts[0], (h, ConvertRequest::Child(want)));
                }
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hh, expect_h, "child {child:#x}");
            assert_eq!(*e, e_before, "child {child:#x}");
            let mut w = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let mut wfake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let w_ret = wrong_passthrough(&mut w, event, Handle32::new(child), &mut wfake, &state);
            if answer_word(w_ret) != got
                || w.pending() != lift.pending()
                || wfake.lookups.len() as u32 != LOOKUP_COUNT.load(Ordering::SeqCst)
                || wfake.converts.len() as u32 != CONVERT_COUNT.load(Ordering::SeqCst)
            {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong child never caught ({cases} cases)");
    }
}
