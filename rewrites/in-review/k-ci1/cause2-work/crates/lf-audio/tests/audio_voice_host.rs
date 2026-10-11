//! Host tests for the lifted `audio_voice` free functions: edge cases a
//! reader would ask about, every panic domain, and the registry counts.
//! Runs on the 64-bit host (no rewrites here).

use lf_audio::audio_voice::activation::{
    ActivationWorld, BITSET_WORDS, BitSet, BoundSlot, LockHandle, SLOT_COUNT, SlotHandle,
    VoiceActivation, VoiceLocks,
};
use lf_audio::audio_voice::banked::{BankRecord, BankedVoices, VoiceSel};
use lf_audio::audio_voice::params::{BLOCK_COUNT, ParamBlock, ParamBlocks, RESET_TAG, VoiceHead};
use lf_audio::audio_voice::registry::{MISSING, PROVEN, ROWS, State};
use lf_audio::audio_voice::slots::VoiceSlots;
use lf_audio::audio_voice::tracker::{
    INVALID_HANDLE, LinkedSlot, PARK_CACHED, PARK_TAG, PoolHandle, TrackerWorld, VoiceHandle,
    VoiceTracker,
};
use lf_core::Handle32;

fn h(raw: u32) -> Option<VoiceHandle> {
    Handle32::new(raw)
}

struct Rec {
    calls: Vec<String>,
    refresh_ans: u32,
    bind_ans: u32,
    resolve_ans: u32,
}

impl Rec {
    fn new() -> Self {
        Self {
            calls: Vec::new(),
            refresh_ans: 0,
            bind_ans: 0,
            resolve_ans: 0,
        }
    }
}

impl TrackerWorld for Rec {
    fn release(&mut self, voice: VoiceHandle) {
        self.calls.push(format!("release {}", voice.get()));
    }
    fn install_pair(&mut self, tracker: &mut VoiceTracker, voice: VoiceHandle, count: u32) {
        self.calls.push(format!("install {} {count}", voice.get()));
        tracker.voice = Some(voice);
        tracker.count = count;
    }
    fn bind(&mut self, owner: Option<VoiceHandle>) -> u32 {
        self.calls
            .push(format!("bind {}", Handle32::raw_or_zero(owner)));
        self.bind_ans
    }
    fn resolve(&mut self, voice: VoiceHandle, arg: u32) -> u32 {
        self.calls.push(format!("resolve {} {arg}", voice.get()));
        self.resolve_ans
    }
    fn attach(&mut self, handle: u32) {
        self.calls.push(format!("attach {handle}"));
    }
    fn refresh(&mut self, pool: Option<PoolHandle>, voice: Option<VoiceHandle>) -> u32 {
        self.calls.push(format!(
            "refresh {} {}",
            Handle32::raw_or_zero(pool),
            Handle32::raw_or_zero(voice)
        ));
        self.refresh_ans
    }
}

#[test]
fn param_default_is_zero_one_zero() {
    assert_eq!(ParamBlock::DEFAULT.code, 0);
    assert_eq!(ParamBlock::DEFAULT.gain.to_bits(), 0x3F80_0000);
    assert_eq!(ParamBlock::DEFAULT.flags, 0);
    assert_eq!(BLOCK_COUNT, 5);
}

#[test]
fn param_reset_sets_all_five_and_is_idempotent() {
    let mut p = ParamBlocks::new(
        [ParamBlock {
            code: 7,
            gain: 2.5,
            flags: 9,
        }; BLOCK_COUNT],
    );
    p.reset();
    assert!(p.blocks.iter().all(|b| *b == ParamBlock::DEFAULT));
    p.reset();
    assert!(p.blocks.iter().all(|b| *b == ParamBlock::DEFAULT));
}

#[test]
fn param_reset_and_init_runs_helper_once() {
    let mut p = ParamBlocks::new([ParamBlock::DEFAULT; BLOCK_COUNT]);
    let mut n = 0;
    p.reset_and_init_sub(&mut || n += 1);
    assert_eq!(n, 1);
}

#[test]
fn header_reset_clears_and_tags() {
    let mut head = VoiceHead {
        seq: 0x1234_5678,
        tag: 0,
    };
    head.reset();
    assert_eq!(head.seq, 0);
    assert_eq!(head.tag, RESET_TAG);
    assert_eq!(RESET_TAG, 0xFFFF);
}

#[test]
fn tracker_install_without_link_answers_count_and_skips_world() {
    let mut t = VoiceTracker::new(h(0x100), None, 0);
    let mut w = Rec::new();
    w.refresh_ans = 0x77;
    let pool: Option<PoolHandle> = Handle32::new(0x300);
    let out = t.install(h(0x200), 0x1234, pool, &mut w);
    assert_eq!(out, 0x1234);
    assert!(w.calls.is_empty());
    assert_eq!(t.voice, h(0x200));
    assert_eq!(t.count, 0x1234);
}

