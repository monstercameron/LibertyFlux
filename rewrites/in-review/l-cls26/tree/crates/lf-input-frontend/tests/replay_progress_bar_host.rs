//! Host tests of the lifted replay progress bar on the 64-bit host.
//!
//! These cover the edge cases a reader of the lifted code would ask about:
//! which collaborator calls happen, in which order, on each branch, and how
//! the unsigned comparison and the long-signal choice behave at the edges.
//! They use a small recorder with fixed answers; the differential cases
//! against the verified 32-bit rewrites live in the `lf-replaybar-diff`
//! test crate.

use lf_core::Handle32;
use lf_input_frontend::replay_progress_bar::{
    BarTag, InnerTag, OWNED_SLOTS, OwnedTag, Part, ReplayBarGlobals, ReplayBarWorld,
    ReplayProgressBar, UpdateOutcome,
};

/// Records each call as text and answers with fixed words.
#[derive(Default)]
struct Recorder {
    events: Vec<String>,
    produced: [u32; 3],
    fill: (u32, u32, u32),
    measured: u32,
    compared: u32,
    base: u32,
    inner_flag: bool,
}

impl ReplayBarWorld for Recorder {
    fn base_teardown(&mut self, _bar: Handle32<BarTag>) {
        self.events.push("base_teardown".into());
    }
    fn open_token(&mut self) -> u32 {
        self.events.push("open_token".into());
        7
    }
    fn attach_token(&mut self, token: u32) {
        self.events.push(format!("attach_token({token})"));
    }
    fn destruct(&mut self, owned: Handle32<OwnedTag>) {
        self.events.push(format!("destruct({})", owned.get()));
    }
    fn free_owned(&mut self, owned: Handle32<OwnedTag>) {
        self.events.push(format!("free_owned({})", owned.get()));
    }
    fn close_token(&mut self, token: u32) {
        self.events.push(format!("close_token({token})"));
    }
    fn finalize_a(&mut self) {
        self.events.push("finalize_a".into());
    }
    fn finalize_b(&mut self) {
        self.events.push("finalize_b".into());
    }
    fn release_storage(&mut self, _bar: Handle32<BarTag>) {
        self.events.push("release_storage".into());
    }
    fn setup_inner(&mut self, _inner: Handle32<InnerTag>, arg: u32) {
        self.events.push(format!("setup_inner({arg})"));
    }
    fn produce(&mut self, _bar: Handle32<BarTag>) -> [u32; 3] {
        self.events.push("produce".into());
        self.produced
    }
    fn fill_pair(&mut self) -> (u32, u32, u32) {
        self.events.push("fill_pair".into());
        self.fill
    }
    fn consume(&mut self, _bar: Handle32<BarTag>, token: u32, _first: &mut u32, _second: &mut u32) {
        self.events.push(format!("consume({token})"));
    }
    fn measure(&mut self) -> u32 {
        self.events.push("measure".into());
        self.measured
    }
    fn use_a(&mut self, _a: u32) {
        self.events.push("use_a".into());
    }
    fn use_b(&mut self, _bar: Handle32<BarTag>, _a: u32, _second: u32) -> u32 {
        self.events.push("use_b".into());
        0
    }
    fn use_c(&mut self, _b: u32) {
        self.events.push("use_c".into());
    }
    fn use_d(&mut self, _c: u32, _b: u32) {
        self.events.push("use_d".into());
    }
    fn use_e(&mut self, _d: u32) {
        self.events.push("use_e".into());
    }
    fn combine(&mut self, part: Part, _first: u32, _second: u32) {
        self.events.push(format!("combine({part:?})"));
    }
    fn step(&mut self, _bar: Handle32<BarTag>) {
        self.events.push("step".into());
    }
    fn compare(&mut self) -> u32 {
        self.events.push("compare".into());
        self.compared
    }
    fn signal(&mut self, code: u32) {
        self.events.push(format!("signal({code})"));
    }
    fn push_s4(&mut self, _bar: Handle32<BarTag>, _word: u32) {
        self.events.push("push_s4".into());
    }
    fn inner_flag_set(&mut self, _inner: Handle32<InnerTag>) -> bool {
        self.inner_flag
    }
    fn gate_a(&mut self, _bar: Handle32<BarTag>) {
        self.events.push("gate_a".into());
    }
    fn gate_b(&mut self, _bar: Handle32<BarTag>) {
        self.events.push("gate_b".into());
    }
    fn gate_c(&mut self, _bar: Handle32<BarTag>) {
        self.events.push("gate_c".into());
    }
    fn finish(&mut self, _bar: Handle32<BarTag>) {
        self.events.push("finish".into());
    }
    fn base_update(&mut self, _bar: Handle32<BarTag>) -> u32 {
        self.events.push("base_update".into());
        self.base
    }
}

