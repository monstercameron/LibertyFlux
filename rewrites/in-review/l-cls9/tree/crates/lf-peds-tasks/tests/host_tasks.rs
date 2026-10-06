//! Host edge tests for the lifted pedestrian tasks.
//!
//! One case per question a reader of the code would ask: the event
//! splits, the timeout and sample arms of the duck update, the drain
//! clamp, and the pool call shapes. Bit-exactness against the verified
//! rewrites is proven by the 32-bit differential crate, not here.

use lf_core::Handle32;
use lf_peds_tasks::tasks::registry::{self, State};
use lf_peds_tasks::tasks::{
    DAMP_RATE_BITS, DuckEvent, DuckPed, DuckPedSide, DuckPool, DuckQuery, DuckSink, DuckSpec,
    DuckTask, DuckTaskSide, FINISH_FLAG, FistHeld, FistLink, FistPool, FistTarget, HitBase,
    HitHandler, HitPool, HitResponse, HitStart, QUERY_CODE, QUERY_STATE, SUSTAIN_FLAG, ShakeFist,
    TaskMgr, UninitTask,
};

fn manager(v: u32) -> Option<Handle32<TaskMgr>> {
    Handle32::new(v)
}

fn held(v: u32) -> Option<Handle32<FistHeld>> {
    Handle32::new(v)
}

fn link(v: u32) -> Option<Handle32<FistLink>> {
    Handle32::new(v)
}

#[test]
fn registry_counts_eight_proven() {
    let (proven, lifted, missing) = registry::counts();
    assert_eq!(proven, 8);
    assert_eq!(lifted, 0);
    assert_eq!(missing, 22);
    assert_eq!(registry::ROWS.len(), 30);
    for row in registry::ROWS {
        if row.state == State::Proven {
            assert!(
                [
                    ("HitResponse", "ctor"),
                    ("HitResponse", "vf1"),
                    ("HitResponse", "vf19"),
                    ("Duck", "vf1"),
                    ("Duck", "vf5"),
                    ("Duck", "vf17"),
                    ("ShakeFist", "vf1"),
                    ("ShakeFist", "vf5"),
                ]
                .contains(&(row.class, row.method)),
                "unexpected proven row {} {}",
                row.class,
                row.method
            );
        } else {
            assert!(
                !row.narrows.is_empty(),
                "missing row {} {} needs a reason",
                row.class,
                row.method
            );
        }
    }
}

// The fist shake.

struct FistPoolScript {
    alloc_answer: u32,
    construct_answer: u32,
    allocs: Vec<u32>,
    constructs: Vec<(u32, u32)>,
}

impl FistPool for FistPoolScript {
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
        self.allocs.push(Handle32::raw_or_zero(manager));
        Handle32::new(self.alloc_answer)
    }

    fn construct(
        &mut self,
        block: Handle32<UninitTask>,
        link: Option<Handle32<FistLink>>,
    ) -> Option<Handle32<ShakeFist>> {
        self.constructs
            .push((block.get(), Handle32::raw_or_zero(link)));
        Handle32::new(self.construct_answer)
    }
}

#[test]
fn fist_clone_builds_from_the_link() {
    let task = ShakeFist::new(held(7), link(0xABCD));
    let mut pool = FistPoolScript {
        alloc_answer: 0x1000,
        construct_answer: 0x2000,
        allocs: Vec::new(),
        constructs: Vec::new(),
    };
    let got = task.clone_task(manager(0x3333), &mut pool);
    assert_eq!(got, Handle32::new(0x2000));
    assert_eq!(pool.allocs, vec![0x3333]);
    assert_eq!(pool.constructs, vec![(0x1000, 0xABCD)]);
    assert_eq!(task.held(), held(7));
    assert_eq!(task.link(), link(0xABCD));
}

#[test]
fn fist_clone_answers_null_without_constructing() {
    let task = ShakeFist::new(held(7), link(0xABCD));
    let mut pool = FistPoolScript {
        alloc_answer: 0,
        construct_answer: 0x2000,
        allocs: Vec::new(),
        constructs: Vec::new(),
    };
    assert_eq!(task.clone_task(manager(0x3333), &mut pool), None);
    assert_eq!(pool.allocs, vec![0x3333]);
    assert!(pool.constructs.is_empty());
}

#[test]
fn fist_clone_carries_a_null_link() {
    let task = ShakeFist::new(None, None);
    let mut pool = FistPoolScript {
        alloc_answer: 0x1000,
        construct_answer: 0x2000,
        allocs: Vec::new(),
        constructs: Vec::new(),
    };
    assert_eq!(
        task.clone_task(None, &mut pool),
        Handle32::new(0x2000)
    );
    assert_eq!(pool.allocs, vec![0]);
    assert_eq!(pool.constructs, vec![(0x1000, 0)]);
}

