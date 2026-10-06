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
fn registry_counts_seventeen_proven() {
    let (proven, lifted, missing) = registry::counts();
    assert_eq!(proven, 17);
    assert_eq!(lifted, 0);
    assert_eq!(missing, 13);
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
                    ("ShockingEventFlee", "vf1"),
                    ("ShockingEventFlee", "vf7"),
                    ("ShockingEventFlee", "vf18"),
                    ("ShockingEventFlee", "vf19"),
                    ("ShockingEventFlee", "vf20"),
                    ("ShockingEventGoto", "ctor"),
                    ("ShockingEventGoto", "vf1"),
                    ("ShockingEventGoto", "vf19"),
                    ("ShockingEventGoto", "vf20"),
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
    assert_eq!(task.clone_task(None, &mut pool), Handle32::new(0x2000));
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

struct DuckPedScript {
    sample_answer: f32,
    samples: Vec<u32>,
    announces: Vec<(u32, u32)>,
}

impl DuckPedSide for DuckPedScript {
    fn sample(&mut self, ped: Handle32<DuckPed>) -> f32 {
        self.samples.push(ped.get());
        self.sample_answer
    }

    fn announce(&mut self, ped: Handle32<DuckPed>, flag: u32) {
        self.announces.push((ped.get(), flag));
    }
}

struct DuckPartsScript {
    elapsed_answer: u32,
    finish_answer: u32,
    sustains: Vec<u32>,
    elapsed_calls: u32,
    finish_calls: Vec<u32>,
}

impl DuckTaskSide for DuckPartsScript {
    fn sustain(&mut self, ped: Handle32<DuckPed>) {
        self.sustains.push(ped.get());
    }

    fn elapsed(&mut self) -> u32 {
        self.elapsed_calls += 1;
        self.elapsed_answer
    }

    fn finish_check(&mut self, ped: Handle32<DuckPed>) -> u32 {
        self.finish_calls.push(ped.get());
        self.finish_answer
    }
}

fn updater(sample: f32, elapsed: u32, finish: u32) -> (DuckPedScript, DuckPartsScript) {
    (
        DuckPedScript {
            sample_answer: sample,
            samples: Vec::new(),
            announces: Vec::new(),
        },
        DuckPartsScript {
            elapsed_answer: elapsed,
            finish_answer: finish,
            sustains: Vec::new(),
            elapsed_calls: 0,
            finish_calls: Vec::new(),
        },
    )
}

fn ped(v: u32) -> Handle32<DuckPed> {
    Handle32::new(v).unwrap()
}

#[test]
fn update_timeout_sets_done_at_the_span() {
    // One tick short: no timeout, sustain path answers 0.
    let mut task = duck();
    let (mut peds, mut parts) = updater(9.0, 0, 1);
    assert_eq!(task.update(ped(0x60), 1099, 1.0, &mut peds, &mut parts), 0);
    assert!(!task.done());
    // At the span: done is set and the update finishes.
    for tick in [1100, 1101, u32::MAX] {
        let mut task = duck();
        let (mut peds, mut parts) = updater(9.0, 0, 1);
        assert_eq!(task.update(ped(0x60), tick, 1.0, &mut peds, &mut parts), 1);
        assert!(task.done(), "tick {tick}");
    }
    // The subtraction is unsigned: a wrapped tick counts from the start.
    let mut task = DuckTask::new(0, u32::MAX, 10, 5, false, false, 0xAB);
    let (mut peds, mut parts) = updater(9.0, 0, 1);
    assert_eq!(task.update(ped(0x60), 5, 1.0, &mut peds, &mut parts), 0);
    assert!(!task.done());
    let mut task = DuckTask::new(0, u32::MAX, 10, 5, false, false, 0xAB);
    let (mut peds, mut parts) = updater(9.0, 0, 1);
    assert_eq!(task.update(ped(0x60), 9, 1.0, &mut peds, &mut parts), 1);
    assert!(task.done());
    // A zero span disables the timeout arm entirely.
    let mut task = DuckTask::new(0, 1000, 0, 5, false, false, 0xAB);
    let (mut peds, mut parts) = updater(9.0, 0, 1);
    assert_eq!(
        task.update(ped(0x60), u32::MAX, 1.0, &mut peds, &mut parts),
        0
    );
    assert!(!task.done());
}

#[test]
fn update_done_skips_the_sample() {
    let mut task = DuckTask::new(0, 1000, 100, 5, true, false, 0xAB);
    let (mut peds, mut parts) = updater(9.0, 0, 1);
    assert_eq!(task.update(ped(0x60), 1000, 1.0, &mut peds, &mut parts), 1);
    assert!(peds.samples.is_empty());
    assert_eq!(peds.announces, vec![(0x60, FINISH_FLAG)]);
}

#[test]
fn update_sample_below_limit_finishes() {
    assert_eq!(SUSTAIN_FLAG, 1);
    assert_eq!(FINISH_FLAG, 0);
    // Just below: finishing path.
    let mut task = duck();
    let (mut peds, mut parts) = updater(0.99, 0, 1);
    assert_eq!(task.update(ped(0x60), 1000, 1.0, &mut peds, &mut parts), 1);
    assert_eq!(peds.announces, vec![(0x60, FINISH_FLAG)]);
    // At the limit: sustain path.
    let mut task = duck();
    let (mut peds, mut parts) = updater(1.0, 0, 1);
    assert_eq!(task.update(ped(0x60), 1000, 1.0, &mut peds, &mut parts), 0);
    assert_eq!(peds.announces, vec![(0x60, SUSTAIN_FLAG)]);
    // A NaN sample never exceeds the limit: sustain path.
    let mut task = duck();
    let (mut peds, mut parts) = updater(f32::NAN, 0, 1);
    assert_eq!(task.update(ped(0x60), 1000, 1.0, &mut peds, &mut parts), 0);
    // A NaN limit never exceeds the sample either.
    let mut task = duck();
    let (mut peds, mut parts) = updater(0.0, 0, 1);
    assert_eq!(
        task.update(ped(0x60), 1000, f32::NAN, &mut peds, &mut parts),
        0
    );
}

#[test]
fn update_flagged_sustain_skips_helpers() {
    let mut task = DuckTask::new(0, 1000, 0, 5, false, true, 0xAB);
    let (mut peds, mut parts) = updater(9.0, 7, 1);
    assert_eq!(task.update(ped(0x60), 1000, 1.0, &mut peds, &mut parts), 0);
    assert_eq!(peds.announces, vec![(0x60, SUSTAIN_FLAG)]);
    assert!(parts.sustains.is_empty());
    assert_eq!(parts.elapsed_calls, 0);
    assert_eq!(task.level(), 5);
}

#[test]
fn update_sustain_needs_a_wrapped_timeout() {
    // start + span wraps below the tick while tick - start stays short.
    let mut task = DuckTask::new(0, 0xFFFF_FF00, 0x200, 5, false, false, 0xAB);
    let (mut peds, mut parts) = updater(9.0, 0, 1);
    assert_eq!(
        task.update(ped(0x60), 0xFFFF_FF50, 1.0, &mut peds, &mut parts),
        0
    );
    assert!(!task.done());
    assert_eq!(parts.sustains, vec![0x60]);
    // Without the wrap the same shape never sustains.
    let mut task = DuckTask::new(0, 1000, 100, 5, false, false, 0xAB);
    let (mut peds, mut parts) = updater(9.0, 0, 1);
    assert_eq!(task.update(ped(0x60), 1099, 1.0, &mut peds, &mut parts), 0);
    assert!(parts.sustains.is_empty());
}

#[test]
fn update_nonpositive_level_skips_the_drain() {
    for level in [0, -1, i16::MIN] {
        let mut task = DuckTask::new(0, 1000, 0, level, false, false, 0xAB);
        let (mut peds, mut parts) = updater(9.0, 7, 1);
        assert_eq!(
            task.update(ped(0x60), 1000, 1.0, &mut peds, &mut parts),
            0,
            "level {level}"
        );
        assert_eq!(parts.elapsed_calls, 0);
        assert_eq!(task.level(), level);
    }
}

#[test]
fn update_sustain_announces_and_drains() {
    let mut task = duck();
    let (mut peds, mut parts) = updater(9.0, 30, 1);
    assert_eq!(task.update(ped(0x60), 1000, 1.0, &mut peds, &mut parts), 0);
    assert_eq!(peds.announces, vec![(0x60, SUSTAIN_FLAG)]);
    assert_eq!(parts.elapsed_calls, 1);
    assert_eq!(task.span(), 100);
    assert_eq!(task.level(), 70);
}

#[test]
fn update_level_floor_and_truncation() {
    // (span, subtrahend, resulting level).
    for (span, sub, want) in [
        (0x12345, 0, 0x2345),
        (100, 101, 0),
        (100, 100, 0),
        (0x8000, 0, -0x8000),
        (0x8000_0000, 0, 0),
        (0xFFFF_FFFF, 0, 0),
    ] {
        let mut task = DuckTask::new(0, 1000, span, 5, false, false, 0xAB);
        let (mut peds, mut parts) = updater(9.0, sub, 1);
        assert_eq!(
            task.update(ped(0x60), 1000, 1.0, &mut peds, &mut parts),
            0,
            "span {span:#x} sub {sub:#x}"
        );
        assert_eq!(task.level(), want, "span {span:#x} sub {sub:#x}");
    }
}

#[test]
fn update_finishing_runs_the_check_unless_gated() {
    // A passing low byte sets marks bit 1 and keeps the rest.
    let mut task = DuckTask::new(0x100, 1000, 0, 5, true, false, 0xAB);
    let (mut peds, mut parts) = updater(9.0, 0, 1);
    assert_eq!(task.update(ped(0x60), 1000, 1.0, &mut peds, &mut parts), 1);
    assert_eq!(parts.finish_calls, vec![0x60]);
    assert_eq!(task.marks(), 0x102);
    assert_eq!(peds.announces, vec![(0x60, FINISH_FLAG)]);
    // A zero low byte leaves the marks alone.
    let mut task = DuckTask::new(0x100, 1000, 0, 5, true, false, 0xAB);
    let (mut peds, mut parts) = updater(9.0, 0, 0x100);
    assert_eq!(task.update(ped(0x60), 1000, 1.0, &mut peds, &mut parts), 1);
    assert_eq!(task.marks(), 0x100);
    // The flag and marks bit 0 each skip the check.
    let mut task = DuckTask::new(0, 1000, 0, 5, true, true, 0xAB);
    let (mut peds, mut parts) = updater(9.0, 0, 1);
    assert_eq!(task.update(ped(0x60), 1000, 1.0, &mut peds, &mut parts), 1);
    assert!(parts.finish_calls.is_empty());
    let mut task = DuckTask::new(1, 1000, 0, 5, true, false, 0xAB);
    let (mut peds, mut parts) = updater(9.0, 0, 1);
    assert_eq!(task.update(ped(0x60), 1000, 1.0, &mut peds, &mut parts), 1);
    assert!(parts.finish_calls.is_empty());
    assert_eq!(task.marks(), 1);
}

// The flee task.

use lf_peds_tasks::tasks::{
    FleeEntity, FleePed, FleePoll, FleeSpawn, FleeTask, GotoEntity, GotoPool, GotoTask, SubTask,
};

fn flee() -> FleeTask {
    FleeTask::new(
        Handle32::new(0xBEEF),
        0,
        0x1B,
        [3.0, 4.0, 0.0],
        false,
        None,
        0x5A,
    )
}

fn entity(v: u32) -> Option<Handle32<FleeEntity>> {
    Handle32::new(v)
}

fn subtask(v: u32) -> Option<Handle32<SubTask>> {
    Handle32::new(v)
}

fn flee_ped(v: u32) -> Option<Handle32<FleePed>> {
    Handle32::new(v)
}

struct FleeSpawnScript {
    alloc_answer: u32,
    entity_answer: u32,
    vector_answer: u32,
    allocs: Vec<u32>,
    entities: Vec<(u32, u32, u32)>,
    vectors: Vec<(u32, [u32; 3], u32)>,
}

impl FleeSpawn for FleeSpawnScript {
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
        self.allocs.push(Handle32::raw_or_zero(manager));
        Handle32::new(self.alloc_answer)
    }

    fn construct_entity(
        &mut self,
        block: Handle32<UninitTask>,
        entity: Handle32<FleeEntity>,
        kind: u32,
    ) -> Option<Handle32<SubTask>> {
        self.entities.push((block.get(), entity.get(), kind));
        Handle32::new(self.entity_answer)
    }

    fn construct_vector(
        &mut self,
        block: Handle32<UninitTask>,
        pos: [f32; 3],
        kind: u32,
    ) -> Option<Handle32<SubTask>> {
        self.vectors.push((
            block.get(),
            [pos[0].to_bits(), pos[1].to_bits(), pos[2].to_bits()],
            kind,
        ));
        Handle32::new(self.vector_answer)
    }
}