#[test]
fn tracker_install_null_voice_and_pool_still_refreshes() {
    let mut t = VoiceTracker::new(h(1), Some(LinkedSlot { tag: 0, cached: 0 }), 0);
    let mut w = Rec::new();
    w.refresh_ans = 0x55;
    let out = t.install(None, 0xAB, None, &mut w);
    assert_eq!(out, 0x55);
    assert_eq!(w.calls, vec!["refresh 0 0".to_string()]);
    assert_eq!(t.voice, None);
    assert_eq!(t.link.unwrap().tag, 0xAB);
}

#[test]
fn tracker_release_without_voice_is_quiet_but_parks() {
    let mut t = VoiceTracker::new(None, Some(LinkedSlot { tag: 1, cached: 2 }), 9);
    let mut w = Rec::new();
    assert!(t.release_and_park(&mut w));
    assert!(w.calls.is_empty());
    assert_eq!(t.link.unwrap().tag, PARK_TAG);
    assert_eq!(t.link.unwrap().cached, PARK_CACHED);
}

#[test]
fn tracker_release_zero_and_negative_counts_make_no_call() {
    for count in [0u32, 0xFFFF_FFFF, 0x8000_0000] {
        let mut t = VoiceTracker::new(h(0x40), None, count);
        let mut w = Rec::new();
        assert!(!t.release_and_park(&mut w));
        assert!(w.calls.is_empty(), "count {count:#x} must not call");
        assert_eq!(t.voice, None, "voice clears even without a call");
        assert_eq!(t.count, count, "count survives without a call");
    }
}

#[test]
fn tracker_release_positive_count_calls_and_clears() {
    let mut t = VoiceTracker::new(h(0x40), None, 1);
    let mut w = Rec::new();
    assert!(!t.release_and_park(&mut w));
    assert_eq!(w.calls, vec!["release 64".to_string()]);
    assert_eq!(t.count, 0);
}

#[test]
fn tracker_install_and_bind_without_voice_is_quiet_zero() {
    let mut t = VoiceTracker::new(h(9), None, 9);
    let mut w = Rec::new();
    assert_eq!(t.install_and_bind(None, 3, &mut w), 0);
    assert!(w.calls.is_empty());
    assert_eq!(t.voice, h(9));
}

#[test]
fn tracker_handle_resolve_invalid_and_null_paths() {
    let mut t = VoiceTracker::new(None, None, 0);
    let mut w = Rec::new();
    assert!(!t.handle_resolve(1, &mut w));
    assert!(w.calls.is_empty());
    let mut t = VoiceTracker::new(h(0x50), None, 0);
    let mut w = Rec::new();
    w.resolve_ans = INVALID_HANDLE;
    assert!(!t.handle_resolve(0x77, &mut w));
    assert_eq!(w.calls, vec!["resolve 80 119".to_string()]);
}

#[test]
fn slots_flag_check_rejects_range_without_reading() {
    let slots = VoiceSlots::from_bytes(Vec::new());
    assert!(!slots.flag_check(3));
    assert!(!slots.flag_check(u32::MAX));
}

#[test]
#[should_panic(expected = "past the 0-byte store")]
fn slots_flag_check_empty_store_panics() {
    let slots = VoiceSlots::from_bytes(Vec::new());
    let _ = slots.flag_check(0);
}

#[test]
#[should_panic(expected = "past the")]
fn slots_slot_update_out_of_store_panics() {
    let mut slots = VoiceSlots::from_bytes(vec![0u8; 0x400]);
    // a = 0x98 reads the index byte at 0x400: past the store.
    slots.slot_update(0x98, 1, 2, 3, &mut |_, _| {});
}

#[test]
#[should_panic(expected = "past the")]
fn slots_flag_advance_negative_index_panics() {
    // t = -1, r = -1, idx = -1 + 0: address wraps out of store.
    let mut slots = VoiceSlots::from_bytes(vec![0u8; 4096]);
    slots.flag_advance(0, 0xFFFF_FFFE);
}

#[test]
fn slots_clear_zero_id_clears_zero_owners() {
    // Equality only: id 0 matches every zero owner word.
    let mut mem = vec![0u8; 0x600];
    mem[0x370..0x374].copy_from_slice(&1u32.to_le_bytes());
    let mut slots = VoiceSlots::from_bytes(mem);
    slots.clear_by_id(0);
    let img = slots.bytes();
    assert_eq!(u32::from_le_bytes(img[0x370..0x374].try_into().unwrap()), 1);
    assert_eq!(img[0x36C], 0, "a non-matching flag is untouched");
    assert_eq!(img[0x3CC], 3, "a zero owner matches id 0");
}

