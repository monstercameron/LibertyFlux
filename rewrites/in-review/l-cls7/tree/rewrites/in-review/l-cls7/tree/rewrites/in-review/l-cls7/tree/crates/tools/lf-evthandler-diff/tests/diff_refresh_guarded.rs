//! Differential case: the owner-gated refresh slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value (the owner, the event, or the conversion),
//! the full handler, event and owner blobs, and the two factory calls
//! against the lift's trait calls, both sides given the same scripted
//! answers. A deliberately wrong lift must be caught at least once. One
//! binary per factory slot, so the stub registry is never shared.
//! 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_evthandler_diff::rewrites::*;
    use lf_evthandler_diff::{set_callee1, set_callee2, set_manager};
    use lf_peds_tasks::event_handler::{
        ConvertRequest, EventHandler, EventRef, FactoryHandle, FactoryState, GuardedAnswer, Owner,
        Task, TaskFactory, TaskManager,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        E_SUBJECT, H_OWNER, H_PAD, H_PENDING, H_VTABLE, OWNER_FLAGS_BYTE, Rng, U32_EDGE, addr,
        blob_byte, event_blob, handler_blob, owner_blob, set_blob_byte,
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

    // Worker stub (slot 2: handle, subject) and its recording.
    static WORK_H: AtomicU32 = AtomicU32::new(0);
    static WORK_SUBJECT: AtomicU32 = AtomicU32::new(0);
    static WORK_ANS: AtomicU32 = AtomicU32::new(0);
    static WORK_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn worker_stub(h: u32, subject: u32) -> u32 {
        WORK_H.store(h, Ordering::SeqCst);
        WORK_SUBJECT.store(subject, Ordering::SeqCst);
        WORK_COUNT.fetch_add(1, Ordering::SeqCst);
        WORK_ANS.load(Ordering::SeqCst)
    }

    fn lookup_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = lookup_stub;
        f as usize as u32
    }

    fn worker_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = worker_stub;
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

    /// Ignores the flag byte, always taking the unflagged path (the bug:
    /// the gate is missing, so a flagged owner still converts).
    fn wrong_ignores_flag(
        h: &mut EventHandler,
        owner: Handle32<Owner>,
        event: Handle32<EventRef>,
        subject: u32,
        f: &mut Fake,
        s: &FactoryState,
    ) -> GuardedAnswer {
        h.refresh_unless_owner_flagged(owner, 0, event, Handle32::new(subject), f, s)
    }

    #[test]
    fn refresh_unless_owner_flagged_matches() {
        set_callee1(lookup_addr());
        set_callee2(worker_addr());
        let mut rng = Rng(0x6A1D);
        let mut caught = 0;
        let mut cases = 0;
        // Flag bytes around the keep bit (bit 2): clear, set, and
        // neighbours that isolate it.
        let flags = [0u8, 1, 2, 3, 4, 5, 6, 7, 0xFB, 0xFF];
        let subjects = [0u32, 1, 0x8000_0000, 0xFFFF_FFFF, rng.u32() | 1];
        let handles = [0u32, 1, 0xFFFF_FFFF, rng.u32()];
        let mut inputs = Vec::new();
        for &flags in &flags {
            for &subject in &subjects {
                for &h in &handles {
                    for &ans in &U32_EDGE {
                        inputs.push((rng.u32(), h, ans, subject, flags));
                    }
                }
            }
        }
        for (manager, h, ans, subject, flags) in inputs {
            let mut own = owner_blob();
            for w in own.iter_mut() {
                *w = rng.u32();
            }
            set_blob_byte(&mut own[..], OWNER_FLAGS_BYTE, flags);
            let owner_addr = addr(&own[0]);
            let mut hh = handler_blob();
            hh[H_VTABLE] = rng.u32();
            hh[H_OWNER] = owner_addr;
            hh[H_PAD] = rng.u32();
            let pending = rng.u32();
            hh[H_PENDING] = pending;
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            e[E_SUBJECT] = subject;
            let h_before = *hh;
            let e_before = *e;
            let o_before = own.clone();
            let event_addr = addr(&e[0]);
            set_manager(manager);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            WORK_ANS.store(ans, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            WORK_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CAA5B0::rw_00caa5b0(addr(&hh[0]), event_addr, 0xB0B, 0xC0C) };
            let flagged = flags & 4 != 0;
            let live = subject != 0;
            if flagged {
                assert_eq!(got, owner_addr);
            } else if !live {
                assert_eq!(got, event_addr);
            }
            let want_lookups = u32::from(!flagged && live);
            assert_eq!(LOOKUP_COUNT.load(Ordering::SeqCst), want_lookups);
            if !flagged && live {
                assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
            }
            let want_converts = u32::from(!flagged && live && h != 0);
            assert_eq!(WORK_COUNT.load(Ordering::SeqCst), want_converts);
            if !flagged && live && h != 0 {
                assert_eq!(WORK_H.load(Ordering::SeqCst), h);
                assert_eq!(WORK_SUBJECT.load(Ordering::SeqCst), subject);
            }
            let owner: Handle32<Owner> = Handle32::new(owner_addr).unwrap();
            let event: Handle32<EventRef> = Handle32::new(event_addr).unwrap();
            let mut lift = EventHandler::new(Some(owner), Handle32::new(pending));
            let state = FactoryState::new(Handle32::new(manager));
            let mut fake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let flag_byte = blob_byte(&own[..], OWNER_FLAGS_BYTE);
            let lift_ret = lift.refresh_unless_owner_flagged(
                owner,
                flag_byte,
                event,
                Handle32::new(subject),
                &mut fake,
                &state,
            );
            let lift_word = match lift_ret {
                GuardedAnswer::KeepOwner(o) => o.get(),
                GuardedAnswer::KeepEvent(e) => e.get(),
                GuardedAnswer::Converted(t) => Handle32::raw_or_zero(t),
            };
            assert_eq!(got, lift_word);
            assert_eq!(fake.lookups.len() as u32, want_lookups);
            if !flagged && live {
                assert_eq!(fake.lookups, vec![manager]);
            }
            assert_eq!(fake.converts.len() as u32, want_converts);
            if !flagged && live && h != 0 {
                let subject_h = Handle32::new(subject).unwrap();
                assert_eq!(fake.converts[0], (h, ConvertRequest::Subject(subject_h)));
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hh, expect_h);
            assert_eq!(*e, e_before);
            assert_eq!(*own, *o_before);
            // The wrong lift converts flagged owners too: caught
            // wherever the flag is set but the subject is live (the
            // stub made no calls there).
            let mut w = EventHandler::new(Some(owner), Handle32::new(pending));
            let mut wfake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let w_ret = wrong_ignores_flag(&mut w, owner, event, subject, &mut wfake, &state);
            let w_word = match w_ret {
                GuardedAnswer::KeepOwner(o) => o.get(),
                GuardedAnswer::KeepEvent(e) => e.get(),
                GuardedAnswer::Converted(t) => Handle32::raw_or_zero(t),
            };
            if w_word != got || !wfake.lookups.is_empty() && want_lookups == 0 {
                caught += 1;
            }
            cases += 1;
        }
        assert!(
            caught > 0,
            "wrong guarded refresh never caught ({cases} cases)"
        );
    }
}
