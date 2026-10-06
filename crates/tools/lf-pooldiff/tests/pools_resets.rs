//! Differential cases, part 6 (second lane): the small resets.
//!
//! Each case plants a real 32-bit region, runs the rewrite and the lifted
//! method on the same inputs, and compares returns and every byte. Each
//! method has a deliberately wrong lift that must be caught. 32-bit only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_pooldiff::rewrites::*;
    use lf_world::pools::{
        HANDLE_SIZE, HandleState, PAIR_SIZE, PairSlot, ROW_SIZE, RowPairs, SLOT_SIZE, SlotPair,
        SmallSlot,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, lock};

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_world::pools::{HandleState, RowPairs, SlotPair, SmallSlot};

        /// Clears bit 1 instead of bit 0.
        pub fn handle_bit1(img: &mut [u8; super::HANDLE_SIZE]) -> HandleState {
            img[0x5b] &= !0x02;
            for off in [0x40, 0x44, 0x48, 0x390] {
                img[off..off + 4].copy_from_slice(&0u32.to_le_bytes());
            }
            HandleState::from_bytes(*img)
        }

        /// Forgets the word at +0x6c.
        pub fn slot_skip_6c(img: &mut [u8; super::SLOT_SIZE]) -> SmallSlot {
            img[0x5b] &= !0x02;
            for off in [0x04, 0x18] {
                img[off..off + 4].copy_from_slice(&0u32.to_le_bytes());
            }
            SmallSlot::from_bytes(*img)
        }

        /// Wipes 15 rows instead of 16.
        pub fn rows_15(img: &mut [u8; super::ROW_SIZE]) -> usize {
            let mut cur = 0x84;
            for _ in 0..15 {
                img[cur - 4..cur].copy_from_slice(&0u32.to_le_bytes());
                img[cur..cur + 4].copy_from_slice(&0u32.to_le_bytes());
                cur += 0x30;
            }
            cur
        }

        /// Writes the low pair setter's words to the high slots.
        pub fn pair_swapped(img: &mut [u8; super::PAIR_SIZE], w0: u32, w1: u32) -> u32 {
            img[0xA10..0xA14].copy_from_slice(&w0.to_le_bytes());
            img[0xA14..0xA18].copy_from_slice(&w1.to_le_bytes());
            w1
        }
    }

    /// Copies test memory the rewrite wrote back out (raw reads: the
    /// borrow checker cannot see writes through the planted address).
    unsafe fn snapshot(base: u32, len: usize) -> Vec<u8> {
        unsafe {
            let mut out = vec![0u8; len];
            std::ptr::copy_nonoverlapping(base as *const u8, out.as_mut_ptr(), len);
            out
        }
    }

    #[test]
    fn handle_reset_matches() {
        let _guard = lock();
        let mut rng = Rng(0x9E01);
        let mut cases = 0;
        let mut caught = 0;
        for _ in 0..24 {
            let mut img = [0u8; HANDLE_SIZE];
            rng.bytes(&mut img);
            let boxed = Box::new(img);
            let this = addr(&boxed[0]);
            assert!(
                this.checked_add(HANDLE_SIZE as u32).is_some(),
                "region wraps"
            );
            let before = unsafe { snapshot(this, HANDLE_SIZE) };
            let got = unsafe { fn_00A0B770::rw_00a0b770(this) };
            let after = unsafe { snapshot(this, HANDLE_SIZE) };
            assert_eq!(got, this, "echoes this");
            let mut lift = HandleState::from_bytes(before.clone().try_into().unwrap());
            lift.reset();
            assert_eq!(after, lift.bytes(), "full region");
            assert_eq!(after[0x5b] & 0x01, 0, "bit 0 cleared");
            let mut wimg: [u8; HANDLE_SIZE] = before.clone().try_into().unwrap();
            let w = wrong::handle_bit1(&mut wimg);
            if w.bytes() != after.as_slice() {
                caught += 1;
            }
            cases += 1;
            std::hint::black_box(&boxed);
            std::mem::forget(boxed);
        }
        assert!(cases == 24, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong reset never caught ({cases} cases)");
    }

    #[test]
    fn slot_reset_matches() {
        let _guard = lock();
        let mut rng = Rng(0x9E02);
        let mut cases = 0;
        let mut caught = 0;
        for _ in 0..24 {
            let mut img = [0u8; SLOT_SIZE];
            rng.bytes(&mut img);
            let boxed = Box::new(img);
            let this = addr(&boxed[0]);
            assert!(this.checked_add(SLOT_SIZE as u32).is_some(), "region wraps");
            let before = unsafe { snapshot(this, SLOT_SIZE) };
            let got = unsafe { fn_00A0BA50::rw_00a0ba50(this) };
            let after = unsafe { snapshot(this, SLOT_SIZE) };
            assert_eq!(got, 0, "rewrite answers 0");
            let mut lift = SmallSlot::from_bytes(before.clone().try_into().unwrap());
            lift.reset();
            assert_eq!(after, lift.bytes(), "full region");
            let mut wimg: [u8; SLOT_SIZE] = before.clone().try_into().unwrap();
            let w = wrong::slot_skip_6c(&mut wimg);
            if w.bytes() != after.as_slice() {
                caught += 1;
            }
            cases += 1;
            std::hint::black_box(&boxed);
            std::mem::forget(boxed);
        }
        assert!(cases == 24, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong reset never caught ({cases} cases)");
    }

    #[test]
    fn row_pairs_zero_matches() {
        let _guard = lock();
        let mut rng = Rng(0x9E03);
        let mut cases = 0;
        let mut caught = 0;
        for round in 0..12 {
            let mut img = [0u8; ROW_SIZE];
            rng.bytes(&mut img);
            // The last pair (0x350/0x354) stays nonzero so the 15-row
            // mutant is caught on the image too.
            if round % 2 == 0 {
                img[0x350..0x354].copy_from_slice(&0x0102_0304u32.to_le_bytes());
                img[0x354..0x358].copy_from_slice(&0x0506_0708u32.to_le_bytes());
            }
            let boxed = Box::new(img);
            let this = addr(&boxed[0]);
            assert!(this.checked_add(ROW_SIZE as u32).is_some(), "region wraps");
            let before = unsafe { snapshot(this, ROW_SIZE) };
            let got = unsafe { fn_00A0BA70::rw_00a0ba70(this) };
            let after = unsafe { snapshot(this, ROW_SIZE) };
            assert_eq!(got, this.wrapping_add(ROW_SIZE as u32), "end cursor");
            let mut lift = RowPairs::from_bytes(before.clone().try_into().unwrap());
            let end = lift.zero_all();
            assert_eq!(end, ROW_SIZE, "lift end");
            assert_eq!(after, lift.bytes(), "full region");
            let mut wimg: [u8; ROW_SIZE] = before.clone().try_into().unwrap();
            let wend = wrong::rows_15(&mut wimg);
            if wend != end || wimg.as_slice() != after.as_slice() {
                caught += 1;
            }
            cases += 1;
            std::hint::black_box(&boxed);
            std::mem::forget(boxed);
        }
        assert!(cases == 12, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong wipe never caught ({cases} cases)");
    }

    fn run_pair(
        slot: PairSlot,
        seed: u32,
        set: extern "thiscall" fn(u32, u32) -> u32,
    ) -> (u32, u32) {
        let mut rng = Rng(seed);
        let mut cases = 0;
        let mut caught = 0;
        for _ in 0..24 {
            let mut img = [0u8; PAIR_SIZE];
            rng.bytes(&mut img);
            let w0 = rng.u32();
            let w1 = rng.u32();
            let src = Box::new([w0, w1]);
            let boxed = Box::new(img);
            let this = addr(&boxed[0]);
            let src_addr = addr(&src[0]);
            assert!(this.checked_add(PAIR_SIZE as u32).is_some(), "region wraps");
            let before = unsafe { snapshot(this, PAIR_SIZE) };
            let got = unsafe { set(this, src_addr) };
            let after = unsafe { snapshot(this, PAIR_SIZE) };
            assert_eq!(got, w1, "answers the second word");
            let mut lift = SlotPair::from_bytes(before.clone().try_into().unwrap());
            let lret = lift.set_pair(slot, w0, w1);
            assert_eq!(lret, w1);
            assert_eq!(after, lift.bytes(), "full region");
            // The swapped mutant writes the other slot; it differs
            // whenever the two slots did not already hold these words.
            let mut wimg: [u8; PAIR_SIZE] = before.clone().try_into().unwrap();
            let wret = match slot {
                PairSlot::Low => wrong::pair_swapped(&mut wimg, w0, w1),
                PairSlot::High => {
                    wimg[0xA08..0xA0C].copy_from_slice(&w0.to_le_bytes());
                    wimg[0xA0C..0xA10].copy_from_slice(&w1.to_le_bytes());
                    w1
                }
            };
            assert_eq!(wret, w1, "mutant keeps the answer");
            if wimg.as_slice() != after.as_slice() {
                caught += 1;
            }
            cases += 1;
            std::hint::black_box((&boxed, &src));
            std::mem::forget(boxed);
        }
        (cases, caught)
    }

    #[test]
    fn set_pair_low_matches() {
        let _guard = lock();
        let (cases, caught) = run_pair(PairSlot::Low, 0x9E04, fn_00A8A3E0::rw_00a8a3e0);
        assert!(cases == 24, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong setter never caught ({cases} cases)");
    }

    #[test]
    fn set_pair_high_matches() {
        let _guard = lock();
        let (cases, caught) = run_pair(PairSlot::High, 0x9E05, fn_00A8A400::rw_00a8a400);
        assert!(cases == 24, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong setter never caught ({cases} cases)");
    }
}