fn bar_with(owned: [Option<u32>; OWNED_SLOTS], phase: u32) -> ReplayProgressBar {
    let mut slots = [None; OWNED_SLOTS];
    for (slot, value) in slots.iter_mut().zip(owned) {
        *slot = value.and_then(Handle32::new);
    }
    ReplayProgressBar {
        handle: Handle32::new(0x1000).expect("non-zero"),
        inner: Handle32::new(0x2000).expect("non-zero"),
        owned: slots,
        mark: 0,
        total: 0,
        phase,
        torn: false,
    }
}

fn globals(mode: u32, flag: u8, gate: u8) -> ReplayBarGlobals {
    ReplayBarGlobals {
        mode,
        flag,
        gate,
        or_word: 0x10,
        save_word: 0,
    }
}

#[test]
fn teardown_with_empty_slots_opens_attaches_closes_and_finalizes() {
    let mut bar = bar_with([None; OWNED_SLOTS], 0);
    let mut world = Recorder::default();
    bar.teardown(&mut world);
    assert_eq!(
        world.events,
        [
            "base_teardown",
            "open_token",
            "attach_token(7)",
            "close_token(7)",
            "finalize_a",
            "finalize_b",
        ]
    );
    assert!(bar.torn);
}

#[test]
fn teardown_releases_second_destructor_only_when_slot_still_holds_an_object() {
    // Slot 3 holds an object: one destructor call, then the slot is read
    // again. The recorder does not clear it, so the second destructor and
    // the free run too.
    let mut bar = bar_with([None, None, None, Some(0x30), None], 0);
    let mut world = Recorder::default();
    bar.teardown(&mut world);
    let calls: Vec<&str> = world.events.iter().map(String::as_str).collect();
    assert_eq!(
        &calls[3..7],
        [
            "destruct(48)",
            "destruct(48)",
            "free_owned(48)",
            "close_token(7)"
        ]
    );
    assert_eq!(bar.owned[3], None);
}

#[test]
fn teardown_frees_the_first_three_slots_in_order_and_empties_them() {
    let mut bar = bar_with([Some(0x10), None, Some(0x20), Some(0x30), None], 0);
    let mut world = Recorder::default();
    bar.teardown(&mut world);
    let calls: Vec<&str> = world.events.iter().map(String::as_str).collect();
    assert_eq!(
        &calls[3..9],
        [
            "destruct(16)",
            "free_owned(16)",
            "destruct(32)",
            "free_owned(32)",
            "destruct(48)",
            "destruct(48)"
        ]
    );
    assert!(bar.owned.iter().all(Option::is_none));
}

#[test]
fn deleting_destructor_frees_storage_only_on_bit_zero() {
    let mut bar = bar_with([None; OWNED_SLOTS], 0);
    let mut world = Recorder::default();
    let returned = bar.deleting_destructor(&mut world, 2);
    assert_eq!(returned.get(), 0x1000);
    assert!(!world.events.iter().any(|e| e == "release_storage"));

    let mut bar = bar_with([None; OWNED_SLOTS], 0);
    let mut world = Recorder::default();
    bar.deleting_destructor(&mut world, 3);
    assert_eq!(
        world.events.last().map(String::as_str),
        Some("release_storage")
    );
}

#[test]
fn update_skips_the_second_phase_for_phase_words_one_and_two() {
    for phase in [1, 2] {
        let mut bar = bar_with([None; OWNED_SLOTS], phase);
        let mut world = Recorder::default();
        bar.update(&mut world, &mut globals(0, 0, 0));
        assert_eq!(world.events.iter().filter(|e| *e == "produce").count(), 1);
        assert!(!world.events.iter().any(|e| e == "use_a"));
    }
    let mut bar = bar_with([None; OWNED_SLOTS], 3);
    let mut world = Recorder::default();
    bar.update(&mut world, &mut globals(0, 0, 0));
    assert_eq!(world.events.iter().filter(|e| *e == "produce").count(), 2);
    assert!(world.events.iter().any(|e| e == "use_e"));
}

