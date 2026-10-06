//! Differential case: the tagged slot against its verified rewrite on
//! the same generated inputs.
//!
//! Compares the return value, the full handler and event blobs, and the
//! two factory calls (slot and arguments in order) against the lift's
//! trait calls, both sides given the same scripted answers. The staged
//! request word folds the event address's bits, so the proof pins its
//! exact shape. A deliberately wrong lift must be caught at least once.
//! One binary per factory slot, so the stub registry is never shared.
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
        ConvertRequest, EventHandler, EventRef, FactoryHandle, FactoryState, Task, TaskFactory,
        TaskManager,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{E_KIND, H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, U32_EDGE, addr, event_blob, handler_blob};

    /// Event blob word holding the tag byte (byte offset 0xC, low byte).
    const E_TAG_WORD: usize = 3;

    // Lookup stub (slot 1) and its recording.
    static LOOKUP_MGR: AtomicU32 = AtomicU32::new(0);
    static LOOKUP_ANS: AtomicU32 = AtomicU32::new(0);
    static LOOKUP_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn lookup_stub(mgr: u32) -> u32 {
        LOOKUP_MGR.store(mgr, Ordering::SeqCst);
        LOOKUP_COUNT.fetch_add(1, Ordering::SeqCst);
        LOOKUP_ANS.load(Ordering::SeqCst)
    }

    // Convert stub (slot 2, three request words) and its recording.
    static CONVERT_H: AtomicU32 = AtomicU32::new(0);
    static CONVERT_ID: AtomicU32 = AtomicU32::new(0);
    static CONVERT_STAGED: AtomicU32 = AtomicU32::new(0);
    static CONVERT_PAD: AtomicU32 = AtomicU32::new(0);
    static CONVERT_ANS: AtomicU32 = AtomicU32::new(0);
    static CONVERT_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn convert_stub(h: u32, id: u32, staged: u32, pad: u32) -> u32 {
        CONVERT_H.store(h, Ordering::SeqCst);
        CONVERT_ID.store(id, Ordering::SeqCst);
        CONVERT_STAGED.store(staged, Ordering::SeqCst);
        CONVERT_PAD.store(pad, Ordering::SeqCst);
        CONVERT_COUNT.fetch_add(1, Ordering::SeqCst);
        CONVERT_ANS.load(Ordering::SeqCst)
    }

    fn lookup_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = lookup_stub;
        f as usize as u32
    }

    fn convert_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = convert_stub;
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

    /// Stages the request word with a flipped bit.
    fn wrong_tagged(
        id: u32,
        tag: u8,
        event: Handle32<EventRef>,
        f: &mut Fake,
        s: &FactoryState,
    ) -> Option<Handle32<Task>> {
        let Some(handle) = f.lookup(s.manager()) else {
            return None;
        };
        let staged = ((event.get() & 0xFFFF_FF00) | u32::from(tag)) ^ 0x01;
        f.convert(handle, ConvertRequest::Tagged { id, staged })
    }

    #[test]
    fn tagged_request_matches() {
        set_callee1(lookup_addr());
        set_callee2(convert_addr());
        let mut rng = Rng(0x7A66);
        let mut caught = 0;
        let mut cases = 0;
        let tags = [0u8, 1, 0x7F, 0x80, 0xFE, 0xFF, (rng.u32() & 0xFF) as u8];
        let mut inputs = Vec::new();
        for &id in &U32_EDGE {
            for &tag in &tags {
                for &manager in &[0u32, 1, rng.u32()] {
                    for &h in &[0u32, 0xCAFE, rng.u32() | 1] {
                        for &ans in &[0u32, rng.u32()] {
                            inputs.push((id, tag, manager, h, ans, rng.u32(), rng.u32()));
                        }
                    }
                }
            }
        }
        for (id, tag, manager, h, ans, owner, pending) in inputs {
            let mut hh = handler_blob();
            hh[H_VTABLE] = rng.u32();
            hh[H_OWNER] = owner;
            hh[H_PAD] = rng.u32();
            hh[H_PENDING] = pending;
            let mut e = event_blob();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            // The tag is the low byte; the other bytes of its word are
            // unread filler, proving only the tag byte matters.
            e[E_TAG_WORD] = (rng.u32() & 0xFFFF_FF00) | u32::from(tag);
            e[E_KIND] = id;
            let h_before = *hh;
            let e_before = *e;
            let ev = addr(&e[0]);
            set_manager(manager);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            CONVERT_ANS.store(ans, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            CONVERT_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA5640::rw_00ca5640(addr(&hh[0]), ev, 0xB0B, 0xC0C) };
            assert_eq!(LOOKUP_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
            assert_eq!(CONVERT_COUNT.load(Ordering::SeqCst), u32::from(h != 0));
            let want_staged = (ev & 0xFFFF_FF00) | u32::from(tag);
            if h != 0 {
                assert_eq!(CONVERT_H.load(Ordering::SeqCst), h);
                assert_eq!(CONVERT_ID.load(Ordering::SeqCst), id);
                assert_eq!(CONVERT_STAGED.load(Ordering::SeqCst), want_staged);
                assert_eq!(CONVERT_PAD.load(Ordering::SeqCst), 0);
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
            let lift_ret = lift.answer_tagged(id, tag, event, &mut fake, &state);
            assert_eq!(got, Handle32::raw_or_zero(lift_ret));
            assert_eq!(fake.lookups, vec![manager]);
            assert_eq!(fake.converts.len() as u32, u32::from(h != 0));
            if h != 0 {
                assert_eq!(
                    fake.converts[0],
                    (
                        h,
                        ConvertRequest::Tagged {
                            id,
                            staged: want_staged
                        }
                    )
                );
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hh, expect_h);
            assert_eq!(*e, e_before);
            let mut wfake = Fake {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let _ = wrong_tagged(id, tag, event, &mut wfake, &state);
            // The wrong lift converts a flipped staged word: its request
            // must differ from the stub's whenever a conversion ran. (It
            // also skips the task store, which the pending compare below
            // would catch on its own.)
            let stub_staged = CONVERT_STAGED.load(Ordering::SeqCst);
            if CONVERT_COUNT.load(Ordering::SeqCst) == 1
                && (wfake.converts.is_empty()
                    || wfake.converts[0].1
                        != ConvertRequest::Tagged {
                            id,
                            staged: stub_staged,
                        })
            {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong tagged never caught ({cases} cases)");
    }
}