fn spawner() -> FleeSpawnScript {
    FleeSpawnScript {
        alloc_answer: 0x1000,
        entity_answer: 0x2000,
        vector_answer: 0x3000,
        allocs: Vec::new(),
        entities: Vec::new(),
        vectors: Vec::new(),
    }
}

#[test]
fn flee_round_trips_its_words() {
    let task = flee();
    assert_eq!(task.subtask(), subtask(0xBEEF));
    assert_eq!(task.marks(), 0);
    assert_eq!(task.kind(), 0x1B);
    assert_eq!(task.pos(), [3.0, 4.0, 0.0]);
    assert!(!task.flag());
    assert_eq!(task.mode(), None);
    assert_eq!(task.state(), 0x5A);
}

#[test]
fn flee_gate_flag_paths_ignore_the_position() {
    // Set flag with a set mode fires at once: entity form.
    let task = FleeTask::new(subtask(1), 0, 7, [0.0, 0.0, 0.0], true, entity(0xE0), 0);
    let mut spawn = spawner();
    assert_eq!(
        task.spawn(f32::MAX, manager(9), &mut spawn),
        subtask(0x2000)
    );
    assert_eq!(spawn.allocs, vec![9]);
    assert_eq!(spawn.entities, vec![(0x1000, 0xE0, 7)]);
    assert!(spawn.vectors.is_empty());
    // Mixed pairs never fire, whatever the position.
    for (flag, mode) in [(true, None), (false, entity(0xE0))] {
        let task = FleeTask::new(subtask(1), 0, 7, [1.0e30, 1.0e30, 1.0e30], flag, mode, 0);
        let mut spawn = spawner();
        assert_eq!(task.spawn(0.0, manager(9), &mut spawn), None);
        assert!(spawn.allocs.is_empty());
    }
}

