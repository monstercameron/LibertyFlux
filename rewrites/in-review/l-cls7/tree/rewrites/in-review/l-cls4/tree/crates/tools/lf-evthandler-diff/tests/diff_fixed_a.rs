//! Differential case: the first fixed-request slot against its verified
//! rewrite on the same generated inputs.
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
        ConvertRequest, EventHandler, FactoryHandle, FactoryState, Task, TaskFactory, TaskManager,
        FIXED_REQUEST_A,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, U32_EDGE, addr, event_blob, handler_blob};

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
    static CONVERT_CODE: AtomicU32 = AtomicU32::new(0);
    static CONVERT_ANS: AtomicU32 = AtomicU32::new(0);
    static CONVERT_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn convert_stub(h: u32, code: u32) -> u32 {
        CONVERT_H.store(h, Ordering::SeqCst);
        CONVERT_CODE.store(code, Ordering::SeqCst);
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

    /// Converts a neighbouring code: the request word must differ.
    fn wrong_code(h: &mut EventHandler, code: u32, f: &mut Fake, s: &FactoryState) {
        let _ = h.answer_fixed_request(code.wrapping_add(1), f, s);
    }

    #[test]
    fn fixed_request_a_matches() {
        set_callee1(lookup_addr());
        set_callee2(convert_addr());
        let mut rng = Rng(0xF1A1);
        let mut caught = 0;
        let mut cases = 0;
        let managers = {
            let mut v = U32_EDGE.to_vec();
            for _ in 0..8 {
                v.push(rng.u32());
            }
            v
        };
        let handles = [0u32, 1, 0x1234_5678, 0xFFFF_FFFF, rng.u32()];
        let mut inputs = Vec::new();
        for &manager in &managers {
            for &h in &handles {
                for &ans in &U32_EDGE {
                    inputs.push((manager, h, ans, rng.u32(), rng.u32()));
                }
            }
        }
        for (manager, h, ans, owner, pending) in inputs {
            let mut hh = handler_blob();
            hh[H_VTABLE] = rng.u32();
            hh[H_OWNER] = owner;
            hh[H_PAD] = rng.u32();
            hh[H_PENDING] = pending;
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            let h_before = *hh;
            let e_before = *e;
            set_manager(manager);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            CONVERT_ANS.store(ans, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            CONVERT_COUNT.store(0, Ordering::SeqCst);
            let got =
                unsafe { fn_00CA6C30::rw_00ca6c30(addr(&hh[0]), addr(&e[0]), 0xB0B, 0xC0C) };
            assert_eq!(LOOKUP_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
            let want_converts = u32::from(h != 0);
            assert_eq!(CONVERT_COUNT.load(Ordering::SeqCst), want_converts);
            if h != 0 {
                assert_eq!(CONVERT_H.load(Ordering::SeqCst), h);
                assert_eq!(CONVERT_CODE.load(Ordering::SeqCst), FIXED_REQUEST_A);
            }
            let mut lift =
                EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let state = FactoryState::new(Handle32::new(manager));
            let mut fake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let lift_ret = lift.answer_fixed_request(FIXED_REQUEST_A, &mut fake, &state);
            assert_eq!(got, Handle32::raw_or_zero(lift_ret));
            assert_eq!(fake.lookups, vec![manager]);
            assert_eq!(fake.converts.len() as u32, want_converts);
            if h != 0 {
                assert_eq!(
                    fake.converts[0],
                    (h, ConvertRequest::Code(FIXED_REQUEST_A))
                );
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hh, expect_h);
            assert_eq!(*e, e_before);
            let mut w =
                EventHandler::new(Handle32::new(owner), Handle32::new(pending));
            let mut wfake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            wrong_code(&mut w, FIXED_REQUEST_A, &mut wfake, &state);
            let stub_code = CONVERT_CODE.load(Ordering::SeqCst);
            if CONVERT_COUNT.load(Ordering::SeqCst) == 1
                && (wfake.converts.is_empty()
                    || wfake.converts[0].1 != ConvertRequest::Code(stub_code))
            {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong fixed-a never caught ({cases} cases)");
    }
}
