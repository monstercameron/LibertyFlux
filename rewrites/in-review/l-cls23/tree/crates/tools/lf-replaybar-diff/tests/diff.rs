//! Differential cases: each lifted method against its verified 32-bit rewrite.
//!
//! Every case generates trials from a seeded generator. A trial sets the
//! object's words, the inner object's flag byte, the owned slots, the
//! globals and the scripted answers of the collaborators. The rewrite runs
//! on one copy of the object and the lifted method on another; the two
//! must agree on the ordered collaborator calls with their arguments, the
//! returned word, the object's bytes and the globals. Each trial that the
//! original completes counts; the generator also covers the edge words.
//!
//! The wrong lifts in `mutants` run through the same trials and must each
//! be caught by at least one of them.

#![cfg(target_arch = "x86")]
// The cases read and write the object's words through the addresses the
// rewrites are given, in buffers the cases own.
#![allow(unsafe_code)]

use lf_core::Handle32;
use lf_input_frontend::replay_progress_bar::{
    BarTag, InnerTag, OWNED_SLOTS, OwnedTag, ReplayBarGlobals, ReplayProgressBar, UpdateOutcome,
};
use lf_replaybar_diff::fake::{
    Answer, Ctx, CallSpec, DELETING_CALLS, FakeWorld, Script, TEARDOWN_CALLS, UPDATE_CALLS,
    normalise,
};
use lf_replaybar_diff::mutants::{DeletingMutant, TeardownMutant, UpdateMutant};
use lf_replaybar_diff::{mutants, rewrites, rt};
use lf_replaybar_diff::rt::Kind;
use std::collections::VecDeque;

/// Byte size of the object the cases build (the rewrites touch words up to
/// offset `0x104`).
const BAR_BYTES: usize = 0x110;
/// Byte offset of the inner object pointer.
const INNER_WORD: usize = 0x04;
/// Byte offset of the first owned collaborator slot.
const OWNED_WORD: usize = 0x88;
/// Byte offset of the update's mark word.
const MARK_WORD: usize = 0xA4;
/// Byte offset of the update's total word.
const TOTAL_WORD: usize = 0x100;
/// Byte offset of the update's phase word.
const PHASE_WORD: usize = 0x104;
/// Byte size of the inner object the cases build.
const INNER_BYTES: usize = 0x40;
/// Byte offset of the inner object's flag byte.
const INNER_FLAG_BYTE: usize = 0x1B;

/// Trials per case. Each trial is a few microseconds; the bound is fixed.
const TRIALS: usize = 3000;

/// Words that exercise the sign boundaries, zero, one and all ones.
const EDGES: [u32; 10] = [
    0,
    1,
    2,
    0x7FFF_FFFF,
    0x8000_0000,
    0x8000_0001,
    0xFFFF_FFFF,
    0xFFFF_FFFE,
    0x0000_FFFF,
    0x0001_0000,
];

/// A small seeded generator (SplitMix64), so every run sees the same trials.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn word(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// An edge word in two of five draws, otherwise a random word.
    fn pick_word(&mut self) -> u32 {
        if self.next_u64() % 5 < 2 {
            EDGES[(self.next_u64() % EDGES.len() as u64) as usize]
        } else {
            self.word()
        }
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }
}

/// The generated inputs of one trial, before either side runs.
struct Trial {
    /// The object's words.
    words: Vec<u32>,
    /// The inner object's flag byte is set.
    inner_flag: bool,
    /// The owned collaborator slots (zero when empty).
    owned: [u32; OWNED_SLOTS],
    /// The globals: mode word, flag word (its low byte is read), gate word
    /// (its low byte is read), or-word and save-word.
    mode: u32,
    flag_word: u32,
    gate_word: u32,
    or_word: u32,
    save_word: u32,
    /// The scripted answers, by call name.
    script: Script,
    /// The flags word the deleting destructor receives.
    flags: u32,
}

