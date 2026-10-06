//! Differential cases, part 2: the voice list.
//!
//! Each case plants the list object, the bit-words buffer and (for the
//! sweep) the chain cells and row table, runs the rewrite and the lift
//! on the same inputs, and compares returns, every written byte and
//! every callee call in order. The lock-guard addresses never cross the
//! trait: the rewrite's guard words are pinned to the planted lock
//! address instead. Each method has a deliberately wrong lift that must
//! be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::audio_slot::banked::{VoiceBankFile, VoiceNode};
    use lf_audio::audio_slot::voicelist::{
        BIT_WORDS, NONE, SPILL_COUNT, VOICES, ChainCell, ChainStore, VoiceList, VoiceSlot,
    };
    use lf_audioslotdiff::rewrites::*;
    use lf_audioslotdiff::rt::{self, StubKind};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{TABLE_VA, STRIDE_VA, Rng, addr, get_u32, lock, put_u32};

    /// List object bytes: covers slots, bitset pointer, spill and lock.
    const OBJ: usize = 0x3300;
    /// Row-table bytes for banks 0..4.
    const ROWTAB: usize = 3 * 0x6F40 + 0x6F10 + 4;

    /// Fresh view of a test image; rebuilt after every rewrite call so no
    /// pre-call borrow is read back (the compiler would forward it).
    unsafe fn image(base: u32, len: usize) -> &'static mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(base as *mut u8, len) }
    }

    /// Builds the lift's list from the planted images.
    fn lift_list(bitwords: &[u32], initial: &[u8]) -> VoiceList {
        let mut slots = Vec::with_capacity(VOICES as usize);
        for i in 0..VOICES {
            let at = 0xFA0 + i as usize * 8;
            slots.push(VoiceSlot {
                value: get_u32(initial, at),
                head: u16::from_le_bytes(initial[at + 4..at + 6].try_into().unwrap()),
            });
        }
        let mut spill = Vec::with_capacity(SPILL_COUNT);
        for i in 0..SPILL_COUNT {
            spill.push(get_u32(initial, 0x28A8 + i * 4));
        }
        VoiceList::from_parts(bitwords.to_vec(), slots, spill)
    }

    /// A recording lock: the guard addresses stay on the 32-bit side.
    struct FakeLock {
        log: Vec<u8>,
    }

    impl lf_audio::audio_slot::voicelist::SlotLock for FakeLock {
        fn lock(&mut self) {
            self.log.push(1);
        }
        fn unlock(&mut self) {
            self.log.push(2);
        }
    }

    #[test]
    fn alloc_matches() {
        let _guard = lock();
        let mut rng = Rng(0x9FE0);
        let mut caught = 0;
        for trial in 0..40u32 {
            let arg = rng.u32();
            // Bit patterns: full, first-clear at k, slot-0-only-clear
            // (the scan starts at 1), random mixes.
            let mut bits = vec![0u32; BIT_WORDS];
            let first_clear = match trial % 6 {
                0 => {
                    bits = vec![0xFFFF_FFFF; BIT_WORDS];
                    None
                }
                1 => {
                    bits = vec![0xFFFF_FFFF; BIT_WORDS];
                    bits[0] &= !1; // only slot 0 clear: still full
                    None
                }
                2 => {
                    bits = vec![0xFFFF_FFFF; BIT_WORDS];
                    let k = 1 + rng.below(799);
                    bits[(k >> 5) as usize] &= !(1 << (k & 31));
                    Some(k)
                }
                3 => {
                    bits = vec![0; BIT_WORDS];
                    Some(1)
                }
                _ => {
                    for b in bits.iter_mut() {
                        *b = rng.u32();
                    }
                    (1..VOICES).find(|i| bits[(i >> 5) as usize] & (1 << (i & 31)) == 0)
                }
            };
            let mut initial = vec![0u8; OBJ];
            rng.bytes(&mut initial);
            let bit_box: Box<[u32]> = bits.clone().into_boxed_slice();
            let bit_addr = addr(&bit_box[0]);
            put_u32(&mut initial, 0x28A0, bit_addr);
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            rt::set_script(&[
                (1, StubKind::Thiscall2, vec![0]),
                (2, StubKind::Thiscall1, vec![0]),
            ]);
            let ret = unsafe { fn_008A9FE0::rw_008A9FE0(this, arg) };
            let calls = rt::take_calls();
            let mut lift = lift_list(&bits, &initial);
            let mut lk = FakeLock { log: Vec::new() };
            let out = lift.alloc(arg, &mut lk);
            let expect = first_clear.unwrap_or(u32::from(NONE));
            assert_eq!(ret, expect, "trial {trial}: answer matches");
            assert_eq!(out, expect, "trial {trial}: lift answers it too");
            // Lock order matches; the guard words are pinned, not passed.
            assert_eq!(calls.len(), 2, "trial {trial}: ctor and dtor run");
            assert_eq!(calls[0].0, 1, "trial {trial}: ctor first");
            assert_eq!(calls[0].1[1], this.wrapping_add(0x3210), "trial {trial}: ctor on the lock");
            assert_eq!(calls[1], (2, vec![calls[0].1[0]]), "trial {trial}: dtor on the guard");
            assert_eq!(lk.log, vec![1, 2], "trial {trial}: lift locks once");
            let after = unsafe { image(this, OBJ) }.to_vec();
            let bits_after =
                unsafe { std::slice::from_raw_parts(bit_addr as *const u32, BIT_WORDS) }.to_vec();
            let mut expect_bits = bits.clone();
            let mut expect_img = initial.clone();
            if let Some(k) = first_clear {
                expect_bits[(k >> 5) as usize] |= 1 << (k & 31);
                let at = 0xFA0 + k as usize * 8;
                expect_img[at..at + 4].copy_from_slice(&arg.to_le_bytes());
                expect_img[at + 4..at + 6].copy_from_slice(&NONE.to_le_bytes());
            }
            assert_eq!(bits_after, expect_bits, "trial {trial}: bit-words match");
            assert_eq!(after, expect_img, "trial {trial}: image matches");
            // Wrong version: the scan starts at slot 0.
            let wrong = (0..VOICES).find(|i| bits[(i >> 5) as usize] & (1 << (i & 31)) == 0);
            if wrong != first_clear {
                caught += 1;
            }
            let _ = (obj, bit_box);
        }
        assert!(caught > 0, "scan-from-0 mutant was never caught");
    }

    #[test]
    fn spill_alloc_matches() {
        let _guard = lock();
        let mut rng = Rng(0xA080);
        let mut caught = 0;
        for trial in 0..30u32 {
            let value = match trial % 4 {
                0 => 0,
                1 => rng.u32() | 1,
                _ => rng.u32(),
            };
            let mut initial = vec![0u8; OBJ];
            rng.bytes(&mut initial);
            if trial % 3 == 0 {
                // Guarantee two zeros: first-zero and last-zero differ.
                put_u32(&mut initial, 0x28A8, 0);
                put_u32(&mut initial, 0x28A8 + 4 * (SPILL_COUNT - 1), 0);
            }
            if trial % 5 == 0 {
                // A full table stores nothing.
                for i in 0..SPILL_COUNT {
                    put_u32(&mut initial, 0x28A8 + i * 4, 0x0101_0101);
                }
            }
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            unsafe { fn_008AA080::rw_008aa080(this as *mut u8, value) };
            let mut lift = lift_list(&vec![0u32; BIT_WORDS], &initial);
            lift.spill_alloc(value);
            let after = unsafe { image(this, OBJ) }.to_vec();
            let first_zero = (0..SPILL_COUNT)
                .find(|i| get_u32(&initial, 0x28A8 + i * 4) == 0);
            let mut expect = initial.clone();
            if value != 0 {
                if let Some(i) = first_zero {
                    put_u32(&mut expect, 0x28A8 + i * 4, value);
                }
            }
            assert_eq!(after, expect, "trial {trial}: image matches");
            // Wrong version: the last zero wins instead of the first.
            let last_zero = (0..SPILL_COUNT)
                .rev()
                .find(|i| get_u32(&initial, 0x28A8 + i * 4) == 0);
            if value != 0 && last_zero != first_zero {
                caught += 1;
            }
            let _ = obj;
        }
        assert!(caught > 0, "last-zero mutant was never caught");
    }

    #[test]
    fn sweep_matches() {
        let _guard = lock();
        let mut rng = Rng(0xAA0F0 & 0xFFFF_FFFF);
        let mut caught = 0;
        for trial in 0..30u32 {
            let arg = rng.u32();
            let scale = rng.u32();
            // Live slots: a spread of NONE heads, short chains and (every
            // fourth trial) an all-quiet list.
            let quiet = trial % 4 == 3;
            let mut bits = vec![0u32; BIT_WORDS];
            // Chain cells live at link*4 in the object; links stay small.
            let mut cells: Vec<Option<(u16, u8, u8)>> = vec![None; 48];
            let mut heads = vec![NONE; VOICES as usize];
            let mut nlinks = 0;
            if !quiet {
                for _ in 0..(2 + trial % 4) {
                    let slot = rng.below(VOICES);
                    bits[(slot >> 5) as usize] |= 1 << (slot & 31);
                    if trial % 3 == 0 {
                        continue; // live bit, NONE head: skipped
                    }
                    // A chain of one to three links.
                    let len = 1 + rng.below(3);
                    let mut prev: Option<u16> = None;
                    let mut first = NONE;
                    for _ in 0..len {
                        if nlinks + 1 >= 48 {
                            break;
                        }
                        nlinks += 1;
                        let link = nlinks as u16;
                        if first == NONE {
                            first = link;
                        }
                        let cell = (NONE, rng.below(4) as u8, rng.below(0xFF) as u8);
                        cells[link as usize] = Some(cell);
                        if let Some(p) = prev {
                            cells[p as usize].as_mut().unwrap().0 = link;
                        }
                        prev = Some(link);
                    }
                    heads[slot as usize] = first;
                }
            }
            let mut initial = vec![0u8; OBJ];
            rng.bytes(&mut initial);
            for i in 0..VOICES {
                let at = 0xFA0 + i as usize * 8;
                initial[at + 4..at + 6].copy_from_slice(&heads[i as usize].to_le_bytes());
            }
            for (link, cell) in cells.iter().enumerate() {
                if let Some((next, b2, b3)) = cell {
                    let at = link * 4;
                    initial[at..at + 2].copy_from_slice(&next.to_le_bytes());
                    initial[at + 2] = *b2;
                    initial[at + 3] = *b3;
                }
            }
            let mut rows = vec![0u32; 4];
            for r in rows.iter_mut() {
                *r = rng.u32();
            }
            let mut rowtab = vec![0u8; ROWTAB];
            rng.bytes(&mut rowtab);
            for (b, row) in rows.iter().enumerate() {
                put_u32(&mut rowtab, b * 0x6F40 + 0x6F10, *row);
            }
            let bit_box: Box<[u32]> = bits.clone().into_boxed_slice();
            let bit_addr = addr(&bit_box[0]);
            put_u32(&mut initial, 0x28A0, bit_addr);
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            let table_box: Box<[u8]> = rowtab.into_boxed_slice();
            let table = addr(&table_box[0]);
            rt::set_global(TABLE_VA, table);
            rt::set_global(STRIDE_VA, scale);
            // Refresh answers alternate match/mismatch so both commit
            // paths run.
            let mut refresh_answers = Vec::new();
            for i in 0..nlinks {
                refresh_answers.push(if i % 2 == 0 { arg } else { arg.wrapping_add(1) });
            }
            rt::set_script(&[
                (1, StubKind::Thiscall2, vec![0]),
                (2, StubKind::Thiscall1, vec![0]),
                (3, StubKind::Thiscall1, refresh_answers.clone()),
                (4, StubKind::Thiscall1, vec![]),
            ]);
            let ret = unsafe { fn_008AA0F0::rw_008AA0F0(this, arg) };
            let calls = rt::take_calls();
            // The lift runs on the same planted data.
            let lift = lift_list(&bits, &initial);
            let mut store_cells = Vec::new();
            for link in 0..48u16 {
                if let Some((next, b2, b3)) = cells[link as usize] {
                    while store_cells.len() <= link as usize {
                        store_cells.push(ChainCell {
                            next: NONE,
                            bank: 0,
                            slot: 0,
                        });
                    }
                    store_cells[link as usize] = ChainCell {
                        next,
                        bank: b2,
                        slot: b3,
                    };
                }
            }
            while store_cells.len() < 48 {
                store_cells.push(ChainCell {
                    next: NONE,
                    bank: 0,
                    slot: 0,
                });
            }
            let chains = ChainStore::from_cells(store_cells);
            let file = VoiceBankFile::from_parts(scale, table, rows.clone());
            struct FakeVoices {
                answers: Vec<u32>,
                log: Vec<(u8, VoiceNode)>,
            }
            impl lf_audio::audio_slot::voicelist::SweepVoices for FakeVoices {
                fn refresh(&mut self, node: VoiceNode) -> u32 {
                    self.log.push((3, node));
                    self.answers.remove(0)
                }
                fn commit(&mut self, node: VoiceNode) {
                    self.log.push((4, node));
                }
            }
            let mut voices = FakeVoices {
                answers: refresh_answers.clone(),
                log: Vec::new(),
            };
            let mut lk = FakeLock { log: Vec::new() };
            let out = lift.sweep(&chains, &file, arg, &mut lk, &mut voices);
            assert_eq!(ret, 0, "trial {trial}: the sweep answers 0");
            assert_eq!(out, 0, "trial {trial}: lift answers 0");
            // Expected log: ctor, per-link refresh (+commit on match), dtor.
            let mut expect_keys: Vec<(u8, VoiceNode)> = Vec::new();
            for i in 0..VOICES {
                if bits[(i >> 5) as usize] & (1 << (i & 31)) == 0 {
                    continue;
                }
                let mut link = heads[i as usize];
                if link == NONE {
                    continue;
                }
                loop {
                    let (next, b2, b3) = cells[link as usize].unwrap();
                    let node = VoiceNode {
                        bank: b2,
                        slot: b3,
                    };
                    expect_keys.push((3, node));
                    let ans = refresh_answers[expect_keys.iter().filter(|(id, _)| *id == 3).count() - 1];
                    if ans == arg {
                        expect_keys.push((4, node));
                    }
                    link = next;
                    if link == NONE {
                        break;
                    }
                }
            }
            assert_eq!(calls.len(), expect_keys.len() + 2, "trial {trial}: call count");
            assert_eq!(calls[0].0, 1, "trial {trial}: ctor first");
            assert_eq!(calls[0].1[1], this.wrapping_add(0x3210), "trial {trial}: ctor on the lock");
            for (k, (id, node)) in expect_keys.iter().enumerate() {
                let h = rows[node.bank as usize]
                    .wrapping_add(scale.wrapping_mul(u32::from(node.slot)));
                assert_eq!(
                    calls[k + 1],
                    (*id as u32, vec![h]),
                    "trial {trial}: call {} matches",
                    k + 1
                );
                // The handle rebuilds from the lifted key.
                assert_eq!(
                    file.node_addr(voices.log[k].1),
                    h,
                    "trial {trial}: handle {} rebuilds",
                    k + 1
                );
            }
            assert_eq!(
                calls[expect_keys.len() + 1],
                (2, vec![calls[0].1[0]]),
                "trial {trial}: dtor on the guard"
            );
            assert_eq!(voices.log, expect_keys, "trial {trial}: lift log matches");
            assert_eq!(lk.log, vec![1, 2], "trial {trial}: lift locks once");
            let after = unsafe { image(this, OBJ) }.to_vec();
            assert_eq!(after, initial, "trial {trial}: the sweep writes nothing");
            let bits_after =
                unsafe { std::slice::from_raw_parts(bit_addr as *const u32, BIT_WORDS) }.to_vec();
            assert_eq!(bits_after, bits, "trial {trial}: bit-words unchanged");
            // Wrong version: commit on mismatch instead of match.
            let mismatch_commits = expect_keys.iter().filter(|(id, _)| *id == 4).count();
            let refreshes = expect_keys.iter().filter(|(id, _)| *id == 3).count();
            if mismatch_commits > 0 && mismatch_commits < refreshes {
                caught += 1;
            }
            let _ = (obj, bit_box, table_box);
        }
        assert!(caught > 0, "inverted-commit mutant was never caught");
    }
}
