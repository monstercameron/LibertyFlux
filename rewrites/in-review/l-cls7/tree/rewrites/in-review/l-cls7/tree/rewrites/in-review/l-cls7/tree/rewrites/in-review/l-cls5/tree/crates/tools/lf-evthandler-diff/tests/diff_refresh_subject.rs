//! Differential case: the subject refresh slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value (the event on the pass-through path, the
//! conversion otherwise), the full handler and event blobs, and the two
//! factory calls against the lift's trait calls, both sides given the
//! same scripted answers. A deliberately wrong lift must be caught at
//! least once. One binary per factory slot, so the stub registry is
//! never shared. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_evthandler_diff::rewrites::*;
    use lf_evthandler_diff::{set_callee1, set_callee2, set_manager};
    use lf_peds_tasks::event_handler::{
        ConvertRequest, EventHandler, EventRef, FactoryAnswer, FactoryHandle, FactoryState, Task,
        TaskFactory, TaskManager,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        E_SUBJECT, H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, U32_EDGE, addr, event_blob,
        handler_blob,
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

    // Worker stub (slot 2: handle, subject, trailing zero) and its recording.
    static WORK_H: AtomicU32 = AtomicU32::new(0);
    static WORK_SUBJECT: AtomicU32 = AtomicU32::new(0);
    static WORK_TRAIL: AtomicU32 = AtomicU32::new(0);
    static WORK_ANS: AtomicU32 = AtomicU32::new(0);
    static WORK_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn worker_stub(h: u32, subject: u32, trail: u32) -> u32 {
        WORK_H.store(h, Ordering::SeqCst);
        WORK_SUBJECT.store(subject, Ordering::SeqCst);
        WORK_TRAIL.store(trail, Ordering::SeqCst);
        WORK_COUNT.fetch_add(1, Ordering::SeqCst);
        WORK_ANS.load(Ordering::SeqCst)
    }

    fn lookup_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = lookup_stub;
        f as usize as u32
    }

    fn worker_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 = worker_stub;
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

    /// Always passes the event through, never converting (the bug: the
    /// null-subject check answers every subject).
    fn wrong_always_passes(event: Handle32<EventRef>) -> FactoryAnswer {
        FactoryAnswer::Passthrough(event)
    }

    #[test]
    fn refresh_for_subject_matches() {
        set_callee1(lookup_addr());
        set_callee2(worker_addr());
        let mut rng = Rng(0x5EED);
        let mut caught = 0;
        let mut cases = 0;
        // Null subjects are crossed with everything (the pass-through
        // path makes no calls); live ones convert.
        let subjects = {
            let mut v = vec![0u32, 0, 1, 0x8000_0000, 0xFFFF_FFFF];
            for _ in 0..3 {
                v.push(rng.u32() | 1);
            }
            v
        };
        let managers = {
            let mut v = U32_EDGE.to_vec();
            for _ in 0..4 {
                v.push(rng.u32());
            }
            v
        };
        let handles = [0u32, 1, 0x1234_5678, 0xFFFF_FFFF, rng.u32()];
        let answers = [0u32, 1, 0x8000_0000, 0xFFFF_FFFF, rng.u32(), rng.u32()];
        let mut inputs = Vec::new();
        for &subject in &subjects {
            for &manager in &managers {
                for &h in &handles {
                    for &ans in &answers {
                        inputs.push((manager, h, ans, subject));
                    }
                }
            }
        }
        for (manager, h, ans, subject) in inputs {
            let mut hh = handler_blob();
            let owner = rng.u32();
            let pending = rng.u32();
            hh[H_VTABLE] = rng.u32();
            hh[H_OWNER] = owner;
            hh[H_PAD] = rng.u32();
            hh[H_PENDING] = pending;
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            e[E_SUBJECT] = subject;
            let h_before = *hh;
            let e_before = *e;
            let event_addr = addr(&e[0]);
            set_manager(manager);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            WORK_ANS.store(ans, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            WORK_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA8240::rw_00ca8240(addr(&hh[0]), event_addr, 0xB0B, 0xC0C) };
            let live = subject != 0;
            let want_lookups = u32::from(live);
            assert_eq!(LOOKUP_COUNT.load(Ordering::SeqCst), want_lookups);
            if live {
                assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
            }
            let want_converts = u32::from(live && h != 0);
            assert_eq!(WORK_COUNT.load(Ordering::SeqCst), want_converts);
            if live && h != 0 {
                assert_eq!(WORK_H.load(Ordering::SeqCst), h);
                assert_eq!(WORK_SUBJECT.load(Ordering::SeqCst), subject);
                assert_eq!(WORK_TRAIL.load(Ordering::SeqCst), 0);
            } else if !live {
                assert_eq!(got, event_addr);
            }
            let event: Handle32<EventRef> = Handle32::new(event_addr).unwrap();
            let mut lift = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let state = FactoryState::new(Handle32::new(manager));
            let mut fake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let lift_ret =
                lift.refresh_for_subject(event, Handle32::new(subject), &mut fake, &state);
            let lift_word = match lift_ret {
                FactoryAnswer::Passthrough(e) => e.get(),
                FactoryAnswer::Converted(t) => Handle32::raw_or_zero(t),
            };
            assert_eq!(got, lift_word);
            assert_eq!(fake.lookups.len() as u32, want_lookups);
            if live {
                assert_eq!(fake.lookups, vec![manager]);
            }
            assert_eq!(fake.converts.len() as u32, want_converts);
            if live && h != 0 {
                let subject_h = Handle32::new(subject).unwrap();
                assert_eq!(fake.converts[0], (h, ConvertRequest::Subject(subject_h)));
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hh, expect_h);
            assert_eq!(*e, e_before);
            // The wrong lift never converts: caught wherever the stub
            // looked the factory up.
            let w_ret = wrong_always_passes(event);
            let w_word = match w_ret {
                FactoryAnswer::Passthrough(e) => e.get(),
                FactoryAnswer::Converted(t) => Handle32::raw_or_zero(t),
            };
            if w_word != got || LOOKUP_COUNT.load(Ordering::SeqCst) != 0 {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong subject refresh never caught ({cases} cases)");
    }
}