#[test]
fn flee_gate_measures_length_strictly() {
    // Squared length exactly 25: fires above, rests at or below.
    let task = FleeTask::new(subtask(1), 0, 7, [3.0, 4.0, 0.0], false, None, 0);
    let mut spawn = spawner();
    assert_eq!(task.spawn(24.0, manager(9), &mut spawn), subtask(0x3000));
    assert_eq!(spawn.vectors.len(), 1);
    assert_eq!(
        spawn.vectors[0].1,
        [3.0f32.to_bits(), 4.0f32.to_bits(), 0.0f32.to_bits()]
    );
    assert_eq!(spawn.vectors[0].2, 7);
    for threshold in [25.0, 26.0, f32::INFINITY] {
        let mut spawn = spawner();
        assert_eq!(task.spawn(threshold, manager(9), &mut spawn), None);
        assert!(spawn.allocs.is_empty());
    }
    // A NaN component never exceeds the threshold.
    let task = FleeTask::new(subtask(1), 0, 7, [f32::NAN, 0.0, 0.0], false, None, 0);
    let mut spawn = spawner();
    assert_eq!(task.spawn(0.0, manager(9), &mut spawn), None);
}

#[test]
fn flee_spawn_answers_null_without_a_block() {
    let task = flee();
    let mut spawn = spawner();
    spawn.alloc_answer = 0;
    assert_eq!(task.spawn(0.0, manager(9), &mut spawn), None);
    assert_eq!(spawn.allocs, vec![9]);
    assert!(spawn.entities.is_empty());
    assert!(spawn.vectors.is_empty());
}

struct FleePollScript {
    probe_answer: u32,
    check_answer: u32,
    probes: Vec<u32>,
    checks: Vec<u32>,
    dispatches: Vec<u32>,
}

impl FleePoll for FleePollScript {
    fn probe(&mut self, ped: Option<Handle32<FleePed>>) -> u32 {
        self.probes.push(Handle32::raw_or_zero(ped));
        self.probe_answer
    }

    fn check(&mut self, ped: Option<Handle32<FleePed>>) -> u32 {
        self.checks.push(Handle32::raw_or_zero(ped));
        self.check_answer
    }

    fn dispatch(&mut self, ped: Option<Handle32<FleePed>>) {
        self.dispatches.push(Handle32::raw_or_zero(ped));
    }
}

fn poller(probe_answer: u32, check_answer: u32) -> FleePollScript {
    FleePollScript {
        probe_answer,
        check_answer,
        probes: Vec::new(),
        checks: Vec::new(),
        dispatches: Vec::new(),
    }
}

#[test]
fn flee_poll_keeps_on_a_fired_probe() {
    // Live gate (long position, clear flag and mode) + set low byte.
    let mut task = flee();
    let mut poll = poller(1, 0);
    assert_eq!(task.poll(flee_ped(0x60), 0.0, &mut poll), subtask(0xBEEF));
    assert_eq!(poll.probes, vec![0x60]);
    assert!(poll.checks.is_empty());
    assert!(poll.dispatches.is_empty());
    assert_eq!(task.marks(), 0);
    // Only the low byte decides: 0x100 refuses.
    let mut task = flee();
    let mut poll = poller(0x100, 0);
    assert_eq!(task.poll(flee_ped(0x60), 0.0, &mut poll), subtask(0xBEEF));
    assert_eq!(poll.checks, vec![0x60]);
}

#[test]
fn flee_poll_checks_then_dispatches() {
    // Refused check keeps the subtask without dispatching.
    let mut task = flee();
    let mut poll = poller(0, 0);
    assert_eq!(task.poll(flee_ped(0x60), 0.0, &mut poll), subtask(0xBEEF));
    assert_eq!(task.marks(), 0);
    assert!(poll.dispatches.is_empty());
    // Passed check marks and dispatches, answering null.
    let mut task = flee();
    let mut poll = poller(0, 0x1FF);
    assert_eq!(task.poll(flee_ped(0x60), 0.0, &mut poll), None);
    assert_eq!(task.marks(), 2);
    assert_eq!(poll.dispatches, vec![0x60]);
    // The gate bit skips the check but still dispatches.
    let mut task = FleeTask::new(subtask(1), 1, 7, [0.0, 0.0, 0.0], false, None, 0);
    let mut poll = poller(0, 0xFF);
    assert_eq!(task.poll(flee_ped(0x60), 99.0, &mut poll), None);
    assert!(poll.probes.is_empty());
    assert!(poll.checks.is_empty());
    assert_eq!(poll.dispatches, vec![0x60]);
    assert_eq!(task.marks(), 1);
}

#[test]
fn flee_poll_never_probes_a_dead_gate() {
    let mut task = FleeTask::new(subtask(1), 0, 7, [0.0, 0.0, 0.0], false, None, 0);
    let mut poll = poller(0xFF, 0xFF);
    assert_eq!(task.poll(flee_ped(0x60), 99.0, &mut poll), None);
    assert!(poll.probes.is_empty());
    assert_eq!(poll.checks, vec![0x60]);
}

// The goto task.

fn goto_task() -> GotoTask {
    GotoTask::new(
        subtask(0xBEEF),
        0x1B,
        [1.0, 2.0, 3.0],
        true,
        Handle32::new(0xE0),
        5000,
        1000,
        5000,
        true,
        false,
        2.5,
    )
}

fn goto_entity(v: u32) -> Option<Handle32<GotoEntity>> {
    Handle32::new(v)
}

struct GotoPoolScript {
    alloc_answer: u32,
    construct_answer: u32,
    allocs: Vec<u32>,
    constructs: Vec<(u32, u32)>,
}

impl GotoPool for GotoPoolScript {
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
        self.allocs.push(Handle32::raw_or_zero(manager));
        Handle32::new(self.alloc_answer)
    }

    fn construct(
        &mut self,
        block: Handle32<UninitTask>,
        member: u32,
    ) -> Option<Handle32<GotoTask>> {
        self.constructs.push((block.get(), member));
        Handle32::new(self.construct_answer)
    }
}

#[test]
fn goto_round_trips_its_words() {
    let task = goto_task();
    assert_eq!(task.subtask(), subtask(0xBEEF));
    assert_eq!(task.kind(), 0x1B);
    assert_eq!(task.pos(), [1.0, 2.0, 3.0]);
    assert!(task.flag());
    assert_eq!(task.mode(), goto_entity(0xE0));
    assert_eq!(task.wait(), 5000);
    assert_eq!(task.stamp(), 1000);
    assert_eq!(task.wait_copy(), 5000);
    assert!(task.armed());
    assert!(!task.restamp());
    assert_eq!(task.speed(), 2.5);
}

