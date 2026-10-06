//! Differential case: the goto constructor against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full task blob (cleared words against
//! the lift, the table stamp pinned, the speed bits exact), and the base
//! and radius calls against the lift's trait calls. A deliberately wrong
//! lift must be caught at least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{GotoBase, GotoTask};
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::set_callee;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        G_ARMED_BYTE, G_FLAG_BYTE, G_KIND, G_MODE, G_POS, G_RESTAMP_BYTE, G_SPEED, G_STAMP, G_SUB,
        G_WAIT, G_WAITCP, Rng, U32_EDGE, addr, blob_byte, goto_blob, set_blob_byte,
    };

    /// The stamped table word the proof pins (the rewrite's file VA,
    /// carried through the identity relocation).
    const GOTO_TABLE: u32 = 0x00EEF6F4;

    // Base-constructor stub (slot 1) and its recording.
    static BASE_THIS: AtomicU32 = AtomicU32::new(0);
    static BASE_INIT: AtomicU32 = AtomicU32::new(0);
    static BASE_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn base_stub(this: u32, init: u32) -> u32 {
        BASE_THIS.store(this, Ordering::SeqCst);
        BASE_INIT.store(init, Ordering::SeqCst);
        BASE_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    // Radius stub (slot 2) and its recording.
    static RADIUS_KIND: AtomicU32 = AtomicU32::new(0);
    static RADIUS_ANS: AtomicU32 = AtomicU32::new(0);
    static RADIUS_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "cdecl" fn radius_stub(kind: u32) -> f32 {
        RADIUS_KIND.store(kind, Ordering::SeqCst);
        RADIUS_COUNT.fetch_add(1, Ordering::SeqCst);
        f32::from_bits(RADIUS_ANS.load(Ordering::SeqCst))
    }

    fn base_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = base_stub;
        f as usize as u32
    }

    fn radius_addr() -> u32 {
        let f: extern "cdecl" fn(u32) -> f32 = radius_stub;
        f as usize as u32
    }

    struct Fake {
        inits: Vec<u32>,
        queries: Vec<u32>,
        speed_answer: f32,
    }

    impl GotoBase for Fake {
        fn construct_base(&mut self, init: u32) {
            self.inits.push(init);
        }

        fn query_speed(&mut self, kind: u32) -> f32 {
            self.queries.push(kind);
            self.speed_answer
        }
    }

    /// A constructor that forgets to clear the wait word.
    #[allow(clippy::too_many_arguments)]
    fn wrong_ctor(
        subtask: Option<Handle32<lf_peds_tasks::tasks::SubTask>>,
        kind: u32,
        pos: [f32; 3],
        flag: bool,
        mode: Option<Handle32<lf_peds_tasks::tasks::GotoEntity>>,
        speed: f32,
    ) -> GotoTask {
        GotoTask::new(subtask, kind, pos, flag, mode, 1, 0, 0, false, false, speed)
    }

    #[test]
    fn goto_ctor_matches() {
        set_callee(1, base_addr());
        set_callee(2, radius_addr());
        let mut rng = Rng(0x6070_C70E);
        let mut caught = 0;
        let mut cases = 0;
        // Speed answers: edges of the float range plus random bits.
        let speeds = [
            0.0f32.to_bits(),
            1.0f32.to_bits(),
            (-1.0f32).to_bits(),
            f32::MAX.to_bits(),
            f32::MIN_POSITIVE.to_bits(),
            f32::INFINITY.to_bits(),
            f32::NEG_INFINITY.to_bits(),
            f32::NAN.to_bits(),
            rng.u32(),
            rng.u32(),
        ];
        let mut inputs = Vec::new();
        for &init in &U32_EDGE {
            for &speed in &speeds {
                inputs.push((init, speed, rng.u32()));
            }
        }
        for _ in 0..40 {
            inputs.push((
                rng.u32(),
                speeds[(rng.next() % speeds.len() as u64) as usize],
                rng.u32(),
            ));
        }
        for (init, speed_bits, kind) in inputs {
            let mut blob = *goto_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[G_KIND] = kind;
            let boxed = Box::new(blob);
            let before = *boxed;
            BASE_COUNT.store(0, Ordering::SeqCst);
            RADIUS_COUNT.store(0, Ordering::SeqCst);
            RADIUS_ANS.store(speed_bits, Ordering::SeqCst);
            let got = unsafe { fn_00DA4E00::rw_00da4e00(addr(&boxed[0]), init) };
            assert_eq!(got, addr(&boxed[0]));
            assert_eq!(BASE_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(BASE_THIS.load(Ordering::SeqCst), addr(&boxed[0]));
            assert_eq!(BASE_INIT.load(Ordering::SeqCst), init);
            assert_eq!(RADIUS_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(RADIUS_KIND.load(Ordering::SeqCst), kind);
            let subtask = Handle32::new(before[G_SUB]);
            let pos = [
                f32::from_bits(before[G_POS]),
                f32::from_bits(before[G_POS + 1]),
                f32::from_bits(before[G_POS + 2]),
            ];
            let flag = blob_byte(&before[..], G_FLAG_BYTE) != 0;
            let mode = Handle32::new(before[G_MODE]);
            let mut fake = Fake {
                inits: Vec::new(),
                queries: Vec::new(),
                speed_answer: f32::from_bits(speed_bits),
            };
            let lift = GotoTask::from_init(subtask, kind, pos, flag, mode, init, &mut fake);
            assert_eq!(fake.inits, vec![init]);
            assert_eq!(fake.queries, vec![kind]);
            let mut expect = before;
            expect[0] = GOTO_TABLE;
            expect[G_WAIT] = 0;
            expect[G_STAMP] = 0;
            expect[G_WAITCP] = 0;
            set_blob_byte(&mut expect, G_ARMED_BYTE, 0);
            set_blob_byte(&mut expect, G_RESTAMP_BYTE, 0);
            expect[G_SPEED] = speed_bits;
            assert_eq!(*boxed, expect);
            assert_eq!(lift.subtask(), subtask);
            assert_eq!(lift.kind(), kind);
            let lift_pos = lift.pos();
            assert_eq!(lift_pos[0].to_bits(), pos[0].to_bits());
            assert_eq!(lift_pos[1].to_bits(), pos[1].to_bits());
            assert_eq!(lift_pos[2].to_bits(), pos[2].to_bits());
            assert_eq!(lift.flag(), flag);
            assert_eq!(lift.mode(), mode);
            assert_eq!(lift.wait(), 0);
            assert_eq!(lift.stamp(), 0);
            assert_eq!(lift.wait_copy(), 0);
            assert!(!lift.armed());
            assert!(!lift.restamp());
            assert_eq!(lift.speed().to_bits(), speed_bits);
            let wrong = wrong_ctor(subtask, kind, pos, flag, mode, f32::from_bits(speed_bits));
            if wrong.wait() != boxed[G_WAIT] {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong goto ctor never caught ({cases} cases)");
    }
}