fn push(script: &mut Script, name: &'static str, answers: Vec<Answer>) {
    script.insert(name, VecDeque::from(answers));
}

fn answer(rng: &mut Rng) -> Answer {
    Answer {
        ret: rng.pick_word(),
        outs: [rng.pick_word(), rng.pick_word(), rng.pick_word()],
    }
}

/// Generates the trial with index `index`. Inputs are drawn from the same
/// generator in the same order, so a trial is reproducible from its index.
fn make_trial(rng: &mut Rng) -> Trial {
    // Every word starts random, so words the object does not model carry
    // values that both sides must leave alone.
    let mut words: Vec<u32> = (0..BAR_BYTES / 4).map(|_| rng.word()).collect();
    let phase = match rng.below(6) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 0xFFFF_FFFF,
        _ => rng.word(),
    };
    words[PHASE_WORD / 4] = phase;
    words[MARK_WORD / 4] = rng.word();
    words[TOTAL_WORD / 4] = rng.word();
    let mut owned = [0u32; OWNED_SLOTS];
    for (index, slot) in owned.iter_mut().enumerate() {
        *slot = if rng.below(3) == 0 {
            0
        } else {
            rng.word() | 0xC000_0000 | (index as u32) << 8
        };
    }
    let mode = match rng.below(6) {
        0 => 0,
        1 => 2,
        2 => 7,
        3 => 6,
        4 => 1,
        _ => rng.word(),
    };
    let flag_low = match rng.below(3) {
        0 => 0,
        1 => 1,
        _ => rng.word() as u8,
    };
    let gate_low = match rng.below(3) {
        0 => 0,
        1 => 1,
        _ => rng.word() as u8,
    };
    let flag_word = (rng.word() & 0xFFFF_FF00) | u32::from(flag_low);
    let gate_word = (rng.word() & 0xFFFF_FF00) | u32::from(gate_low);
    let mut script = Script::new();
    push(
        &mut script,
        "open_token",
        vec![Answer {
            ret: rng.word(),
            outs: [0; 3],
        }],
    );
    let produced: Vec<Answer> = (0..4).map(|_| answer(rng)).collect();
    // The compare word is often the first produced word, so the compare
    // lands on the boundary between below and not below.
    let boundary = produced[0].outs[1];
    let compare: Vec<Answer> = (0..3)
        .map(|_| {
            let ret = match rng.below(4) {
                0 => boundary,
                _ => rng.pick_word(),
            };
            Answer {
                ret,
                outs: [0; 3],
            }
        })
        .collect();
    push(&mut script, "produce", produced);
    push(
        &mut script,
        "fill_pair",
        (0..2).map(|_| answer(rng)).collect(),
    );
    push(&mut script, "measure", (0..2).map(|_| answer(rng)).collect());
    push(&mut script, "use_b", (0..4).map(|_| answer(rng)).collect());
    push(&mut script, "compare", compare);
    push(
        &mut script,
        "base_update",
        (0..2).map(|_| answer(rng)).collect(),
    );
    Trial {
        words,
        inner_flag: rng.below(2) == 1,
        owned,
        mode,
        flag_word,
        gate_word,
        or_word: rng.word(),
        save_word: rng.word(),
        script,
        flags: rng.word(),
    }
}

/// A test object the rewrite and the lift each get a copy of.
struct Scene {
    /// The object's words before either side runs.
    initial: Vec<u32>,
    bar: Vec<u32>,
    inner: Vec<u8>,
}

impl Scene {
    fn new(trial: &Trial) -> Self {
        let mut inner = vec![0u8; INNER_BYTES];
        inner[INNER_FLAG_BYTE] = if trial.inner_flag { 0x80 } else { 0 };
        let mut bar = trial.words.clone();
        // The object's inner pointer word names the inner object, which the
        // rewrite and the lifted method both read from the object's bytes.
        bar[INNER_WORD / 4] = inner.as_ptr() as usize as u32;
        for (slot, value) in trial.owned.iter().enumerate() {
            bar[OWNED_WORD / 4 + slot] = *value;
        }
        Self {
            initial: bar.clone(),
            bar,
            inner,
        }
    }

