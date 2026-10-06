//! Differential case: the hit-response constructor against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full task blob (the kind word against
//! the lift, the table stamp pinned), and the base-constructor call
//! against the lift's trait call. A deliberately wrong lift must be
//! caught at least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_peds_tasks::tasks::{HitBase, HitResponse};
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::set_callee;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{H_KIND, Rng, U32_EDGE, addr, hit_blob};

    /// The stamped table word the proof pins (the rewrite's file VA,
    /// carried through the identity relocation).
    const HIT_TABLE: u32 = 0x00ED9FD4;

    // Base-constructor stub (slot 1) and its recording.
    static BASE_THIS: AtomicU32 = AtomicU32::new(0);
    static BASE_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn base_stub(this: u32) -> u32 {
        BASE_THIS.store(this, Ordering::SeqCst);
        BASE_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    fn base_addr() -> u32 {
        let f: extern "thiscall" fn(u32) -> u32 = base_stub;
        f as usize as u32
    }

    struct Fake {
        calls: u32,
    }

    impl HitBase for Fake {
        fn construct_base(&mut self) {
            self.calls += 1;
        }
    }

    /// Stores the neighbouring kind.
    fn wrong_ctor(kind: u32, base: &mut Fake) -> HitResponse {
        HitResponse::new(kind.wrapping_add(1), base)
    }

    #[test]
    fn hit_ctor_matches() {
        set_callee(1, base_addr());
        let mut rng = Rng(0x41C7);
        let mut caught = 0;
        let mut cases = 0;
        let kinds = {
            let mut v = U32_EDGE.to_vec();
            v.extend([4, 5]);
            for _ in 0..16 {
                v.push(rng.u32());
            }
            v
        };
        for &kind in &kinds {
            let mut blob = *hit_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            let boxed = Box::new(blob);
            let before = *boxed;
            BASE_COUNT.store(0, Ordering::SeqCst);
            let got = unsafe { fn_00CCA040::rw_00cca040(addr(&boxed[0]) as *mut u8, kind) };
            assert_eq!(got, addr(&boxed[0]));
            assert_eq!(BASE_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(BASE_THIS.load(Ordering::SeqCst), addr(&boxed[0]));
            let mut fake = Fake { calls: 0 };
            let lift = HitResponse::new(kind, &mut fake);
            assert_eq!(fake.calls, 1);
            let mut expect = before;
            expect[0] = HIT_TABLE;
            expect[H_KIND] = lift.kind();
            assert_eq!(*boxed, expect);
            let mut wfake = Fake { calls: 0 };
            let wrong = wrong_ctor(kind, &mut wfake);
            if wrong.kind() != boxed[H_KIND] {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong hit ctor never caught ({cases} cases)");
    }
}