#[test]
fn goto_clone_builds_from_the_member() {
    let task = goto_task();
    let mut pool = GotoPoolScript {
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
    assert_eq!(pool.constructs, vec![(0x1000, 0x1B)]);
}

#[test]
fn goto_clone_answers_null_without_constructing() {
    let task = goto_task();
    let mut pool = GotoPoolScript {
        alloc_answer: 0,
        construct_answer: 0x2000,
        allocs: Vec::new(),
        constructs: Vec::new(),
    };
    assert_eq!(task.clone_task(manager(9), &mut pool), None);
    assert!(pool.constructs.is_empty());
}

// The goto periodic update.

use lf_peds_tasks::tasks::{GotoPed, GotoPoll, SubVerdict};

fn goto_ped(v: u32) -> Option<Handle32<GotoPed>> {
    Handle32::new(v)
}

struct GotoPollScript {
    probe_answer: u32,
    verdict: SubVerdict,
    probes: Vec<u32>,
    fallbacks: Vec<u32>,
    checks: Vec<(u32, u32)>,
    dispatches: Vec<u32>,
}

impl GotoPoll for GotoPollScript {
    fn probe(&mut self, ped: Option<Handle32<GotoPed>>) -> u32 {
        self.probes.push(Handle32::raw_or_zero(ped));
        self.probe_answer
    }

    fn fallback(&mut self, ped: Option<Handle32<GotoPed>>) {
        self.fallbacks.push(Handle32::raw_or_zero(ped));
    }

    fn check_subtask(
        &mut self,
        sub: Handle32<SubTask>,
        ped: Option<Handle32<GotoPed>>,
    ) -> SubVerdict {
        self.checks.push((sub.get(), Handle32::raw_or_zero(ped)));
        self.verdict
    }

    fn dispatch(&mut self, ped: Option<Handle32<GotoPed>>) {
        self.dispatches.push(Handle32::raw_or_zero(ped));
    }
}

fn goto_poller(probe_answer: u32, verdict: SubVerdict) -> GotoPollScript {
    GotoPollScript {
        probe_answer,
        verdict,
        probes: Vec::new(),
        fallbacks: Vec::new(),
        checks: Vec::new(),
        dispatches: Vec::new(),
    }
}

fn armed_goto() -> GotoTask {
    // Armed with a pending restamp, a long position and a live subtask.
    GotoTask::new(
        subtask(0xBEEF),
        0x1B,
        [3.0, 4.0, 0.0],
        false,
        None,
        5000,
        1000,
        5000,
        true,
        true,
        2.5,
    )
}

#[test]
fn goto_poll_restamps_then_times_the_wait() {
    // Restamp writes the tick and clears itself; the fresh deadline
    // (tick + wait) lies ahead, so the gate still runs.
    let mut task = armed_goto();
    let mut poll = goto_poller(0, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 2000, 0.0, &mut poll), None);
    assert_eq!(task.stamp(), 2000);
    assert!(!task.restamp());
    assert_eq!(poll.probes, vec![0x60]);
    assert_eq!(poll.dispatches, vec![0x60]);
    // A restamp with a zero wait lapses at once: deadline equals tick.
    let mut task = GotoTask::new(
        subtask(1),
        0,
        [3.0, 4.0, 0.0],
        false,
        None,
        0,
        1000,
        0,
        true,
        true,
        0.0,
    );
    let mut poll = goto_poller(0xFF, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 2000, 0.0, &mut poll), None);
    assert_eq!(task.stamp(), 2000);
    assert!(poll.probes.is_empty());
    // A lapsed wait skips the gate: no probe, straight to the check.
    let mut task = GotoTask::new(
        subtask(0xBEEF),
        0x1B,
        [3.0, 4.0, 0.0],
        false,
        None,
        5000,
        1000,
        5000,
        true,
        false,
        2.5,
    );
    let mut poll = goto_poller(0xFF, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 7000, 0.0, &mut poll), None);
    assert_eq!(task.stamp(), 1000);
    assert!(poll.probes.is_empty());
    assert_eq!(poll.checks, vec![(0xBEEF, 0x60)]);
    // At the deadline exactly: skipped (>= is <= here).
    let mut task = GotoTask::new(
        subtask(1),
        0,
        [3.0, 4.0, 0.0],
        false,
        None,
        0,
        1000,
        5000,
        true,
        false,
        0.0,
    );
    let mut poll = goto_poller(0xFF, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 6000, 0.0, &mut poll), None);
    assert!(poll.probes.is_empty());
    // One tick short: the gate runs and the probe fires.
    let mut task = GotoTask::new(
        subtask(1),
        0,
        [3.0, 4.0, 0.0],
        false,
        None,
        0,
        1000,
        5000,
        true,
        false,
        0.0,
    );
    let mut poll = goto_poller(0xFF, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 5999, 0.0, &mut poll), subtask(1));
    assert_eq!(poll.probes, vec![0x60]);
    assert_eq!(poll.fallbacks, vec![0x60]);
    assert!(poll.checks.is_empty());
}

#[test]
fn goto_poll_disarmed_runs_the_gate() {
    let mut task = GotoTask::new(
        subtask(1),
        0,
        [3.0, 4.0, 0.0],
        false,
        None,
        777,
        888,
        999,
        false,
        true,
        0.0,
    );
    let mut poll = goto_poller(0, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 2000, 0.0, &mut poll), None);
    // Disarmed: the stamp and restamp bytes are untouched.
    assert_eq!(task.stamp(), 888);
    assert!(task.restamp());
    assert_eq!(poll.probes, vec![0x60]);
}

#[test]
fn goto_poll_refused_check_falls_back() {
    let mut task = armed_goto();
    let mut poll = goto_poller(0, SubVerdict::Refused);
    assert_eq!(
        task.poll(goto_ped(0x60), 2000, 0.0, &mut poll),
        subtask(0xBEEF)
    );
    assert_eq!(poll.checks, vec![(0xBEEF, 0x60)]);
    assert_eq!(poll.fallbacks, vec![0x60]);
    assert!(poll.dispatches.is_empty());
}

#[test]
fn goto_poll_probe_keeps_without_a_subtask() {
    // The probe-keeps path never touches the subtask: no panic.
    let mut task = GotoTask::new(
        None,
        0,
        [3.0, 4.0, 0.0],
        false,
        None,
        0,
        0,
        0,
        false,
        false,
        0.0,
    );
    let mut poll = goto_poller(0xFF, SubVerdict::Passed);
    assert_eq!(task.poll(goto_ped(0x60), 0, 0.0, &mut poll), None);
    assert_eq!(poll.fallbacks, vec![0x60]);
    assert!(poll.checks.is_empty());
}

#[test]
#[should_panic(expected = "without a subtask")]
fn goto_poll_panics_past_the_probe_without_a_subtask() {
    let mut task = GotoTask::new(
        None,
        0,
        [3.0, 4.0, 0.0],
        false,
        None,
        0,
        0,
        0,
        false,
        false,
        0.0,
    );
    let mut poll = goto_poller(0, SubVerdict::Passed);
    let _ = task.poll(goto_ped(0x60), 0, 0.0, &mut poll);
}