    fn bar_addr(&self) -> u32 {
        self.bar.as_ptr() as usize as u32
    }

    fn inner_addr(&self) -> u32 {
        self.inner.as_ptr() as usize as u32
    }

    fn ctx(&self, trial: &Trial) -> Ctx {
        let bar = self.bar_addr();
        Ctx {
            bar,
            sub1c: bar.wrapping_add(0x1C),
            inner: self.inner_addr(),
            owned: trial.owned,
            token_cell: rt::token_cell_addr(),
        }
    }
}

/// Decodes the object's modelled words into the lifted object.
fn decode(words: &[u32], bar_addr: u32, inner_addr: u32) -> ReplayProgressBar {
    let mut owned = [None; OWNED_SLOTS];
    for (slot, value) in owned.iter_mut().enumerate() {
        *value = Handle32::<OwnedTag>::new(words[OWNED_WORD / 4 + slot]);
    }
    ReplayProgressBar {
        handle: Handle32::<BarTag>::new(bar_addr).expect("the object has an address"),
        inner: Handle32::<InnerTag>::new(inner_addr).expect("the inner object has an address"),
        owned,
        mark: words[MARK_WORD / 4],
        total: words[TOTAL_WORD / 4],
        phase: words[PHASE_WORD / 4],
        torn: false,
    }
}

/// Writes the lifted object's modelled words back over the object's bytes.
/// The vtable word changes only when the object was torn down.
fn encode(bar: &ReplayProgressBar, words: &mut [u32]) {
    for (slot, value) in bar.owned.iter().enumerate() {
        words[OWNED_WORD / 4 + slot] = Handle32::raw_or_zero(*value);
    }
    words[MARK_WORD / 4] = bar.mark;
    words[TOTAL_WORD / 4] = bar.total;
    words[PHASE_WORD / 4] = bar.phase;
    if bar.torn {
        words[0] = rt::dead_cell_addr();
    }
}

fn teardown_table() -> Vec<(CallSpec, Kind)> {
    let kinds = [
        Kind::T1,
        Kind::C1,
        Kind::C1,
        Kind::T1,
        Kind::C1,
        Kind::C1,
        Kind::C0,
        Kind::C0,
    ];
    TEARDOWN_CALLS
        .iter()
        .zip(kinds)
        .map(|(spec, kind)| (*spec, kind))
        .collect()
}

fn update_table() -> Vec<(CallSpec, Kind)> {
    let kinds = [
        Kind::T2,
        Kind::T4,
        Kind::C2,
        Kind::T4,
        Kind::C0,
        Kind::C1,
        Kind::T3,
        Kind::C1,
        Kind::C2,
        Kind::C1,
        Kind::T3,
        Kind::T1,
        Kind::C0,
        Kind::C1,
        Kind::T2,
        Kind::T1,
        Kind::T1,
        Kind::T1,
        Kind::T1,
        Kind::T1,
    ];
    UPDATE_CALLS
        .iter()
        .zip(kinds)
        .map(|(spec, kind)| (*spec, kind))
        .collect()
}

/// Runs the verified teardown on the object; returns the record and the
/// object's words afterwards.
fn run_teardown_rewrite(scene: &mut Scene, trial: &Trial, ctx: &Ctx) -> Vec<String> {
    rt::install(0, &teardown_table(), &trial.script);
    rt::begin(0);
    let _ = rewrites::rw_00d6bcf0(scene.bar_addr());
    let log = rt::take_log();
    log.iter()
        .map(|(_, id, args)| normalise(ctx, TEARDOWN_CALLS, &[(*id, args.clone())]).remove(0))
        .collect()
}

