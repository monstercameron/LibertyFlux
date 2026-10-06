//! Differential case: the dual-path float slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full handler and event blobs, the
//! scalar call and the lookup plus the one sibling conversion that runs
//! (slot and arguments in order) against the lift's trait calls, both
//! sides given the same scripted answers, the scalar bitwise. A
//! deliberately wrong lift must be caught at least once. One binary per
//! slot shape, so the stub registry is never shared. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_evthandler_diff::rewrites::*;
    use lf_evthandler_diff::{set_callee, set_manager};
    use lf_peds_tasks::event_handler::{
        BLOCK_SELECT_BIT, BlockWords, ConvertRequest, EventHandler, EventRef, FactoryHandle,
        FactoryState, ScalarEval, Task, TaskFactory, TaskManager,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, W_SELECT_BYTE, W_W30, W_W34, W_W3C, W_W40, addr,
        blob_byte, event_blob_wide, handler_blob,
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

    fn lookup_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = lookup_stub;
        f as usize as u32
    }

    // Scalar stub (slot 2, one input word plus the vector block) and
    // its recording. Answers scripted bits as a float.
    static EVAL_ARGS: [AtomicU32; 7] = [const { AtomicU32::new(0) }; 7];
    static EVAL_ANS: AtomicU32 = AtomicU32::new(0);
    static EVAL_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "cdecl" fn eval_stub(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> f32 {
        EVAL_ARGS[0].store(a, Ordering::SeqCst);
        EVAL_ARGS[1].store(b, Ordering::SeqCst);
        EVAL_ARGS[2].store(c, Ordering::SeqCst);
        EVAL_ARGS[3].store(d, Ordering::SeqCst);
        EVAL_ARGS[4].store(e, Ordering::SeqCst);
        EVAL_ARGS[5].store(f, Ordering::SeqCst);
        EVAL_ARGS[6].store(g, Ordering::SeqCst);
        EVAL_COUNT.fetch_add(1, Ordering::SeqCst);
        f32::from_bits(EVAL_ANS.load(Ordering::SeqCst))
    }

    fn eval_addr() -> u32 {
        let f: extern "cdecl" fn(u32, u32, u32, u32, u32, u32, u32) -> f32 = eval_stub;
        f as usize as u32
    }

    // Sibling converter stubs (slots 3 and 4) and their recordings.
    static CONV3_ARGS: [AtomicU32; 8] = [const { AtomicU32::new(0) }; 8];
    static CONV3_ANS: AtomicU32 = AtomicU32::new(0);
    static CONV3_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn conv3_stub(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
        g: u32,
        h: u32,
    ) -> u32 {
        CONV3_ARGS[0].store(a, Ordering::SeqCst);
        CONV3_ARGS[1].store(b, Ordering::SeqCst);
        CONV3_ARGS[2].store(c, Ordering::SeqCst);
        CONV3_ARGS[3].store(d, Ordering::SeqCst);
        CONV3_ARGS[4].store(e, Ordering::SeqCst);
        CONV3_ARGS[5].store(f, Ordering::SeqCst);
        CONV3_ARGS[6].store(g, Ordering::SeqCst);
        CONV3_ARGS[7].store(h, Ordering::SeqCst);
        CONV3_COUNT.fetch_add(1, Ordering::SeqCst);
        CONV3_ANS.load(Ordering::SeqCst)
    }

    fn conv3_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 = conv3_stub;
        f as usize as u32
    }

    static CONV4_ARGS: [AtomicU32; 8] = [const { AtomicU32::new(0) }; 8];
    static CONV4_ANS: AtomicU32 = AtomicU32::new(0);
    static CONV4_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn conv4_stub(
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
        g: u32,
        h: u32,
    ) -> u32 {
        CONV4_ARGS[0].store(a, Ordering::SeqCst);
        CONV4_ARGS[1].store(b, Ordering::SeqCst);
        CONV4_ARGS[2].store(c, Ordering::SeqCst);
        CONV4_ARGS[3].store(d, Ordering::SeqCst);
        CONV4_ARGS[4].store(e, Ordering::SeqCst);
        CONV4_ARGS[5].store(f, Ordering::SeqCst);
        CONV4_ARGS[6].store(g, Ordering::SeqCst);
        CONV4_ARGS[7].store(h, Ordering::SeqCst);
        CONV4_COUNT.fetch_add(1, Ordering::SeqCst);
        CONV4_ANS.load(Ordering::SeqCst)
    }

    fn conv4_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 = conv4_stub;
        f as usize as u32
    }

    struct FakeScalar {
        answer: u32,
        calls: Vec<(u32, BlockWords)>,
    }

    impl ScalarEval for FakeScalar {
        fn eval_word(&mut self, _input: u32) -> f32 {
            panic!("dual-path cases never evaluate single words");
        }

        fn eval_block(&mut self, event: Handle32<EventRef>, words: BlockWords) -> f32 {
            self.calls.push((event.get(), words));
            f32::from_bits(self.answer)
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

    /// Wrong lift: picks the other sibling converter.
    fn wrong_block(
        h: &mut EventHandler,
        selector: u8,
        event: Handle32<EventRef>,
        words: BlockWords,
        s: &mut FakeScalar,
        f: &mut FakeFactory,
        st: &FactoryState,
    ) -> Option<Handle32<Task>> {
        h.answer_scalar_block(selector ^ BLOCK_SELECT_BIT, event, words, s, f, st)
    }

    #[test]
    fn scalar_block_matches() {
        set_callee(1, lookup_addr());
        set_callee(2, eval_addr());
        set_callee(3, conv3_addr());
        set_callee(4, conv4_addr());
        let mut rng = Rng(0xB10C);
        let mut caught = 0;
        let mut cases = 0;
        // Scalar answers as bits: zeros, ones, quiet values, payloads.
        let scalars = [
            0u32,
            0x8000_0000,
            1,
            0x3F80_0000,
            0xBF80_0000,
            0x7FC0_0001,
            0xFF80_0000,
            rng.u32(),
        ];
        let selectors = [0u8, 1, 2, 3, 0xFD, 0xFE, 0xFF, (rng.u32() & 0xFF) as u8];
        let mut inputs = Vec::new();
        for &selector in &selectors {
            for &scalar in &scalars {
                for &manager in &[0u32, rng.u32()] {
                    for &h in &[0u32, rng.u32() | 1] {
                        inputs.push((selector, scalar, manager, h, rng.u32()));
                    }
                }
            }
        }
        for (selector, scalar, manager, h, ans) in inputs {
            let mut hh = handler_blob();
            hh[H_VTABLE] = rng.u32();
            hh[H_OWNER] = rng.u32();
            hh[H_PAD] = rng.u32();
            hh[H_PENDING] = rng.u32();
            let mut e = event_blob_wide();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            // Only the selector bit of its byte matters; the rest of
            // the word is unread filler.
            let sel_word = (rng.u32() & 0xFFFF_FF00) | u32::from(selector);
            e[W_SELECT_BYTE / 4] = sel_word;
            let h_before = *hh;
            let e_before = *e;
            let ev = addr(&e[0]);
            assert_eq!(blob_byte(&e[..], W_SELECT_BYTE), selector);
            set_manager(manager);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            EVAL_ANS.store(scalar, Ordering::SeqCst);
            CONV3_ANS.store(ans, Ordering::SeqCst);
            CONV4_ANS.store(ans, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            EVAL_COUNT.store(0, Ordering::SeqCst);
            CONV3_COUNT.store(0, Ordering::SeqCst);
            CONV4_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA52E0::rw_00ca52e0(addr(&hh[0]), ev, 0xB0B, 0xC0C) };
            assert_eq!(LOOKUP_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
            let converted = u32::from(h != 0);
            assert_eq!(EVAL_COUNT.load(Ordering::SeqCst), converted);
            let second = selector & BLOCK_SELECT_BIT == 0;
            let (want3, want4) = if converted == 0 {
                (0, 0)
            } else if second {
                (0, 1)
            } else {
                (1, 0)
            };
            assert_eq!(CONV3_COUNT.load(Ordering::SeqCst), want3);
            assert_eq!(CONV4_COUNT.load(Ordering::SeqCst), want4);
            let words = BlockWords {
                w34: e[W_W34],
                w30: e[W_W30],
                w3c: e[W_W3C],
                w40: e[W_W40],
            };
            let p10 = ev.wrapping_add(0x10);
            let p20 = ev.wrapping_add(0x20);
            let p50 = ev.wrapping_add(0x50);
            if converted == 1 {
                let got_eval: Vec<u32> =
                    EVAL_ARGS.iter().map(|a| a.load(Ordering::SeqCst)).collect();
                assert_eq!(got_eval, vec![words.w34, p10, words.w30, p20, words.w3c, words.w40, p50]);
                let args = if second { &CONV4_ARGS } else { &CONV3_ARGS };
                let got_conv: Vec<u32> =
                    args.iter().map(|a| a.load(Ordering::SeqCst)).collect();
                assert_eq!(
                    got_conv,
                    vec![h, scalar, p10, words.w30, p20, words.w3c, words.w40, p50]
                );
            }
            let mut lift = EventHandler::new(
                Handle32::new(h_before[H_OWNER]),
                Handle32::new(h_before[H_PENDING]),
            );
            let state = FactoryState::new(Handle32::new(manager));
            let mut scalar_fake = FakeScalar {
                answer: scalar,
                calls: Vec::new(),
            };
            let mut factory = FakeFactory {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let event = Handle32::new(ev).expect("test event address is live");
            let lift_ret =
                lift.answer_scalar_block(selector, event, words, &mut scalar_fake, &mut factory, &state);
            assert_eq!(got, Handle32::raw_or_zero(lift_ret));
            assert_eq!(factory.lookups, vec![manager]);
            assert_eq!(scalar_fake.calls.len() as u32, converted);
            if converted == 1 {
                assert_eq!(scalar_fake.calls[0], (ev, words));
            }
            assert_eq!(factory.converts.len() as u32, converted);
            if converted == 1 {
                assert_eq!(
                    factory.converts[0],
                    (
                        h,
                        ConvertRequest::ScalarBlock {
                            scalar_bits: scalar,
                            words,
                            second,
                        }
                    )
                );
            }
            let mut expect_h = h_before;
            expect_h[H_PENDING] = Handle32::raw_or_zero(lift.pending());
            assert_eq!(*hh, expect_h);
            assert_eq!(*e, e_before);
            let mut wscalar = FakeScalar {
                answer: scalar,
                calls: Vec::new(),
            };
            let mut wfactory = FakeFactory {
                lookup_answer: h,
                convert_answer: ans,
                lookups: Vec::new(),
                converts: Vec::new(),
            };
            let mut wlift = EventHandler::new(
                Handle32::new(h_before[H_OWNER]),
                Handle32::new(h_before[H_PENDING]),
            );
            let _ = wrong_block(
                &mut wlift,
                selector,
                event,
                words,
                &mut wscalar,
                &mut wfactory,
                &state,
            );
            // The wrong lift converts through the other sibling: its
            // recorded choice must disagree with the stub's whenever a
            // conversion ran.
            let stub_second = CONV4_COUNT.load(Ordering::SeqCst) == 1;
            if converted == 1
                && (wfactory.converts.is_empty()
                    || wfactory.converts[0].1
                        != ConvertRequest::ScalarBlock {
                            scalar_bits: scalar,
                            words,
                            second: stub_second,
                        })
            {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong block never caught ({cases} cases)");
    }
}
