//! Differential case: the hit-response start slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the untouched task blob, and the lookup and
//! case calls against the lift's trait calls, both sides given the same
//! scripted answers. Each kind's case stub answers a distinct word, so a
//! misrouted kind is caught. A deliberately wrong lift must be caught at
//! least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{HitBase, HitHandler, HitResponse, HitStart, TaskMgr};
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::{set_callee, set_manager};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{H_KIND, Rng, U32_EDGE, addr, hit_blob};

    // Lookup stub (slot 1) and its recording.
    static LOOKUP_MGR: AtomicU32 = AtomicU32::new(0);
    static LOOKUP_ANS: AtomicU32 = AtomicU32::new(0);
    static LOOKUP_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn lookup_stub(mgr: u32) -> u32 {
        LOOKUP_MGR.store(mgr, Ordering::SeqCst);
        LOOKUP_COUNT.fetch_add(1, Ordering::SeqCst);
        LOOKUP_ANS.load(Ordering::SeqCst)
    }

    // Case stubs (slots 2 to 5): one recording each.
    static CASE_H: [AtomicU32; 4] = [
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
    ];
    static CASE_ANS: [AtomicU32; 4] = [
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
    ];
    static CASE_COUNT: [AtomicU32; 4] = [
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
        AtomicU32::new(0),
    ];

    macro_rules! case_stub {
        ($name:ident, $idx:expr) => {
            extern "thiscall" fn $name(h: u32) -> u32 {
                CASE_H[$idx].store(h, Ordering::SeqCst);
                CASE_COUNT[$idx].fetch_add(1, Ordering::SeqCst);
                CASE_ANS[$idx].load(Ordering::SeqCst)
            }
        };
    }

    case_stub!(case0_stub, 0);
    case_stub!(case1_stub, 1);
    case_stub!(case2_stub, 2);
    case_stub!(case3_stub, 3);

    fn lookup_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = lookup_stub;
        f as usize as u32
    }

    fn case_addr(i: usize) -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = match i {
            0 => case0_stub,
            1 => case1_stub,
            2 => case2_stub,
            _ => case3_stub,
        };
        f as usize as u32
    }

    struct Fake {
        lookup_answer: u32,
        case_answer: u32,
        lookups: Vec<u32>,
        cases: Vec<(u32, u32)>,
    }

    impl HitStart for Fake {
        fn lookup(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<HitHandler>> {
            self.lookups.push(Handle32::raw_or_zero(manager));
            Handle32::new(self.lookup_answer)
        }

        fn run_case(&mut self, handler: Handle32<HitHandler>, kind: u32) -> u32 {
            self.cases.push((handler.get(), kind));
            self.case_answer
        }
    }

    struct Base;

    impl HitBase for Base {
        fn construct_base(&mut self) {}
    }

    /// Routes kind 0 to case 1 (the checker's own mutant shape).
    fn wrong_start(kind: u32, manager: Option<Handle32<TaskMgr>>, run: &mut Fake) -> u32 {
        let routed = if kind == 0 { 1 } else { kind };
        if routed > 3 {
            return 0;
        }
        let Some(handler) = run.lookup(manager) else {
            return 0;
        };
        run.run_case(handler, routed)
    }

    #[test]
    fn hit_start_matches() {
        set_callee(1, lookup_addr());
        for i in 0..4 {
            set_callee(2 + i as u32, case_addr(i));
        }
        let mut rng = Rng(0x4177);
        let mut caught = 0;
        let mut cases = 0;
        let kinds = {
            let mut v = vec![0, 1, 2, 3, 4, 5];
            v.extend(U32_EDGE);
            for _ in 0..8 {
                v.push(rng.u32());
            }
            v
        };
        // Handler answers: null plus nonzero cookies the stubs never dereference.
        let handles = [0u32, 1, 0x1111, 0xFFFF_FFFF, rng.u32() | 1];
        let mut inputs = Vec::new();
        for &manager in &U32_EDGE {
            for &kind in &kinds {
                for &h in &handles {
                    inputs.push((manager, kind, h, rng.u32(), rng.u32(), rng.u32(), rng.u32()));
                }
            }
        }
        for (manager, kind, h, a0, a1, a2, a3) in inputs {
            let answers = [a0, a1, a2, a3];
            let mut blob = *hit_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[H_KIND] = kind;
            let boxed = Box::new(blob);
            let before = *boxed;
            set_manager(manager);
            LOOKUP_ANS.store(h, Ordering::SeqCst);
            LOOKUP_COUNT.store(0, Ordering::SeqCst);
            for i in 0..4 {
                CASE_ANS[i].store(answers[i], Ordering::SeqCst);
                CASE_COUNT[i].store(0, Ordering::SeqCst);
            }
            // The ignored stack word carries garbage, proving it unread.
            let got = unsafe { fn_00CCB8D0::rw_00ccb8d0(addr(&boxed[0]), 0xB0B) };
            let routed = kind <= 3;
            assert_eq!(LOOKUP_COUNT.load(Ordering::SeqCst), u32::from(routed));
            if routed {
                assert_eq!(LOOKUP_MGR.load(Ordering::SeqCst), manager);
            }
            for i in 0..4 {
                let want = u32::from(routed && h != 0 && kind == i as u32);
                assert_eq!(CASE_COUNT[i].load(Ordering::SeqCst), want, "kind {kind}");
                if want == 1 {
                    assert_eq!(CASE_H[i].load(Ordering::SeqCst), h);
                }
            }
            let expect_ret = if routed && h != 0 {
                answers[kind as usize]
            } else {
                0
            };
            assert_eq!(got, expect_ret, "kind {kind:#x}");
            let lift = HitResponse::new(kind, &mut Base);
            let mut fake = Fake {
                lookup_answer: h,
                case_answer: if routed { answers[kind as usize] } else { 0 },
                lookups: Vec::new(),
                cases: Vec::new(),
            };
            let lift_ret = lift.start(Handle32::new(manager), &mut fake);
            assert_eq!(lift_ret, got, "kind {kind:#x}");
            assert_eq!(fake.lookups.len() as u32, u32::from(routed));
            if routed {
                assert_eq!(fake.lookups[0], manager);
            }
            assert_eq!(fake.cases.len() as u32, u32::from(routed && h != 0));
            if routed && h != 0 {
                assert_eq!(fake.cases[0], (h, kind));
            }
            // The task blob is never written.
            assert_eq!(*boxed, before);
            let mut wfake = Fake {
                lookup_answer: h,
                case_answer: if routed { answers[kind as usize] } else { 0 },
                lookups: Vec::new(),
                cases: Vec::new(),
            };
            let w_ret = wrong_start(kind, Handle32::new(manager), &mut wfake);
            // The fired case stub decides the kind the rewrite ran.
            let mut fired = Vec::new();
            for i in 0..4 {
                if CASE_COUNT[i].load(Ordering::SeqCst) == 1 {
                    fired.push(i as u32);
                }
            }
            let wrong_kinds: Vec<u32> = wfake.cases.iter().map(|&(_, k)| k).collect();
            if w_ret != got || wrong_kinds != fired {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong hit start never caught ({cases} cases)");
    }
}