#[test]
fn teardown_matches_its_rewrite() {
    let _guard = rt::case_lock();
    let mut rng = Rng(0x7EA2_D0F0_0001);
    let mut torn_with_owned = 0usize;
    for index in 0..TRIALS {
        let trial = make_trial(&mut rng);
        let mut scene = Scene::new(&trial);
        let ctx = scene.ctx(&trial);
        let rewrite_events = run_teardown_rewrite(&mut scene, &trial, &ctx);

        let mut lifted = scene.initial.clone();
        let mut bar = decode(&lifted, scene.bar_addr(), scene.inner_addr());
        if bar.owned.iter().any(Option::is_some) {
            torn_with_owned += 1;
        }
        let mut world = FakeWorld::new(ctx, trial.inner_flag, trial.script.clone());
        bar.teardown(&mut world);
        encode(&bar, &mut lifted);

        assert_eq!(world.events, rewrite_events, "trial {index}: calls differ");
        assert_eq!(lifted, scene.bar, "trial {index}: object bytes differ");
    }
    assert!(torn_with_owned > TRIALS / 2, "owned slots are exercised");
}

#[test]
fn deleting_destructor_matches_its_rewrite() {
    let _guard = rt::case_lock();
    let mut rng = Rng(0xDE1E_7E00_0002);
    let mut freed = 0usize;
    for index in 0..TRIALS {
        let trial = make_trial(&mut rng);
        let mut scene = Scene::new(&trial);
        let ctx = scene.ctx(&trial);
        let nested = CallSpec {
            id: 1,
            name: "nested_teardown",
            outs: &[],
        };
        let outer: Vec<(CallSpec, Kind)> =
            vec![(nested, Kind::NestedTeardown), (DELETING_CALLS[0], Kind::C1)];
        rt::install(0, &outer, &trial.script);
        rt::install(1, &teardown_table(), &trial.script);
        rt::begin(0);
        let eax = rewrites::rw_00d6be10(scene.bar_addr(), trial.flags);
        let log = rt::take_log();
        let rewrite_events: Vec<String> = log
            .iter()
            .map(|(scope, id, args)| {
                let table: &[CallSpec] = if *scope == 0 {
                    DELETING_CALLS
                } else {
                    TEARDOWN_CALLS
                };
                normalise(&ctx, table, &[(*id, args.clone())]).remove(0)
            })
            .collect();
        if trial.flags & 1 != 0 {
            freed += 1;
        }

        let mut lifted = scene.initial.clone();
        let mut bar = decode(&lifted, scene.bar_addr(), scene.inner_addr());
        let mut world = FakeWorld::new(ctx, trial.inner_flag, trial.script.clone());
        let returned = bar.deleting_destructor(&mut world, trial.flags);
        encode(&bar, &mut lifted);

        assert_eq!(world.events, rewrite_events, "trial {index}: calls differ");
        assert_eq!(returned.get(), eax, "trial {index}: returned object differs");
        assert_eq!(lifted, scene.bar, "trial {index}: object bytes differ");
    }
    assert!(freed > TRIALS / 3, "the storage release is exercised");
}

