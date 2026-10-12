//! Host tests for the lifted base sound class: edge cases a reader would ask about.
//!
//! These run on the 64-bit host against a scripted world fake that records
//! every call. The differential proof against the verified rewrites lives in
//! the `lf-soundbase-diff` test crate (32-bit target only).

use std::collections::VecDeque;

use lf_audio::sound_base::registry::{self, State};
use lf_audio::sound_base::{
    AudSound, FallbackTag, ObjectTag, RegistryTag, SlotTag, SoundWorld, FAST_BIT, INIT_DONE,
    KIND_FAST, NEEDS_INIT, NO_VOICE, SEQ_UNSET,
};
use lf_core::Handle32;

/// A world fake: records calls in order, answers from queues, and holds the
/// globals as plain values.
#[derive(Default)]
struct Fake {
    log: Vec<String>,
    lookups: VecDeque<u32>,
    hashes: VecDeque<u32>,
    scalar_a: u32,
    scalar_b: u32,
    row_word: u32,
}

impl Fake {
    fn new() -> Self {
        Self {
            scalar_a: 0x40,
            scalar_b: 0x10,
            row_word: 0x1000,
            ..Self::default()
        }
    }
}

impl SoundWorld for Fake {
    fn scalar_a(&self) -> u32 {
        self.scalar_a
    }
    fn scalar_b(&self) -> u32 {
        self.scalar_b
    }
    fn voice_table_word(&self, _row: u8, _offset: u32) -> u32 {
        // A read of the table: not an effect, so it is not logged.
        self.row_word
    }
    fn registry(&self) -> Handle32<RegistryTag> {
        Handle32::new(0x9000).expect("nonzero")
    }
    fn fallback_cell(&self) -> Handle32<FallbackTag> {
        Handle32::new(0x9100).expect("nonzero")
    }
    fn install_vtable(&mut self, this: Handle32<ObjectTag>) {
        self.log.push(format!("vtable {:#x}", this.get()));
    }
    fn run_init(&mut self, this: Handle32<ObjectTag>, sub_id: u8) -> u32 {
        self.log.push(format!("init {sub_id} {:#x}", this.get()));
        0
    }
    fn set_cursor(&mut self, cursor: u32) -> u32 {
        self.log.push(format!("cursor {cursor:#x}"));
        0
    }
    fn register(&mut self, registry: Handle32<RegistryTag>, voice_id: u32, kind: u32) -> u32 {
        self.log
            .push(format!("register {:#x} {voice_id} {kind}", registry.get()));
        0
    }
    fn joaat(&mut self, name: u32, seed: u32) -> u32 {
        self.log.push(format!("joaat {name:#x} {seed}"));
        self.hashes.pop_front().unwrap_or(0)
    }
    fn find_slot0(&mut self, object: Handle32<ObjectTag>, hash: u32) -> u32 {
        self.log.push(format!("find {:#x} {hash:#x}", object.get()));
        self.lookups.pop_front().unwrap_or(0)
    }
    fn slot2(&mut self, object: Handle32<ObjectTag>, hash: u32, value_bits: u32) -> u32 {
        self.log
            .push(format!("slot2 {:#x} {hash:#x} {value_bits:#x}", object.get()));
        0
    }
    fn slot4(&mut self, target: u32, arg: u32) -> u32 {
        self.log.push(format!("slot4 {target:#x} {arg:#x}"));
        arg
    }
    fn forward_fallback(&mut self, cell: Handle32<FallbackTag>, arg: u32) -> u32 {
        self.log.push(format!("fallback {:#x} {arg:#x}", cell.get()));
        arg
    }
    fn store_word(&mut self, slot: Handle32<SlotTag>, value: u32) {
        self.log.push(format!("store {:#x} {value:#x}", slot.get()));
    }
    fn zero_byte(&mut self, slot: Handle32<SlotTag>) {
        self.log.push(format!("zero_byte {:#x}", slot.get()));
    }
    fn zero_word(&mut self, slot: Handle32<SlotTag>) {
        self.log.push(format!("zero_word {:#x}", slot.get()));
    }
    fn rebuild(&mut self, this: Handle32<ObjectTag>) -> u32 {
        self.log.push(format!("rebuild {:#x}", this.get()));
        0
    }
    fn unregister(
        &mut self,
        registry: Handle32<RegistryTag>,
        this: Handle32<ObjectTag>,
        voice_id: u32,
    ) -> u32 {
        self.log.push(format!(
            "unregister {:#x} {:#x} {voice_id}",
            registry.get(),
            this.get()
        ));
        0
    }
}