#[test]
fn slots_clear_is_idempotent() {
    let mut mem = vec![0u8; 0x600];
    mem[0x370..0x374].copy_from_slice(&9u32.to_le_bytes());
    let mut slots = VoiceSlots::from_bytes(mem);
    slots.clear_by_id(9);
    let once = slots.bytes().to_vec();
    slots.clear_by_id(9);
    assert_eq!(slots.bytes(), once.as_slice());
}

#[test]
#[should_panic(expected = "0xff")]
fn banked_set_flag8_fault_selector_panics() {
    let mut t = BankedVoices {
        scale: 0xEC,
        banks: vec![vec![BankRecord { slot: 0, flags: 0 }]],
    };
    t.set_flag8(VoiceSel { sel: 0xFF, sub: 0 });
}

#[test]
#[should_panic(expected = "0xff")]
fn banked_store_fault_selector_panics() {
    let mut t = BankedVoices {
        scale: 0xEC,
        banks: vec![vec![BankRecord { slot: 0, flags: 0 }]],
    };
    t.store_slot(VoiceSel { sel: 0xFF, sub: 0 }, 1);
}

#[test]
#[should_panic(expected = "past 1 banks")]
fn banked_missing_bank_panics() {
    let mut t = BankedVoices {
        scale: 0xEC,
        banks: vec![vec![BankRecord { slot: 0, flags: 0 }]],
    };
    t.set_flag8(VoiceSel { sel: 0, sub: 1 });
}

#[test]
fn banked_set_bit1_truth_table() {
    // (flag bit 1, arg bit 0) -> (changed, final bit 1, other bits kept).
    for flag in [0x00u8, 0x02, 0xFDu8, 0xFF] {
        for arg in [0x00u32, 0x01, 0xFE, 0xFF] {
            let mut t = BankedVoices {
                scale: 0xEC,
                banks: vec![vec![BankRecord {
                    slot: 0,
                    flags: flag,
                }]],
            };
            let changed = t.set_bit1(VoiceSel { sel: 0, sub: 0 }, arg);
            let want_bit = arg & 1 == 1;
            assert_eq!(
                t.banks[0][0].flags & 2 == 2,
                want_bit,
                "flag {flag:#x} arg {arg:#x}"
            );
            assert_eq!(t.banks[0][0].flags & !3, flag & !3, "other bits survive");
            assert_eq!(
                changed,
                (flag & 2 == 2) != want_bit,
                "changed reports the flip"
            );
        }
    }
}

struct AFake {
    calls: Vec<String>,
    probes: Vec<bool>,
}

impl AFake {
    fn new(probes: Vec<bool>) -> Self {
        Self {
            calls: Vec::new(),
            probes,
        }
    }
}

impl ActivationWorld for AFake {
    fn reset(&mut self) {
        self.calls.push("reset".to_string());
    }
    fn lock(&mut self, handle: Option<LockHandle>) {
        self.calls
            .push(format!("lock {}", Handle32::raw_or_zero(handle)));
    }
    fn unlock(&mut self, handle: Option<LockHandle>) {
        self.calls
            .push(format!("unlock {}", Handle32::raw_or_zero(handle)));
    }
    fn free_bits(&mut self, bits: BitSet) {
        self.calls.push(format!(
            "free {}",
            bits.iter().map(|w| w.count_ones()).sum::<u32>()
        ));
    }
    fn probe(&mut self, slot: SlotHandle) -> bool {
        self.calls.push(format!("probe {}", slot.get()));
        self.probes.remove(0)
    }
    fn notify(&mut self, slot: SlotHandle, arg: u32) {
        self.calls.push(format!("notify {} {arg}", slot.get()));
    }
}

fn locks(counter: u32, gate: bool, filter: bool) -> VoiceLocks {
    VoiceLocks {
        handle: Handle32::new(0x70),
        counter,
        gate,
        filter,
    }
}

fn activation(live: bool, active: bool, bits: Option<BitSet>) -> VoiceActivation {
    VoiceActivation {
        live,
        active,
        bits,
        slots: [Handle32::new(0x8000); SLOT_COUNT],
    }
}

#[test]
fn activation_update_quiet_paths_make_no_calls() {
    for (live, active, gate) in [
        (false, false, false),
        (false, true, true),
        (true, false, false),
    ] {
        let mut act = activation(live, active, Some([0; BITSET_WORDS]));
        let mut shared = locks(7, gate, true);
        let mut w = AFake::new(vec![]);
        act.update(0x99, &mut shared, &mut w);
        assert!(
            w.calls.is_empty(),
            "live={live} active={active} gate={gate}"
        );
        assert_eq!(shared.counter, 7);
        assert_eq!((act.live, act.active), (live, active));
    }
}