// The second wave: the goto constructor, the flee clone, the flee
// reaction update and the goto target picker.

use lf_peds_tasks::tasks::{
    EventChild, FleeAnswer, FleeEvent, FleePool, FleeProbe, FleeReact, FleeTarget, GotoBase,
    GotoChild, GotoPick, ReactPed, Reaction, ReactionTask,
};

struct GotoBaseScript {
    speed_answer: f32,
    inits: Vec<u32>,
    queries: Vec<u32>,
}

impl GotoBase for GotoBaseScript {
    fn construct_base(&mut self, init: u32) {
        self.inits.push(init);
    }

    fn query_speed(&mut self, kind: u32) -> f32 {
        self.queries.push(kind);
        self.speed_answer
    }
}

#[test]
fn goto_ctor_clears_and_queries() {
    let mut base = GotoBaseScript {
        speed_answer: 7.5,
        inits: Vec::new(),
        queries: Vec::new(),
    };
    let task = GotoTask::from_init(
        subtask(0xBEEF),
        0x1B,
        [1.0, 2.0, 3.0],
        true,
        goto_entity(0xE0),
        0xA5,
        &mut base,
    );
    assert_eq!(base.inits, vec![0xA5]);
    assert_eq!(base.queries, vec![0x1B]);
    assert_eq!(task.subtask(), subtask(0xBEEF));
    assert_eq!(task.kind(), 0x1B);
    assert_eq!(task.wait(), 0);
    assert_eq!(task.stamp(), 0);
    assert_eq!(task.wait_copy(), 0);
    assert!(!task.armed());
    assert!(!task.restamp());
    assert_eq!(task.speed(), 7.5);
}

#[test]
fn goto_ctor_keeps_a_negative_zero_speed() {
    let mut base = GotoBaseScript {
        speed_answer: -0.0,
        inits: Vec::new(),
        queries: Vec::new(),
    };
    let task = GotoTask::from_init(None, 0, [0.0; 3], false, None, 0, &mut base);
    assert_eq!(task.speed().to_bits(), (-0.0f32).to_bits());
}

struct FleePoolScript {
    alloc_answer: u32,
    construct_answer: u32,
    allocs: Vec<u32>,
    constructs: Vec<(u32, u32, u8)>,
}

impl FleePool for FleePoolScript {
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
        self.allocs.push(Handle32::raw_or_zero(manager));
        Handle32::new(self.alloc_answer)
    }

    fn construct(
        &mut self,
        block: Handle32<UninitTask>,
        member: u32,
        state: u8,
    ) -> Option<Handle32<FleeTask>> {
        self.constructs.push((block.get(), member, state));
        Handle32::new(self.construct_answer)
    }
}

#[test]
fn flee_clone_carries_kind_and_state() {
    let task = flee();
    let mut pool = FleePoolScript {
        alloc_answer: 0x1000,
        construct_answer: 0x2000,
        allocs: Vec::new(),
        constructs: Vec::new(),
    };
    let clone = task.clone_task(manager(0x77), &mut pool);
    assert_eq!(Handle32::raw_or_zero(clone), 0x2000);
    assert_eq!(pool.allocs, vec![0x77]);
    assert_eq!(pool.constructs, vec![(0x1000, 0x1B, 0x5A)]);
}

#[test]
#[should_panic(expected = "without a block")]
fn flee_clone_panics_without_a_block() {
    let task = flee();
    let mut pool = FleePoolScript {
        alloc_answer: 0,
        construct_answer: 0x2000,
        allocs: Vec::new(),
        constructs: Vec::new(),
    };
    let _ = task.clone_task(manager(0x77), &mut pool);
}

fn reaction(task: u32, marks: u32) -> Option<Reaction> {
    Some(Reaction::new(
        Handle32::<ReactionTask>::new(task).expect("host reaction is live"),
        marks,
    ))
}

struct FleeReactScript {
    type_answer: u32,
    kind_answer: u32,
    probe_answer: u32,
    alloc_answer: u32,
    build_answer: Option<Reaction>,
    dispatches: Vec<u32>,
}

impl FleeReact for FleeReactScript {
    fn subtask_type(&mut self, _sub: Handle32<SubTask>) -> u32 {
        self.type_answer
    }

    fn entity_kind(&mut self, _entity: Handle32<FleeEntity>) -> u32 {
        self.kind_answer
    }

    fn probe(
        &mut self,
        _ped: Option<Handle32<FleePed>>,
        _entity: Handle32<FleeEntity>,
    ) -> u32 {
        self.probe_answer
    }

    fn alloc(&mut self, _manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
        Handle32::new(self.alloc_answer)
    }

    fn build(
        &mut self,
        _block: Handle32<UninitTask>,
        _entity: Handle32<FleeEntity>,
    ) -> Option<Reaction> {
        self.build_answer
    }

    fn dispatch(&mut self, ped: Option<Handle32<FleePed>>) {
        self.dispatches.push(Handle32::raw_or_zero(ped));
    }
}

fn reactor() -> FleeReactScript {
    FleeReactScript {
        type_answer: 0x16E,
        kind_answer: 0xC0,
        probe_answer: 0,
        alloc_answer: 0x1000,
        build_answer: reaction(0x3000, 0),
        dispatches: Vec::new(),
    }
}

fn flee_react_task(kind: u32) -> FleeTask {
    FleeTask::new(subtask(0xBEEF), 0, kind, [0.0; 3], false, entity(0xE0), 0)
}

#[test]
fn flee_react_dispatches_on_each_refused_gate() {
    // Wrong subtask type.
    let task = flee_react_task(0);
    let mut react = reactor();
    react.type_answer = 0x16D;
    assert_eq!(task.react(flee_ped(0x60), manager(1), &mut react), None);
    assert_eq!(react.dispatches, vec![0x60]);
    // Missing entity.
    let task = FleeTask::new(subtask(0xBEEF), 0, 0, [0.0; 3], false, None, 0);
    let mut react = reactor();
    assert_eq!(task.react(flee_ped(0x60), manager(1), &mut react), None);
    assert_eq!(react.dispatches, vec![0x60]);
    // Wrong kind bits (neighbouring mask values).
    for kind_word in [0u32, 0x80, 0x3C0, 0x1C0, 0x200] {
        let task = flee_react_task(0);
        let mut react = reactor();
        react.kind_answer = kind_word;
        assert_eq!(task.react(flee_ped(0x60), manager(1), &mut react), None);
        assert_eq!(react.dispatches, vec![0x60]);
    }
    // A set probe low byte refuses even when the high bytes are clear.
    for probe in [1u32, 0xFF, 0x100 | 0x42, 0xDEAD_BEEF] {
        let task = flee_react_task(0);
        let mut react = reactor();
        react.probe_answer = probe;
        assert_eq!(task.react(flee_ped(0x60), manager(1), &mut react), None);
        assert_eq!(react.dispatches, vec![0x60]);
    }
    // A clear low byte passes even with high bytes set.
    let task = flee_react_task(0x1B);
    let mut react = reactor();
    react.probe_answer = 0xFF00;
    let built = task.react(flee_ped(0x60), manager(1), &mut react);
    assert!(built.is_some());
    assert!(react.dispatches.is_empty());
}

