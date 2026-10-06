//! Host tests for the lifted `peds_task` group: edge cases a reader would ask about.
//!
//! The differential crate proves the lifts equal their verified rewrites on
//! the 32-bit target; these run everywhere and pin the boundary behaviour
//! (empty inputs, exact ties, NaN paths, gate edges, tail arms).

use lf_core::boundary::Handle32;
use lf_peds_tasks::peds_task::{
    Check, CODE_FAR_HI, CODE_FAR_LO, CODE_MID_HI, CODE_MID_LO, CODE_NEAR_HI, CODE_NEAR_LO, EMPTY,
    Enumerate, FacingQuery, MAX_ENTRIES, ObjTag, PickerFill, PickerRand, ReactTuning, TaskFloats,
    TaskStamp, TaskStateBlock, TaskTarget, TaskVec, VTask, WeightedPicker, Worker, registry,
};

// ---------- the weighted picker ----------

fn picker(weights: [f32; MAX_ENTRIES], slots: [u32; MAX_ENTRIES], count: i32) -> WeightedPicker {
    WeightedPicker {
        weights,
        slots,
        count,
    }
}

struct ScriptRand {
    calls: u32,
    answer: u32,
}

impl PickerRand for ScriptRand {
    fn draw(&mut self) -> u32 {
        self.calls += 1;
        self.answer
    }
}

struct ScriptFill {
    calls: Vec<usize>,
    answer: u32,
}

impl PickerFill for ScriptFill {
    fn fill(&mut self, index: usize) -> u32 {
        self.calls.push(index);
        self.answer
    }
}

#[test]
fn pick_zero_count_answers_empty_after_drawing() {
    let mut p = picker([1.0; MAX_ENTRIES], [7; MAX_ENTRIES], 0);
    let mut rand = ScriptRand { calls: 0, answer: 0xDEAD_BEEF };
    let mut fill = ScriptFill { calls: vec![], answer: 0 };
    assert_eq!(p.pick(&mut rand, &mut fill), EMPTY);
    assert_eq!(rand.calls, 1);
    assert!(fill.calls.is_empty());
}

#[test]
fn pick_negative_count_answers_empty() {
    let mut p = picker([1.0; MAX_ENTRIES], [EMPTY; MAX_ENTRIES], -16);
    let mut rand = ScriptRand { calls: 0, answer: 1 };
    let mut fill = ScriptFill { calls: vec![], answer: 1 };
    assert_eq!(p.pick(&mut rand, &mut fill), EMPTY);
    assert!(fill.calls.is_empty());
}

#[test]
fn pick_single_entry_always_zero() {
    let mut p = picker([0.25; MAX_ENTRIES], [EMPTY; MAX_ENTRIES], 1);
    let mut rand = ScriptRand { calls: 0, answer: 0xFFFF_FFFF };
    let mut fill = ScriptFill { calls: vec![], answer: 42 };
    assert_eq!(p.pick(&mut rand, &mut fill), 42);
    assert_eq!(fill.calls, [0]);
    assert_eq!(p.slots[0], 42);
}

#[test]
fn pick_tied_threshold_takes_last() {
    // All-zero weights and a zero draw: every prefix ties the zero
    // threshold, and the strict test passes none of them.
    let mut p = picker([0.0; MAX_ENTRIES], [EMPTY; MAX_ENTRIES], 16);
    let mut rand = ScriptRand { calls: 0, answer: 0 };
    let mut fill = ScriptFill { calls: vec![], answer: 9 };
    assert_eq!(p.pick(&mut rand, &mut fill), 9);
    assert_eq!(fill.calls, [15]);
}

#[test]
fn pick_nan_weights_take_last() {
    let mut p = picker([f32::NAN; MAX_ENTRIES], [3; MAX_ENTRIES], 16);
    let mut rand = ScriptRand { calls: 0, answer: 1234 };
    let mut fill = ScriptFill { calls: vec![], answer: 0 };
    assert_eq!(p.pick(&mut rand, &mut fill), 3);
    assert!(fill.calls.is_empty());
}

#[test]
fn pick_negative_threshold_takes_first_positive_prefix() {
    let mut p = picker([1.0; MAX_ENTRIES], [EMPTY; MAX_ENTRIES], 16);
    let mut rand = ScriptRand { calls: 0, answer: 0x8000_0000 };
    let mut fill = ScriptFill { calls: vec![], answer: 11 };
    assert_eq!(p.pick(&mut rand, &mut fill), 11);
    assert_eq!(fill.calls, [0]);
}

