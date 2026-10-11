//! Host tests for the lifted `audio_slot` module: edge cases on the 64-bit
//! host, covering empty stores, single slots, dead values, wrap edges
//! and every panic domain.

use lf_audio::audio_slot::banked::{BankedSlots, NO_SLOT, VoiceBankFile, VoiceNode};
use lf_audio::audio_slot::misc::{GateState, LivenessProbe, OwnedSlot, SlotHead, TrackerCell};
use lf_audio::audio_slot::pools::{PtrArray, StridedPool, TRIPLET_FREE, Triplet, TripletTable};
use lf_audio::audio_slot::registry;
use lf_audio::audio_slot::voicelist::{
    BIT_WORDS, ChainCell, ChainStore, NONE, SPILL_COUNT, VOICES, VoiceList, VoiceSlot,
};

fn bank_file() -> VoiceBankFile {
    VoiceBankFile::from_parts(0x70, 0x1111_0000, vec![0x2222_0000, 0x3333_0000])
}

fn banked() -> BankedSlots {
    BankedSlots::from_parts(1, vec![3, NO_SLOT], 0x5A5A, 7)
}

#[test]
fn lookup_store_null_value_stores_empty() {
    let file = bank_file();
    let mut obj = banked();
    assert_eq!(obj.lookup_store(&file, 1, 0), 1);
    assert_eq!(obj.lookup_store(&file, 0, 0), 0);
}

#[test]
fn lookup_store_divides_wrapped_difference() {
    let file = bank_file();
    let mut obj = banked();
    // row(1) = 0x33330000, stride 0x70: value row + 5 * stride -> 5.
    let value = 0x3333_0000u32.wrapping_add(5 * 0x70);
    assert_eq!(obj.lookup_store(&file, 0, value), 0);
    // A value below the row wraps before dividing.
    let mut obj = banked();
    let value = 0x3333_0000u32.wrapping_sub(1);
    let out = obj.lookup_store(&file, 0, value);
    assert_eq!(out, 0);
    assert_eq!(
        obj.lookup_store(&file, 1, value),
        1,
        "low byte of the wrapped quotient is stored"
    );
}