#[test]
fn activation_update_deactivate_wraps_counter() {
    let mut act = activation(true, true, Some([0; BITSET_WORDS]));
    let mut shared = locks(0, false, false);
    let mut w = AFake::new(vec![]);
    act.update(1, &mut shared, &mut w);
    assert_eq!(w.calls, vec!["reset", "lock 112", "unlock 112"]);
    assert_eq!(shared.counter, u32::MAX);
    assert!(!act.active);
}

#[test]
fn activation_update_activate_then_sweeps() {
    let mut bits = [0u32; BITSET_WORDS];
    bits[0] = 0b110; // slots 1 and 2.
    let mut act = activation(true, false, Some(bits));
    let mut shared = locks(9, true, false);
    let mut w = AFake::new(vec![]);
    act.update(0xA, &mut shared, &mut w);
    assert_eq!(shared.counter, 10);
    assert!(act.active);
    assert_eq!(
        w.calls,
        vec![
            "lock 112",
            "unlock 112",
            "notify 32768 10",
            "notify 32768 10"
        ]
    );
}

#[test]
fn activation_update_probe_gates_notify() {
    let mut bits = [0u32; BITSET_WORDS];
    bits[0] = 0b110;
    let mut act = activation(true, true, Some(bits));
    let mut shared = locks(3, true, true);
    let mut w = AFake::new(vec![false, true]);
    act.update(5, &mut shared, &mut w);
    assert_eq!(
        w.calls,
        vec!["probe 32768", "probe 32768", "notify 32768 5"]
    );
}

#[test]
#[should_panic(expected = "null voice bitset")]
fn activation_update_null_bitset_panics() {
    let mut act = activation(true, true, None);
    let mut shared = locks(0, true, false);
    let mut w = AFake::new(vec![]);
    act.update(0, &mut shared, &mut w);
}

#[test]
#[should_panic(expected = "null voice object in live slot 1")]
fn activation_update_null_slot_panics() {
    let mut bits = [0u32; BITSET_WORDS];
    bits[0] = 0b10; // slot 1 set.
    let mut act = activation(true, true, Some(bits));
    act.slots[0] = None;
    let mut shared = locks(0, true, false);
    let mut w = AFake::new(vec![]);
    act.update(0, &mut shared, &mut w);
}

#[test]
fn activation_teardown_paths() {
    // Quiet.
    let mut act = activation(false, true, Some([1; BITSET_WORDS]));
    let mut shared = locks(4, false, false);
    let mut w = AFake::new(vec![]);
    act.teardown(&mut shared, &mut w);
    assert!(w.calls.is_empty());
    assert_eq!(shared.counter, 4);
    // Inactive: free only.
    let mut act = activation(true, false, Some([0xFF; BITSET_WORDS]));
    let mut w = AFake::new(vec![]);
    act.teardown(&mut shared, &mut w);
    assert_eq!(w.calls, vec![format!("free {}", 25 * 8)]);
    assert!(!act.live && !act.active && act.bits.is_none());
    // Active: reset, lock pair, free.
    let mut act = activation(true, true, Some([1; BITSET_WORDS]));
    let mut w = AFake::new(vec![]);
    act.teardown(&mut shared, &mut w);
    assert_eq!(w.calls, vec!["reset", "lock 112", "unlock 112", "free 25"]);
    assert_eq!(shared.counter, 3);
    // Null bitset frees as zero words.
    let mut act = activation(true, false, None);
    let mut w = AFake::new(vec![]);
    act.teardown(&mut shared, &mut w);
    assert_eq!(w.calls, vec!["free 0"]);
}

#[test]
fn bound_check_edges() {
    assert!(!BoundSlot { flag: 0, id: 5 }.is_active(6));
    assert!(!BoundSlot { flag: 1, id: 0 }.is_active(6));
    assert!(!BoundSlot { flag: 1, id: 6 }.is_active(6));
    assert!(BoundSlot { flag: 0xFF, id: 5 }.is_active(6));
}

#[test]
fn registry_counts_are_pinned() {
    assert_eq!(ROWS.len(), 78);
    assert_eq!(PROVEN, 17);
    assert_eq!(MISSING, 61);
    assert_eq!(
        ROWS.iter().filter(|r| r.state == State::Proven).count(),
        PROVEN
    );
    assert_eq!(
        ROWS.iter().filter(|r| r.state == State::Missing).count(),
        MISSING
    );
    let mut addrs: Vec<&str> = ROWS.iter().map(|r| r.addr).collect();
    addrs.sort_unstable();
    addrs.dedup();
    assert_eq!(addrs.len(), 78, "one row per group routine");
}
