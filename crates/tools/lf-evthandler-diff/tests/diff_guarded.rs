//! Differential case: the guarded-probe slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full handler, event and probe blobs,
//! the event probe, the registry check and release and the factory
//! pair (slot and arguments in order) against the lift's trait calls,
//! both sides given the same scripted answers. A deliberately wrong
//! lift must be caught at least once. One binary per slot shape, so the
//! stub registry is never shared. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_evthandler_diff::rewrites::*;
    use lf_evthandler_diff::{set_callee, set_manager};
    use lf_peds_tasks::event_handler::{
        ConvertRequest, EventHandler, EventProbe, EventSource, FactoryHandle, FactoryState,
        GuardedProbeAnswer, KIND_GUARDED_CONVERT, MARKER_MASK, MARKER_WANT, Owner, ProbeInput,
        ProbeRegistry, Task, TaskFactory, TaskManager,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        E_KIND, E_VTABLE, H_OWNER, H_PAD, H_PENDING, H_VTABLE, OWNER_REG_WORD, P_STATUS, Rng, addr,
        event_blob, fake_table, handler_blob, owner_blob, probe_blob,
    };

    // Probe stub (event table slot 13) and its recording.
    static PROBE_THIS: AtomicU32 = AtomicU32::new(0);
    static PROBE_ANS: AtomicU32 = AtomicU32::new(0);
    static PROBE_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn probe_stub(this: u32) -> u32 {
        PROBE_THIS.store(this, Ordering::SeqCst);
        PROBE_COUNT.fetch_add(1, Ordering::SeqCst);
        PROBE_ANS.load(Ordering::SeqCst)
    }

    fn probe_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = probe_stub;
        f as usize as u32
    }

    // Registry check stub (slot 2) and its recording.
    static CHECK_REG: AtomicU32 = AtomicU32::new(0);
    static CHECK_KIND: AtomicU32 = AtomicU32::new(0);
    static CHECK_ANS: AtomicU32 = AtomicU32::new(0);
    static CHECK_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn check_stub(reg: u32, kind: u32) -> u32 {
        CHECK_REG.store(reg, Ordering::SeqCst);
        CHECK_KIND.store(kind, Ordering::SeqCst);
        CHECK_COUNT.fetch_add(1, Ordering::SeqCst);
        CHECK_ANS.load(Ordering::SeqCst)
    }

    fn check_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = check_stub;
        f as usize as u32
    }

    // Lookup stub (slot 3) and its recording.
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

    // Convert stub (slot 4: probe plus a trailing zero) and its
    // recording.
    static CONVERT_H: AtomicU32 = AtomicU32::new(0);
    static CONVERT_FOUND: AtomicU32 = AtomicU32::new(0);
    static CONVERT_PAD: AtomicU32 = AtomicU32::new(0);
    static CONVERT_ANS: AtomicU32 = AtomicU32::new(0);
    static CONVERT_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn convert_stub(h: u32, found: u32, pad: u32) -> u32 {
        CONVERT_H.store(h, Ordering::SeqCst);
        CONVERT_FOUND.store(found, Ordering::SeqCst);
        CONVERT_PAD.store(pad, Ordering::SeqCst);
        CONVERT_COUNT.fetch_add(1, Ordering::SeqCst);
        CONVERT_ANS.load(Ordering::SeqCst)
    }

    fn convert_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 = convert_stub;
        f as usize as u32
    }

    // Release stub (slot 5) and its recording.
    static REL_REG: AtomicU32 = AtomicU32::new(0);
    static REL_FOUND: AtomicU32 = AtomicU32::new(0);
    static REL_ONE: AtomicU32 = AtomicU32::new(0);
    static REL_ANS: AtomicU32 = AtomicU32::new(0);
    static REL_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn release_stub(reg: u32, found: u32, one: u32) -> u32 {
        REL_REG.store(reg, Ordering::SeqCst);
        REL_FOUND.store(found, Ordering::SeqCst);
        REL_ONE.store(one, Ordering::SeqCst);
        REL_COUNT.fetch_add(1, Ordering::SeqCst);
        REL_ANS.load(Ordering::SeqCst)
    }

    fn release_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 = release_stub;
        f as usize as u32
    }

    struct FakeEvent {
        probe_answer: u32,
        probe_calls: u32,
    }

    impl EventSource for FakeEvent {
        fn poll_owner(&mut self) -> Option<Handle32<Owner>> {
            panic!("guarded cases never poll");
        }

        fn clone_task(&mut self) -> Option<Handle32<Task>> {
            panic!("guarded cases never clone");
        }

        fn probe_kind(&mut self) -> u32 {
            panic!("guarded cases never probe kinds");
        }

        fn readiness(&mut self) -> u32 {
            panic!("guarded cases never check readiness");
        }

        fn probe(&mut self) -> Option<Handle32<EventProbe>> {
            self.probe_calls += 1;
            Handle32::new(self.probe_answer)
        }
    }

    struct FakeRegistry {
        check_answer: u32,
        release_answer: u32,
        checks: Vec<(u32, u32)>,
        releases: Vec<(u32, u32)>,
    }

    impl ProbeRegistry for FakeRegistry {
        fn check(&mut self, owner: Handle32<Owner>, kind: u32) -> u32 {
            self.checks.push((owner.get(), kind));
            self.check_answer
        }

        fn release(&mut self, owner: Handle32<Owner>, probe: Handle32<EventProbe>) -> u32 {
            self.releases.push((owner.get(), probe.get()));
            self.release_answer
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

    /// Wrong lift: converts without the marker-bits gate.
    fn wrong_guarded(
        h: &mut EventHandler,
        owner: Handle32<Owner>,
        probe: ProbeInput,
        e: &mut FakeEvent,
        r: &mut FakeRegistry,
        f: &mut FakeFactory,
        st: &FactoryState,
    ) -> GuardedProbeAnswer {
        let forced = ProbeInput {
            kind: probe.kind,
            status: MARKER_WANT,
        };
        h.answer_guarded_probe(owner, forced, e, r, f, st)
    }

    fn answer_word(a: &GuardedProbeAnswer) -> u32 {
        match *a {
            GuardedProbeAnswer::Registry(r) => r,
            GuardedProbeAnswer::Rejected(w) => w,
            GuardedProbeAnswer::Converted { released, .. } => released,
        }
    }

    #[test]
    fn guarded_probe_matches() {
        set_callee(2, check_addr());
        set_callee(3, lookup_addr());
        set_callee(4, convert_addr());
        set_callee(5, release_addr());
        let mut rng = Rng(0x6AAD);
        let mut caught = 0;
        let mut cases = 0;
        // Status words: the marker pair set, set with neighbours, and
        // cleared in each position.
        let statuses = [
            MARKER_WANT,
            MARKER_WANT | 0x3F,
            MARKER_WANT | 0xFFFF_FC00,
            0u32,
            0x40,
            0x80,
            0x100,
            0x200,
            0x3C0 ^ 0x40,
            rng.u32(),
        ];
        let kinds = [KIND_GUARDED_CONVERT, 0u32, 1, 0x76B, 0x76D, rng.u32()];
        let mut inputs = Vec::new();
        for &kind in &kinds {
            for &status in &statuses {
                for &check in &[0u32, 1, rng.u32() | 1] {
                    for &live_probe in &[false, true] {
                        for &h in &[0u32, rng.u32() | 1] {
                            inputs.push((kind, status, check, live_probe, h));
                        }
                    }
                }
            }
        }
        for (kind, status, check, live_probe, h) in inputs {
            let manager = rng.u32();
            let conv = rng.u32();
            let rel = rng.u32();
            let reg = rng.u32();
            let pending = rng.u32();
            let mut o = owner_blob();
            for w in o.iter_mut() {
                *w = rng.u32();
            }
            o[OWNER_REG_WORD] = reg;
            let owner_addr = addr(&o[0]);
            let mut hh = handler_blob();
            hh[H_VTABLE] = rng.u32();
            hh[H_OWNER] = owner_addr;
            hh[H_PAD] = rng.u32();
            hh[H_PENDING] = pending;
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            let ptable = fake_table(0x40 / 4, 0x34 / 4, probe_addr());
            e[E_VTABLE] = addr(&ptable[0]);
            e[E_KIND] = kind;
            let mut p = probe_blob();
            for w in p.iter_mut() {
                *w = rng.u32();
            }
            p[P_STATUS] = status;
            let found = if live_probe { addr(&p[0]) } else { 0 };
            let h_before = *hh;
            let e_before = *e;
            let o_before = *o;
            let p_before = *p;
            let ev = addr(&e[0]);
            set_manager(manager);
            PROBE_ANS.store(found, Ordering::SeqCst);
            CHECK_ANS.store(check, Ordering::SeqCst);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            CONVERT_ANS.store(conv, Ordering::SeqCst);
            REL_ANS.store(rel, Ordering::SeqCst);
            PROBE_COUNT.store(0, Ordering::SeqCst);
            CHECK_COUNT.store(0, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            CONVERT_COUNT.store(0, Ordering::SeqCst);
            REL_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA65D0::rw_00ca65d0(addr(&hh[0]), ev, 0xB0B, 0xC0C) };
            assert_eq!(PROBE_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(PROBE_THIS.load(Ordering::SeqCst), ev);
            assert_eq!(CHECK_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(CHECK_REG.load(Ordering::SeqCst), reg);
            assert_eq!(CHECK_KIND.load(Ordering::SeqCst), kind);
            let gated = check == 0
                && kind == KIND_GUARDED_CONVERT
                && found != 0
                && status & MARKER_MASK == MARKER_WANT;
            let lookups = LOOKUP_COUNT.load(Ordering::SeqCst);
            assert_eq!(lookups, u32::from(gated));
            if gated {
                assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
            }
            let converts = CONVERT_COUNT.load(Ordering::SeqCst);
            assert_eq!(converts, u32::from(gated && h != 0));
            if gated && h != 0 {
                assert_eq!(CONVERT_H.load(Ordering::SeqCst), h);
                assert_eq!(CONVERT_FOUND.load(Ordering::SeqCst), found);
                assert_eq!(CONVERT_PAD.load(Ordering::SeqCst), 0);
            }
            let releases = REL_COUNT.load(Ordering::SeqCst);
            assert_eq!(releases, u32::from(gated));
            if gated {
                assert_eq!(REL_REG.load(Ordering::SeqCst), reg);
                assert_eq!(REL_FOUND.load(Ordering::SeqCst), found);
                assert_eq!(REL_ONE.load(Ordering::SeqCst), 1);
            }
            let mut lift = EventHandler::new(Handle32::new(owner_addr), Handle32::new(pending));
            let state = FactoryState::new(Handle32::new(manager));
            let mut event = FakeEvent {
                probe_answer: found,
                probe_calls: 0,
            };
            let mut registry = FakeRegistry {
                check_answer: check,
                release_answer: rel,
                checks: Vec::new(),
                releases: Vec::new(),
            };
            let mut factory = FakeFactory {
                lookup_answer: h,
                convert_answer: conv,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let owner = Handle32::new(owner_addr).expect("test owner address is live");
            let probe = ProbeInput { kind, status };
            let lift_ret = lift.answer_guarded_probe(
                owner,
                probe,
                &mut event,
                &mut registry,
                &mut factory,
                &state,
            );
            assert_eq!(got, answer_word(&lift_ret), "kind {kind:#x}");
            assert_eq!(event.probe_calls, 1);
            assert_eq!(registry.checks, vec![(owner_addr, kind)]);
            assert_eq!(factory.lookups, vec![manager; lookups as usize]);
            assert_eq!(factory.converts.len() as u32, converts, "kind {kind:#x}");
            if converts == 1 {
                let found_h = Handle32::new(found).expect("probe is live here");
                assert_eq!(factory.converts[0], (h, ConvertRequest::Probe(found_h)));
            }
            assert_eq!(registry.releases.len() as u32, releases, "kind {kind:#x}");
            if releases == 1 {
                assert_eq!(registry.releases[0], (owner_addr, found));
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hh, expect_h, "kind {kind:#x}");
            assert_eq!(*e, e_before, "kind {kind:#x}");
            assert_eq!(*o, o_before, "kind {kind:#x}");
            assert_eq!(*p, p_before, "kind {kind:#x}");
            let mut wlift = EventHandler::new(Handle32::new(owner_addr), Handle32::new(pending));
            let mut wevent = FakeEvent {
                probe_answer: found,
                probe_calls: 0,
            };
            let mut wregistry = FakeRegistry {
                check_answer: check,
                release_answer: rel,
                checks: Vec::new(),
                releases: Vec::new(),
            };
            let mut wfactory = FakeFactory {
                lookup_answer: h,
                convert_answer: conv,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let wrong_ret = wrong_guarded(
                &mut wlift,
                owner,
                probe,
                &mut wevent,
                &mut wregistry,
                &mut wfactory,
                &state,
            );
            // The wrong lift converts past a failed marker gate: its
            // release log or its answer must disagree with the stub's.
            if wregistry.releases.len() as u32 != releases || answer_word(&wrong_ret) != got {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong guarded never caught ({cases} cases)");
    }
}
