//! Differential cases, part 4: the voice-slot file.
//!
//! Each case plants the flat slot image, runs the rewrite and the lift on
//! the same inputs, and compares the return and the whole image (plus the
//! notify call for the slot update, whose address rebuilds from the
//! lifted offset per case). Each method has a deliberately wrong lift
//! that must be caught. 32-bit target only.
//!
//! All trials stay inside the planted store; out-of-store offsets are
//! the lift's documented panic domain (host tests), never run here.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::audio_voice::slots::VoiceSlots;
    use lf_audiovoicediff::rewrites::*;
    use lf_audiovoicediff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, lock};

    /// Planted store size: covers full-range index bytes and flag keys.
    const STORE: usize = 32768;

    /// Fresh view of a test image; rebuilt after every rewrite call so no
    /// pre-call borrow is read back (the compiler would forward it).
    unsafe fn image(base: u32, len: usize) -> &'static mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(base as *mut u8, len) }
    }

    /// Deliberately wrong lifts, each caught below. The image-shaped
    /// wrong versions run inline in their tests (the lift owns its bytes,
    /// so a wrong store pattern applies to a scratch image and must
    /// differ from the rewrite's image).
    mod wrong {
        use lf_audio::audio_voice::slots::VoiceSlots;

        /// Tests the flag against 3 instead of 2.
        pub fn flag_check_3(slots: &VoiceSlots, index: u32) -> bool {
            if index >= 3 {
                return false;
            }
            let mem = slots.bytes();
            let b = mem[index.wrapping_add(0x540) as usize];
            let k = (b as u32).wrapping_add(index.wrapping_mul(2)).wrapping_add(0x6a);
            mem[k.wrapping_mul(3).wrapping_mul(4) as usize] == 3
        }

    }

    #[test]
    fn flag_check_matches() {
        let _guard = lock();
        let mut rng = Rng(0xAAB10);
        let mut caught = 0;
        for trial in 0..60u32 {
            let mut initial = vec![0u8; STORE];
            rng.bytes(&mut initial);
            let index = match trial % 8 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 4,
                5 => 0xFFFF_FFFF,
                _ => rng.below(6),
            };
            if trial == 0 {
                // Table byte 0 selects flag address 4*3*(0+0+0x6a) = 1272.
                initial[0x540] = 0;
                initial[1272] = 2;
            }
            if trial == 1 {
                // Index 1, table byte 1: flag at 4*3*(1+2+106) = 1308.
                initial[0x541] = 1;
                initial[1308] = 3;
            }
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let lift = VoiceSlots::from_bytes(initial.clone());
            let ret = unsafe { fn_009AAB10::rw_009AAB10(this, index) };
            let out = lift.flag_check(index);
            assert_eq!(ret, u8::from(out), "trial {trial}: answer matches");
            let after = unsafe { image(this, STORE) }.to_vec();
            assert_eq!(after, initial, "trial {trial}: the check writes nothing");
            if wrong::flag_check_3(&lift, index) != out {
                caught += 1;
            }
        }
        assert!(caught > 0, "flag-3 mutant was never caught");
    }

    #[test]
    fn slot_update_matches() {
        let _guard = lock();
        let mut rng = Rng(0xAABC0);
        let mut caught = 0;
        for trial in 0..48u32 {
            let a = trial % 3;
            let t = if trial == 0 { 0 } else { rng.below(150) as u8 };
            let b = rng.u32();
            let c = rng.u32();
            let d = if trial == 0 { 0xDDDD_DDDD } else { rng.u32() };
            let mut initial = vec![0u8; STORE];
            rng.bytes(&mut initial);
            initial[0x368 + a as usize] = t;
            let rec = (t as u32 + a * 3) * 96;
            assert!(rec + 0x58 + 4 <= STORE as u32, "trial stays in store");
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            rt::set_script(&[(1, StubKind::Cdecl2, vec![0])]);
            let mut lift = VoiceSlots::from_bytes(initial.clone());
            let ret = unsafe { fn_009AABC0::rw_009AABC0(this, a, b, c, d) };
            assert_eq!(ret, 0, "trial {trial}: the rewrite answers nothing");
            let notify_addr = this.wrapping_add(rec).wrapping_add(0x18);
            assert_eq!(
                rt::take_calls(),
                vec![(1, vec![notify_addr, c])],
                "trial {trial}: notify runs once on record+0x18"
            );
            let mut seen = Vec::new();
            lift.slot_update(a, b, c, d, &mut |at: u32, cc: u32| seen.push((at, cc)));
            assert_eq!(seen, vec![(rec.wrapping_add(0x18), c)], "trial {trial}: lifted notify offset");
            assert_eq!(
                this.wrapping_add(seen[0].0),
                notify_addr,
                "trial {trial}: address rebuilds from the offset"
            );
            let after = unsafe { image(this, STORE) }.to_vec();
            assert_eq!(after, lift.bytes(), "trial {trial}: whole image matches");
            // Wrong version: the stores minus the last one, over a scratch
            // image (the lift owns its bytes, so the wrong shape runs on a
            // copy and its image must differ from the rewrite's).
            let mut wimg = initial.clone();
            wimg[rec as usize + 0x14..rec as usize + 0x18].copy_from_slice(&b.to_le_bytes());
            if wimg != after {
                caught += 1;
            }
        }
        assert!(caught > 0, "skip-second-store mutant was never caught");
    }

    #[test]
    fn clear_by_id_matches() {
        let _guard = lock();
        let mut rng = Rng(0xAAD00);
        let mut caught = 0;
        for trial in 0..48u32 {
            let mut initial = vec![0u8; STORE];
            rng.bytes(&mut initial);
            // Full-range index bytes stay in store (255*96+0x24c+4 < STORE).
            for k in 0..3usize {
                initial[0x368 + k] = rng.u32() as u8;
            }
            // Trial 0 forces every owner to the id; even trials reuse one
            // planted owner so a match is guaranteed.
            let id = if trial == 0 {
                let id = 0x1D1D_1D1D;
                for k in 0..3usize {
                    let rec = initial[0x368 + k] as u32 * 96;
                    let base = [0x0cu32, 0x12c, 0x24c][k];
                    initial[rec as usize + base as usize..rec as usize + base as usize + 4]
                        .copy_from_slice(&id.to_le_bytes());
                }
                for f in [0x370usize, 0x3d0, 0x430] {
                    initial[f..f + 4].copy_from_slice(&id.to_le_bytes());
                }
                id
            } else if trial % 2 == 0 {
                let k = rng.below(3) as usize;
                let rec = initial[0x368 + k] as u32 * 96;
                let base = [0x0cu32, 0x12c, 0x24c][k];
                u32::from_le_bytes(
                    initial[rec as usize + base as usize..rec as usize + base as usize + 4]
                        .try_into()
                        .unwrap(),
                )
            } else {
                rng.u32()
            };
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let mut lift = VoiceSlots::from_bytes(initial.clone());
            let ret = unsafe { fn_009AAD00::rw_009AAD00(this, id) };
            assert_eq!(ret, 0, "trial {trial}: the rewrite answers nothing");
            lift.clear_by_id(id);
            let after = unsafe { image(this, STORE) }.to_vec();
            assert_eq!(after, lift.bytes(), "trial {trial}: whole image matches");
            // Wrong version: the indexed sweep only, skipping fixed slots.
            let mut wimg = initial.clone();
            for k in 0..3u32 {
                let rec = wimg[0x368 + k as usize] as u32 * 96;
                let owner = rec + [0x0cu32, 0x12c, 0x24c][k as usize];
                let o = owner as usize;
                if u32::from_le_bytes(wimg[o..o + 4].try_into().unwrap()) == id {
                    wimg[o..o + 4].copy_from_slice(&0u32.to_le_bytes());
                    wimg[o - 4] = 3;
                }
            }
            if wimg != after {
                caught += 1;
            }
        }
        assert!(caught > 0, "skip-fixed mutant was never caught");
    }

    #[test]
    fn flag_advance_matches() {
        let _guard = lock();
        let mut rng = Rng(0xAAE00);
        let mut caught = 0;
        for trial in 0..60u32 {
            // Build (a, b) from an in-store record: idx -> a=idx/3,
            // r=idx%3, t=r so b=r-1 (wrapping).
            let idx = if trial == 0 { 0 } else { rng.below(300) };
            let a = idx / 3;
            let r = idx % 3;
            let b = r.wrapping_sub(1);
            let at = idx * 96 + 8;
            let mut initial = vec![0u8; STORE];
            rng.bytes(&mut initial);
            if trial == 0 {
                initial[at as usize] = 2;
            } else if trial % 3 == 0 {
                initial[at as usize] = [0, 1, 2, 3, 0xFF][rng.below(5) as usize];
            }
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let mut lift = VoiceSlots::from_bytes(initial.clone());
            let ret = unsafe { fn_009AAE00::rw_009AAE00(this, a, b) };
            assert_eq!(ret, 0, "trial {trial}: the rewrite answers nothing");
            lift.flag_advance(a, b);
            let after = unsafe { image(this, STORE) }.to_vec();
            assert_eq!(after, lift.bytes(), "trial {trial}: whole image matches");
            // Wrong version: 2 advances to 4 instead of 3.
            let mut wimg = initial.clone();
            if wimg[at as usize] == 2 {
                wimg[at as usize] = 4;
            }
            if wimg != after {
                caught += 1;
            }
        }
        assert!(caught > 0, "advance-to-4 mutant was never caught");
    }
}