fn obj(raw: u32) -> Handle32<ObjectTag> {
    Handle32::new(raw).expect("nonzero object")
}

fn sound() -> AudSound {
    AudSound {
        voice_column: 3,
        link: 2,
        kind: 0,
        fast_flags: 0,
        init_flags: 0,
        sub_id: 1,
        voice_id: 4,
        slot_ids: [NO_VOICE; 8],
        sequence: 0,
        slot_a: 0,
        slot_b: 0,
    }
}

#[test]
fn fast_path_and_disable_answer_zero_without_calls() {
    let mut w = Fake::new();
    let mut s = sound();
    s.slot_ids = [0, 1, 2, 3, 4, 5, 6, 7];
    s.fast_flags = FAST_BIT;
    s.kind = KIND_FAST;
    assert_eq!(s.find_by_hash(&mut w, 0x55), 0);
    assert!(w.log.is_empty(), "fast path makes no calls: {:?}", w.log);

    // The fast bit alone does not take the fast path: the kind word must match.
    s.kind = 1;
    s.fast_flags = FAST_BIT;
    w.lookups.push_back(0x7000);
    assert_eq!(s.find_by_hash(&mut w, 0x55), 0x7000);

    let mut w = Fake::new();
    s.fast_flags = 0;
    s.init_flags = 0x04;
    assert_eq!(s.find_by_hash(&mut w, 0x55), 0);
    assert!(w.log.is_empty(), "disable bit makes no calls: {:?}", w.log);
}

#[test]
fn lookup_skips_empty_and_zero_cursors_and_stops_at_first_hit() {
    let mut w = Fake::new();
    let mut s = sound();
    s.slot_ids = [NO_VOICE, 1, NO_VOICE, 2, 3, NO_VOICE, NO_VOICE, NO_VOICE];
    // Cursor for id 1 is row + scalar; scalar 0x40 and row 0x1000.
    w.lookups.extend([0, 0x7777, 0x8888]);
    assert_eq!(s.find_by_hash(&mut w, 0xAB), 0x7777);
    let finds: Vec<&String> = w.log.iter().filter(|l| l.starts_with("find")).collect();
    assert_eq!(finds.len(), 2, "stops after the first non-zero answer: {:?}", w.log);
    assert!(finds[0].contains("0x1040"), "cursor of id 1: {finds:?}");
    assert!(finds[1].contains("0x1080"), "cursor of id 2: {finds:?}");
}

#[test]
fn zero_row_word_and_zero_cursor_are_skipped_without_wrapping_surprises() {
    let mut w = Fake::new();
    w.scalar_a = 0x10;
    w.row_word = 0u32.wrapping_sub(0x10); // cursor for id 1 is zero
    let mut s = sound();
    s.slot_ids = [1, 2, NO_VOICE, NO_VOICE, NO_VOICE, NO_VOICE, NO_VOICE, NO_VOICE];
    w.lookups.push_back(0x2222);
    assert_eq!(s.find_by_hash(&mut w, 0), 0x2222);
    assert_eq!(w.log.iter().filter(|l| l.starts_with("find")).count(), 1);
}

#[test]
fn find_by_name_hashes_with_zero_seed_then_dispatches() {
    let mut w = Fake::new();
    w.hashes.push_back(0xDEAD_BEEF);
    w.lookups.push_back(0x4242);
    assert_eq!(AudSound::find_by_name(obj(0x5000), 0x6000, &mut w), 0x4242);
    assert_eq!(
        w.log,
        vec!["joaat 0x6000 0", "find 0x5000 0xdeadbeef"],
        "name, zero seed, then slot 0 with the hash"
    );
}

#[test]
fn store_writes_only_when_the_lookup_answers() {
    let mut w = Fake::new();
    assert_eq!(AudSound::store_word_at_key(obj(0x5000), 9, 0x3F80_0000, &mut w), 0);
    assert!(!w.log.iter().any(|l| l.starts_with("store")));

    let mut w = Fake::new();
    w.lookups.push_back(0x6000);
    assert_eq!(AudSound::store_word_at_key(obj(0x5000), 9, 0x3F80_0000, &mut w), 0x6000);
    assert_eq!(w.log.last().map(String::as_str), Some("store 0x6000 0x3f800000"));
}