#[test]
fn flee_react_marks_narrow_at_the_flag_limit() {
    // Just below the limit the wide-range bit clears.
    let task = flee_react_task(0x1A);
    let mut react = reactor();
    react.build_answer = reaction(0x3000, 0xFFFF_FFFF);
    let built = task.react(flee_ped(0x60), manager(1), &mut react).expect("builds");
    assert_eq!(built.task().get(), 0x3000);
    assert_eq!(built.marks(), 0xFFFF_FFFF & 0xFFFB_FFFF);
    // At the limit it survives.
    let task = flee_react_task(0x1B);
    let mut react = reactor();
    react.build_answer = reaction(0x3000, 0xFFFF_FFFF);
    let built = task.react(flee_ped(0x60), manager(1), &mut react).expect("builds");
    assert_eq!(built.marks(), 0xFFFF_FFFF);
    // The mark bit sets from clear.
    let task = flee_react_task(0x1C);
    let mut react = reactor();
    react.build_answer = reaction(0x3000, 0);
    let built = task.react(flee_ped(0x60), manager(1), &mut react).expect("builds");
    assert_eq!(built.marks(), 8);
}

#[test]
fn flee_react_dispatches_without_a_ped_past_the_early_gates() {
    let task = flee_react_task(0);
    let mut react = reactor();
    react.type_answer = 0;
    assert_eq!(task.react(None, manager(1), &mut react), None);
    assert_eq!(react.dispatches, vec![0]);
}

#[test]
#[should_panic(expected = "without a subtask")]
fn flee_react_panics_without_a_subtask() {
    let task = FleeTask::new(None, 0, 0, [0.0; 3], false, entity(0xE0), 0);
    let mut react = reactor();
    let _ = task.react(flee_ped(0x60), manager(1), &mut react);
}

#[test]
#[should_panic(expected = "without a ped")]
fn flee_react_panics_without_a_ped_on_the_probe_path() {
    let task = flee_react_task(0x1B);
    let mut react = reactor();
    let _ = task.react(None, manager(1), &mut react);
}

#[test]
#[should_panic(expected = "without a block")]
fn flee_react_panics_without_a_block() {
    let task = flee_react_task(0x1B);
    let mut react = reactor();
    react.alloc_answer = 0;
    let _ = task.react(flee_ped(0x60), manager(1), &mut react);
}

#[test]
#[should_panic(expected = "without a build")]
fn flee_react_panics_without_a_build() {
    let task = flee_react_task(0x1B);
    let mut react = reactor();
    react.build_answer = None;
    let _ = task.react(flee_ped(0x60), manager(1), &mut react);
}

struct GotoPickScript {
    hash_answers: [u32; 2],
    hash_next: usize,
    rand_answer: u32,
    alloc_answers: [u32; 3],
    alloc_next: usize,
    child1_answer: u32,
    child2_answer: u32,
    combine_answer: u32,
    hashes: Vec<u32>,
    firsts: Vec<(u32, u32, u32)>,
    seconds: Vec<u32>,
    combines: Vec<(u32, u32, u32)>,
}

impl GotoPick for GotoPickScript {
    fn seed(&mut self, _ped: Option<Handle32<GotoPed>>) {}

    fn hash_kind(&mut self, kind: u32) -> u32 {
        self.hashes.push(kind);
        let ans = self.hash_answers[self.hash_next];
        self.hash_next += 1;
        ans
    }

    fn rand_word(&mut self) -> u32 {
        self.rand_answer
    }

    fn find_goal(&mut self, _kind: u32) {}

    fn alloc(&mut self, _manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
        let ans = self.alloc_answers[self.alloc_next];
        self.alloc_next += 1;
        Handle32::new(ans)
    }

    fn build_first(
        &mut self,
        block: Handle32<UninitTask>,
        speed: f32,
        rate: f32,
    ) -> Option<Handle32<GotoChild>> {
        self.firsts.push((block.get(), speed.to_bits(), rate.to_bits()));
        Handle32::new(self.child1_answer)
    }

    fn build_second(&mut self, block: Handle32<UninitTask>) -> Option<Handle32<GotoChild>> {
        self.seconds.push(block.get());
        Handle32::new(self.child2_answer)
    }

    fn combine(
        &mut self,
        block: Handle32<UninitTask>,
        first: Option<Handle32<GotoChild>>,
        second: Option<Handle32<GotoChild>>,
    ) -> Option<Handle32<GotoChild>> {
        self.combines.push((
            block.get(),
            Handle32::raw_or_zero(first),
            Handle32::raw_or_zero(second),
        ));
        Handle32::new(self.combine_answer)
    }
}

fn picker() -> GotoPickScript {
    GotoPickScript {
        hash_answers: [0x1234_5678, 0x9ABC_DEF0],
        hash_next: 0,
        rand_answer: 0x7FFF,
        alloc_answers: [0x1000, 0x2000, 0x3000],
        alloc_next: 0,
        child1_answer: 0x4000,
        child2_answer: 0x5000,
        combine_answer: 0x6000,
        hashes: Vec::new(),
        firsts: Vec::new(),
        seconds: Vec::new(),
        combines: Vec::new(),
    }
}

#[test]
fn goto_pick_rolls_stamps_arms_and_combines() {
    let mut task = goto_task();
    let mut pick = picker();
    let ret = task.pick_target(goto_ped(0x60), 0x7777, 0.5, manager(0x11), &mut pick);
    assert_eq!(Handle32::raw_or_zero(ret), 0x6000);
    assert_eq!(pick.hashes, vec![0x1B, 0x1B]);
    assert_eq!(pick.firsts, vec![(0x1000, 2.5f32.to_bits(), 0.5f32.to_bits())]);
    assert_eq!(pick.seconds, vec![0x2000]);
    assert_eq!(pick.combines, vec![(0x3000, 0x4000, 0x5000)]);
    assert_eq!(task.wait(), task.wait_copy());
    assert_eq!(task.stamp(), 0x7777);
    assert!(task.armed());
    assert!(!task.restamp());
    // The same inputs roll the same wait.
    let mut again = goto_task();
    let mut pick2 = picker();
    pick2.hash_answers = pick.hash_answers;
    let _ = again.pick_target(goto_ped(0x60), 0x7777, 0.5, manager(0x11), &mut pick2);
    assert_eq!(again.wait(), task.wait());
}

#[test]
fn goto_pick_scales_the_wait_with_the_roll() {
    // A zero low half rolls the range start: wait equals first * 1000.
    let mut task = goto_task();
    let mut pick = picker();
    pick.hash_answers = [0x5555_5555, 0x5555_5555];
    pick.rand_answer = 0;
    let _ = task.pick_target(goto_ped(0x60), 0, 0.0, manager(0), &mut pick);
    let base = task.wait();
    // Equal hashes roll the same wait whatever the random word is.
    for rand in [0u32, 1, 0xFFFF, 0xFFFF_FFFF] {
        let mut task = goto_task();
        let mut pick = picker();
        pick.hash_answers = [0x5555_5555, 0x5555_5555];
        pick.rand_answer = rand;
        let _ = task.pick_target(goto_ped(0x60), 0, 0.0, manager(0), &mut pick);
        assert_eq!(task.wait(), base, "random word {rand:#x}");
    }
}