#[test]
fn update_signals_only_when_compare_is_below_the_produced_word_unsigned() {
    // The produced word is 1. A compare word with the top bit set is a large
    // unsigned value, so it is not below 1 and no signal runs.
    let mut bar = bar_with([None; OWNED_SLOTS], 1);
    let mut world = Recorder {
        produced: [0, 1, 0],
        compared: 0x8000_0000,
        ..Recorder::default()
    };
    let mut g = globals(0, 0, 0);
    bar.update(&mut world, &mut g);
    assert!(!world.events.iter().any(|e| e.starts_with("signal")));
    assert_eq!(g.save_word, 0);

    // Zero is below 1: the signal runs, the or-word gets bit 1 and the saved
    // word takes the produced word.
    let mut bar = bar_with([None; OWNED_SLOTS], 1);
    let mut world = Recorder {
        produced: [0, 1, 0],
        compared: 0,
        ..Recorder::default()
    };
    let mut g = globals(0, 0, 0);
    bar.update(&mut world, &mut g);
    assert!(world.events.iter().any(|e| e == "signal(6)"));
    assert_eq!(g.or_word, 0x12);
    assert_eq!(g.save_word, 1);
}

#[test]
fn update_long_signal_needs_mode_seven_or_two_and_a_set_flag() {
    let cases = [(7, 1, 7), (7, 0, 6), (2, 1, 7), (6, 1, 6), (0, 1, 6)];
    for (mode, flag, code) in cases {
        let mut bar = bar_with([None; OWNED_SLOTS], 1);
        let mut world = Recorder {
            produced: [0, 5, 0],
            compared: 0,
            ..Recorder::default()
        };
        bar.update(&mut world, &mut globals(mode, flag, 0));
        assert!(
            world.events.iter().any(|e| *e == format!("signal({code})")),
            "mode {mode} flag {flag}"
        );
    }
}

#[test]
fn update_returns_the_inner_object_when_its_flag_is_set() {
    let mut bar = bar_with([None; OWNED_SLOTS], 1);
    let mut world = Recorder {
        inner_flag: true,
        base: 0x55,
        ..Recorder::default()
    };
    let outcome = bar.update(&mut world, &mut globals(0, 0, 1));
    assert_eq!(outcome, UpdateOutcome::Inner(bar.inner));
    assert!(!world.events.iter().any(|e| e == "finish"));
    assert!(world.events.iter().any(|e| e == "gate_c"));
    assert!(!world.events.iter().any(|e| e == "gate_a"));
}

#[test]
fn update_runs_finish_and_returns_the_base_word_when_the_inner_flag_is_clear() {
    let mut bar = bar_with([None; OWNED_SLOTS], 1);
    let mut world = Recorder {
        inner_flag: false,
        base: 0x55,
        ..Recorder::default()
    };
    let outcome = bar.update(&mut world, &mut globals(0, 0, 1));
    assert_eq!(outcome, UpdateOutcome::Base(0x55));
    assert!(world.events.iter().any(|e| e == "finish"));
    assert!(world.events.iter().any(|e| e == "gate_a"));
}

#[test]
fn update_without_gate_runs_no_gate_step_and_sets_mark_and_total() {
    let mut bar = bar_with([None; OWNED_SLOTS], 1);
    let mut world = Recorder {
        measured: 10,
        ..Recorder::default()
    };
    bar.update(&mut world, &mut globals(0, 0, 0));
    assert!(!world.events.iter().any(|e| e.starts_with("gate")));
    assert_eq!(bar.mark, u32::MAX);
    // The total is the measurement plus the second word of fill_pair, which
    // the default recorder leaves at zero.
    assert_eq!(bar.total, 10);
}

#[test]
fn total_wraps_on_overflow() {
    let mut bar = bar_with([None; OWNED_SLOTS], 1);
    let mut world = Recorder {
        measured: u32::MAX,
        fill: (0, 0, 2),
        ..Recorder::default()
    };
    bar.update(&mut world, &mut globals(0, 0, 0));
    assert_eq!(bar.total, 1);
}

#[test]
fn combine_receives_the_sub_object_part() {
    let mut bar = bar_with([None; OWNED_SLOTS], 1);
    let mut world = Recorder::default();
    bar.update(&mut world, &mut globals(0, 0, 0));
    assert!(world.events.iter().any(|e| e == "combine(Sub1c)"));
}
