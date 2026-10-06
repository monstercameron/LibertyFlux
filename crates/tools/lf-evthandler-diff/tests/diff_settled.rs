//! Differential case: the settle refresh slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full handler and owner blobs, the
//! owner ready probe, the follow-up check and the factory pair (slot
//! and arguments in order) against the lift's trait calls, both sides
//! given the same scripted answers. A deliberately wrong lift must be
//! caught at least once. One binary per slot shape, so the stub
//! registry is never shared. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_evthandler_diff::rewrites::*;
    use lf_evthandler_diff::{set_callee, set_manager};
    use lf_peds_tasks::event_handler::{
        ConvertRequest, EventHandler, FactoryHandle, FactoryState, Owner, SETTLE_BUILD_KIND,
        SettleStatus, SettledAnswer, Task, TaskFactory, TaskManager,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, addr, fake_table, handler_blob, owner_blob,
    };

    // Ready stub (owner table slot 0x128) and its recording.
    static READY_THIS: AtomicU32 = AtomicU32::new(0);
    static READY_ANS: AtomicU32 = AtomicU32::new(0);
    static READY_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn ready_stub(this: u32) -> u32 {
        READY_THIS.store(this, Ordering::SeqCst);
        READY_COUNT.fetch_add(1, Ordering::SeqCst);
        READY_ANS.load(Ordering::SeqCst)
    }

    fn ready_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = ready_stub;
        f as usize as u32
    }

    // Follow-up check stub (slot 2, no arguments) and its recording.
    static CHECK_ANS: AtomicU32 = AtomicU32::new(0);
    static CHECK_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "cdecl" fn check_stub() -> u32 {
        CHECK_COUNT.fetch_add(1, Ordering::SeqCst);
        CHECK_ANS.load(Ordering::SeqCst)
    }

    fn check_addr() -> u32 {
        let f: extern "cdecl" fn() -> u32 = check_stub;
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

    // Convert stub (slot 4, one request word) and its recording.
    static CONVERT_H: AtomicU32 = AtomicU32::new(0);
    static CONVERT_CODE: AtomicU32 = AtomicU32::new(0);
    static CONVERT_ANS: AtomicU32 = AtomicU32::new(0);
    static CONVERT_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn convert_stub(h: u32, code: u32) -> u32 {
        CONVERT_H.store(h, Ordering::SeqCst);
        CONVERT_CODE.store(code, Ordering::SeqCst);
        CONVERT_COUNT.fetch_add(1, Ordering::SeqCst);
        CONVERT_ANS.load(Ordering::SeqCst)
    }

    fn convert_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = convert_stub;
        f as usize as u32
    }

    struct FakeSettle {
        ready_answer: u32,
        check_answer: u32,
        readies: Vec<u32>,
        checks: u32,
    }

    impl SettleStatus for FakeSettle {
        fn owner_ready(&mut self, owner: Handle32<Owner>) -> u32 {
            self.readies.push(owner.get());
            self.ready_answer
        }

        fn confirm_settled(&mut self) -> u32 {
            self.checks += 1;
            self.check_answer
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

    /// Wrong lift: settles whenever the owner is ready, skipping the
    /// follow-up check.
    fn wrong_settled(
        h: &mut EventHandler,
        owner: Handle32<Owner>,
        s: &mut FakeSettle,
        f: &mut FakeFactory,
        st: &FactoryState,
    ) -> SettledAnswer {
        if s.owner_ready(owner) & 0xFF != 0 {
            // The check's answer without running the check.
            return SettledAnswer::Settled(s.check_answer);
        }
        h.refresh_unless_settled(owner, s, f, st)
    }

    fn answer_word(a: &SettledAnswer) -> u32 {
        match *a {
            SettledAnswer::Settled(c) => c,
            SettledAnswer::Refreshed(t) => Handle32::raw_or_zero(t),
        }
    }

    #[test]
    fn settled_refresh_matches() {
        set_callee(2, check_addr());
        set_callee(3, lookup_addr());
        set_callee(4, convert_addr());
        let mut rng = Rng(0x5E77);
        let mut caught = 0;
        let mut cases = 0;
        // Ready and check answers: the low byte decides, so cover set
        // and clear low bytes with noisy high bytes.
        let words = [
            0u32,
            0x100,
            0xFF00,
            1,
            0x101,
            0xFFFF_FFFF,
            rng.u32(),
            rng.u32() | 1,
        ];
        let mut inputs = Vec::new();
        for &ready in &words {
            for &check in &words {
                for &manager in &[0u32, rng.u32()] {
                    for &h in &[0u32, rng.u32() | 1] {
                        inputs.push((ready, check, manager, h, rng.u32()));
                    }
                }
            }
        }
        for (ready, check, manager, h, ans) in inputs {
            let mut o = owner_blob();
            for w in o.iter_mut() {
                *w = rng.u32();
            }
            let rtable = fake_table(0x130 / 4, 0x128 / 4, ready_addr());
            o[0] = addr(&rtable[0]);
            let owner_addr = addr(&o[0]);
            let mut hh = handler_blob();
            hh[H_VTABLE] = rng.u32();
            hh[H_OWNER] = owner_addr;
            hh[H_PAD] = rng.u32();
            hh[H_PENDING] = rng.u32();
            let h_before = *hh;
            let o_before = *o;
            set_manager(manager);
            READY_ANS.store(ready, Ordering::SeqCst);
            CHECK_ANS.store(check, Ordering::SeqCst);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            CONVERT_ANS.store(ans, Ordering::SeqCst);
            READY_COUNT.store(0, Ordering::SeqCst);
            CHECK_COUNT.store(0, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            CONVERT_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA76F0::rw_00ca76f0(addr(&hh[0]), 0xB0B, 0xC0C, 0xD0D) };
            assert_eq!(READY_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(READY_THIS.load(Ordering::SeqCst), owner_addr);
            let settled = ready & 0xFF != 0 && check & 0xFF != 0;
            // The check runs whenever the owner is ready, even when it
            // then fails its own low-byte test.
            assert_eq!(
                CHECK_COUNT.load(Ordering::SeqCst),
                u32::from(ready & 0xFF != 0)
            );
            let lookups = LOOKUP_COUNT.load(Ordering::SeqCst);
            assert_eq!(lookups, u32::from(!settled));
            if !settled {
                assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
            }
            let converts = CONVERT_COUNT.load(Ordering::SeqCst);
            assert_eq!(converts, u32::from(!settled && h != 0));
            if !settled && h != 0 {
                assert_eq!(CONVERT_H.load(Ordering::SeqCst), h);
                assert_eq!(CONVERT_CODE.load(Ordering::SeqCst), SETTLE_BUILD_KIND);
            }
            let mut lift = EventHandler::new(
                Handle32::new(owner_addr),
                Handle32::new(h_before[H_PENDING]),
            );
            let state = FactoryState::new(Handle32::new(manager));
            let mut settle = FakeSettle {
                ready_answer: ready,
                check_answer: check,
                readies: Vec::new(),
                checks: 0,
            };
            let mut factory = FakeFactory {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let owner = Handle32::new(owner_addr).expect("test owner address is live");
            let lift_ret = lift.refresh_unless_settled(owner, &mut settle, &mut factory, &state);
            assert_eq!(got, answer_word(&lift_ret));
            assert_eq!(settle.readies, vec![owner_addr]);
            assert_eq!(settle.checks, u32::from(ready & 0xFF != 0));
            assert_eq!(factory.lookups, vec![manager; lookups as usize]);
            assert_eq!(factory.converts.len() as u32, converts);
            if converts == 1 {
                assert_eq!(
                    factory.converts[0],
                    (h, ConvertRequest::Code(SETTLE_BUILD_KIND))
                );
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hh, expect_h);
            assert_eq!(*o, o_before);
            let mut wlift = EventHandler::new(
                Handle32::new(owner_addr),
                Handle32::new(h_before[H_PENDING]),
            );
            let mut wsettle = FakeSettle {
                ready_answer: ready,
                check_answer: check,
                readies: Vec::new(),
                checks: 0,
            };
            let mut wfactory = FakeFactory {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let wrong_ret = wrong_settled(&mut wlift, owner, &mut wsettle, &mut wfactory, &state);
            // The wrong lift keeps the task past a failed check: its
            // pending word or its answer must disagree with the stub's.
            if Handle32::raw_or_zero(wlift.pending()) != hh[H_PENDING]
                || answer_word(&wrong_ret) != got
            {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong settled never caught ({cases} cases)");
    }
}