#[test]
fn goto_pick_null_blocks_yield_null_children() {
    let mut task = goto_task();
    let mut pick = picker();
    pick.alloc_answers = [0, 0, 0x3000];
    let ret = task.pick_target(goto_ped(0x60), 0, 0.0, manager(0), &mut pick);
    assert_eq!(Handle32::raw_or_zero(ret), 0x6000);
    assert!(pick.firsts.is_empty());
    assert!(pick.seconds.is_empty());
    assert_eq!(pick.combines, vec![(0x3000, 0, 0)]);
    // The wait is still rolled and the timer still arms.
    assert_eq!(task.wait(), task.wait_copy());
    assert!(task.armed());
}

#[test]
fn goto_pick_null_third_block_answers_null_after_building() {
    let mut task = goto_task();
    let mut pick = picker();
    pick.alloc_answers = [0x1000, 0x2000, 0];
    let ret = task.pick_target(goto_ped(0x60), 0x9999, 0.0, manager(0), &mut pick);
    assert_eq!(ret, None);
    assert_eq!(pick.firsts.len(), 1);
    assert_eq!(pick.seconds.len(), 1);
    assert!(pick.combines.is_empty());
    assert_eq!(task.stamp(), 0x9999);
    assert!(task.armed());
}

// The flee event handler.

fn probe(v: u32) -> Option<Handle32<FleeProbe>> {
    Handle32::new(v)
}

fn flee_target(v: u32) -> Option<Handle32<FleeTarget>> {
    Handle32::new(v)
}

fn react_ped() -> ReactPed {
    ReactPed::new(
        flee_ped(0x60).expect("host ped is live"),
        Some(0),
        probe(0x70),
        0,
        4,
        flee_target(0x80),
    )
}

fn event_task(kind: u32, state: u8) -> FleeTask {
    FleeTask::new(subtask(0xBEEF), 0, kind, [0.0; 3], true, entity(0xE0), state)
}

struct FleeEventScript {
    rand_answer: u32,
    kind_answer: u32,
    check_answer: u32,
    alloc_answer: u32,
    spawn_answer: u32,
    build_a_answer: Option<Reaction>,
    build_b_answer: Option<Reaction>,
    build_c_answer: u32,
    rands: u32,
    seeds: Vec<(u32, u32)>,
    builds_b: Vec<(u32, [u32; 3], u32)>,
}

impl FleeEvent for FleeEventScript {
    fn rand_word(&mut self) -> u32 {
        self.rands += 1;
        self.rand_answer
    }

    fn seed(&mut self, ped: Handle32<FleePed>, seed_arg: u32) {
        self.seeds.push((ped.get(), seed_arg));
    }

    fn entity_kind(&mut self, _entity: Handle32<FleeEntity>) -> u32 {
        self.kind_answer
    }

    fn check(
        &mut self,
        _probe: Option<Handle32<FleeProbe>>,
        _entity: Handle32<FleeEntity>,
    ) -> u32 {
        self.check_answer
    }