#[test]
#[should_panic(expected = "past the 2 modelled slots")]
fn lookup_store_past_slots_panics() {
    let file = bank_file();
    let mut obj = banked();
    let _ = obj.lookup_store(&file, 2, 1);
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn lookup_store_zero_stride_divides() {
    // Stride 0 with a nonzero value: the original divides regardless.
    let file = VoiceBankFile::from_parts(0, 0, vec![0]);
    let mut obj = BankedSlots::from_parts(0, vec![0], 0, 0);
    let _ = obj.lookup_store(&file, 0, 1);
}

#[test]
fn node_lookup_empty_is_none() {
    let file = bank_file();
    let obj = banked();
    assert_eq!(obj.node_lookup(&file, 1), None);
    assert_eq!(
        obj.node_lookup(&file, 0),
        Some(VoiceNode { bank: 1, slot: 3 })
    );
}

#[test]
#[should_panic(expected = "past the 2 modelled slots")]
fn node_lookup_past_slots_panics() {
    let file = bank_file();
    let obj = banked();
    let _ = obj.node_lookup(&file, 9);
}

#[test]
#[should_panic(expected = "past the 2 recorded rows")]
fn node_lookup_past_rows_panics() {
    let file = bank_file();
    let obj = BankedSlots::from_parts(9, vec![1], 0, 0);
    let _ = obj.node_lookup(&file, 0);
}

#[test]
fn op_forward_empty_answers_zero_without_calling() {
    let file = bank_file();
    let obj = BankedSlots::from_parts(0, vec![NO_SLOT], 0, 0);
    let mut calls = 0;
    let out = obj.op_forward(&file, 0xDEAD, &mut |_: VoiceNode, _: u32| {
        calls += 1;
        0xBEEF
    });
    assert_eq!(out, 0);
    assert_eq!(calls, 0);
}

#[test]
fn op_forward_null_target_answers_zero() {
    // row 0, stride 0, slot 0: target 0, no call.
    let file = VoiceBankFile::from_parts(0, 0x9999, vec![0]);
    let obj = BankedSlots::from_parts(0, vec![0], 0, 0);
    let mut calls = 0;
    let out = obj.op_forward(&file, 1, &mut |_: VoiceNode, _: u32| {
        calls += 1;
        7
    });
    assert_eq!(out, 0);
    assert_eq!(calls, 0);
}

#[test]
fn op_forward_live_calls_once() {
    let file = bank_file();
    let obj = banked();
    let mut seen = Vec::new();
    let out = obj.op_forward(&file, 0x11, &mut |node: VoiceNode, v: u32| {
        seen.push((node, v));
        0x22
    });
    assert_eq!(out, 0x22);
    assert_eq!(seen, vec![(VoiceNode { bank: 1, slot: 3 }, 0x11)]);
}

#[test]
fn retrigger_paths() {
    struct Ops {
        log: Vec<&'static str>,
    }
    impl lf_audio::audio_slot::banked::Retrigger for Ops {
        fn setup(&mut self, _: VoiceNode, _: u32) {
            self.log.push("setup");
        }
        fn retrigger(&mut self, _: VoiceNode, _: u32) {
            self.log.push("retrigger");
        }
        fn chain(&mut self) -> u32 {
            self.log.push("chain");
            0xC0DA
        }
    }
    let file = bank_file();
    // Empty slot: 0xFF, nothing runs.
    let obj = BankedSlots::from_parts(0, vec![NO_SLOT], 0, 0);
    let mut ops = Ops { log: Vec::new() };
    assert_eq!(obj.retrigger(&file, 1, &mut ops), 0xFF);
    assert!(ops.log.is_empty());
    // Null target: the table base, nothing runs.
    let file0 = VoiceBankFile::from_parts(0, 0xABCD, vec![0]);
    let obj = BankedSlots::from_parts(0, vec![0], 0, 0);
    let mut ops = Ops { log: Vec::new() };
    assert_eq!(obj.retrigger(&file0, 1, &mut ops), 0xABCD);
    assert!(ops.log.is_empty());
    // Live: setup, retrigger, chain in order.
    let obj = banked();
    let mut ops = Ops { log: Vec::new() };
    assert_eq!(obj.retrigger(&file, 1, &mut ops), 0xC0DA);
    assert_eq!(ops.log, vec!["setup", "retrigger", "chain"]);
}

#[test]
fn probe_tag_paths() {
    struct Fake {
        live: bool,
        tags: [u16; 2],
        offers: Vec<u32>,
        log: Vec<&'static str>,
    }
    impl lf_audio::audio_slot::banked::Probe for Fake {
        fn entry_live(&mut self, _: VoiceNode) -> bool {
            self.log.push("live");
            self.live
        }
        fn setup_this(&mut self, _: VoiceNode) {
            self.log.push("setup_this");
        }
        fn setup_entry(&mut self, _: VoiceNode) {
            self.log.push("setup_entry");
        }
        fn entry_tag(&mut self, _: VoiceNode) -> u16 {
            self.log.push("tag");
            self.tags[0]
        }
        fn offer(&mut self, _: VoiceNode, _: u32) -> u32 {
            self.log.push("offer");
            self.offers.remove(0)
        }
    }
    let file = bank_file();
    // Dead indexed node, tag-1 slot: latches without setups.
    let obj = BankedSlots::from_parts(1, vec![3, NO_SLOT], 0, 7);
    let mut fake = Fake {
        live: false,
        tags: [1, 0],
        offers: vec![],
        log: Vec::new(),
    };
    assert!(obj.probe(&file, 0, &mut fake));
    assert_eq!(fake.log, vec!["live", "tag"]);
    // Tag 2 with a zero low byte latches nothing.
    let mut fake = Fake {
        live: false,
        tags: [2, 0],
        offers: vec![0x100],
        log: Vec::new(),
    };
    assert!(!obj.probe(&file, 0, &mut fake));
    // Unknown tags latch nothing and offer nothing.
    let mut fake = Fake {
        live: false,
        tags: [9, 0],
        offers: vec![],
        log: Vec::new(),
    };
    assert!(!obj.probe(&file, 0, &mut fake));
    assert_eq!(fake.log, vec!["live", "tag"]);
}

#[test]
fn voice_list_alloc_edges() {
    struct Nop;
    impl lf_audio::audio_slot::voicelist::SlotLock for Nop {
        fn lock(&mut self) {}
        fn unlock(&mut self) {}
    }
    let slots = vec![
        VoiceSlot {
            value: 0,
            head: NONE
        };
        VOICES as usize
    ];
    // Full list answers 0xFFFF.
    let mut full = VoiceList::from_parts(
        vec![0xFFFF_FFFF; BIT_WORDS],
        slots.clone(),
        vec![0; SPILL_COUNT],
    );
    assert_eq!(full.alloc(0xAA, &mut Nop), u32::from(NONE));
    // Slot 0 alone clear still answers 0xFFFF: the scan starts at 1.
    let mut bits = vec![0xFFFF_FFFF; BIT_WORDS];
    bits[0] &= !1;
    let mut list = VoiceList::from_parts(bits, slots.clone(), vec![0; SPILL_COUNT]);
    assert_eq!(list.alloc(0xAA, &mut Nop), u32::from(NONE));
    // Empty list takes slot 1.
    let mut list = VoiceList::from_parts(vec![0; BIT_WORDS], slots, vec![0; SPILL_COUNT]);
    assert_eq!(list.alloc(0xAA, &mut Nop), 1);
    assert_eq!(list.alloc(0xBB, &mut Nop), 2);
}

#[test]
fn spill_alloc_edges() {
    struct Nop;
    impl lf_audio::audio_slot::voicelist::SlotLock for Nop {
        fn lock(&mut self) {}
        fn unlock(&mut self) {}
    }
    let slots = vec![
        VoiceSlot {
            value: 0,
            head: NONE
        };
        VOICES as usize
    ];
    let mut list = VoiceList::from_parts(vec![0; BIT_WORDS], slots, vec![1; SPILL_COUNT]);
    list.spill_alloc(0); // zero stores nothing, even when free cells exist
    list.spill_alloc(9); // full table stores nothing
    let mut list2 = VoiceList::from_parts(
        vec![0; BIT_WORDS],
        vec![
            VoiceSlot {
                value: 0,
                head: NONE
            };
            VOICES as usize
        ],
        vec![5, 0, 0],
    );
    list2.spill_alloc(9); // first zero wins
    let _ = (&mut list, &mut Nop);
}

#[test]
fn sweep_quiet_list_calls_nothing() {
    struct Nop;
    impl lf_audio::audio_slot::voicelist::SlotLock for Nop {
        fn lock(&mut self) {}
        fn unlock(&mut self) {}
    }
    struct Voices {
        calls: u32,
    }
    impl lf_audio::audio_slot::voicelist::SweepVoices for Voices {
        fn refresh(&mut self, _: VoiceNode) -> u32 {
            self.calls += 1;
            0
        }
        fn commit(&mut self, _: VoiceNode) {
            self.calls += 1;
        }
    }
    let slots = vec![
        VoiceSlot {
            value: 0,
            head: NONE
        };
        VOICES as usize
    ];
    let list = VoiceList::from_parts(vec![0; BIT_WORDS], slots, vec![0; SPILL_COUNT]);
    let chains = ChainStore::from_cells(vec![]);
    let file = bank_file();
    let mut voices = Voices { calls: 0 };
    assert_eq!(list.sweep(&chains, &file, 0, &mut Nop, &mut voices), 0);
    assert_eq!(voices.calls, 0);
}

#[test]
fn sweep_commits_only_on_match() {
    struct Nop;
    impl lf_audio::audio_slot::voicelist::SlotLock for Nop {
        fn lock(&mut self) {}
        fn unlock(&mut self) {}
    }
    struct Voices {
        answer: u32,
        commits: u32,
    }
    impl lf_audio::audio_slot::voicelist::SweepVoices for Voices {
        fn refresh(&mut self, _: VoiceNode) -> u32 {
            self.answer
        }
        fn commit(&mut self, _: VoiceNode) {
            self.commits += 1;
        }
    }
    let mut slots = vec![
        VoiceSlot {
            value: 0,
            head: NONE
        };
        VOICES as usize
    ];
    slots[4] = VoiceSlot { value: 1, head: 0 };
    let mut bits = vec![0; BIT_WORDS];
    bits[0] |= 1 << 4;
    let list = VoiceList::from_parts(bits, slots, vec![0; SPILL_COUNT]);
    let chains = ChainStore::from_cells(vec![ChainCell {
        next: NONE,
        bank: 0,
        slot: 2,
    }]);
    let file = VoiceBankFile::from_parts(0x70, 0, vec![0x1000]);
    let mut voices = Voices {
        answer: 0x51,
        commits: 0,
    };
    assert_eq!(list.sweep(&chains, &file, 0x51, &mut Nop, &mut voices), 0);
    assert_eq!(voices.commits, 1);
    let mut voices = Voices {
        answer: 0x52,
        commits: 0,
    };
    assert_eq!(list.sweep(&chains, &file, 0x51, &mut Nop, &mut voices), 0);
    assert_eq!(voices.commits, 0);
}

#[test]
fn strided_pool_full_answers_zero() {
    let mut pool = StridedPool::from_parts(0, 0, 0x5000, vec![]);
    let mut calls = 0;
    assert_eq!(
        pool.alloc(1, 2, &mut |_: u32, _: u32, _: u32| calls += 1),
        0
    );
    assert_eq!(calls, 0);
    // Count past capacity answers zero too.
    let mut pool = StridedPool::from_parts(2, 9, 0x5000, vec![(0, 0); 10]);
    assert_eq!(
        pool.alloc(1, 2, &mut |_: u32, _: u32, _: u32| calls += 1),
        0
    );
    assert_eq!(calls, 0);
}

#[test]
fn triplet_table_reuses_freed_markers() {
    let mut table = TripletTable::from_entries(
        [Triplet {
            marker: 1,
            first: 0,
            second: 0,
        }; 32],
    );
    assert_eq!(table.alloc(1, 2, 3), None);
    // A stored FREE marker reads as free again, like the 32-bit scan.
    let mut entries = [Triplet {
        marker: 1,
        first: 0,
        second: 0,
    }; 32];
    entries[0].marker = TRIPLET_FREE;
    let mut table = TripletTable::from_entries(entries);
    assert_eq!(table.alloc(0xAA, 0xBB, TRIPLET_FREE), Some(0));
    assert_eq!(table.alloc(0xCC, 0xDD, 0xEE), Some(0));
}

#[test]
fn ptr_array_skips_zeros() {
    let mut seen = Vec::new();
    let mut array = PtrArray::from_slots(vec![0, 0x100, 0]);
    assert_eq!(array.release_all(&mut |i: u32| seen.push(i)), 0);
    assert_eq!(seen, vec![1]);
}

#[test]
fn tracker_cell_wraps() {
    let cell = TrackerCell(0xFFFF_FFFF);
    let mut seen = Vec::new();
    assert_eq!(
        cell.add_and_dispatch(1, &mut |s: u32| {
            seen.push(s);
            0x77
        }),
        0x77
    );
    assert_eq!(seen, vec![0]);
}

#[test]
fn slot_head_flag_mapping() {
    assert_eq!(SlotHead::fresh(0x00).flag, 0x02);
    assert_eq!(SlotHead::fresh(0xFF).flag, 0xFA);
    assert_eq!(SlotHead::fresh(0x05).flag, 0x02);
}

#[test]
fn gate_state_edges() {
    assert!(
        GateState {
            g1: 0,
            g2: 5,
            g3: 5,
            g4: 0
        }
        .live()
    );
    assert!(
        !GateState {
            g1: 1,
            g2: 5,
            g3: 5,
            g4: 0
        }
        .live()
    );
    assert!(
        !GateState {
            g1: 0,
            g2: 5,
            g3: 6,
            g4: 0
        }
        .live()
    );
    assert!(
        !GateState {
            g1: 0,
            g2: 5,
            g3: 5,
            g4: 0x12
        }
        .live()
    );
}

#[test]
fn owned_slot_absent_calls_nothing() {
    struct Ops {
        calls: u32,
    }
    impl lf_audio::audio_slot::misc::GatedRelease for Ops {
        fn slot_param(&mut self, _: lf_audio::audio_slot::misc::SlotRef) -> u32 {
            self.calls += 1;
            0
        }
        fn notify(&mut self, _: u32) {
            self.calls += 1;
        }
        fn release(&mut self, _: lf_audio::audio_slot::misc::SlotRef) {
            self.calls += 1;
        }
    }
    let gates = GateState {
        g1: 0,
        g2: 1,
        g3: 1,
        g4: 0,
    };
    let mut ops = Ops { calls: 0 };
    OwnedSlot(0).release_gated(&gates, &mut ops);
    assert_eq!(ops.calls, 0);
}

#[test]
fn liveness_gate_float_edges() {
    use lf_audio::audio_slot::misc::LIVE_FAIL_TOP;
    // Negative zero passes the zero check; NaN fails everything.
    let probe = LivenessProbe {
        tag: 1,
        zero: -0.0,
        pos: 1.0,
    };
    let mut calls = 0;
    assert_eq!(
        probe.gate(0x1234, &mut |_: u32| {
            calls += 1;
            0x55
        }),
        0x55
    );
    assert_eq!(calls, 1);
    for zero in [f32::NAN, 1.0, -1.0, f32::INFINITY] {
        let probe = LivenessProbe {
            tag: 1,
            zero,
            pos: 1.0,
        };
        let out = probe.gate(0x1234, &mut |_: u32| 0x55);
        assert_eq!(out & 0xFFFF_0000, LIVE_FAIL_TOP);
        assert_eq!(out & 0xFF, 0, "zero {zero}: latch clear");
    }
    // NaN flag byte is 0x47, zero is 0x42.
    let probe = LivenessProbe {
        tag: 1,
        zero: f32::NAN,
        pos: 1.0,
    };
    assert_eq!((probe.gate(0, &mut |_: u32| 0) >> 8) & 0xFF, 0x47);
    let probe = LivenessProbe {
        tag: 0,
        zero: 0.0,
        pos: 1.0,
    };
    let out = probe.gate(0, &mut |_: u32| 0);
    assert_eq!((out >> 8) & 0xFF, 0x42);
    assert_eq!(out & 0xFF, 1, "float checks passed, tag failed");
}

#[test]
fn registry_counts_cover_the_group() {
    assert_eq!(registry::proven_count(), 15);
    assert_eq!(registry::missing_count(), 42);
    assert_eq!(registry::proven_count() + registry::missing_count(), 57);
}