struct FistTargetScript {
    damp_clears: bool,
    damps: Vec<(u32, u32)>,
    releases: Vec<u32>,
}

impl FistTarget for FistTargetScript {
    fn damp(&mut self, slot: &mut Option<Handle32<FistHeld>>, rate: f32) {
        self.damps
            .push((Handle32::raw_or_zero(*slot), rate.to_bits()));
        if self.damp_clears {
            *slot = None;
        }
    }

    fn release(&mut self, held: Handle32<FistHeld>) {
        self.releases.push(held.get());
    }
}

fn fist_script(damp_clears: bool) -> FistTargetScript {
    FistTargetScript {
        damp_clears,
        damps: Vec::new(),
        releases: Vec::new(),
    }
}

#[test]
fn damp_rate_is_negative_four() {
    assert_eq!(DAMP_RATE_BITS, 0xC080_0000);
    assert_eq!(f32::from_bits(DAMP_RATE_BITS), -4.0);
}

#[test]
fn fist_taken_events_damp_then_release() {
    for event in [1, 2] {
        let mut task = ShakeFist::new(held(0x77), link(3));
        let mut target = fist_script(false);
        assert_eq!(task.handle_event(event, &mut target), 1);
        assert_eq!(task.held(), None);
        assert_eq!(target.damps, vec![(0x77, DAMP_RATE_BITS)]);
        assert_eq!(target.releases, vec![0x77]);
    }
}

#[test]
fn fist_other_events_only_damp() {
    for event in [0, 3, 4, 0x100, u32::MAX] {
        let mut task = ShakeFist::new(held(0x77), link(3));
        let mut target = fist_script(false);
        assert_eq!(task.handle_event(event, &mut target), 0, "event {event}");
        assert_eq!(task.held(), held(0x77));
        assert_eq!(target.damps, vec![(0x77, DAMP_RATE_BITS)]);
        assert!(target.releases.is_empty());
    }
}

#[test]
fn fist_empty_slot_makes_no_calls() {
    for event in [0, 1, 2, u32::MAX] {
        let mut task = ShakeFist::new(None, link(3));
        let mut target = fist_script(false);
        let want = u32::from(event == 1 || event == 2);
        assert_eq!(task.handle_event(event, &mut target), want);
        assert!(target.damps.is_empty());
        assert!(target.releases.is_empty());
    }
}

#[test]
fn fist_damp_clearing_the_slot_skips_the_release() {
    let mut task = ShakeFist::new(held(0x77), link(3));
    let mut target = fist_script(true);
    assert_eq!(task.handle_event(1, &mut target), 1);
    assert_eq!(task.held(), None);
    assert_eq!(target.damps, vec![(0x77, DAMP_RATE_BITS)]);
    assert!(target.releases.is_empty());
}

// The hit response.

struct HitBaseRec {
    calls: u32,
}

impl HitBase for HitBaseRec {
    fn construct_base(&mut self) {
        self.calls += 1;
    }
}

#[test]
fn hit_ctor_constructs_then_stores() {
    let mut base = HitBaseRec { calls: 0 };
    let task = HitResponse::new(2, &mut base);
    assert_eq!(base.calls, 1);
    assert_eq!(task.kind(), 2);
    let mut base = HitBaseRec { calls: 0 };
    assert_eq!(HitResponse::new(u32::MAX, &mut base).kind(), u32::MAX);
    assert_eq!(base.calls, 1);
}

struct HitPoolScript {
    alloc_answer: u32,
    construct_answer: u32,
    allocs: Vec<u32>,
    constructs: Vec<(u32, u32)>,
}

impl HitPool for HitPoolScript {
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
        self.allocs.push(Handle32::raw_or_zero(manager));
        Handle32::new(self.alloc_answer)
    }

    fn construct(
        &mut self,
        block: Handle32<UninitTask>,
        kind: u32,
    ) -> Option<Handle32<HitResponse>> {
        self.constructs.push((block.get(), kind));
        Handle32::new(self.construct_answer)
    }
}

#[test]
fn hit_clone_builds_from_the_kind() {
    let task = HitResponse::new(3, &mut HitBaseRec { calls: 0 });
    let mut pool = HitPoolScript {
        alloc_answer: 0x1000,
        construct_answer: 0x2000,
        allocs: Vec::new(),
        constructs: Vec::new(),
    };
    assert_eq!(
        task.clone_task(manager(9), &mut pool),
        Handle32::new(0x2000)
    );
    assert_eq!(pool.allocs, vec![9]);
    assert_eq!(pool.constructs, vec![(0x1000, 3)]);
}