#[test]
fn pick_count_clamps_to_sixteen() {
    let mut p = picker([0.0; MAX_ENTRIES], [EMPTY; MAX_ENTRIES], i32::MAX);
    let mut rand = ScriptRand { calls: 0, answer: 0 };
    let mut fill = ScriptFill { calls: vec![], answer: 8 };
    assert_eq!(p.pick(&mut rand, &mut fill), 8);
    assert_eq!(fill.calls, [15]);
}

#[test]
fn pick_tail_accumulates_past_group_of_four() {
    let mut w = [0.0f32; MAX_ENTRIES];
    w[4] = 2.0;
    let mut p = picker(w, [EMPTY; MAX_ENTRIES], 5);
    let mut rand = ScriptRand { calls: 0, answer: 0 };
    let mut fill = ScriptFill { calls: vec![], answer: 6 };
    assert_eq!(p.pick(&mut rand, &mut fill), 6);
    assert_eq!(fill.calls, [4]);
}

#[test]
fn pick_filled_slot_runs_no_fill() {
    let mut p = picker([1.0; MAX_ENTRIES], [77; MAX_ENTRIES], 16);
    let mut rand = ScriptRand { calls: 0, answer: 0 };
    let mut fill = ScriptFill { calls: vec![], answer: 0 };
    assert_eq!(p.pick(&mut rand, &mut fill), 77);
    assert!(fill.calls.is_empty());
}

// ---------- the reaction code ----------

fn img_tuning() -> ReactTuning {
    ReactTuning {
        threshold: 0xFFFF_FFFF,
        lower: -0.2,
        far: -1.2,
        mid: 0.2,
        near: 1.2,
    }
}

fn query(obj: [f32; 3], dir: [f32; 3], ped: [f32; 3]) -> FacingQuery {
    FacingQuery {
        obj,
        dir,
        ped,
        mode_hi: false,
        flag: false,
        counter: 0,
    }
}

struct ScriptState {
    calls: u32,
    answer: u32,
}

impl lf_peds_tasks::peds_task::PedState for ScriptState {
    fn probe(&mut self) -> u32 {
        self.calls += 1;
        self.answer
    }
}

fn code_of(q: &FacingQuery, ans: u32, t: &ReactTuning) -> (u8, u32) {
    let mut s = ScriptState { calls: 0, answer: ans };
    let c = q.code(&mut s, t);
    (c, s.calls)
}

#[test]
fn react_mid_dot_answers_mid() {
    let q = query([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 0.0, 0.0]);
    assert_eq!(code_of(&q, 0, &img_tuning()), (CODE_MID_LO, 1));
}

