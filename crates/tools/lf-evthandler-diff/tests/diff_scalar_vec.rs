//! Differential case: the scalar-vector slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full handler and event blobs, the
//! scalar call and the factory pair (slot and arguments in order)
//! against the lift's trait calls, both sides given the same scripted
//! answers, the scalar and float words bitwise. A deliberately wrong
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
        BlockWords, ConvertRequest, EventHandler, EventRef, FactoryHandle, FactoryState,
        ScalarEval, Task, TaskFactory, TaskManager, VEC_FIRST_BASE, VEC_FIRST_FLAG,
        VEC_SECOND_BASE, VecInput,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        H_OWNER, H_PAD, H_PENDING, H_VTABLE, Rng, W_FLAG, W_INPUT, W_WEIGHT, addr, event_blob_wide,
        handler_blob,
    };

    // Scalar stub (slot 1, one input word) and its recording. Answers
    // scripted bits as a float.
    static EVAL_INPUT: AtomicU32 = AtomicU32::new(0);
    static EVAL_ANS: AtomicU32 = AtomicU32::new(0);
    static EVAL_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "cdecl" fn eval_stub(input: u32) -> f32 {
        EVAL_INPUT.store(input, Ordering::SeqCst);
        EVAL_COUNT.fetch_add(1, Ordering::SeqCst);
        f32::from_bits(EVAL_ANS.load(Ordering::SeqCst))
    }

    fn eval_addr() -> u32 {
        let f: extern "cdecl" fn(u32) -> f32 = eval_stub;
        f as usize as u32
    }

    // Lookup stub (slot 2) and its recording.
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

    // Convert stub (slot 3: scalar bits, vector base, float word, pad)
    // and its recording.
    static CONVERT_H: AtomicU32 = AtomicU32::new(0);
    static CONVERT_SCALAR: AtomicU32 = AtomicU32::new(0);
    static CONVERT_VEC: AtomicU32 = AtomicU32::new(0);
    static CONVERT_WEIGHT: AtomicU32 = AtomicU32::new(0);
    static CONVERT_PAD: AtomicU32 = AtomicU32::new(0);
    static CONVERT_ANS: AtomicU32 = AtomicU32::new(0);
    static CONVERT_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn convert_stub(h: u32, scalar: u32, vec: u32, weight: u32, pad: u32) -> u32 {
        CONVERT_H.store(h, Ordering::SeqCst);
        CONVERT_SCALAR.store(scalar, Ordering::SeqCst);
        CONVERT_VEC.store(vec, Ordering::SeqCst);
        CONVERT_WEIGHT.store(weight, Ordering::SeqCst);
        CONVERT_PAD.store(pad, Ordering::SeqCst);
        CONVERT_COUNT.fetch_add(1, Ordering::SeqCst);
        CONVERT_ANS.load(Ordering::SeqCst)
    }

    fn convert_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 = convert_stub;
        f as usize as u32
    }

    struct FakeScalar {
        answer: u32,
        calls: Vec<u32>,
    }

    impl ScalarEval for FakeScalar {
        fn eval_word(&mut self, input: u32) -> f32 {
            self.calls.push(input);
            f32::from_bits(self.answer)
        }

        fn eval_block(&mut self, _event: Handle32<EventRef>, _words: BlockWords) -> f32 {
            panic!("vector cases never evaluate blocks");
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

    /// Wrong lift: picks the other vector base.
    fn wrong_vec(
        h: &mut EventHandler,
        input: VecInput,
        event: Handle32<EventRef>,
        s: &mut FakeScalar,
        f: &mut FakeFactory,
        st: &FactoryState,
    ) -> Option<Handle32<Task>> {
        let flipped = VecInput {
            flag: if input.flag == VEC_FIRST_FLAG {
                VEC_FIRST_FLAG.wrapping_add(1)
            } else {
                VEC_FIRST_FLAG
            },
            scalar_input: input.scalar_input,
            weight: input.weight,
        };
        h.answer_scalar_vec(flipped, event, s, f, st)
    }

    #[test]
    fn scalar_vec_matches() {
        set_callee(1, eval_addr());
        set_callee(2, lookup_addr());
        set_callee(3, convert_addr());
        let mut rng = Rng(0x5EC7);
        let mut caught = 0;
        let mut cases = 0;
        // Scalar and float words as bits: zeros, ones, quiet values,
        // infinities and payloads.
        let bits = [
            0u32,
            0x8000_0000,
            1,
            0x3F80_0000,
            0xBF80_0000,
            0x7FC0_0001,
            0xFF80_0000,
            rng.u32(),
        ];
        let flags = [0u32, 1, 2, 0xFFFF_FFFF, rng.u32()];
        let mut inputs = Vec::new();
        for &flag in &flags {
            for &scalar in &bits {
                for &weight in &bits {
                    for &manager in &[0u32, rng.u32()] {
                        for &h in &[0u32, rng.u32() | 1] {
                            inputs.push((flag, scalar, weight, manager, h, rng.u32(), rng.u32()));
                        }
                    }
                }
            }
        }
        for (flag, scalar, weight, manager, h, ans, input) in inputs {
            let mut hh = handler_blob();
            hh[H_VTABLE] = rng.u32();
            hh[H_OWNER] = rng.u32();
            hh[H_PAD] = rng.u32();
            hh[H_PENDING] = rng.u32();
            let mut e = event_blob_wide();
            for w in e.iter_mut() {
                *w = rng.u32();
            }
            e[W_FLAG] = flag;
            e[W_WEIGHT] = weight;
            e[W_INPUT] = input;
            let h_before = *hh;
            let e_before = *e;
            let ev = addr(&e[0]);
            set_manager(manager);
            EVAL_ANS.store(scalar, Ordering::SeqCst);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            CONVERT_ANS.store(ans, Ordering::SeqCst);
            EVAL_COUNT.store(0, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            CONVERT_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CA5260::rw_00ca5260(addr(&hh[0]), ev, 0xB0B, 0xC0C) };
            // The scalar call runs before the lookup on every path.
            assert_eq!(EVAL_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(EVAL_INPUT.load(Ordering::SeqCst), input);
            assert_eq!(LOOKUP_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
            let converted = u32::from(h != 0);
            assert_eq!(CONVERT_COUNT.load(Ordering::SeqCst), converted);
            let base = if flag == VEC_FIRST_FLAG {
                VEC_FIRST_BASE
            } else {
                VEC_SECOND_BASE
            };
            if converted == 1 {
                assert_eq!(CONVERT_H.load(Ordering::SeqCst), h);
                assert_eq!(CONVERT_SCALAR.load(Ordering::SeqCst), scalar);
                assert_eq!(CONVERT_VEC.load(Ordering::SeqCst), ev.wrapping_add(base));
                assert_eq!(CONVERT_WEIGHT.load(Ordering::SeqCst), weight);
                assert_eq!(CONVERT_PAD.load(Ordering::SeqCst), 0);
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
            let vin = VecInput {
                flag,
                scalar_input: input,
                weight: f32::from_bits(weight),
            };
            let lift_ret =
                lift.answer_scalar_vec(vin, event, &mut scalar_fake, &mut factory, &state);
            assert_eq!(got, Handle32::raw_or_zero(lift_ret));
            assert_eq!(scalar_fake.calls, vec![input]);
            assert_eq!(factory.lookups, vec![manager]);
            assert_eq!(factory.converts.len() as u32, converted);
            if converted == 1 {
                assert_eq!(
                    factory.converts[0],
                    (
                        h,
                        ConvertRequest::ScalarVec {
                            scalar_bits: scalar,
                            vec_offset: base,
                            weight_bits: weight,
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
            let _ = wrong_vec(&mut wlift, vin, event, &mut wscalar, &mut wfactory, &state);
            // The wrong lift converts through the other base: its
            // recorded offset must disagree with the stub's whenever a
            // conversion ran.
            let stub_vec = CONVERT_VEC.load(Ordering::SeqCst);
            if converted == 1
                && (wfactory.converts.is_empty()
                    || ev.wrapping_add(match wfactory.converts[0].1 {
                        ConvertRequest::ScalarVec { vec_offset, .. } => vec_offset,
                        _ => u32::MAX,
                    }) != stub_vec)
            {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong vec never caught ({cases} cases)");
    }
}
