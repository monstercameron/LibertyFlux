//! Differential case: the routed-probe slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full handler and event blobs, the
//! kind probes, the readiness check, the two-stage lookup and the
//! factory pair (slot and arguments in order) against the lift's trait
//! calls, both sides given the same scripted answers. A deliberately
//! wrong lift must be caught at least once. One binary per slot shape,
//! so the stub registry is never shared. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_evthandler_diff::rewrites::*;
    use lf_evthandler_diff::{set_callee, set_manager};
    use lf_peds_tasks::event_handler::{
        ConvertRequest, EventHandler, EventLink, EventLinks, EventProbe, EventSource, FactoryHandle,
        FactoryState, KIND_CLEAR, KIND_ROUTED_CONVERT, Owner, PROBE_EARLY_KIND, PROBE_LATE_KIND,
        RoutedAnswer, StageHandle, StagedLookup, Task, TaskFactory, TaskManager,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        E_KIND, E_LINK2, E_PAYLOAD, E_VTABLE, H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, addr,
        event_blob, fake_table, handler_blob,
    };

    // Probe stub (event table slot 1) and its recording.
    static PROBE_THIS: AtomicU32 = AtomicU32::new(0);
    static PROBE_ANS: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];
    static PROBE_SEEN: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];
    static PROBE_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn probe_stub(this: u32) -> u32 {
        PROBE_THIS.store(this, Ordering::SeqCst);
        let i = PROBE_COUNT.fetch_add(1, Ordering::SeqCst) as usize;
        let a = PROBE_ANS[i.min(1)].load(Ordering::SeqCst);
        if i < 2 {
            PROBE_SEEN[i].store(a, Ordering::SeqCst);
        }
        a
    }

    fn probe_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = probe_stub;
        f as usize as u32
    }

    // Readiness stub (slot 2) and its recording.
    static READY_EV: AtomicU32 = AtomicU32::new(0);
    static READY_ANS: AtomicU32 = AtomicU32::new(0);
    static READY_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn ready_stub(ev: u32) -> u32 {
        READY_EV.store(ev, Ordering::SeqCst);
        READY_COUNT.fetch_add(1, Ordering::SeqCst);
        READY_ANS.load(Ordering::SeqCst)
    }

    fn ready_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = ready_stub;
        f as usize as u32
    }

    // Stage stub (slot 3) and its recording.
    static STAGE_A: AtomicU32 = AtomicU32::new(0);
    static STAGE_B: AtomicU32 = AtomicU32::new(0);
    static STAGE_ANS: AtomicU32 = AtomicU32::new(0);
    static STAGE_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "cdecl" fn stage_stub(a: u32, b: u32) -> u32 {
        STAGE_A.store(a, Ordering::SeqCst);
        STAGE_B.store(b, Ordering::SeqCst);
        STAGE_COUNT.fetch_add(1, Ordering::SeqCst);
        STAGE_ANS.load(Ordering::SeqCst)
    }

    fn stage_addr() -> u32 {
        let f: extern "cdecl" fn(u32, u32) -> u32 = stage_stub;
        f as usize as u32
    }

    // Finish stub (slot 4) and its recording.
    static FIN_M: AtomicU32 = AtomicU32::new(0);
    static FIN_A: AtomicU32 = AtomicU32::new(0);
    static FIN_B: AtomicU32 = AtomicU32::new(0);
    static FIN_ANS: AtomicU32 = AtomicU32::new(0);
    static FIN_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn finish_stub(m: u32, a: u32, b: u32) -> u32 {
        FIN_M.store(m, Ordering::SeqCst);
        FIN_A.store(a, Ordering::SeqCst);
        FIN_B.store(b, Ordering::SeqCst);
        FIN_COUNT.fetch_add(1, Ordering::SeqCst);
        FIN_ANS.load(Ordering::SeqCst)
    }

    fn finish_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 = finish_stub;
        f as usize as u32
    }

    // Lookup stub (slot 5) and its recording.
    static LOOKUP_MGR: AtomicU32 = AtomicU32::new(0);
    static LOOKUP_ANS: AtomicU32 = AtomicU32::new(0);
    static LOOKUP_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn lookup_stub(mgr: u32) -> u32 {
        LOOKUP_MGR.store(mgr, Ordering::SeqCst);
        LOOKUP_COUNT.fetch_add(1, Ordering::SeqCst);
        LOOKUP_ANS.load(Ordering::SeqCst)
    }

    fn lookup_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = lookup_stub;
        f as usize as u32
    }

    // Convert stub (slot 6, the link pair) and its recording.
    static CONVERT_H: AtomicU32 = AtomicU32::new(0);
    static CONVERT_A: AtomicU32 = AtomicU32::new(0);
    static CONVERT_B: AtomicU32 = AtomicU32::new(0);
    static CONVERT_ANS: AtomicU32 = AtomicU32::new(0);
    static CONVERT_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn convert_stub(h: u32, a: u32, b: u32) -> u32 {
        CONVERT_H.store(h, Ordering::SeqCst);
        CONVERT_A.store(a, Ordering::SeqCst);
        CONVERT_B.store(b, Ordering::SeqCst);
        CONVERT_COUNT.fetch_add(1, Ordering::SeqCst);
        CONVERT_ANS.load(Ordering::SeqCst)
    }

    fn convert_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 = convert_stub;
        f as usize as u32
    }

    struct FakeEvent {
        probes: [u32; 2],
        probe_calls: u32,
        ready_answer: u32,
        ready_calls: u32,
    }

    impl EventSource for FakeEvent {
        fn poll_owner(&mut self) -> Option<Handle32<Owner>> {
            panic!("routed cases never poll");
        }

        fn clone_task(&mut self) -> Option<Handle32<Task>> {
            panic!("routed cases never clone");
        }

        fn probe_kind(&mut self) -> u32 {
            let a = self.probes[(self.probe_calls as usize).min(1)];
            self.probe_calls += 1;
            a
        }

        fn readiness(&mut self) -> u32 {
            self.ready_calls += 1;
            self.ready_answer
        }

        fn probe(&mut self) -> Option<Handle32<EventProbe>> {
            panic!("routed cases never probe");
        }
    }

    struct FakeStaged {
        stage_answer: u32,
        finish_answer: u32,
        stages: Vec<(u32, u32)>,
        finishes: Vec<(u32, u32, u32)>,
    }

    impl StagedLookup for FakeStaged {
        fn stage(
            &mut self,
            primary: Option<Handle32<EventLink>>,
            secondary: Option<Handle32<EventLink>>,
        ) -> Option<Handle32<StageHandle>> {
            self.stages.push((
                Handle32::raw_or_zero(primary),
                Handle32::raw_or_zero(secondary),
            ));
            Handle32::new(self.stage_answer)
        }

        fn finish(
            &mut self,
            staged: Option<Handle32<StageHandle>>,
            primary: Option<Handle32<EventLink>>,
            secondary: Option<Handle32<EventLink>>,
        ) -> u32 {
            self.finishes.push((
                Handle32::raw_or_zero(staged),
                Handle32::raw_or_zero(primary),
                Handle32::raw_or_zero(secondary),
            ));
            self.finish_answer
        }
    }

    struct FakeFactory {
        lookup_answer: u32,
        convert_answer: u32,
        lookups: Vec<u32>,
        converts: Vec<(u32, ConvertRequest)>,
    }

    impl TaskFactory for FakeFactory {
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

    /// Wrong lift: gates the early probe on the primary link instead of
    /// the secondary one.
    fn wrong_routed(
        h: &mut EventHandler,
        links: EventLinks,
        kind: u32,
        e: &mut FakeEvent,
        s: &mut FakeStaged,
        f: &mut FakeFactory,
        st: &FactoryState,
    ) -> RoutedAnswer {
        let swapped = EventLinks {
            primary: links.secondary,
            secondary: links.primary,
        };
        h.answer_routed_probe(swapped, kind, e, s, f, st)
    }

    fn answer_word(a: &RoutedAnswer) -> u32 {
        match *a {
            RoutedAnswer::EarlyProbe(k) => k,
            RoutedAnswer::StoredNull => 0,
            RoutedAnswer::Unrouted(w) => w,
            RoutedAnswer::LookupFailed(f) => f,
            RoutedAnswer::Converted(t) => Handle32::raw_or_zero(t),
        }
    }

    #[test]
    fn routed_probe_matches() {
        set_callee(2, ready_addr());
        set_callee(3, stage_addr());
        set_callee(4, finish_addr());
        set_callee(5, lookup_addr());
        set_callee(6, convert_addr());
        let mut rng = Rng(0xB025);
        let mut caught = 0;
        let mut cases = 0;
        let kinds = [
            KIND_CLEAR,
            KIND_ROUTED_CONVERT,
            0u32,
            1,
            0xC7,
            0xC9,
            0xE8,
            0xEA,
            rng.u32(),
        ];
        let mut inputs = Vec::new();
        for &kind in &kinds {
            for &k1 in &[PROBE_EARLY_KIND, 0u32, rng.u32()] {
                for &k2 in &[PROBE_LATE_KIND, 0u32, rng.u32()] {
                    for &ready in &[0u32, 0x100, rng.u32() | 1] {
                        for &ebp in &[0u32, rng.u32() | 1] {
                            for &ebx in &[0u32, rng.u32() | 1] {
                                inputs.push((kind, k1, k2, ready, ebp, ebx));
                            }
                        }
                    }
                }
            }
        }
        for (kind, k1, k2, ready, ebp, ebx) in inputs {
            let mid = rng.u32();
            let fin = rng.u32();
            let manager = rng.u32();
            let h = rng.u32();
            let ans = rng.u32();
            let owner = rng.u32();
            let pending = rng.u32();
            let mut hh = handler_blob();
            hh[H_VTABLE] = rng.u32();
            hh[H_OWNER] = owner;
            hh[H_PAD] = rng.u32();
            hh[H_PENDING] = pending;
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            let ptable = fake_table(8, 1, probe_addr());
            e[E_VTABLE] = addr(&ptable[0]);
            e[E_KIND] = kind;
            e[E_PAYLOAD] = ebp;
            e[E_LINK2] = ebx;
            let h_before = *hh;
            let e_before = *e;
            let ev = addr(&e[0]);
            set_manager(manager);
            PROBE_ANS[0].store(k1, Ordering::SeqCst);
            PROBE_ANS[1].store(k2, Ordering::SeqCst);
            READY_ANS.store(ready, Ordering::SeqCst);
            STAGE_ANS.store(mid, Ordering::SeqCst);
            FIN_ANS.store(fin, Ordering::SeqCst);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            CONVERT_ANS.store(ans, Ordering::SeqCst);
            PROBE_COUNT.store(0, Ordering::SeqCst);
            READY_COUNT.store(0, Ordering::SeqCst);
            STAGE_COUNT.store(0, Ordering::SeqCst);
            FIN_COUNT.store(0, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            CONVERT_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA4FB0::rw_00ca4fb0(addr(&hh[0]), ev, 0xB0B, 0xC0C) };
            let probes = PROBE_COUNT.load(Ordering::SeqCst);
            assert!((1..=2).contains(&probes), "probe called {probes}x");
            assert_eq!(PROBE_THIS.load(Ordering::SeqCst), ev);
            let mut stub_probes = Vec::new();
            for i in 0..probes {
                stub_probes.push(PROBE_SEEN[i as usize].load(Ordering::SeqCst));
            }
            let readies = READY_COUNT.load(Ordering::SeqCst);
            assert!(readies <= 1);
            if readies == 1 {
                assert_eq!(READY_EV.load(Ordering::SeqCst), ev);
            }
            let stages = STAGE_COUNT.load(Ordering::SeqCst);
            assert!(stages <= 1);
            if stages == 1 {
                assert_eq!(STAGE_A.load(Ordering::SeqCst), ebp);
                assert_eq!(STAGE_B.load(Ordering::SeqCst), ebx);
            }
            let finishes = FIN_COUNT.load(Ordering::SeqCst);
            assert!(finishes <= 1);
            if finishes == 1 {
                assert_eq!(FIN_M.load(Ordering::SeqCst), mid);
                assert_eq!(FIN_A.load(Ordering::SeqCst), ebp);
                assert_eq!(FIN_B.load(Ordering::SeqCst), ebx);
            }
            let lookups = LOOKUP_COUNT.load(Ordering::SeqCst);
            assert!(lookups <= 1);
            if lookups == 1 {
                assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
            }
            let converts = CONVERT_COUNT.load(Ordering::SeqCst);
            assert!(converts <= 1);
            if converts == 1 {
                assert_eq!(CONVERT_H.load(Ordering::SeqCst), h);
                assert_eq!(CONVERT_A.load(Ordering::SeqCst), ebp);
                assert_eq!(CONVERT_B.load(Ordering::SeqCst), ebx);
            }
            let mut lift =
                EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let state = FactoryState::new(Handle32::new(manager));
            let mut event = FakeEvent {
                probes: [k1, k2],
                probe_calls: 0,
                ready_answer: ready,
                ready_calls: 0,
            };
            let mut staged = FakeStaged {
                stage_answer: mid,
                finish_answer: fin,
                stages: Vec::new(),
                finishes: Vec::new(),
            };
            let mut factory = FakeFactory {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let links = EventLinks {
                primary: Handle32::new(ebp),
                secondary: Handle32::new(ebx),
            };
            let lift_ret =
                lift.answer_routed_probe(links, kind, &mut event, &mut staged, &mut factory, &state);
            assert_eq!(got, answer_word(&lift_ret), "kind {kind:#x}");
            assert_eq!(event.probe_calls, probes, "kind {kind:#x}");
            let mut fake_probes = Vec::new();
            for i in 0..probes {
                fake_probes.push(event.probes[(i as usize).min(1)]);
            }
            assert_eq!(fake_probes, stub_probes, "kind {kind:#x}");
            assert_eq!(event.ready_calls, readies, "kind {kind:#x}");
            assert_eq!(staged.stages.len() as u32, stages, "kind {kind:#x}");
            if stages == 1 {
                assert_eq!(staged.stages[0], (ebp, ebx));
            }
            assert_eq!(staged.finishes.len() as u32, finishes, "kind {kind:#x}");
            if finishes == 1 {
                assert_eq!(staged.finishes[0], (mid, ebp, ebx));
            }
            assert_eq!(factory.lookups, vec![manager; lookups as usize]);
            assert_eq!(factory.converts.len() as u32, converts, "kind {kind:#x}");
            if converts == 1 {
                assert_eq!(
                    factory.converts[0],
                    (
                        h,
                        ConvertRequest::Pair {
                            primary: Handle32::new(ebp),
                            secondary: Handle32::new(ebx),
                        }
                    )
                );
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hh, expect_h, "kind {kind:#x}");
            assert_eq!(*e, e_before, "kind {kind:#x}");
            let mut wlift =
                EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let mut wevent = FakeEvent {
                probes: [k1, k2],
                probe_calls: 0,
                ready_answer: ready,
                ready_calls: 0,
            };
            let mut wstaged = FakeStaged {
                stage_answer: mid,
                finish_answer: fin,
                stages: Vec::new(),
                finishes: Vec::new(),
            };
            let mut wfactory = FakeFactory {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let wrong_ret = wrong_routed(
                &mut wlift,
                links,
                kind,
                &mut wevent,
                &mut wstaged,
                &mut wfactory,
                &state,
            );
            if wevent.probe_calls != probes || answer_word(&wrong_ret) != got {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong routed never caught ({cases} cases)");
    }
}
