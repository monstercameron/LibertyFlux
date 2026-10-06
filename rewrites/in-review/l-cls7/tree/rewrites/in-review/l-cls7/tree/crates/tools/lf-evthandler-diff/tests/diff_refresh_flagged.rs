//! Differential case: the flagged-owner refresh slot against its
//! verified rewrite on the same generated inputs.
//!
//! Compares the return value (zero, the owner, or the conversion), the
//! full handler, event and owner blobs, and the two factory calls
//! against the lift's trait calls, both sides given the same scripted
//! answers; the float word is compared bit for bit. A deliberately
//! wrong lift must be caught at least once. One binary per factory
//! slot, so the stub registry is never shared. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_evthandler_diff::rewrites::*;
    use lf_evthandler_diff::{set_callee1, set_callee2, set_manager};
    use lf_peds_tasks::event_handler::{
        ConvertRequest, EventHandler, FactoryHandle, FactoryState, FlaggedAnswer,
        OWNER_REFRESH_FLAG, Task, TaskFactory, TaskManager,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        E_FLOAT, E_KIND, E_SUBJECT, H_OWNER, H_PAD, H_PENDING, H_VTABLE, OWNER_FLAGS_BYTE, Rng,
        addr, blob_byte, event_blob, handler_blob, owner_blob, set_blob_byte,
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

    // Worker stub (slot 2: handle, subject, kind, float bits, trailing
    // zero) and its recording.
    static WORK_H: AtomicU32 = AtomicU32::new(0);
    static WORK_SUBJECT: AtomicU32 = AtomicU32::new(0);
    static WORK_KIND: AtomicU32 = AtomicU32::new(0);
    static WORK_FBITS: AtomicU32 = AtomicU32::new(0);
    static WORK_TRAIL: AtomicU32 = AtomicU32::new(0);
    static WORK_ANS: AtomicU32 = AtomicU32::new(0);
    static WORK_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn worker_stub(
        h: u32,
        subject: u32,
        kind: u32,
        fbits: u32,
        trail: u32,
    ) -> u32 {
        WORK_H.store(h, Ordering::SeqCst);
        WORK_SUBJECT.store(subject, Ordering::SeqCst);
        WORK_KIND.store(kind, Ordering::SeqCst);
        WORK_FBITS.store(fbits, Ordering::SeqCst);
        WORK_TRAIL.store(trail, Ordering::SeqCst);
        WORK_COUNT.fetch_add(1, Ordering::SeqCst);
        WORK_ANS.load(Ordering::SeqCst)
    }

    fn lookup_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = lookup_stub;
        f as usize as u32
    }

    fn worker_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 = worker_stub;
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

    /// Tests the flag bit backwards (the bug: flagged owners are kept
    /// and unflagged ones convert).
    fn wrong_flipped_flag(
        h: &mut EventHandler,
        flags: u8,
        subject: u32,
        kind: u32,
        weight: f32,
        f: &mut Fake,
        s: &FactoryState,
    ) -> FlaggedAnswer {
        h.refresh_flagged_owner(
            flags ^ OWNER_REFRESH_FLAG,
            Handle32::new(subject),
            kind,
            weight,
            f,
            s,
        )
    }

    /// Float words: zeroes, ones, infinities, quiet NaNs with two
    /// payloads, a signalling bit pattern, the smallest denormal, and
    /// fraction edges. Compared bit for bit throughout.
    fn float_words(rng: &mut Rng) -> Vec<u32> {
        let mut v = vec![
            0x0000_0000,
            0x8000_0000,
            0x3F80_0000,
            0xBF80_0000,
            0x7F80_0000,
            0xFF80_0000,
            0x7FC0_0000,
            0x7FC0_0001,
            0x7F80_0001,
            0x0000_0001,
            0x3F7F_FFFF,
            0x4B00_0000,
        ];
        for _ in 0..4 {
            v.push(rng.u32());
        }
        v
    }

    #[test]
    fn refresh_flagged_owner_matches() {
        set_callee1(lookup_addr());
        set_callee2(worker_addr());
        let mut rng = Rng(0xF1A6);
        let mut caught = 0;
        let mut cases = 0;
        // Owner presence crossed with flag bytes around the refresh bit.
        let owners = [false, true];
        let flags = [0u8, 1, 2, 3, 4, 5, 0xFB, 0xFF];
        let fwords = float_words(&mut rng);
        let handles = [0u32, 1, 0xFFFF_FFFF, rng.u32()];
        let mut inputs = Vec::new();
        for &live_owner in &owners {
            for &flags in &flags {
                for &fbits in &fwords {
                    for &h in &handles {
                        inputs.push((rng.u32(), h, rng.u32(), live_owner, flags, fbits));
                    }
                }
            }
        }
        for (manager, h, ans, live_owner, flags, fbits) in inputs {
            let mut own = owner_blob();
            for w in own.iter_mut() {
                *w = rng.u32();
            }
            set_blob_byte(&mut own[..], OWNER_FLAGS_BYTE, flags);
            let owner_addr = if live_owner { addr(&own[0]) } else { 0 };
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
            let subject = rng.u32();
            let kind = rng.u32();
            e[E_SUBJECT] = subject;
            e[E_KIND] = kind;
            e[E_FLOAT] = fbits;
            let h_before = *hh;
            let e_before = *e;
            let o_before = own.clone();
            set_manager(manager);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            WORK_ANS.store(ans, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            WORK_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA8D60::rw_00ca8d60(addr(&hh[0]), addr(&e[0]), 0xB0B, 0xC0C) };
            let flagged = flags & OWNER_REFRESH_FLAG != 0;
            let converting = live_owner && flagged;
            if !live_owner {
                assert_eq!(got, 0);
            } else if !flagged {
                assert_eq!(got, owner_addr);
            }
            let want_lookups = u32::from(converting);
            assert_eq!(LOOKUP_COUNT.load(Ordering::SeqCst), want_lookups);
            if converting {
                assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
            }
            let want_converts = u32::from(converting && h != 0);
            assert_eq!(WORK_COUNT.load(Ordering::SeqCst), want_converts);
            if converting && h != 0 {
                assert_eq!(WORK_H.load(Ordering::SeqCst), h);
                assert_eq!(WORK_SUBJECT.load(Ordering::SeqCst), subject);
                assert_eq!(WORK_KIND.load(Ordering::SeqCst), kind);
                assert_eq!(WORK_FBITS.load(Ordering::SeqCst), fbits);
                assert_eq!(WORK_TRAIL.load(Ordering::SeqCst), 0);
            }
            let weight = f32::from_bits(fbits);
            let mut lift = EventHandler::new(Handle32::new(owner_addr), Handle32::new(pending));
            let state = FactoryState::new(Handle32::new(manager));
            let mut fake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let flag_byte = if live_owner {
                blob_byte(&own[..], OWNER_FLAGS_BYTE)
            } else {
                0
            };
            let lift_ret = lift.refresh_flagged_owner(
                flag_byte,
                Handle32::new(subject),
                kind,
                weight,
                &mut fake,
                &state,
            );
            let lift_word = match lift_ret {
                FlaggedAnswer::NoOwner => 0,
                FlaggedAnswer::KeepOwner(o) => o.get(),
                FlaggedAnswer::Converted(t) => Handle32::raw_or_zero(t),
            };
            assert_eq!(got, lift_word);
            assert_eq!(fake.lookups.len() as u32, want_lookups);
            if converting {
                assert_eq!(fake.lookups, vec![manager]);
            }
            assert_eq!(fake.converts.len() as u32, want_converts);
            if converting && h != 0 {
                assert_eq!(
                    fake.converts[0],
                    (
                        h,
                        ConvertRequest::SubjectKindFloat {
                            subject: Handle32::new(subject),
                            kind,
                            weight_bits: fbits,
                        }
                    )
                );
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hh, expect_h);
            assert_eq!(*e, e_before);
            assert_eq!(*own, *o_before);
            // The wrong lift flips the gate: caught wherever the owner
            // is live (the flipped bit takes the other path).
            let mut w = EventHandler::new(Handle32::new(owner_addr), Handle32::new(pending));
            let mut wfake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let w_ret =
                wrong_flipped_flag(&mut w, flag_byte, subject, kind, weight, &mut wfake, &state);
            let w_word = match w_ret {
                FlaggedAnswer::NoOwner => 0,
                FlaggedAnswer::KeepOwner(o) => o.get(),
                FlaggedAnswer::Converted(t) => Handle32::raw_or_zero(t),
            };
            if w_word != got || wfake.lookups.len() as u32 != want_lookups {
                caught += 1;
            }
            cases += 1;
        }
        assert!(
            caught > 0,
            "wrong flagged refresh never caught ({cases} cases)"
        );
    }
}
