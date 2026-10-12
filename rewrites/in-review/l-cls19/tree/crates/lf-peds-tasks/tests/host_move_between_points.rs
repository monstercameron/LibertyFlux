//! Host tests of the move-between-points scenario methods: the edge cases a reader
//! would ask about, on the 64-bit host. The differential proof against the verified
//! 32-bit rewrites lives in the `lf-peds-diff` test crate.
//!
//! Every method here runs in straight-line code: none loops, so no test needs a bound
//! beyond the fixed inputs it builds.

use lf_peds_tasks::move_between_points::{
    ACTIVE_WEIGHT, IDLE_WEIGHT, MoveBetweenPointsHost, MoveBetweenPointsScenario, STATUS_DONE,
    STATUS_STICKY, SubTask,
};

/// A host that records each call as a short string and answers with fixed words.
#[derive(Default)]
struct Recorder {
    calls: Vec<String>,
    alloc_ret: u32,
    ctor_ret: u32,
}

const HEAP: u32 = 0xA0;

impl MoveBetweenPointsHost for Recorder {
    fn heap_singleton(&mut self) -> u32 {
        HEAP
    }

    fn class_destructor(&mut self, _task: &mut MoveBetweenPointsScenario) {
        self.calls.push("dtor".into());
    }

    fn heap_free(&mut self, heap: u32, _task: &mut MoveBetweenPointsScenario) {
        self.calls.push(format!("free {heap:#x}"));
    }

    fn heap_alloc(&mut self, heap: u32) -> u32 {
        self.calls.push(format!("alloc {heap:#x}"));
        self.alloc_ret
    }

    fn construct(&mut self, slot: u32, aux_id: u32, aux_word: u32, pending_guard: u32) -> u32 {
        self.calls
            .push(format!("construct {slot:#x} {aux_id:#x} {aux_word:#x} {pending_guard:#x}"));
        self.ctor_ret
    }

    fn release_payload(&mut self, payload: u32) {
        self.calls.push(format!("release {payload:#x}"));
    }
}

/// A sub-task whose latch and run answer the test sets.
struct Sub {
    latch: u8,
    run_ret: u32,
    runs: Vec<(u32, u32, u32)>,
}

impl SubTask for Sub {
    fn latch(&self) -> u8 {
        self.latch
    }

    fn set_latch(&mut self, value: u8) {
        self.latch = value;
    }

    fn run(&mut self, a1: u32, a2: u32, a3: u32) -> u32 {
        self.runs.push((a1, a2, a3));
        self.run_ret
    }
}

fn sub(latch: u8, run_ret: u32) -> Sub {
    Sub {
        latch,
        run_ret,
        runs: Vec::new(),
    }
}

fn scenario(status: u32, active: u8, pending: u32, state: u32) -> MoveBetweenPointsScenario {
    MoveBetweenPointsScenario {
        status,
        active,
        pending_guard: pending,
        state,
        ..MoveBetweenPointsScenario::default()
    }
}

// ---- blend_weight ----

#[test]
fn done_bit_forces_the_active_weight_even_when_idle_otherwise() {
    // Activity set and guard zero would be idle, but the done bit wins.
    let s = scenario(STATUS_DONE, 1, 0, 2);
    assert_eq!(s.blend_weight().to_bits(), ACTIVE_WEIGHT.to_bits());
}

#[test]
fn sticky_bit_alone_does_not_force_the_weight() {
    // Bit 0 is the sticky stop, not the done bit: the state decides.
    let s = scenario(STATUS_STICKY, 0, 0, 2);
    assert_eq!(s.blend_weight().to_bits(), IDLE_WEIGHT.to_bits());
}

#[test]
fn active_with_zero_guard_is_idle_in_every_state() {
    for state in [0, 1, 2, u32::MAX] {
        assert_eq!(scenario(0, 1, 0, state).blend_weight().to_bits(), IDLE_WEIGHT.to_bits());
    }
}

#[test]
fn active_with_nonzero_guard_falls_through_to_the_state() {
    assert_eq!(scenario(0, 1, 9, 0).blend_weight().to_bits(), ACTIVE_WEIGHT.to_bits());
    assert_eq!(scenario(0, 1, 9, 1).blend_weight().to_bits(), ACTIVE_WEIGHT.to_bits());
    assert_eq!(scenario(0, 1, 9, 2).blend_weight().to_bits(), IDLE_WEIGHT.to_bits());
}

#[test]
fn the_weights_are_exact_bit_patterns() {
    assert_eq!(ACTIVE_WEIGHT.to_bits(), 0x41C8_0000);
    assert_eq!(IDLE_WEIGHT.to_bits(), 0xBF80_0000);
}