    fn alloc(&mut self, _manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>> {
        Handle32::new(self.alloc_answer)
    }

    fn spawn(
        &mut self,
        _block: Handle32<UninitTask>,
        _entity: Handle32<FleeEntity>,
    ) -> Option<Handle32<EventChild>> {
        Handle32::new(self.spawn_answer)
    }

    fn build_a(
        &mut self,
        _block: Handle32<UninitTask>,
        _entity: Handle32<FleeEntity>,
    ) -> Option<Reaction> {
        self.build_a_answer
    }

    fn build_b(
        &mut self,
        block: Handle32<UninitTask>,
        pos: [f32; 3],
        rate: u32,
    ) -> Option<Reaction> {
        self.builds_b.push((
            block.get(),
            [pos[0].to_bits(), pos[1].to_bits(), pos[2].to_bits()],
            rate,
        ));
        self.build_b_answer
    }

    fn build_c(
        &mut self,
        _block: Handle32<UninitTask>,
        _target: Handle32<FleeTarget>,
        _pos: [f32; 3],
    ) -> Option<Handle32<EventChild>> {
        Handle32::new(self.build_c_answer)
    }
}

fn eventer() -> FleeEventScript {
    FleeEventScript {
        rand_answer: 0,
        kind_answer: 0xC0,
        check_answer: 0,
        alloc_answer: 0x1000,
        spawn_answer: 0x4000,
        build_a_answer: reaction(0x5000, 0),
        build_b_answer: reaction(0x6000, 0),
        build_c_answer: 0x7000,
        rands: 0,
        seeds: Vec::new(),
        builds_b: Vec::new(),
    }
}

#[test]
fn flee_event_dead_gate_answers_null_without_calls() {
    let task = FleeTask::new(None, 0, 0x1B, [0.0; 3], false, None, 0);
    let mut event = eventer();
    let mut clock = 100;
    let ret = task.handle_event(Some(react_ped()), 0xA11CE, &mut clock, 200, 0, manager(1), &mut event);
    assert_eq!(ret, None);
    assert_eq!(event.rands, 0);
    assert!(event.seeds.is_empty());
    assert_eq!(clock, 100);
}

#[test]
fn flee_event_seeds_by_freshness_and_kind() {
    // Past the bound the seed is unconditional and draws nothing.
    let task = event_task(0x1B, 0);
    let mut event = eventer();
    let mut clock = 300;
    let _ = task.handle_event(Some(react_ped()), 0xA11CE, &mut clock, 200, 0, manager(1), &mut event);
    assert_eq!(event.seeds, vec![(0x60, 0xA11CE)]);
    // Inside the range the seed needs a won roll; a zero word wins.
    let task = event_task(0x15, 0);
    let mut event = eventer();
    event.rand_answer = 0;
    let mut clock = 300;
    let _ = task.handle_event(Some(react_ped()), 0xA11CE, &mut clock, 200, 0, manager(1), &mut event);
    assert_eq!(event.rands, 1);
    assert_eq!(event.seeds.len(), 1);
    // A huge word loses the roll.
    let task = event_task(0x15, 0);
    let mut event = eventer();
    event.rand_answer = 0x4000_0000;
    let mut clock = 300;
    let _ = task.handle_event(Some(react_ped()), 0xA11CE, &mut clock, 200, 0, manager(1), &mut event);
    assert_eq!(event.rands, 1);
    assert!(event.seeds.is_empty());
    // A stale entry never seeds, whatever the kind.
    let task = event_task(0x1B, 0);
    let mut event = eventer();
    let ped = ReactPed::new(
        flee_ped(0x60).expect("host ped is live"),
        Some(3),
        probe(0x70),
        0,
        4,
        flee_target(0x80),
    );
    let mut clock = 300;
    let _ = task.handle_event(Some(ped), 0xA11CE, &mut clock, 200, 0, manager(1), &mut event);
    assert_eq!(event.rands, 0);
    assert!(event.seeds.is_empty());
    // Below the range there is no draw at all.
    let task = event_task(0x14, 0);
    let mut event = eventer();
    let mut clock = 300;
    let _ = task.handle_event(Some(react_ped()), 0xA11CE, &mut clock, 200, 0, manager(1), &mut event);
    assert_eq!(event.rands, 0);
    assert!(event.seeds.is_empty());
}

#[test]
fn flee_event_spawn_refreshes_the_clock() {
    let task = event_task(0x1C, 0);
    let mut event = eventer();
    event.rand_answer = 0;
    let mut clock = 100;
    let ret = task.handle_event(Some(react_ped()), 0, &mut clock, 200, 0, manager(1), &mut event);
    assert!(matches!(ret, Some(FleeAnswer::Spawned(h)) if h.get() == 0x4000));
    assert_eq!(clock, 200 + 0x4E20);
}

#[test]
fn flee_event_lost_spawn_roll_builds_task_a() {
    let task = event_task(0x1C, 0);
    let mut event = eventer();
    event.rand_answer = 0x4000_0000;
    event.build_a_answer = reaction(0x5000, 0xFFFF_FFFF);
    let mut clock = 100;
    let ret = task.handle_event(Some(react_ped()), 0, &mut clock, 200, 0, manager(1), &mut event);
    match ret {
        Some(FleeAnswer::Built(r)) => {
            assert_eq!(r.task().get(), 0x5000);
            assert_eq!(r.marks(), 0xFFFF_FFEF);
        }
        other => panic!("expected task A, got {other:?}"),
    }
    assert_eq!(clock, 100);
}

#[test]
fn flee_event_task_a_clears_wide_for_low_kinds() {
    // Kind below the limit with the spawn flags off: the wide bit clears.
    let task = event_task(0x1A, 1);
    let mut event = eventer();
    event.build_a_answer = reaction(0x5000, 0xFFFF_FFFF);
    let ped = ReactPed::new(
        flee_ped(0x60).expect("host ped is live"),
        Some(0),
        probe(0x70),
        0xFF,
        0,
        flee_target(0x80),
    );
    let mut clock = 100;
    let ret = task.handle_event(Some(ped), 0, &mut clock, 200, 0, manager(1), &mut event);
    match ret {
        Some(FleeAnswer::Built(r)) => assert_eq!(r.marks(), 0xFFFF_FFFF & 0xFFFB_FFFF),
        other => panic!("expected task A, got {other:?}"),
    }
}

#[test]
fn flee_event_null_spawn_block_answers_null_after_refresh() {
    let task = event_task(0x1B, 0);
    let mut event = eventer();
    event.rand_answer = 0;
    event.alloc_answer = 0;
    let mut clock = 100;
    let ret = task.handle_event(Some(react_ped()), 0, &mut clock, 200, 0, manager(1), &mut event);
    assert_eq!(ret, None);
    assert_eq!(clock, 200 + 0x4E20);
}

#[test]
fn flee_event_fallback_routes_or_builds() {
    // A bad kind word falls back; a set flag bit routes the target.
    let task = event_task(0, 0);
    let mut event = eventer();
    event.kind_answer = 0;
    let ped = ReactPed::new(
        flee_ped(0x60).expect("host ped is live"),
        Some(9),
        probe(0x70),
        4,
        0,
        flee_target(0x80),
    );
    let mut clock = 100;
    let ret = task.handle_event(Some(ped), 0, &mut clock, 200, 0, manager(1), &mut event);
    assert!(matches!(ret, Some(FleeAnswer::Routed(h)) if h.get() == 0x7000));
    // A null target answers null without allocating.
    let ped = ReactPed::new(
        flee_ped(0x60).expect("host ped is live"),
        Some(9),
        probe(0x70),
        4,
        0,
        None,
    );
    let mut event = eventer();
    event.kind_answer = 0;
    event.alloc_answer = 0;
    let mut clock = 100;
    let ret = task.handle_event(Some(ped), 0, &mut clock, 200, 0, manager(1), &mut event);
    assert_eq!(ret, None);
    // A clear flag bit builds task B from the position and the rate.
    let task = FleeTask::new(subtask(0xBEEF), 0, 0, [1.0, 2.0, 3.0], true, entity(0xE0), 0);
    let mut event = eventer();
    event.kind_answer = 0;
    event.build_b_answer = reaction(0x6000, 0);
    let mut clock = 100;
    let ret = task.handle_event(Some(react_ped()), 0, &mut clock, 200, 0xBEEF, manager(1), &mut event);
    match ret {
        Some(FleeAnswer::Built(r)) => {
            assert_eq!(r.task().get(), 0x6000);
            assert_eq!(r.marks(), 8);
        }
        other => panic!("expected task B, got {other:?}"),
    }
    assert_eq!(
        event.builds_b,
        vec![(0x1000, [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()], 0xBEEF)]
    );
}

#[test]
#[should_panic(expected = "without a ped")]
fn flee_event_panics_without_a_ped() {
    let task = event_task(0x1B, 0);
    let mut event = eventer();
    let mut clock = 100;
    let _ = task.handle_event(None, 0, &mut clock, 200, 0, manager(1), &mut event);
}

#[test]
#[should_panic(expected = "without a table entry")]
fn flee_event_panics_without_a_table_entry() {
    let task = event_task(0x1B, 0);
    let mut event = eventer();
    let ped = ReactPed::new(
        flee_ped(0x60).expect("host ped is live"),
        None,
        probe(0x70),
        0,
        4,
        flee_target(0x80),
    );
    let mut clock = 100;
    let _ = task.handle_event(Some(ped), 0, &mut clock, 200, 0, manager(1), &mut event);
}

#[test]
#[should_panic(expected = "without a block")]
fn flee_event_panics_without_a_task_a_block() {
    // Stage two with the spawn kind out of range: task A with no block.
    let task = event_task(0x1E, 0);
    let mut event = eventer();
    event.alloc_answer = 0;
    let mut clock = 100;
    let _ = task.handle_event(Some(react_ped()), 0, &mut clock, 200, 0, manager(1), &mut event);
}

#[test]
#[should_panic(expected = "without a block")]
fn flee_event_panics_without_a_task_b_block() {
    // Fallback with a clear flag bit: task B with no block.
    let task = event_task(0, 0);
    let mut event = eventer();
    event.kind_answer = 0;
    event.alloc_answer = 0;
    let mut clock = 100;
    let _ = task.handle_event(Some(react_ped()), 0, &mut clock, 200, 0, manager(1), &mut event);
}

#[test]
#[should_panic(expected = "without a build")]
fn flee_event_panics_without_a_task_b_build() {
    let task = event_task(0, 0);
    let mut event = eventer();
    event.kind_answer = 0;
    event.build_b_answer = None;
    let mut clock = 100;
    let _ = task.handle_event(Some(react_ped()), 0, &mut clock, 200, 0, manager(1), &mut event);
}
