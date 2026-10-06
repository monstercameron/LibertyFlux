//! Differential case: the fist-shake event slot against its verified
//! rewrite on the same generated inputs.
//!
//! Compares the return value, the full task blob, and the damp and
//! release calls against the lift's trait calls, both sides given the
//! same scripted answers. The damp stub clears the slot through a side
//! channel on scripted cases, proving the re-read. A deliberately wrong
//! lift must be caught at least once. 32-bit only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use core::sync::atomic::{AtomicU32, Ordering};

    use lf_core::Handle32;
    use lf_peds_tasks::tasks::{DAMP_RATE_BITS, FistHeld, FistTarget, ShakeFist};
    use lf_taskdiff::rewrites::*;
    use lf_taskdiff::set_callee;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{F_HELD, F_LINK, Rng, U32_EDGE, addr, fist_blob};

    // Damp stub (slot 1): records its words, optionally clears the slot
    // through the side channel the case aims at the blob.
    static DAMP_W: AtomicU32 = AtomicU32::new(0);
    static DAMP_RATE: AtomicU32 = AtomicU32::new(0);
    static DAMP_COUNT: AtomicU32 = AtomicU32::new(0);
    static DAMP_CLEARS: AtomicU32 = AtomicU32::new(0);
    static SLOT_ADDR: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn damp_stub(w: u32, rate: u32) -> u32 {
        DAMP_W.store(w, Ordering::SeqCst);
        DAMP_RATE.store(rate, Ordering::SeqCst);
        DAMP_COUNT.fetch_add(1, Ordering::SeqCst);
        if DAMP_CLEARS.load(Ordering::SeqCst) != 0 {
            unsafe { (SLOT_ADDR.load(Ordering::SeqCst) as *mut u32).write(0) };
        }
        0
    }

    // Release stub (slot 2) and its recording.
    static RELEASE_W: AtomicU32 = AtomicU32::new(0);
    static RELEASE_TASK: AtomicU32 = AtomicU32::new(0);
    static RELEASE_COUNT: AtomicU32 = AtomicU32::new(0);

    extern "thiscall" fn release_stub(w: u32, task: u32) -> u32 {
        RELEASE_W.store(w, Ordering::SeqCst);
        RELEASE_TASK.store(task, Ordering::SeqCst);
        RELEASE_COUNT.fetch_add(1, Ordering::SeqCst);
        0
    }

    fn damp_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = damp_stub;
        f as usize as u32
    }

    fn release_addr() -> u32 {
        let f: extern "thiscall" fn(u32, u32) -> u32 = release_stub;
        f as usize as u32
    }

    struct Fake {
        damp_clears: bool,
        damps: Vec<(u32, u32)>,
        releases: Vec<u32>,
    }

    impl FistTarget for Fake {
        fn damp(&mut self, slot: &mut Option<Handle32<FistHeld>>, rate: f32) {
            self.damps
                .push((Handle32::raw_or_zero(*slot), rate.to_bits()));
            if self.damp_clears {
                *slot = None;
            }
        }

        fn release(&mut self, held: Handle32<FistHeld>) {
            self.releases.push(held.get());
        }
    }

    /// Never releases: damps only, but still answers the taken events.
    fn wrong_event(task: &mut ShakeFist, event: u32, target: &mut Fake) -> u32 {
        if task.held().is_some() {
            let mut slot = task.held();
            target.damp(&mut slot, f32::from_bits(DAMP_RATE_BITS));
            // The re-read result is dropped instead of released.
            let _ = slot;
        }
        u32::from(event == 1 || event == 2)
    }

    /// Events: edge words, the taken pair with neighbours, random.
    fn events(rng: &mut Rng) -> Vec<u32> {
        let mut v = U32_EDGE.to_vec();
        v.extend([0, 1, 2, 3, 4]);
        for _ in 0..16 {
            v.push(rng.u32());
        }
        v
    }

    #[test]
    fn fist_event_matches() {
        set_callee(1, damp_addr());
        set_callee(2, release_addr());
        let mut rng = Rng(0xF15E);
        let mut caught = 0;
        let mut cases = 0;
        let helds = [0u32, 1, 0x77, 0xFFFF_FFFF, rng.u32() | 1];
        let mut inputs = Vec::new();
        for &event in &events(&mut rng) {
            for &held in &helds {
                for &clears in &[0u32, 1] {
                    inputs.push((event, held, clears, rng.u32()));
                }
            }
        }
        for (event, held, clears, link) in inputs {
            let mut blob = *fist_blob();
            for w in blob.iter_mut() {
                *w = rng.u32();
            }
            blob[F_HELD] = held;
            blob[F_LINK] = link;
            let boxed = Box::new(blob);
            let before = *boxed;
            DAMP_COUNT.store(0, Ordering::SeqCst);
            RELEASE_COUNT.store(0, Ordering::SeqCst);
            DAMP_CLEARS.store(clears, Ordering::SeqCst);
            SLOT_ADDR.store(addr(&boxed[F_HELD]), Ordering::SeqCst);
            let taken = event == 1 || event == 2;
            // The ignored stack words carry garbage, proving them unread.
            let got = unsafe { fn_00D4E000::rw_00d4e000(addr(&boxed[0]), 0xB0B, event, 0xC0C) };
            assert_eq!(got, u32::from(taken));
            let want_damps = u32::from(held != 0);
            assert_eq!(DAMP_COUNT.load(Ordering::SeqCst), want_damps);
            if held != 0 {
                assert_eq!(DAMP_W.load(Ordering::SeqCst), held);
                assert_eq!(DAMP_RATE.load(Ordering::SeqCst), DAMP_RATE_BITS);
            }
            let cleared = clears != 0 && held != 0;
            let reread = if cleared { 0 } else { held };
            let want_releases = u32::from(taken && reread != 0);
            assert_eq!(RELEASE_COUNT.load(Ordering::SeqCst), want_releases);
            if want_releases == 1 {
                assert_eq!(RELEASE_W.load(Ordering::SeqCst), reread);
                assert_eq!(RELEASE_TASK.load(Ordering::SeqCst), addr(&boxed[0]));
            }
            let mut lift = ShakeFist::new(Handle32::new(held), Handle32::new(link));
            let mut fake = Fake {
                damp_clears: cleared,
                damps: Vec::new(),
                releases: Vec::new(),
            };
            let lift_ret = lift.handle_event(event, &mut fake);
            assert_eq!(lift_ret, got, "event {event:#x} held {held:#x}");
            assert_eq!(fake.damps.len() as u32, want_damps);
            if held != 0 {
                assert_eq!(fake.damps[0], (held, DAMP_RATE_BITS));
            }
            assert_eq!(fake.releases.len() as u32, want_releases);
            if want_releases == 1 {
                assert_eq!(fake.releases[0], reread);
            }
            let mut expect = before;
            expect[F_HELD] = Handle32::raw_or_zero(lift.held());
            assert_eq!(*boxed, expect, "event {event:#x} held {held:#x}");
            let mut wrong = ShakeFist::new(Handle32::new(held), Handle32::new(link));
            let mut wfake = Fake {
                damp_clears: cleared,
                damps: Vec::new(),
                releases: Vec::new(),
            };
            let w_ret = wrong_event(&mut wrong, event, &mut wfake);
            if w_ret != got || wrong.held() != lift.held() {
                caught += 1;
            }
            cases += 1;
        }
        assert!(caught > 0, "wrong fist event never caught ({cases} cases)");
    }
}