#[test]
fn react_near_dot_answers_near() {
    let q = query([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [-0.5, 0.0, 0.0]);
    assert_eq!(code_of(&q, 0, &img_tuning()), (CODE_NEAR_LO, 1));
}

#[test]
fn react_far_dot_answers_far() {
    let q = query([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
    assert_eq!(code_of(&q, 0, &img_tuning()), (CODE_FAR_LO, 1));
}

#[test]
fn react_nan_dot_answers_far() {
    let q = query([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [f32::NAN, 0.0, 0.0]);
    assert_eq!(code_of(&q, 0, &img_tuning()), (CODE_FAR_LO, 1));
}

#[test]
fn react_probe_wins_over_distance() {
    let q = query([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
    assert_eq!(code_of(&q, 0xAB00_0001, &img_tuning()), (CODE_NEAR_LO, 1));
}

#[test]
fn react_probe_low_byte_only() {
    // 0x100 has a clear low byte: classification proceeds to mid.
    let q = query([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 0.0, 0.0]);
    assert_eq!(code_of(&q, 0x100, &img_tuning()), (CODE_MID_LO, 1));
}

#[test]
fn react_counter_gate_needs_flag_and_strict_above() {
    let dot_near = query([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [-0.5, 0.0, 0.0]);
    let t = ReactTuning { threshold: 10, ..img_tuning() };
    // Flag set, counter above: far.
    let q = FacingQuery { flag: true, counter: 11, ..dot_near };
    assert_eq!(code_of(&q, 0, &t).0, CODE_FAR_LO);
    // Counter equal to the threshold: proceeds to near.
    let q = FacingQuery { flag: true, counter: 10, ..dot_near };
    assert_eq!(code_of(&q, 0, &t).0, CODE_NEAR_LO);
    // Flag clear: proceeds even above the threshold.
    let q = FacingQuery { flag: false, counter: 11, ..dot_near };
    assert_eq!(code_of(&q, 0, &t).0, CODE_NEAR_LO);
}

#[test]
fn react_window_edges_are_exclusive() {
    // Dot exactly -0.2 is not inside (-1.2, -0.2).
    let q = query([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.2, 0.0, 0.0]);
    assert_eq!(code_of(&q, 0, &img_tuning()).0, CODE_FAR_LO);
}

#[test]
fn react_f32_edge_converts_above_f64_bound() {
    // 0.2 in single precision converts above the 0.2 double bound: mid.
    let q = query([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.2, 0.0, 0.0]);
    assert_eq!(code_of(&q, 0, &img_tuning()).0, CODE_MID_LO);
}

#[test]
fn react_mode_bit_picks_pair_member() {
    let q = query([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 0.0, 0.0]);
    assert_eq!(code_of(&q, 0, &img_tuning()).0, CODE_MID_LO);
    let q = FacingQuery { mode_hi: true, ..q };
    assert_eq!(code_of(&q, 0, &img_tuning()).0, CODE_MID_HI);
    let q = query([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [-0.5, 0.0, 0.0]);
    let q = FacingQuery { mode_hi: true, ..q };
    assert_eq!(code_of(&q, 0, &img_tuning()).0, CODE_NEAR_HI);
    let q = query([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [9.0, 0.0, 0.0]);
    let q = FacingQuery { mode_hi: true, ..q };
    assert_eq!(code_of(&q, 0, &img_tuning()).0, CODE_FAR_HI);
}

#[test]
fn react_custom_tuning_is_honoured() {
    let t = ReactTuning {
        threshold: 0,
        lower: 10.0,
        far: 5.0,
        mid: 100.0,
        near: 200.0,
    };
    let q = query([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [7.0, 0.0, 0.0]);
    assert_eq!(code_of(&q, 0, &t).0, CODE_NEAR_LO);
}

// ---------- the shared state block ----------

fn blank_state() -> TaskStateBlock {
    TaskStateBlock {
        flag: true,
        out_bias: 1.0,
        gate_bias: 2.0,
        range_hi: 3.0,
        gate: [4.0, 5.0, 6.0],
        work: [7.0, 8.0, 9.0],
        raw: [10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0],
        copy_source: 0x5555_AAAA,
    }
}

struct ScriptEnum {
    mids: Vec<[f32; 3]>,
    reenter: bool,
}

impl Enumerate for ScriptEnum {
    fn enumerate(&mut self, state: &mut TaskStateBlock, mid: [f32; 3]) {
        self.mids.push(mid);
        if self.reenter {
            state.flag = true;
        }
    }
}

fn floats(a: f32, c: f32, f: f32, r4: f32, b: f32, e: f32, d: f32, r7: f32) -> TaskFloats {
    TaskFloats { a, c, f, raw4: r4, b, e, d, raw7: r7 }
}

#[test]
fn build_zero_block_is_all_zero_and_true() {
    let mut st = blank_state();
    let mut stamp = TaskStamp::default();
    let mut en = ScriptEnum { mids: vec![], reenter: false };
    let ok = TaskStateBlock::build(&mut st, &floats(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0), &mut stamp, &mut en);
    assert!(ok);
    assert!(!st.flag);
    assert_eq!(st.gate, [0.0, 0.0, 0.0]);
    assert_eq!(st.work, [0.0, 0.0, 0.0]);
    assert_eq!(st.raw, [0.0; 8]);
    assert_eq!(st.gate_bias.to_bits(), 0x8000_0000); // negated +0.0
    assert_eq!(st.out_bias.to_bits(), 0x8000_0000);
    assert_eq!(st.range_hi, 0.0);
    assert_eq!(en.mids, [[0.0, 0.0, 0.0]]);
    assert_eq!(stamp, TaskStamp { dword: 0x5555_AAAA, id: 2000, flag: 1 });
}

#[test]
fn build_reenter_answers_false() {
    let mut st = blank_state();
    let mut stamp = TaskStamp::default();
    let mut en = ScriptEnum { mids: vec![], reenter: true };
    let ok = TaskStateBlock::build(&mut st, &floats(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0), &mut stamp, &mut en);
    assert!(!ok);
    assert!(st.flag);
}

#[test]
fn build_gate_vector_is_unit_length() {
    let mut st = blank_state();
    let mut stamp = TaskStamp::default();
    let mut en = ScriptEnum { mids: vec![], reenter: false };
    TaskStateBlock::build(&mut st, &floats(1.0, 2.0, 3.0, 40.0, 5.0, 6.0, 7.0, 80.0), &mut stamp, &mut en);
    assert_eq!(en.mids, [[3.0, 4.0, 5.0]]);
    assert_eq!(st.raw, [1.0, 2.0, 3.0, 40.0, 5.0, 6.0, 7.0, 80.0]);
    assert_eq!(st.gate_ref(), 3.0);
    let len2 = st.gate[0] * st.gate[0] + st.gate[1] * st.gate[1] + st.gate[2] * st.gate[2];
    assert!((len2 - 1.0).abs() < 1e-6, "unit gate, got {len2}");
    assert_eq!(st.range_hi, 32f32.sqrt());
    // Cross-like terms against zero: [s, -s, 0] for uniform s.
    assert_eq!(st.work[0], st.gate[1]);
    assert_eq!(st.work[1], -st.gate[0]);
    assert_eq!(st.work[2], 0.0);
}

#[test]
fn build_equal_pairs_zero_the_gate() {
    let mut st = blank_state();
    let mut stamp = TaskStamp::default();
    let mut en = ScriptEnum { mids: vec![], reenter: false };
    TaskStateBlock::build(&mut st, &floats(1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0), &mut stamp, &mut en);
    assert_eq!(st.gate, [0.0, 0.0, 0.0]);
    assert_eq!(st.range_hi, 0.0);
    assert_eq!(en.mids, [[1.0, 2.0, 3.0]]);
}

#[test]
fn build_nan_takes_the_division_path() {
    let mut st = blank_state();
    let mut stamp = TaskStamp::default();
    let mut en = ScriptEnum { mids: vec![], reenter: false };
    TaskStateBlock::build(
        &mut st,
        &floats(f32::NAN, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0),
        &mut stamp,
        &mut en,
    );
    assert!(st.gate[0].is_nan());
    assert!(en.mids[0][0].is_nan());
}

fn target(vec: [f32; 3]) -> TaskTarget {
    TaskTarget {
        obj: Handle32::<ObjTag>::new(0x1234_5678).unwrap(),
        vec: TaskVec { v: vec },
    }
}

struct ScriptVTask {
    calls: u32,
    answer: [f32; 3],
}

impl VTask for ScriptVTask {
    fn run(&mut self, _obj: Handle32<ObjTag>) -> [f32; 3] {
        self.calls += 1;
        self.answer
    }
}

struct ScriptWorker {
    seen: Vec<f32>,
}

impl Worker for ScriptWorker {
    fn run(&mut self, _obj: Handle32<ObjTag>, v2: f32) {
        self.seen.push(v2);
    }
}

struct ScriptCheck {
    calls: u32,
    answer: u32,
}

impl Check for ScriptCheck {
    fn check(&mut self) -> u32 {
        self.calls += 1;
        self.answer
    }
}

fn open_state() -> TaskStateBlock {
    TaskStateBlock {
        flag: false,
        out_bias: 0.0,
        gate_bias: 0.0,
        range_hi: 10.0,
        gate: [0.0, 0.0, 0.0],
        work: [0.0, 0.0, 0.0],
        raw: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        copy_source: 0,
    }
}

#[test]
fn consume_closed_gate_makes_no_calls() {
    let mut st = open_state();
    let mut vt = ScriptVTask { calls: 0, answer: [0.0; 3] };
    let mut w = ScriptWorker { seen: vec![] };
    let mut c = ScriptCheck { calls: 0, answer: 0 };
    let r = st.consume(&target([0.0, 0.0, 9.0]), &mut vt, &mut w, &mut c);
    assert_eq!(r, 1);
    assert_eq!((vt.calls, w.seen.len(), c.calls), (0, 0, 0));
    assert!(!st.flag);
}

#[test]
fn consume_gap_exactly_four_stays_shut() {
    let mut st = open_state();
    let mut vt = ScriptVTask { calls: 0, answer: [0.0; 3] };
    let mut w = ScriptWorker { seen: vec![] };
    let mut c = ScriptCheck { calls: 0, answer: 0 };
    let r = st.consume(&target([0.0, 0.0, 4.0]), &mut vt, &mut w, &mut c);
    assert_eq!(r, 1);
    assert_eq!((vt.calls, w.seen.len(), c.calls), (0, 0, 0));
}

#[test]
fn consume_gate_dot_edges_run() {
    for dot in [0.0, 10.0] {
        let mut st = TaskStateBlock { gate: [1.0, 0.0, 0.0], ..open_state() };
        let mut vt = ScriptVTask { calls: 0, answer: [0.0; 3] };
        let mut w = ScriptWorker { seen: vec![] };
        let mut c = ScriptCheck { calls: 0, answer: 1 };
        st.consume(&target([dot, 0.0, 0.0]), &mut vt, &mut w, &mut c);
        assert_eq!((vt.calls, w.seen.len(), c.calls), (1, 1, 1), "dot {dot}");
    }
}

#[test]
fn consume_nan_gate_keeps_running() {
    let mut st = TaskStateBlock { gate: [f32::NAN, 0.0, 0.0], ..open_state() };
    let mut vt = ScriptVTask { calls: 0, answer: [0.0; 3] };
    let mut w = ScriptWorker { seen: vec![] };
    let mut c = ScriptCheck { calls: 0, answer: 0x100 };
    st.consume(&target([1.0, 0.0, 0.0]), &mut vt, &mut w, &mut c);
    assert_eq!((vt.calls, w.seen.len(), c.calls), (1, 1, 1));
    // Low byte clear, work-dot 0 below one half: arm A raises.
    assert!(st.flag);
}

#[test]
fn consume_tail_arms_raise() {
    // Arm A: clear check, work-dot below one half.
    let mut st = open_state();
    let mut vt = ScriptVTask { calls: 0, answer: [0.0; 3] };
    let mut w = ScriptWorker { seen: vec![] };
    let mut c = ScriptCheck { calls: 0, answer: 0 };
    st.consume(&target([0.0, 0.0, 0.0]), &mut vt, &mut w, &mut c);
    assert!(st.flag);
    // Arm B: positive out-dot, work-dot below minus one half.
    let mut st = TaskStateBlock { out_bias: 1.0, work: [1.0, 0.0, 0.0], ..open_state() };
    let mut vt = ScriptVTask { calls: 0, answer: [-1.0, 0.0, 0.0] };
    let mut w = ScriptWorker { seen: vec![] };
    let mut c = ScriptCheck { calls: 0, answer: 1 };
    st.consume(&target([0.0, 0.0, 0.0]), &mut vt, &mut w, &mut c);
    assert!(st.flag);
    // Arm C: negative out-dot, work-dot above one half.
    let mut st = TaskStateBlock { out_bias: -1.0, work: [1.0, 0.0, 0.0], ..open_state() };
    let mut vt = ScriptVTask { calls: 0, answer: [1.0, 0.0, 0.0] };
    let mut w = ScriptWorker { seen: vec![] };
    let mut c = ScriptCheck { calls: 0, answer: 1 };
    st.consume(&target([0.0, 0.0, 0.0]), &mut vt, &mut w, &mut c);
    assert!(st.flag);
}

#[test]
fn consume_arm_a_edge_is_strict() {
    // Work-dot exactly one half with a clear check: no arm fires.
    let mut st = TaskStateBlock { work: [1.0, 0.0, 0.0], ..open_state() };
    let mut vt = ScriptVTask { calls: 0, answer: [0.5, 0.0, 0.0] };
    let mut w = ScriptWorker { seen: vec![] };
    let mut c = ScriptCheck { calls: 0, answer: 0 };
    st.consume(&target([0.0, 0.0, 0.0]), &mut vt, &mut w, &mut c);
    assert!(!st.flag);
    // Just below one half: arm A fires.
    let mut st = TaskStateBlock { work: [1.0, 0.0, 0.0], ..open_state() };
    let mut vt = ScriptVTask { calls: 0, answer: [0.499_999_97, 0.0, 0.0] };
    let mut w = ScriptWorker { seen: vec![] };
    let mut c = ScriptCheck { calls: 0, answer: 0 };
    st.consume(&target([0.0, 0.0, 0.0]), &mut vt, &mut w, &mut c);
    assert!(st.flag);
}

#[test]
fn consume_zero_out_dot_with_set_check_stays_quiet() {
    let mut st = open_state();
    let mut vt = ScriptVTask { calls: 0, answer: [0.0; 3] };
    let mut w = ScriptWorker { seen: vec![] };
    let mut c = ScriptCheck { calls: 0, answer: 7 };
    let r = st.consume(&target([0.0, 0.0, 0.0]), &mut vt, &mut w, &mut c);
    assert_eq!(r, 1);
    assert_eq!((vt.calls, w.seen.len(), c.calls), (1, 1, 1));
    assert_eq!(w.seen, [0.0]);
    assert!(!st.flag);
}

// ---------- the registry ----------

#[test]
fn registry_counts_are_pinned() {
    assert_eq!(registry::counts(), (4, 0, 0));
    assert_eq!(registry::ROWS.len(), 4);
}