#[test]
fn hash_and_forward_passes_the_hash_to_slot_two() {
    let mut w = Fake::new();
    w.hashes.push_back(0x1234);
    let _ = AudSound::hash_and_forward(obj(0x5000), 0x77, 0x0bad_f00d, &mut w);
    assert_eq!(
        w.log,
        vec!["joaat 0x77 0", "slot2 0x5000 0x1234 0xbadf00d"],
        "key hashed with a zero seed; slot 2 gets the object, hash and float bits"
    );
}

#[test]
fn forward_link_zero_argument_answers_zero_and_fallback_handles_link_0xff() {
    let mut w = Fake::new();
    let s = sound();
    assert_eq!(s.forward_link(0, &mut w), 0);
    assert!(w.log.is_empty());

    let mut w = Fake::new();
    let mut s = sound();
    s.link = NO_VOICE;
    assert_eq!(s.forward_link(0x99, &mut w), 0x99);
    assert_eq!(w.log, vec!["fallback 0x9100 0x99"]);
}

#[test]
fn forward_link_target_is_scalar_times_link_plus_row_word() {
    let mut w = Fake::new();
    w.scalar_a = 0x100;
    w.row_word = 0x20000;
    let s = sound(); // link 2
    assert_eq!(s.forward_link(0x5, &mut w), 0x5);
    assert_eq!(w.log.last().map(String::as_str), Some("slot4 0x20200 0x5"));
}

#[test]
fn deleting_destructor_unregisters_only_with_the_free_bit() {
    let mut w = Fake::new();
    let s = sound();
    let back = s.deleting_destructor(obj(0x5000), 0, &mut w);
    assert_eq!(back.get(), 0x5000);
    assert_eq!(w.log, vec!["rebuild 0x5000"]);

    let mut w = Fake::new();
    let _ = s.deleting_destructor(obj(0x5000), 3, &mut w);
    assert_eq!(
        w.log,
        vec!["rebuild 0x5000", "unregister 0x9000 0x5000 4"],
        "bit 0 of free set: unregister with the voice id"
    );
}

#[test]
fn construct_second_first_call_runs_init_and_marks_done() {
    let mut w = Fake::new();
    let mut s = sound();
    s.init_flags = NEEDS_INIT; // INIT_DONE clear
    s.slot_a = 0x7100;
    s.slot_b = 0x7200;
    s.construct_second(obj(0x5000), &mut w);
    assert_eq!(s.init_flags, NEEDS_INIT | INIT_DONE);
    assert_eq!(s.sequence, SEQ_UNSET);
    assert_eq!(s.slot_a, 0);
    assert_eq!(s.slot_b, 0);
    assert_eq!(
        w.log,
        vec![
            "vtable 0x5000",
            "init 1 0x5000",
            "zero_byte 0x7100",
            "zero_word 0x7200",
        ]
    );
}

#[test]
fn construct_second_registers_when_init_done_and_needs_init() {
    let mut w = Fake::new();
    let mut s = sound();
    s.init_flags = INIT_DONE | NEEDS_INIT;
    s.construct_second(obj(0x5000), &mut w);
    assert!(
        w.log.contains(&"cursor 0x1030".to_string()),
        "cursor = scalar_b * column + word: {:?}",
        w.log
    );
    assert!(w.log.contains(&"register 0x9000 4 3".to_string()));
    assert!(!w.log.iter().any(|l| l.starts_with("init")));
}

#[test]
fn construct_second_no_voice_column_skips_registration() {
    let mut w = Fake::new();
    let mut s = sound();
    s.voice_column = NO_VOICE;
    s.init_flags = INIT_DONE | NEEDS_INIT;
    s.construct_second(obj(0x5000), &mut w);
    assert!(!w.log.iter().any(|l| l.starts_with("cursor") || l.starts_with("register")));
    assert_eq!(s.sequence, SEQ_UNSET);
}

#[test]
fn registry_says_what_is_proven_and_what_is_missing() {
    let (proven, missing) = registry::counts();
    assert_eq!(proven, 7);
    assert_eq!(missing, registry::ROWS.len() - 7);
    assert!(registry::ROWS
        .iter()
        .filter(|r| r.state == State::Missing)
        .all(|r| !r.narrows.is_empty()));
}
