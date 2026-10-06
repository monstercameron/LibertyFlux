//! Differential case: the mark-seen refresh slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full handler and event blobs (the
//! seen byte is the only event change), and the two factory calls (slot
//! and arguments in order) against the lift's trait calls, both sides
//! given the same scripted answers. A deliberately wrong lift must be
//! caught at least once. One binary per factory slot, so the stub
//! registry is never shared. 32-bit only.

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
    use support::{
        E_KIND, E_SEEN_BYTE, E_SUBJECT, H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, U32_EDGE, addr,
        blob_byte, event_blob, handler_blob, set_blob_byte,
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

    // Worker stub (slot 2: handle, subject, kind) and its recording.
    static WORK_H: AtomicU32 = AtomicU32::new(0);
    static WORK_SUBJECT: AtomicU32 = AtomicU32::new(0);
    static WORK_KIND: AtomicU32 = AtomicU32::new(0);
    static WORK_ANS: AtomicU32 = AtomicU32::new(0);
    static WORK_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn worker_stub(h: u32, subject: u32, kind: u32) -> u32 {
        WORK_H.store(h, Ordering::SeqCst);
        WORK_SUBJECT.store(subject, Ordering::SeqCst);
        WORK_KIND.store(kind, Ordering::SeqCst);
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

    /// Never marks the event seen (the bug: the flag write is missing).
    fn wrong_never_marks(
        h: &mut EventHandler,
        subject: u32,
        kind: u32,
        seen: &mut bool,
        f: &mut Fake,
        s: &FactoryState,
    ) -> Option<Handle32<Task>> {
        let mut discard = *seen;
        let ret = h.refresh_marking_seen(Handle32::new(subject), kind, &mut discard, f, s);
        // `seen` untouched: the write was forgotten.
        let _ = discard;
        ret
    }

    #[test]
    fn refresh_marking_seen_matches() {
        set_callee1(lookup_addr());
        set_callee2(worker_addr());
        let mut rng = Rng(0x5EE0);
        let mut caught = 0;
        let mut cases = 0;
        // Seen-byte initials: clear, set, and two foreign values the
        // slot overwrites with 1 either way.
        let seens = [0u8, 1, 2, 0xFF];
        let managers = {
            let mut v = U32_EDGE.to_vec();
            for _ in 0..4 {
                v.push(rng.u32());
            }
            v
        };
        let handles = [0u32, 1, 0x1234_5678, 0xFFFF_FFFF, rng.u32()];
        let answers = [0u32, 1, 0x8000_0000, 0xFFFF_FFFF, rng.u32(), rng.u32()];
        // Phase A: factory answers crossed, event words sampled.
        let mut inputs = Vec::new();
        for &manager in &managers {
            for &h in &handles {
                for &ans in &answers {
                    inputs.push((
                        manager,
                        h,
                        ans,
                        rng.u32(),
                        rng.u32(),
                        seens[rng.u32() as usize % 4],
                    ));
                }
            }
        }
        // Phase B: subject and kind edges crossed, factory sampled.
        // Every subject below runs once with a clear seen byte, so the
        // wrong lift (which never marks) must be caught.
        for &subject in &U32_EDGE {
            for &kind in &U32_EDGE {
                inputs.push((
                    rng.u32(),
                    handles[rng.u32() as usize % 5],
                    rng.u32(),
                    subject,
                    kind,
                    0,
                ));
            }
        }
        for (manager, h, ans, subject, kind, seen_init) in inputs {
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
            e[E_KIND] = kind;
            set_blob_byte(&mut e[..], E_SEEN_BYTE, seen_init);
            let h_before = *hh;
            let e_before = *e;
            set_manager(manager);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            WORK_ANS.store(ans, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            WORK_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA8DB0::rw_00ca8db0(addr(&hh[0]), addr(&e[0]), 0xB0B, 0xC0C) };
            assert_eq!(LOOKUP_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
            let want_converts = u32::from(h != 0);
            assert_eq!(WORK_COUNT.load(Ordering::SeqCst), want_converts);
            if h != 0 {
                assert_eq!(WORK_H.load(Ordering::SeqCst), h);
                assert_eq!(WORK_SUBJECT.load(Ordering::SeqCst), subject);
                assert_eq!(WORK_KIND.load(Ordering::SeqCst), kind);
            }
            // The slot marks the event seen whatever it held.
            assert_eq!(blob_byte(&e[..], E_SEEN_BYTE), 1);
            let mut lift = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let state = FactoryState::new(Handle32::new(manager));
            let mut fake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let mut seen = seen_init != 0;
            let lift_ret = lift.refresh_marking_seen(
                Handle32::new(subject),
                kind,
                &mut seen,
                &mut fake,
                &state,
            );
            assert!(seen);
            assert_eq!(got, Handle32::raw_or_zero(lift_ret));
            assert_eq!(fake.lookups, vec![manager]);
            assert_eq!(fake.converts.len() as u32, want_converts);
            if h != 0 {
                assert_eq!(
                    fake.converts[0],
                    (
                        h,
                        ConvertRequest::SubjectKind {
                            subject: Handle32::new(subject),
                            kind
                        }
                    )
                );
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hh, expect_h);
            let mut expect_e = e_before;
            set_blob_byte(&mut expect_e, E_SEEN_BYTE, 1);
            assert_eq!(*e, expect_e);
            // The wrong lift never marks: caught wherever the byte
            // started clear (the stub always sets it).
            let mut w = EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let mut wfake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let mut wseen = seen_init != 0;
            wrong_never_marks(&mut w, subject, kind, &mut wseen, &mut wfake, &state);
            if !wseen && blob_byte(&e[..], E_SEEN_BYTE) == 1 {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong mark-seen never caught ({cases} cases)");
    }
}