#[test]
fn hit_clone_answers_null_without_constructing() {
    let task = HitResponse::new(3, &mut HitBaseRec { calls: 0 });
    let mut pool = HitPoolScript {
        alloc_answer: 0,
        construct_answer: 0x2000,
        allocs: Vec::new(),
        constructs: Vec::new(),
    };
    assert_eq!(task.clone_task(manager(9), &mut pool), None);
    assert!(pool.constructs.is_empty());
}

struct HitStartScript {
    lookup_answer: u32,
    case_answer: u32,
    lookups: Vec<u32>,
    cases: Vec<(u32, u32)>,
}

impl HitStart for HitStartScript {
    fn lookup(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<HitHandler>> {
        self.lookups.push(Handle32::raw_or_zero(manager));
        Handle32::new(self.lookup_answer)
    }

    fn run_case(&mut self, handler: Handle32<HitHandler>, kind: u32) -> u32 {
        self.cases.push((handler.get(), kind));
        self.case_answer
    }
}

fn hit_start(lookup_answer: u32, case_answer: u32) -> HitStartScript {
    HitStartScript {
        lookup_answer,
        case_answer,
        lookups: Vec::new(),
        cases: Vec::new(),
    }
}

#[test]
fn hit_start_runs_each_case() {
    for kind in 0..=3 {
        let task = HitResponse::new(kind, &mut HitBaseRec { calls: 0 });
        let mut run = hit_start(0x1111, 0x2222);
        assert_eq!(task.start(manager(9), &mut run), 0x2222);
        assert_eq!(run.lookups, vec![9]);
        assert_eq!(run.cases, vec![(0x1111, kind)]);
    }
}

#[test]
fn hit_start_rejects_kinds_above_three() {
    for kind in [4, 5, 0x100, u32::MAX] {
        let task = HitResponse::new(kind, &mut HitBaseRec { calls: 0 });
        let mut run = hit_start(0x1111, 0x2222);
        assert_eq!(task.start(manager(9), &mut run), 0, "kind {kind}");
        assert!(run.lookups.is_empty());
        assert!(run.cases.is_empty());
    }
}

#[test]
fn hit_start_answers_null_when_nothing_answers() {
    let task = HitResponse::new(1, &mut HitBaseRec { calls: 0 });
    let mut run = hit_start(0, 0x2222);
    assert_eq!(task.start(manager(9), &mut run), 0);
    assert_eq!(run.lookups, vec![9]);
    assert!(run.cases.is_empty());
}

// The duck.

fn duck() -> DuckTask {
    DuckTask::new(0, 1000, 100, 5, false, false, 0xAB)
}

struct DuckPoolScript {
    alloc_answer: u32,
    construct_answer: u32,
    allocs: Vec<u32>,
    constructs: Vec<(u32, DuckSpec)>,
}

impl DuckPool for DuckPoolScript {
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
        self.allocs.push(Handle32::raw_or_zero(manager));
        Handle32::new(self.alloc_answer)
    }

    fn construct(
        &mut self,
        block: Handle32<UninitTask>,
        spec: DuckSpec,
    ) -> Option<Handle32<DuckTask>> {
        self.constructs.push((block.get(), spec));
        Handle32::new(self.construct_answer)
    }
}

#[test]
fn duck_round_trips_its_words() {
    let task = DuckTask::new(0x1357, 10, 20, -7, true, true, 0xCD);
    assert_eq!(task.marks(), 0x1357);
    assert_eq!(task.start(), 10);
    assert_eq!(task.span(), 20);
    assert_eq!(task.level(), -7);
    assert!(task.done());
    assert!(task.flagged());
    assert_eq!(task.tag(), 0xCD);
}

#[test]
fn duck_clone_builds_from_tag_span_and_level() {
    let task = DuckTask::new(0, 1000, 100, -9, false, false, 0xAB);
    let mut pool = DuckPoolScript {
        alloc_answer: 0x1000,
        construct_answer: 0x2000,
        allocs: Vec::new(),
        constructs: Vec::new(),
    };
    assert_eq!(
        task.clone_task(manager(9), &mut pool),
        Handle32::new(0x2000)
    );
    assert_eq!(pool.allocs, vec![9]);
    assert_eq!(
        pool.constructs,
        vec![(
            0x1000,
            DuckSpec {
                tag: 0xAB,
                span: 100,
                level: -9,
            }
        )]
    );
}

