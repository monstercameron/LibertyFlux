//! Differential cases, part 3: small pools and neighbours.
//!
//! Each case plants the routine's object (and the triplet buffer or the
//! gated-release globals it reads), runs the rewrite and the lift on the
//! same inputs, and compares returns, every written byte and every
//! callee call in order, rebuilding each 32-bit address from the lifted
//! key per case. Each method has a deliberately wrong lift that must be
//! caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::audio_slot::misc::{
        GateState, LIVE_FAIL_TOP, LivenessProbe, OwnedSlot, SlotHead, TrackerCell,
    };
    use lf_audio::audio_slot::pools::{PtrArray, StridedPool, TRIPLET_FREE, Triplet, TripletTable};
    use lf_audioslotdiff::rewrites::*;
    use lf_audioslotdiff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        G1_VA, G2_VA, G3_VA, G4_VA, MIXER_VA, POOL_VA, Rng, TRIPLET_VA, addr, get_u32, lock,
        put_u32,
    };

    /// Fresh view of a test image; rebuilt after every rewrite call so no
    /// pre-call borrow is read back (the compiler would forward it).
    unsafe fn image(base: u32, len: usize) -> &'static mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(base as *mut u8, len) }
    }

    #[test]
    fn strided_pool_alloc_matches() {
        let _guard = lock();
        let mut rng = Rng(0x98CA0);
        let mut caught = 0;
        for trial in 0..40u32 {
            let cap = rng.below(6);
            let count = if trial % 3 == 0 {
                cap.saturating_add(rng.below(2))
            } else {
                rng.below(cap.saturating_add(1))
            };
            let tag = rng.u32();
            let flags = rng.u32();
            let pool_base = if trial % 4 == 0 { 0 } else { rng.u32() };
            // Object: cap at +0x48, table pointer at +0x4c, count at +0x50.
            let mut initial = vec![0u8; 0x60];
            rng.bytes(&mut initial);
            put_u32(&mut initial, 0x48, cap);
            put_u32(&mut initial, 0x50, count);
            let ncells = cap as usize + 2;
            let mut tab = vec![0u8; ncells * 8];
            rng.bytes(&mut tab);
            let tab_box: Box<[u8]> = tab.into_boxed_slice();
            let tab_addr = addr(&tab_box[0]);
            put_u32(&mut initial, 0x4C, tab_addr);
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            rt::set_global(POOL_VA, pool_base);
            rt::set_script(&[(1, StubKind::Thiscall4, vec![0])]);
            let ret = unsafe { fn_00898CA0::rw_00898ca0(this, tag, flags) };
            let calls = rt::take_calls();
            let mut cells = Vec::new();
            for i in 0..ncells {
                let w0 = get_u32(&tab_box, i * 8);
                let w1 = get_u32(&tab_box, i * 8 + 4);
                cells.push((w0, w1));
            }
            let mut lift = StridedPool::from_parts(cap, count, pool_base, cells);
            let mut seen: Vec<(u32, u32, u32)> = Vec::new();
            let out = lift.alloc(tag, flags, &mut |slot: u32, t: u32, f: u32| {
                seen.push((slot, t, f));
            });
            if count >= cap {
                assert_eq!(ret, 0, "trial {trial}: full pool answers 0");
                assert_eq!(out, 0, "trial {trial}: lift answers 0");
                assert!(calls.is_empty(), "trial {trial}: no setup runs");
                assert!(seen.is_empty(), "trial {trial}: lift sets up nothing");
            } else {
                let slot = pool_base.wrapping_add(count.wrapping_mul(0x70));
                assert_eq!(ret, slot, "trial {trial}: element word matches");
                assert_eq!(out, slot, "trial {trial}: lift answers it too");
                assert_eq!(
                    calls,
                    vec![(1, vec![slot, tag, this, flags])],
                    "trial {trial}: setup runs once"
                );
                assert_eq!(
                    seen,
                    vec![(count, tag, flags)],
                    "trial {trial}: lift setup key"
                );
                // The element word rebuilds from the lifted index.
                assert_eq!(
                    pool_base.wrapping_add(seen[0].0.wrapping_mul(0x70)),
                    slot,
                    "trial {trial}: word rebuilds from the index"
                );
            }
            let after = unsafe { image(this, 0x60) }.to_vec();
            let tab_after = unsafe { image(tab_addr, ncells * 8) }.to_vec();
            let mut expect = initial.clone();
            let mut expect_tab = tab_box.to_vec();
            if count < cap {
                put_u32(&mut expect, 0x50, count.wrapping_add(1));
                let slot = pool_base.wrapping_add(count.wrapping_mul(0x70));
                put_u32(&mut expect_tab, count as usize * 8 + 4, tag);
                put_u32(&mut expect_tab, count as usize * 8, slot);
            }
            assert_eq!(after, expect, "trial {trial}: object matches");
            assert_eq!(tab_after, expect_tab, "trial {trial}: table matches");
            // Wrong version: the setup runs before the capacity check, so
            // a full pool still calls once.
            if count >= cap && seen.is_empty() {
                caught += 1;
            }
            let _ = (obj, tab_box);
        }
        assert!(caught > 0, "setup-before-check mutant was never caught");
    }

    #[test]
    fn triplet_alloc_matches() {
        let _guard = lock();
        let mut rng = Rng(0xA56B0);
        let mut caught = 0;
        for trial in 0..40u32 {
            let arg0 = rng.u32();
            let arg1 = rng.u32();
            let arg2 = if trial % 5 == 0 {
                TRIPLET_FREE
            } else {
                rng.u32()
            };
            // Markers: first-free at k, full, edges, random mixes.
            let mut buf = vec![0u8; 32 * 20];
            rng.bytes(&mut buf);
            // Clear stray free markers first (random words rarely hit).
            for i in 0..32 {
                if get_u32(&buf, i * 20) == TRIPLET_FREE {
                    put_u32(&mut buf, i * 20, 1);
                }
            }
            match trial % 5 {
                0 => put_u32(&mut buf, rng.below(32) as usize * 20, TRIPLET_FREE),
                1 => {} // full: nothing planted
                2 => put_u32(&mut buf, 0, TRIPLET_FREE),
                3 => put_u32(&mut buf, 31 * 20, TRIPLET_FREE),
                _ => {
                    for _ in 0..1 + rng.below(4) {
                        put_u32(&mut buf, rng.below(32) as usize * 20, TRIPLET_FREE);
                    }
                }
            }
            // The true first free marker, read from the buffer.
            let first_free = (0..32).find(|i| get_u32(&buf, i * 20) == TRIPLET_FREE);
            let tab_box: Box<[u8]> = buf.clone().into_boxed_slice();
            let base = addr(&tab_box[0]);
            // The signed scan agrees with the unsigned one while the
            // table stays on one side of the sign boundary.
            assert!(
                (base as i32) < (base.wrapping_add(640) as i32),
                "trial {trial}: table stays sign-local"
            );
            rt::set_relocated(TRIPLET_VA, base);
            let ret = unsafe { fn_009A56B0::rw_009A56B0(arg0, arg1, arg2) };
            let mut entries = [Triplet {
                marker: 0,
                first: 0,
                second: 0,
            }; 32];
            for (i, e) in entries.iter_mut().enumerate() {
                *e = Triplet {
                    marker: get_u32(&buf, i * 20),
                    first: get_u32(&buf, i * 20 + 4),
                    second: get_u32(&buf, i * 20 + 8),
                };
            }
            let mut lift = TripletTable::from_entries(entries);
            let out = lift.alloc(arg0, arg1, arg2);
            if let Some(i) = first_free {
                assert_eq!(ret, arg0, "trial {trial}: hit answers arg0");
                assert_eq!(out, Some(i), "trial {trial}: lift answers the index");
            } else {
                assert_eq!(
                    ret,
                    base.wrapping_add(640),
                    "trial {trial}: miss answers the end address"
                );
                assert_eq!(out, None, "trial {trial}: lift answers None");
            }
            let after = unsafe { image(base, 32 * 20) }.to_vec();
            let mut expect = buf.clone();
            if let Some(i) = first_free {
                put_u32(&mut expect, i * 20 + 8, arg1);
                put_u32(&mut expect, i * 20, arg2);
                put_u32(&mut expect, i * 20 + 4, arg0);
            }
            assert_eq!(after, expect, "trial {trial}: table bytes match");
            // Wrong version: the last free entry wins instead of the first.
            let last_free = (0..32)
                .rev()
                .find(|i| get_u32(&buf, i * 20) == TRIPLET_FREE);
            if last_free != first_free {
                caught += 1;
            }
            let _ = tab_box;
        }
        assert!(caught > 0, "last-free mutant was never caught");
    }

    #[test]
    fn ptr_array_release_matches() {
        let _guard = lock();
        let mut rng = Rng(0x6E860);
        let mut caught = 0;
        for trial in 0..30u32 {
            let mut initial = vec![0u8; 0x2A30 + 64 * 4];
            rng.bytes(&mut initial);
            if trial % 3 == 0 {
                for i in 0..64 {
                    put_u32(&mut initial, 0x2A30 + i * 4, 0);
                }
            }
            if trial == 0 {
                for i in 0..64 {
                    put_u32(&mut initial, 0x2A30 + i * 4, 0x0100_0000 + i as u32);
                }
            }
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            rt::set_script(&[(1, StubKind::Stdcall1, vec![])]);
            let ret = unsafe { fn_0096E860::rw_0096e860(this) };
            let calls = rt::take_calls();
            let mut slots = Vec::new();
            for i in 0..64 {
                slots.push(get_u32(&initial, 0x2A30 + i * 4));
            }
            let mut lift = PtrArray::from_slots(slots.clone());
            let mut seen: Vec<u32> = Vec::new();
            let out = lift.release_all(&mut |index: u32| seen.push(index));
            assert_eq!(ret, 0, "trial {trial}: the release answers 0");
            assert_eq!(out, 0, "trial {trial}: lift answers 0");
            let live: Vec<usize> = (0..64).filter(|i| slots[*i] != 0).collect();
            let expect_calls: Vec<(u32, Vec<u32>)> = live
                .iter()
                .map(|i| {
                    (
                        1,
                        vec![this.wrapping_add(0x2A30).wrapping_add(*i as u32 * 4)],
                    )
                })
                .collect();
            assert_eq!(calls, expect_calls, "trial {trial}: call log matches");
            assert_eq!(
                seen,
                live.iter().map(|i| *i as u32).collect::<Vec<_>>(),
                "trial {trial}: lift indexes match"
            );
            for (k, index) in seen.iter().enumerate() {
                assert_eq!(
                    this.wrapping_add(0x2A30)
                        .wrapping_add(index.wrapping_mul(4)),
                    expect_calls[k].1[0],
                    "trial {trial}: address {k} rebuilds from the index"
                );
            }
            let after = unsafe { image(this, initial.len()) }.to_vec();
            let mut expect = initial.clone();
            for i in &live {
                put_u32(&mut expect, 0x2A30 + i * 4, 0);
            }
            assert_eq!(after, expect, "trial {trial}: image matches");
            // Wrong version: live slots are released but not cleared.
            if !live.is_empty() {
                caught += 1;
            }
            let _ = obj;
        }
        assert!(caught > 0, "no-clear mutant was never caught");
    }

    #[test]
    fn tracker_dispatch_matches() {
        let _guard = lock();
        let mut rng = Rng(0xE2C80);
        let mut caught = 0;
        for trial in 0..30u32 {
            let slot = match trial % 4 {
                0 => 0,
                1 => 0xFFFF_FFFF,
                _ => rng.u32(),
            };
            let a = match trial % 4 {
                0 => 0,
                1 => 1,
                2 => 0xFFFF_FFFF,
                _ => rng.u32(),
            };
            let answer = rng.u32();
            let mut initial = vec![0u8; 0xB8C];
            rng.bytes(&mut initial);
            put_u32(&mut initial, 0xB88, slot);
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            rt::set_script(&[(1, StubKind::Thiscall2, vec![answer])]);
            let ret = unsafe { fn_009E2C80::rw_009e2c80(this as *mut u8, a) };
            let calls = rt::take_calls();
            let lift = TrackerCell(slot);
            let mut seen: Vec<u32> = Vec::new();
            let out = lift.add_and_dispatch(a, &mut |sum: u32| {
                seen.push(sum);
                answer
            });
            let sum = a.wrapping_add(slot);
            assert_eq!(ret, answer, "trial {trial}: answer matches");
            assert_eq!(out, answer, "trial {trial}: lift answers it too");
            assert_eq!(
                calls,
                vec![(1, vec![this, sum])],
                "trial {trial}: one dispatch"
            );
            assert_eq!(seen, vec![sum], "trial {trial}: lifted sum matches");
            let after = unsafe { image(this, 0xB8C) }.to_vec();
            assert_eq!(after, initial, "trial {trial}: the dispatch writes nothing");
            // Wrong version: saturating instead of wrapping addition.
            if sum != a.saturating_add(slot) {
                caught += 1;
            }
            let _ = obj;
        }
        assert!(caught > 0, "saturating-add mutant was never caught");
    }

    #[test]
    fn slot_head_fresh_matches() {
        let _guard = lock();
        let mut rng = Rng(0xAB4E0);
        let mut caught = 0;
        for trial in 0..32u32 {
            let mut initial = vec![0u8; 0x40];
            rng.bytes(&mut initial);
            if trial < 4 {
                // Flag edges: the kept bits and the mode bit.
                initial[0x3C] = [0x00, 0xFF, 0x05, 0xFA][trial as usize];
            }
            let flag = initial[0x3C];
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let ret = unsafe { fn_008AB4E0::rw_008AB4E0(this as *mut u8) };
            assert_eq!(ret, this, "trial {trial}: the init answers the object");
            let lift = SlotHead::fresh(flag);
            assert_eq!(
                lift.flag,
                (flag & 0xFA) | 2,
                "trial {trial}: flag mapping matches"
            );
            let after = unsafe { image(this, 0x40) }.to_vec();
            let mut expect = initial.clone();
            expect[0..2].copy_from_slice(&0x100u16.to_le_bytes());
            for o in [4, 8, 0xC, 0x10, 0x14, 0x18] {
                expect[o..o + 4].copy_from_slice(&0u32.to_le_bytes());
            }
            expect[0x20..0x22].copy_from_slice(&0u16.to_le_bytes());
            for o in [0x24, 0x28, 0x2C, 0x30, 0x34, 0x38] {
                expect[o..o + 4].copy_from_slice(&0u32.to_le_bytes());
            }
            expect[0x3C] = (flag & 0xFA) | 2;
            assert_eq!(after, expect, "trial {trial}: image matches");
            // Wrong version: the flag keeps 0xFD instead of 0xFA.
            if (flag & 0xFD) | 2 != lift.flag {
                caught += 1;
            }
            let _ = obj;
        }
        assert!(caught > 0, "flag-mask mutant was never caught");
    }

    #[test]
    fn gated_release_matches() {
        let _guard = lock();
        let mut rng = Rng(0x7B670);
        let mut caught = 0;
        for trial in 0..40u32 {
            let owned = trial % 4 != 0;
            let param = rng.u32();
            // Gate words: live and each failing condition.
            let (g1, g2, g3, g4) = match trial % 6 {
                0 => (0, 7, 7, 0),    // live
                1 => (1, 7, 7, 0),    // g1 fails
                2 => (0, 7, 8, 0),    // equality fails
                3 => (0, 7, 7, 0x12), // g4 fails
                _ => (rng.u32(), rng.u32(), rng.u32(), rng.u32()),
            };
            let live = g1 != 1 && g2 == g3 && g4 != 0x12;
            let mut initial = vec![0u8; 0x20];
            rng.bytes(&mut initial);
            let slot_box: Box<[u8]> = vec![0u8; 0xA8].into_boxed_slice();
            let slot_addr = addr(&slot_box[0]);
            put_u32(&mut initial, 0x14, if owned { slot_addr } else { 0 });
            let view = unsafe { image(slot_addr, 0xA8) };
            view[0xA4..0xA8].copy_from_slice(&param.to_le_bytes());
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let _this = addr(&obj[0]);
            rt::set_global(G1_VA, g1);
            rt::set_global(G2_VA, g2);
            rt::set_global(G3_VA, g3);
            rt::set_global(G4_VA, g4);
            rt::set_script(&[
                (1, StubKind::Cdecl1, vec![0]),
                (2, StubKind::Thiscall2, vec![0]),
            ]);
            let ret = unsafe { fn_0097B670::rw_s103_97b670(_this as *mut u8) };
            let calls = rt::take_calls();
            let lift = OwnedSlot(if owned { slot_addr } else { 0 });
            let gates = GateState { g1, g2, g3, g4 };
            struct Ops {
                param: u32,
                log: Vec<(u8, u32, u32)>,
            }
            impl lf_audio::audio_slot::misc::GatedRelease for Ops {
                fn slot_param(&mut self, slot: lf_audio::audio_slot::misc::SlotRef) -> u32 {
                    self.log.push((0, slot.0, 0));
                    self.param
                }
                fn notify(&mut self, p: u32) {
                    self.log.push((1, p, 0));
                }
                fn release(&mut self, slot: lf_audio::audio_slot::misc::SlotRef) {
                    self.log.push((2, slot.0, 0));
                }
            }
            let mut ops = Ops {
                param,
                log: Vec::new(),
            };
            lift.release_gated(&gates, &mut ops);
            assert_eq!(ret, 0, "trial {trial}: the rewrite answers 0");
            if !owned {
                assert!(calls.is_empty(), "trial {trial}: absent slot calls nothing");
                assert!(ops.log.is_empty(), "trial {trial}: lift calls nothing");
            } else if live {
                assert_eq!(
                    calls,
                    vec![(1, vec![param]), (2, vec![slot_addr, 0])],
                    "trial {trial}: lookup then release"
                );
                // The release receives the planted slot: the lookup is
                // pinned not to retarget the slot word.
                assert_eq!(
                    ops.log,
                    vec![(0, slot_addr, 0), (1, param, 0), (2, slot_addr, 0),],
                    "trial {trial}: lift log matches"
                );
            } else {
                assert_eq!(
                    calls,
                    vec![(2, vec![slot_addr, 0])],
                    "trial {trial}: release only"
                );
                assert_eq!(
                    ops.log,
                    vec![(2, slot_addr, 0)],
                    "trial {trial}: lift releases once"
                );
            }
            let after = unsafe { image(_this, 0x20) }.to_vec();
            assert_eq!(after, initial, "trial {trial}: the release writes nothing");
            // Wrong version: the gate drops the g4 check.
            let wrong_live = g1 != 1 && g2 == g3;
            if owned && wrong_live != live {
                caught += 1;
            }
            let _ = (obj, slot_box);
        }
        assert!(caught > 0, "drop-g4 mutant was never caught");
    }

    #[test]
    fn liveness_gate_matches() {
        let _guard = lock();
        let mut rng = Rng(0x8A800);
        let mut caught = 0;
        // Float edges as bit patterns: zeros, subnormals, NaNs, infinities.
        let edges: [u32; 10] = [
            0x0000_0000,
            0x8000_0000,
            0x0000_0001,
            0x8000_0001,
            0x3F80_0000,
            0xBF80_0000,
            0x7F80_0000,
            0xFF80_0000,
            0x7FC0_0000,
            0x7F80_0001,
        ];
        for trial in 0..60u32 {
            let zero = f32::from_bits(edges[trial as usize % edges.len()]);
            let pos = if trial % 7 == 0 {
                f32::from_bits(edges[(trial as usize / 7) % edges.len()])
            } else {
                f32::from_bits(rng.u32())
            };
            let tag_byte = match trial % 5 {
                0 => 0,
                1 => 1,
                _ => rng.below(256) as u8,
            };
            let first = rng.u32();
            let answer = rng.u32();
            let mut initial = vec![0u8; 0x6C];
            rng.bytes(&mut initial);
            initial[0] = tag_byte;
            initial[0x64..0x68].copy_from_slice(&zero.to_le_bytes());
            initial[0x68..0x6C].copy_from_slice(&pos.to_le_bytes());
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let mixer_box: Box<[u8]> = vec![0u8; 4].into_boxed_slice();
            let mixer = addr(&mixer_box[0]);
            rt::set_relocated(MIXER_VA, mixer);
            rt::set_script(&[(1, StubKind::Thiscall3, vec![answer])]);
            let ret = unsafe { fn_00C8A800::rw_00c8a800(first, this) };
            let calls = rt::take_calls();
            let lift = LivenessProbe {
                tag: tag_byte,
                zero,
                pos,
            };
            let mut seen: Vec<u32> = Vec::new();
            let out = lift.gate(first, &mut |f: u32| {
                seen.push(f);
                answer
            });
            let latch = u8::from(zero == 0.0 && pos > 0.0);
            if tag_byte == 0 || latch == 0 {
                let flags: u32 = if zero.is_nan() {
                    0x47
                } else if zero == 0.0 {
                    0x42
                } else if zero < 0.0 {
                    0x03
                } else {
                    0x02
                };
                let expect = LIVE_FAIL_TOP | (flags << 8) | u32::from(latch);
                assert_eq!(ret, expect, "trial {trial}: failing answer matches");
                assert_eq!(out, expect, "trial {trial}: lift answers it too");
                assert!(calls.is_empty(), "trial {trial}: the mixer never runs");
                assert!(seen.is_empty(), "trial {trial}: lift mixes nothing");
            } else {
                assert_eq!(ret, answer, "trial {trial}: mixer answer matches");
                assert_eq!(out, answer, "trial {trial}: lift answers it too");
                assert_eq!(
                    calls,
                    vec![(1, vec![mixer, first, this.wrapping_add(0x20)])],
                    "trial {trial}: one mixer call"
                );
                assert_eq!(seen, vec![first], "trial {trial}: lifted word matches");
            }
            let after = unsafe { image(this, 0x6C) }.to_vec();
            assert_eq!(after, initial, "trial {trial}: the gate writes nothing");
            // Wrong version: negative zero fails the zero check.
            let wrong_latch = u8::from(zero.to_bits() == 0 && pos > 0.0);
            if wrong_latch != latch && tag_byte != 0 {
                caught += 1;
            }
            let _ = (obj, mixer_box);
        }
        assert!(caught > 0, "neg-zero mutant was never caught");
    }
}