#[test]
fn update_matches_its_rewrite() {
    let _guard = rt::case_lock();
    let mut rng = Rng(0x0BDA_7E00_0003);
    let mut coverage = [0usize; 6];
    for index in 0..TRIALS {
        let trial = make_trial(&mut rng);
        let mut scene = Scene::new(&trial);
        let ctx = scene.ctx(&trial);
        rt::install(0, &update_table(), &trial.script);
        rt::begin(0);
        rt::set_globals(
            trial.mode,
            trial.flag_word,
            trial.gate_word,
            trial.or_word,
            trial.save_word,
        );
        let eax = rewrites::rw_00d70950(scene.bar_addr());
        let log = rt::take_log();
        let rewrite_events: Vec<String> = log
            .iter()
            .map(|(_, id, args)| normalise(&ctx, UPDATE_CALLS, &[(*id, args.clone())]).remove(0))
            .collect();
        let (or_cell, save_cell) = rt::read_or_save();

        let mut globals = ReplayBarGlobals {
            mode: trial.mode,
            flag: trial.flag_word as u8,
            gate: trial.gate_word as u8,
            or_word: trial.or_word,
            save_word: trial.save_word,
        };
        let mut lifted = scene.initial.clone();
        let mut bar = decode(&lifted, scene.bar_addr(), scene.inner_addr());
        let mut world = FakeWorld::new(ctx, trial.inner_flag, trial.script.clone());
        let outcome = bar.update(&mut world, &mut globals);
        encode(&bar, &mut lifted);

        let lifted_word = match outcome {
            UpdateOutcome::Inner(inner) => inner.get(),
            UpdateOutcome::Base(word) => word,
        };
        if rewrite_events.iter().any(|e| e.starts_with("signal")) {
            coverage[0] += 1;
        }
        if rewrite_events.iter().any(|e| e.starts_with("use_e")) {
            coverage[1] += 1;
        }
        if rewrite_events.iter().any(|e| e.starts_with("gate_c")) {
            coverage[2] += 1;
        }
        if matches!(outcome, UpdateOutcome::Inner(_)) {
            coverage[3] += 1;
        }
        if matches!(outcome, UpdateOutcome::Base(_)) {
            coverage[4] += 1;
        }
        if rewrite_events.iter().any(|e| e.starts_with("gate_a")) {
            coverage[5] += 1;
        }

        assert_eq!(world.events, rewrite_events, "trial {index}: calls differ");
        assert_eq!(lifted_word, eax, "trial {index}: result differs");
        assert_eq!(globals.or_word, or_cell, "trial {index}: or-word differs");
        assert_eq!(globals.save_word, save_cell, "trial {index}: saved word differs");
        assert_eq!(lifted, scene.bar, "trial {index}: object bytes differ");
    }
    for (name, count) in ["signal", "second phase", "gate_c", "inner result", "base result", "gate_a"]
        .iter()
        .zip(coverage)
    {
        assert!(count > 0, "coverage: {name} is never reached");
    }
}

/// Runs a wrong teardown through the same trials and counts the trials it
/// fails.
fn teardown_mutant_failures(mutant: TeardownMutant) -> usize {
    let mut rng = Rng(0x7EA2_D0F0_0001);
    let mut failures = 0;
    for _ in 0..TRIALS {
        let trial = make_trial(&mut rng);
        let mut scene = Scene::new(&trial);
        let ctx = scene.ctx(&trial);
        let rewrite_events = run_teardown_rewrite(&mut scene, &trial, &ctx);
        let mut lifted = scene.initial.clone();
        let mut bar = decode(&lifted, scene.bar_addr(), scene.inner_addr());
        let mut world = FakeWorld::new(ctx, trial.inner_flag, trial.script.clone());
        mutants::teardown(&mut bar, &mut world, mutant);
        encode(&bar, &mut lifted);
        if world.events != rewrite_events || lifted != scene.bar {
            failures += 1;
        }
    }
    failures
}

fn deleting_mutant_failures(mutant: DeletingMutant) -> usize {
    let mut rng = Rng(0xDE1E_7E00_0002);
    let mut failures = 0;
    for _ in 0..TRIALS {
        let trial = make_trial(&mut rng);
        let mut scene = Scene::new(&trial);
        let ctx = scene.ctx(&trial);
        let nested = CallSpec {
            id: 1,
            name: "nested_teardown",
            outs: &[],
        };
        let outer: Vec<(CallSpec, Kind)> =
            vec![(nested, Kind::NestedTeardown), (DELETING_CALLS[0], Kind::C1)];
        rt::install(0, &outer, &trial.script);
        rt::install(1, &teardown_table(), &trial.script);
        rt::begin(0);
        let eax = rewrites::rw_00d6be10(scene.bar_addr(), trial.flags);
        let rewrite_events: Vec<String> = rt::take_log()
            .iter()
            .map(|(scope, id, args)| {
                let table: &[CallSpec] = if *scope == 0 {
                    DELETING_CALLS
                } else {
                    TEARDOWN_CALLS
                };
                normalise(&ctx, table, &[(*id, args.clone())]).remove(0)
            })
            .collect();
        let mut lifted = scene.initial.clone();
        let mut bar = decode(&lifted, scene.bar_addr(), scene.inner_addr());
        let mut world = FakeWorld::new(ctx, trial.inner_flag, trial.script.clone());
        let returned = mutants::deleting(&mut bar, &mut world, trial.flags, mutant);
        encode(&bar, &mut lifted);
        if world.events != rewrite_events || returned.get() != eax || lifted != scene.bar {
            failures += 1;
        }
    }
    failures
}