// ---- deleting_destructor ----

#[test]
fn deleting_destructor_without_bit_zero_only_runs_the_base_destructor() {
    let mut s = MoveBetweenPointsScenario::default();
    let mut h = Recorder::default();
    s.deleting_destructor(&mut h, 0xFFFF_FFFE);
    assert_eq!(h.calls, vec!["dtor".to_string()]);
}

#[test]
fn deleting_destructor_with_bit_zero_frees_after_the_base_destructor() {
    let mut s = MoveBetweenPointsScenario::default();
    let mut h = Recorder::default();
    s.deleting_destructor(&mut h, 0x8000_0001);
    assert_eq!(h.calls, vec!["dtor".to_string(), format!("free {HEAP:#x}")]);
}

// ---- clone_scenario ----

#[test]
fn clone_returns_zero_and_does_not_construct_when_allocation_fails() {
    let s = scenario(0, 0, 3, 0);
    let mut h = Recorder { alloc_ret: 0, ctor_ret: 0x77, ..Recorder::default() };
    assert_eq!(s.clone_scenario(&mut h), 0);
    assert_eq!(h.calls, vec![format!("alloc {HEAP:#x}")]);
}

#[test]
fn clone_passes_the_three_words_through_unchanged() {
    let s = MoveBetweenPointsScenario {
        aux_id: 0x8000_0000,
        aux_word: u32::MAX,
        pending_guard: 0x0BAD_F00D,
        ..MoveBetweenPointsScenario::default()
    };
    let mut h = Recorder { alloc_ret: 0x55, ctor_ret: 0x66, ..Recorder::default() };
    assert_eq!(s.clone_scenario(&mut h), 0x66);
    assert_eq!(
        h.calls.last().expect("construct call"),
        "construct 0x55 0x80000000 0xffffffff 0xbadf00d"
    );
}

// ---- run_tick ----

#[test]
fn latched_sub_task_is_not_run_but_the_state_is_still_reset() {
    let mut s = scenario(0, 0, 0, 1);
    s.payload = 0x42;
    let mut h = Recorder::default();
    let mut sb = sub(0b01, 0x1);
    assert!(s.run_tick(&mut sb, &mut h, 1, 2, 3));
    assert!(sb.runs.is_empty(), "a latched sub-task is not run");
    assert_eq!(sb.latch, 0b01, "the latch is left as it was");
    assert_eq!(s.state, 0);
    assert_eq!(s.payload, 0);
    assert_eq!(h.calls, vec!["release 0x42".to_string()]);
}

#[test]
fn a_run_whose_low_byte_is_zero_fails_the_tick_and_leaves_the_latch() {
    let mut s = scenario(0, 0, 0, 1);
    s.payload = 0x42;
    let mut h = Recorder::default();
    let mut sb = sub(0, 0x100);
    assert!(!s.run_tick(&mut sb, &mut h, 1, 2, 3));
    assert_eq!(sb.latch, 0, "a failed run does not set the latch");
    assert_eq!(s.state, 1, "a failed run does not reset the state");
    assert_eq!(s.payload, 0x42, "a failed run does not release the payload");
    assert!(h.calls.is_empty());
}

#[test]
fn a_successful_run_sets_bit_one_and_keeps_bit_zero_clear() {
    let mut s = scenario(0, 0, 0, 0);
    let mut h = Recorder::default();
    let mut sb = sub(0x80, 0x1);
    assert!(s.run_tick(&mut sb, &mut h, 7, 8, 9));
    assert_eq!(sb.runs, vec![(7, 8, 9)]);
    assert_eq!(sb.latch, 0x82, "bit one is set and the other bits are kept");
    assert!(h.calls.is_empty(), "state zero releases nothing");
}

#[test]
fn state_one_without_a_payload_resets_the_state_and_releases_nothing() {
    let mut s = scenario(0, 0, 0, 1);
    let mut h = Recorder::default();
    let mut sb = sub(0, 0x1);
    assert!(s.run_tick(&mut sb, &mut h, 0, 0, 0));
    assert_eq!(s.state, 0);
    assert!(h.calls.is_empty());
}

#[test]
fn state_two_keeps_the_payload_and_releases_nothing() {
    let mut s = scenario(0, 0, 0, 2);
    s.payload = 0x42;
    let mut h = Recorder::default();
    let mut sb = sub(0, 0x1);
    assert!(s.run_tick(&mut sb, &mut h, 0, 0, 0));
    assert_eq!(s.state, 2);
    assert_eq!(s.payload, 0x42);
    assert!(h.calls.is_empty());
}