#[test]
fn duck_clone_answers_null_without_constructing() {
    let mut pool = DuckPoolScript {
        alloc_answer: 0,
        construct_answer: 0x2000,
        allocs: Vec::new(),
        constructs: Vec::new(),
    };
    assert_eq!(duck().clone_task(manager(9), &mut pool), None);
    assert!(pool.constructs.is_empty());
}

struct DuckEventScript {
    query_answer: u32,
    announces: Vec<u32>,
    queries: Vec<u32>,
    triggers: Vec<u32>,
}

impl DuckEvent for DuckEventScript {
    fn announce(&mut self, sink: Option<Handle32<DuckSink>>) {
        self.announces.push(Handle32::raw_or_zero(sink));
    }

    fn query(&mut self, obj: Handle32<DuckQuery>) -> u32 {
        self.queries.push(obj.get());
        self.query_answer
    }

    fn trigger(&mut self, obj: Handle32<DuckQuery>) {
        self.triggers.push(obj.get());
    }
}

fn duck_event(query_answer: u32) -> DuckEventScript {
    DuckEventScript {
        query_answer,
        announces: Vec::new(),
        queries: Vec::new(),
        triggers: Vec::new(),
    }
}

fn sink(v: u32) -> Option<Handle32<DuckSink>> {
    Handle32::new(v)
}

fn query_obj(v: u32) -> Option<Handle32<DuckQuery>> {
    Handle32::new(v)
}

#[test]
fn duck_event_two_announces_without_querying() {
    let mut task = duck();
    let mut target = duck_event(QUERY_CODE);
    assert_eq!(
        task.handle_event(sink(0x40), 2, query_obj(0x50), QUERY_STATE, &mut target),
        1
    );
    assert!(!task.flagged());
    assert_eq!(target.announces, vec![0x40]);
    assert!(target.queries.is_empty());
    assert!(target.triggers.is_empty());
}

#[test]
fn duck_event_one_triggers_on_code_and_state() {
    let mut task = duck();
    let mut target = duck_event(QUERY_CODE);
    assert_eq!(
        task.handle_event(sink(0x40), 1, query_obj(0x50), QUERY_STATE, &mut target),
        1
    );
    assert_eq!(target.queries, vec![0x50]);
    assert_eq!(target.triggers, vec![0x50]);
    assert_eq!(target.announces, vec![0x40]);
}

#[test]
fn duck_event_one_skips_the_trigger_otherwise() {
    // Wrong code, right state: queried, not triggered.
    let mut task = duck();
    let mut target = duck_event(QUERY_CODE + 1);
    assert_eq!(
        task.handle_event(sink(0x40), 1, query_obj(0x50), QUERY_STATE, &mut target),
        1
    );
    assert_eq!(target.queries, vec![0x50]);
    assert!(target.triggers.is_empty());
    // Right code, neighbouring states: queried, not triggered.
    for state in [QUERY_STATE - 1, QUERY_STATE + 1, 0, u32::MAX] {
        let mut task = duck();
        let mut target = duck_event(QUERY_CODE);
        assert_eq!(
            task.handle_event(sink(0x40), 1, query_obj(0x50), state, &mut target),
            1
        );
        assert_eq!(target.queries, vec![0x50]);
        assert!(target.triggers.is_empty());
    }
    // No object: never queried.
    let mut task = duck();
    let mut target = duck_event(QUERY_CODE);
    assert_eq!(
        task.handle_event(sink(0x40), 1, None, QUERY_STATE, &mut target),
        1
    );
    assert!(target.queries.is_empty());
    // Level at or below -1: never queried.
    for level in [-1, -2, i16::MIN] {
        let mut task = DuckTask::new(0, 1000, 100, level, false, false, 0xAB);
        let mut target = duck_event(QUERY_CODE);
        assert_eq!(
            task.handle_event(sink(0x40), 1, query_obj(0x50), QUERY_STATE, &mut target),
            1
        );
        assert!(target.queries.is_empty(), "level {level}");
    }
}

#[test]
fn duck_other_events_set_the_flag() {
    for event in [0, 3, 4, 0x100, u32::MAX] {
        let mut task = duck();
        let mut target = duck_event(QUERY_CODE);
        assert_eq!(
            task.handle_event(sink(0x40), event, query_obj(0x50), QUERY_STATE, &mut target),
            0,
            "event {event}"
        );
        assert!(task.flagged());
        assert!(target.announces.is_empty());
        assert!(target.queries.is_empty());
    }
}

#[
...[truncated 8556 chars]