fn update_mutant_failures(mutant: UpdateMutant) -> usize {
    let mut rng = Rng(0x0BDA_7E00_0003);
    let mut failures = 0;
    for _ in 0..TRIALS {
        let trial = make_trial(&mut rng);
        let mut scene = Scene::new(&trial);
        let ctx = scene.ctx(&trial);
        rt::install(0, &update_table(), &trial.script);
        rt::begin(0);
        rt::set_globals(
            trial.mode,
            trial.flag_word,
            trial.gate_word,
            trial.or_word,
            trial.save_word,
        );
        let eax = rewrites::rw_00d70950(scene.bar_addr());
        let rewrite_events: Vec<String> = rt::take_log()
            .iter()
            .map(|(_, id, args)| normalise(&ctx, UPDATE_CALLS, &[(*id, args.clone())]).remove(0))
            .collect();
        let (or_cell, save_cell) = rt::read_or_save();
        let mut globals = ReplayBarGlobals {
            mode: trial.mode,
            flag: trial.flag_word as u8,
            gate: trial.gate_word as u8,
            or_word: trial.or_word,
            save_word: trial.save_word,
        };
        let mut lifted = scene.initial.clone();
        let mut bar = decode(&lifted, scene.bar_addr(), scene.inner_addr());
        let mut world = FakeWorld::new(ctx, trial.inner_flag, trial.script.clone());
        let outcome = mutants::update(&mut bar, &mut world, &mut globals, mutant);
        encode(&bar, &mut lifted);
        let word = match outcome {
            UpdateOutcome::Inner(inner) => inner.get(),
            UpdateOutcome::Base(word) => word,
        };
        if world.events != rewrite_events
            || word != eax
            || globals.or_word != or_cell
            || globals.save_word != save_cell
            || lifted != scene.bar
        {
            failures += 1;
        }
    }
    failures
}

#[test]
fn teardown_wrong_lifts_are_caught() {
    let _guard = rt::case_lock();
    for mutant in [
        TeardownMutant::SkipFinalizeB,
        TeardownMutant::KeepFirstSlots,
        TeardownMutant::AttachZeroToken,
    ] {
        assert!(
            teardown_mutant_failures(mutant) > 0,
            "teardown mutant {mutant:?} passes every trial"
        );
    }
}

#[test]
fn deleting_wrong_lifts_are_caught() {
    let _guard = rt::case_lock();
    for mutant in [DeletingMutant::FreeOnBitOne, DeletingMutant::ReturnInner] {
        assert!(
            deleting_mutant_failures(mutant) > 0,
            "deleting mutant {mutant:?} passes every trial"
        );
    }
}

#[test]
fn update_wrong_lifts_are_caught() {
    let _guard = rt::case_lock();
    for mutant in [
        UpdateMutant::SecondPhaseAlways,
        UpdateMutant::ConsumeSwapped,
        UpdateMutant::TotalFromFirst,
        UpdateMutant::SignedCompare,
        UpdateMutant::CompareAtMost,
        UpdateMutant::LongSignalMode6,
        UpdateMutant::GateInverted,
        UpdateMutant::NoOrBit,
    ] {
        assert!(
            update_mutant_failures(mutant) > 0,
            "update mutant {mutant:?} passes every trial"
        );
    }
}

