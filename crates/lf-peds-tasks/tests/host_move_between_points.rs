//! Host tests of the move-between-points scenario methods: the edge cases a reader
//! would ask about, on the 64-bit host. The differential proof against the verified
//! 32-bit rewrites lives in the `lf-peds-diff` test crate.
//!
//! Every method here runs in straight-line code: none loops, so no test needs a bound
//! beyond the fixed inputs it builds.

use lf_peds_tasks::move_between_points::{
    ACTIVE_WEIGHT, AUX_CHAT, IDLE_WEIGHT, MoveBetweenPointsHost, MoveBetweenPointsScenario,
    RouteFirst, RouteSecond, STATUS_DONE, STATUS_STICKY, SubTask, TickHost, WorldQuery,
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
        self.calls.push(format!(
            "construct {slot:#x} {aux_id:#x} {aux_word:#x} {pending_guard:#x}"
        ));
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
        assert_eq!(
            scenario(0, 1, 0, state).blend_weight().to_bits(),
            IDLE_WEIGHT.to_bits()
        );
    }
}

#[test]
fn active_with_nonzero_guard_falls_through_to_the_state() {
    assert_eq!(
        scenario(0, 1, 9, 0).blend_weight().to_bits(),
        ACTIVE_WEIGHT.to_bits()
    );
    assert_eq!(
        scenario(0, 1, 9, 1).blend_weight().to_bits(),
        ACTIVE_WEIGHT.to_bits()
    );
    assert_eq!(
        scenario(0, 1, 9, 2).blend_weight().to_bits(),
        IDLE_WEIGHT.to_bits()
    );
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
    let mut h = Recorder {
        alloc_ret: 0,
        ctor_ret: 0x77,
        ..Recorder::default()
    };
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
    let mut h = Recorder {
        alloc_ret: 0x55,
        ctor_ret: 0x66,
        ..Recorder::default()
    };
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

// ---- the main tick ----

/// A probe host for the tick: records the calls it is asked, answers the knobs, and zero otherwise.
#[derive(Default)]
struct Probe {
    calls: Vec<String>,
    sub_answer: u32,
    leg_ok: u32,
    chat_weight: Option<u32>,
}

impl TickHost for Probe {
    fn set_scenario_flag(&mut self, _ped: u32) {
        self.calls.push("flag".into());
    }

    fn sub_operation(
        &mut self,
        _task: &mut MoveBetweenPointsScenario,
        _ped: u32,
        kind: u32,
    ) -> u32 {
        self.calls.push(format!("sub {kind}"));
        self.sub_answer
    }

    fn config(&mut self, _aux_id: u32) -> u32 {
        self.calls.push("config".into());
        0
    }

    fn config_span(&mut self, _config: u32) -> f32 {
        0.0
    }

    fn random(&mut self) -> u32 {
        0
    }

    fn frame_step(&mut self) -> f32 {
        0.0
    }

    fn weight_bits(&mut self, _narrow: bool) -> u32 {
        0
    }

    fn placement_base(&mut self, _ped: u32) -> u32 {
        0
    }

    fn position(&mut self, _base: u32) -> [f32; 3] {
        [0.0; 3]
    }

    fn route_first(&mut self, _base: u32, _weight: u32, _flags: [u32; 3], _ped: u32) -> RouteFirst {
        RouteFirst {
            ok: 0,
            anchor: 0,
            spare: 0,
        }
    }

    fn refine_first(&mut self) -> [u32; 4] {
        [0; 4]
    }

    fn ped_group(&mut self, _ped: u32) -> u32 {
        0
    }

    fn group_seat(&mut self, _group: u32) -> u32 {
        0
    }

    fn intel_anchor(&mut self, _ped: u32) -> u32 {
        0
    }

    fn commit_route(&mut self, _base: u32, _value: u32) -> u32 {
        0
    }

    fn set_anchor_leg(&mut self, _anchor: u32, _leg: u32) {}

    fn set_anchor_state(&mut self, _anchor: u32, _state: u32) {}

    fn leg_ok(&mut self, _leg: u32) -> u32 {
        self.calls.push("leg_ok".into());
        self.leg_ok
    }

    fn schedule(&mut self, _leg: u32) -> u32 {
        self.calls.push("schedule".into());
        0
    }

    fn schedule_use(&mut self, _leg: u32) {
        self.calls.push("schedule_use".into());
    }

    fn route_second(&mut self, _ped: u32, _weight: u32, _flags: [u32; 3]) -> RouteSecond {
        RouteSecond {
            ok: 0,
            anchor: 0,
            aux: 0,
        }
    }

    fn refine_leg(&mut self, _anchor: u32, _aux: u32) {}

    fn commit_a(&mut self, _task: &mut MoveBetweenPointsScenario, _aux: u32) {}

    fn commit_b(&mut self, _task: &mut MoveBetweenPointsScenario, _anchor: u32) {}

    fn world(&mut self) -> u32 {
        0
    }

    fn world_query(&mut self, _query: WorldQuery, _world: u32) -> u32 {
        0
    }

    fn anchor_of(&mut self, _world_a: u32) -> u32 {
        0
    }

    fn dock(&mut self, _world: u32, _args: [u32; 6]) -> u32 {
        0
    }

    fn move_fare(&mut self) -> (u32, u32) {
        (0, 0)
    }

    fn board(&mut self, _world: u32, _fare_lo: u32, _fare_hi: u32, _flags: [u32; 7]) -> u32 {
        0
    }

    fn seat_info(&mut self, _object: u32, _a: u32, _b: u32, _tail: [u32; 2]) -> u32 {
        0
    }

    fn attach(&mut self, _anchor: u32, _boarded: u32) {}

    fn leg_service(&mut self, _anchor: u32) -> (u32, [u32; 4]) {
        (0, [0; 4])
    }

    fn leg_length(&mut self, _len: u32) {}

    fn sign_mask(&mut self) -> u32 {
        0
    }

    fn heading(&mut self, _leg_vector: [f32; 3]) -> f64 {
        0.0
    }

    fn approach_fare(&mut self) -> (u32, u32) {
        (0, 0)
    }

    fn approach(&mut self, _world: u32, _approach_bits: u32, _fare_lo: u32, _fare_hi: u32) -> u32 {
        0
    }

    fn anchor_aux(&mut self, _anchor: u32) -> u32 {
        0
    }

    fn anchor_sub(&mut self, _anchor: u32) -> u32 {
        0
    }

    fn chat_ping(&mut self, _ped: u32, weight_bits: u32) {
        self.chat_weight = Some(weight_bits);
        self.calls.push("chat".into());
    }
}

#[test]
fn a_sticky_status_ends_the_tick_before_any_sub_operation() {
    let mut s = scenario(STATUS_STICKY, 1, 0, 0);
    let mut h = Probe::default();
    assert_eq!(s.tick(&mut h, 7), 0);
    assert_eq!(h.calls, vec!["flag".to_string()]);
}

#[test]
fn an_accepted_first_sub_operation_sets_done_and_skips_the_second() {
    let mut s = scenario(0, 1, 0, 0);
    let mut h = Probe {
        sub_answer: 1,
        ..Probe::default()
    };
    assert_eq!(s.tick(&mut h, 7), 0);
    assert_eq!(s.status & STATUS_DONE, STATUS_DONE);
    assert_eq!(h.calls, vec!["flag".to_string(), "sub 1".to_string()]);
}

#[test]
fn an_idle_scenario_skips_the_polls_and_returns_its_sub_task() {
    let mut s = MoveBetweenPointsScenario {
        sub_task: 0x55,
        aux_id: 0x05,
        ..scenario(0, 0, 0, 9)
    };
    let mut h = Probe::default();
    assert_eq!(s.tick(&mut h, 7), 0x55);
    assert_eq!(h.calls, vec!["flag".to_string(), "config".to_string()]);
    assert!(h.chat_weight.is_none(), "only the chat variant pings");
}

#[test]
fn the_chat_variant_pings_with_the_fixed_weight_before_the_sub_task_is_returned() {
    let mut s = MoveBetweenPointsScenario {
        sub_task: 0x55,
        aux_id: AUX_CHAT,
        ..scenario(0, 0, 0, 9)
    };
    let mut h = Probe::default();
    assert_eq!(s.tick(&mut h, 7), 0x55);
    assert_eq!(
        h.chat_weight,
        Some(0x3D4C_CCCD),
        "the chat ping weight is 0.05"
    );
}

#[test]
fn state_one_with_no_valid_leg_returns_the_sub_task_and_keeps_the_leg() {
    let mut s = MoveBetweenPointsScenario {
        sub_task: 0x66,
        payload: 0x77,
        ..scenario(0, 0, 0, 1)
    };
    let mut h = Probe {
        leg_ok: 0,
        ..Probe::default()
    };
    assert_eq!(s.tick(&mut h, 7), 0x66);
    assert_eq!(s.payload, 0x77, "an invalid leg is not cleared");
    assert_eq!(s.state, 1);
    assert!(!h.calls.iter().any(|c| c == "schedule"));
}

#[test]
fn an_unknown_state_falls_through_to_the_tail_without_a_wait() {
    let mut s = MoveBetweenPointsScenario {
        sub_task: 0x88,
        ..scenario(0, 0, 0, 5)
    };
    let mut h = Probe::default();
    assert_eq!(s.tick(&mut h, 7), 0x88);
    assert_eq!(s.state, 5);
    assert_eq!(h.calls, vec!["flag".to_string(), "config".to_string()]);
}
