//! Differential cases: the lifted dispatcher against its verified 32-bit rewrite.
//!
//! Every case generates trials from a seeded generator. A trial plants the
//! container object, its collaborators, their vtables and the state object,
//! scripts every collaborator answer, and runs the rewrite on the planted
//! object and the lifted dispatcher on a decoded copy. The two must agree
//! on the ordered collaborator calls with their arguments, the returned
//! word, and every byte of the object. Trials cycle through the eight
//! dispatch paths so each region, toggle and exit is exercised.
//!
//! The wrong dispatchers in `mutants` run through the same trials and must
//! each be caught by at least one of them.

#![cfg(target_arch = "x86")]
// The cases plant objects at real addresses for the rewrite to run on, in
// buffers the cases own.
#![allow(unsafe_code)]

use std::collections::VecDeque;

use lf_core::Handle32;
use lf_input_frontend::montage_container::{
    AuxTag, CfgTag, ListTag, MontageContainer, MontageGlobals, OutTag, OwnTag, SrcTag, ViewTag,
};
use lf_montage_diff::fake::{
    CallSpec, Ctx, DISPATCH_CALLS, DISPATCH_VIRTUALS, FakeWorld, Script, normalise,
};
use lf_montage_diff::mutants::{self, DispatchMutant};
use lf_montage_diff::rt::{self, Kind, VStubs};
use lf_montage_diff::rewrites;

/// Words of the container object the cases build (the rewrite touches
/// bytes up to offset `0x21c`).
const OBJ_WORDS: usize = 0x230 / 4;
/// Word offsets of the collaborator handles.
const LIST_WORD: usize = 0x1E0 / 4;
/// Word offsets of the collaborator handles.
const AUX_WORD: usize = 0x1E8 / 4;
/// Word offsets of the collaborator handles.
const CFG_WORD: usize = 0x1EC / 4;
/// Word offsets of the collaborator handles.
const OUT_WORD: usize = 0x1F0 / 4;
/// Word offsets of the collaborator handles.
const VIEW_WORD: usize = 0x1F4 / 4;
/// Word offsets of the collaborator handles.
const SRC_WORD: usize = 0x1F8 / 4;
/// Word offset of the wanted index.
const WANT_WORD: usize = 0x204 / 4;
/// Word offset of the dropped index.
const DROPPED_WORD: usize = 0x20C / 4;
/// Word offset of the pending index.
const PENDING_WORD: usize = 0x210 / 4;
/// Word offset of the cursor index.
const CURSOR_WORD: usize = 0x214 / 4;
/// Word holding the mode byte (low byte).
const MODE_WORD: usize = 0x218 / 4;
/// Word holding the phase byte (low byte).
const PHASE_WORD: usize = 0x21C / 4;

/// Trials per dispatch path. Each trial is small and bounded; the total is
/// fixed.
const TRIALS_PER_PATH: usize = 300;

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

/// Blend bit patterns: zero, one, quiet NaN payloads, infinities,
/// subnormals and a negative value.
const BLENDS: [u32; 8] = [
    0x0000_0000,
    0x3F80_0000,
    0x7FC0_0001,
    0xFFC0_00BE,
    0x7F80_0000,
    0xFF80_0000,
    0x0000_0001,
    0xC120_0000,
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

/// The dispatch path a trial forces through the identity gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Path {
    /// A zero state flag: clear and return.
    FlagZero,
    /// The first gate matches: the list walk.
    List,
    /// The second gate matches with the mode byte set: the toggled refresh.
    RefreshToggle,
    /// The second gate matches with the mode byte clear: the probe refresh.
    RefreshProbe,
    /// The second gate matches and the freshness threshold trips.
    Threshold,
    /// The third gate matches and the pending entry drops.
    ReconcileDrop,
    /// The third gate matches and the container re-arms.
    ReconcileRearm,
    /// No gate matches: the last wanted word is the result.
    Fallthrough,
}

const PATHS: &[Path] = &[
    Path::FlagZero,
    Path::List,
    Path::RefreshToggle,
    Path::RefreshProbe,
    Path::Threshold,
    Path::ReconcileDrop,
    Path::ReconcileRearm,
    Path::Fallthrough,
];

/// The generated inputs of one trial, before either side runs. Address
/// answers are patched in after the scene plants its objects.
struct Trial {
    /// The object's words, with handles and directed words planted.
    words: Vec<u32>,
    /// The mode byte.
    mode: u8,
    /// The phase byte.
    phase: u8,
    /// The state flag.
    state_flag: u32,
    /// The scripted answers, by call name.
    script: Script,
    /// The forced path.
    path: Path,
}

fn push(script: &mut Script, name: &'static str, answers: Vec<u32>) {
    script.insert(name, VecDeque::from(answers));
}

/// A word guaranteed to differ from `probe`.
fn differ(rng: &mut Rng, probe: u32) -> u32 {
    let flip = probe ^ 1;
    if rng.below(2) == 0 { flip } else { rng.pick_word() ^ 1 }
}

/// Generates the trial for `path`. Gate answers force the path; every
/// other input mixes edge values with seeded random words.
fn make_trial(rng: &mut Rng, path: Path) -> Trial {
    let mut words: Vec<u32> = (0..OBJ_WORDS).map(|_| rng.word()).collect();
    let mut script = Script::new();
    let mode: u8;
    let phase: u8 = (rng.word() & 0xFF) as u8;
    let state_flag: u32;
    match path {
        Path::FlagZero => {
            state_flag = 0;
            mode = rng.word() as u8;
            // Nonzero cursor and pending words, so the clearing is seen.
            words[CURSOR_WORD] = rng.word() | 1;
            words[PENDING_WORD] = rng.word() | 1;
        }
        Path::List => {
            state_flag = rng.word() | 1;
            mode = rng.word() as u8;
            let probe = rng.pick_word();
            push(&mut script, "session_tag", vec![probe]);
            push(&mut script, "tag_id", vec![probe]);
            // The walk re-reads the count after every element; counts stay
            // at most four so the walk ends within four elements and the
            // rewrite never reads past the scripted element answers.
            // Half the trials hold the count steady, half vary it.
            let steady = rng.below(2) == 0;
            let first = (rng.below(5)) as u32;
            let mut counts = vec![first];
            for _ in 0..7 {
                counts.push(if steady { first } else { rng.below(5) as u32 });
            }
            push(&mut script, "list_count", counts);
        }
        Path::RefreshToggle | Path::RefreshProbe | Path::Threshold => {
            state_flag = rng.word() | 1;
            mode = match path {
                Path::RefreshToggle => (rng.word() as u8) | 1,
                _ => 0,
            };
            // Non-tripping freshness mixes small values with words whose
            // high bit is set (negative when signed); tripping trials use
            // the limit and above.
            let freshness = match path {
                Path::Threshold => match rng.below(4) {
                    0 => 100,
                    1 => 101,
                    2 => 0x7FFF_FFFF,
                    _ => 500 + (rng.word() % 1000),
                },
                _ => match rng.below(6) {
                    0 => 0,
                    1 => 1,
                    2 => 99,
                    3 => 0xFFFF_FFFF,
                    4 => 0x8000_0000,
                    _ => rng.word() & 0x7FFF_FFFF,
                },
            };
            push(&mut script, "freshness", vec![freshness]);
            let probe1 = rng.pick_word();
            let probe2 = rng.pick_word();
            push(&mut script, "session_tag", vec![probe1, probe2]);
            push(&mut script, "tag_id", vec![differ(rng, probe1), probe2, rng.pick_word()]);
            push(&mut script, "blend_level", vec![BLENDS[rng.below(8) as usize]]);
            push(&mut script, "session_first", vec![rng.pick_word(), rng.pick_word()]);
            push(&mut script, "list_rank", vec![rng.pick_word(), rng.pick_word()]);
            push(&mut script, "prop_read", vec![rng.pick_word(), rng.pick_word()]);
            push(&mut script, "release_output", vec![rng.pick_word()]);
        }
        Path::ReconcileDrop | Path::ReconcileRearm => {
            state_flag = rng.word() | 1;
            mode = rng.word() as u8;
            let probe1 = rng.pick_word();
            let probe2 = rng.pick_word();
            let probe3 = rng.pick_word();
            push(&mut script, "session_tag", vec![probe1, probe2, probe3]);
            push(
                &mut script,
                "tag_id",
                vec![differ(rng, probe1), differ(rng, probe2), probe3],
            );
            match path {
                Path::ReconcileDrop => {
                    // The pending entry is nonzero, at least one index
                    // misses, and cursor and wanted are not adjacent.
                    let count = rng.below(6) as u32;
                    let cursor = count.wrapping_add(5 + rng.below(10) as u32);
                    let want = cursor.wrapping_add(10 + rng.below(10) as u32);
                    let (count_a, count_b) = if rng.below(2) == 0 {
                        (count, count)
                    } else {
                        // Exactly one read misses: the cursor matches the
                        // first read while the wanted index misses the
                        // second.
                        (cursor.wrapping_add(1), count)
                    };
                    push(&mut script, "list_count", vec![count_a, count_b]);
                    words[CURSOR_WORD] = cursor;
                    words[WANT_WORD] = want;
                    words[PENDING_WORD] = rng.word() | 1;
                    words[DROPPED_WORD] = rng.pick_word();
                }
                _ => {
                    // Three re-arm shapes: no pending entry, an adjacent
                    // cursor with a pending entry (including the wrapping
                    // case), or matching indices with a pending entry.
                    match rng.below(3) {
                        0 => {
                            push(
                                &mut script,
                                "list_count",
                                vec![rng.pick_word(), rng.pick_word()],
                            );
                            words[CURSOR_WORD] = rng.pick_word();
                            words[WANT_WORD] = rng.pick_word();
                            words[PENDING_WORD] = 0;
                        }
                        1 => {
                            // Adjacent: equal, one ahead, or across the wrap.
                            let (cursor, want) = if rng.below(5) == 0 {
                                (0xFFFF_FFFF, 0)
                            } else {
                                let cursor = rng.pick_word();
                                if rng.below(2) == 0 {
                                    (cursor, cursor)
                                } else {
                                    (cursor, cursor.wrapping_add(1))
                                }
                            };
                            let count = rng.below(6) as u32;
                            push(&mut script, "list_count", vec![count, count]);
                            words[CURSOR_WORD] = cursor;
                            words[WANT_WORD] = want;
                            words[PENDING_WORD] = rng.word() | 1;
                        }
                        _ => {
                            let count_b = 20 + rng.below(50) as u32;
                            // The cursor matches one less than the first
                            // read and the wanted index matches the second.
                            let cursor = count_b.wrapping_sub(11);
                            let count_a = cursor.wrapping_add(1);
                            push(&mut script, "list_count", vec![count_a, count_b]);
                            words[CURSOR_WORD] = cursor;
                            words[WANT_WORD] = count_b;
                            words[PENDING_WORD] = rng.word() | 1;
                        }
                    }
                    words[DROPPED_WORD] = rng.pick_word();
                    push(&mut script, "own_rearm", vec![rng.pick_word()]);
                }
            }
        }
        Path::Fallthrough => {
            state_flag = rng.word() | 1;
            mode = rng.word() as u8;
            let probe1 = rng.pick_word();
            let probe2 = rng.pick_word();
            let probe3 = rng.pick_word();
            push(&mut script, "session_tag", vec![probe1, probe2, probe3]);
            push(
                &mut script,
                "tag_id",
                vec![differ(rng, probe1), differ(rng, probe2), differ(rng, probe3)],
            );
        }
    }
    words[WANT_WORD] = if path == Path::ReconcileDrop || path == Path::ReconcileRearm {
        words[WANT_WORD]
    } else {
        rng.pick_word()
    };
    Trial { words, mode, phase, state_flag, script, path }
}

/// A planted word buffer the rewrite reads and writes through.
struct Block {
    words: Vec<u32>,
}

impl Block {
    fn filled(size_words: usize, rng: &mut Rng) -> Self {
        Self { words: (0..size_words).map(|_| rng.word()).collect() }
    }

    fn addr(&self) -> u32 {
        self.words.as_ptr() as usize as u32
    }
}

/// The planted scene: the container, every collaborator, their vtables and
/// the state object, kept alive together so addresses stay valid.
#[allow(dead_code)]
struct Scene {
    obj: Block,
    obj_vt: Block,
    list: Block,
    list_vt: Block,
    sess: Block,
    sess_vt: Block,
    sess2: Block,
    aux: Block,
    aux_vt: Block,
    tail: Block,
    tail_vt: Block,
    inner: Block,
    inner_vt: Block,
    probes: [Block; 2],
    probe_vt: Block,
    view: Block,
    view_vt: Block,
    cfg: Block,
    cfgsub: Block,
    cfgsub_vt: Block,
    src: Block,
    out: Block,
    elems: [Block; 4],
    actives: [Block; 2],
    target: Block,
    state: Block,
    stubs: VStubs,
}

impl Scene {
    #[allow(clippy::too_many_lines)]
    fn build(rng: &mut Rng, trial: &mut Trial) -> Self {
        let stubs = VStubs::new();
        let mut obj_vt = Block::filled(0x1C4 / 4, rng);
        obj_vt.words[0x28 / 4] = stubs.own_rearm;
        obj_vt.words[0x1C0 / 4] = stubs.own_open;
        let mut list_vt = Block::filled(0x1E8 / 4, rng);
        list_vt.words[0x18 / 4] = stubs.list_mode;
        list_vt.words[0x1D4 / 4] = stubs.list_count;
        list_vt.words[0x1E0 / 4] = stubs.list_at;
        list_vt.words[0x1E4 / 4] = stubs.list_rank;
        let mut sess_vt = Block::filled(0x50 / 4, rng);
        sess_vt.words[0] = stubs.session_tag;
        sess_vt.words[0x18 / 4] = stubs.session_mode;
        sess_vt.words[0x4C / 4] = stubs.session_first;
        let mut aux_vt = Block::filled(0x124 / 4, rng);
        aux_vt.words[0x120 / 4] = stubs.aux_set;
        let mut tail_vt = Block::filled(0x124 / 4, rng);
        tail_vt.words[0x120 / 4] = stubs.tail_set;
        let mut inner_vt = Block::filled(0x224 / 4, rng);
        inner_vt.words[0x21C / 4] = stubs.prop_read;
        inner_vt.words[0x220 / 4] = stubs.probe_open;
        let mut probe_vt = Block::filled(0x1B4 / 4, rng);
        probe_vt.words[0x18 / 4] = stubs.probe_set;
        probe_vt.words[0x1B0 / 4] = stubs.probe_prime;
        let mut view_vt = Block::filled(0x124 / 4, rng);
        view_vt.words[0x120 / 4] = stubs.view_set;
        let mut cfgsub_vt = Block::filled(0x124 / 4, rng);
        cfgsub_vt.words[0x120 / 4] = stubs.cfg_sub_set;
        let mut obj = Block::filled(OBJ_WORDS, rng);
        let mut list = Block::filled(0x224 / 4, rng);
        let mut sess = Block::filled(4, rng);
        let mut sess2 = Block::filled(4, rng);
        let mut aux = Block::filled(4, rng);
        let mut tail = Block::filled(4, rng);
        let mut inner = Block::filled(4, rng);
        let mut probes = [Block::filled(4, rng), Block::filled(4, rng)];
        let mut view = Block::filled(4, rng);
        let mut cfg = Block::filled(0x200 / 4, rng);
        let mut cfgsub = Block::filled(4, rng);
        let src = Block::filled(4, rng);
        let out = Block::filled(4, rng);
        let elems = [
            Block::filled(0x318 / 4, rng),
            Block::filled(0x318 / 4, rng),
            Block::filled(0x318 / 4, rng),
            Block::filled(0x318 / 4, rng),
        ];
        let actives = [Block::filled(0x210 / 4, rng), Block::filled(0x210 / 4, rng)];
        let target = Block::filled(4, rng);
        let mut state = Block::filled(0x204 / 4, rng);
        obj.words[0] = obj_vt.addr();
        list.words[0] = list_vt.addr();
        sess.words[0] = sess_vt.addr();
        sess2.words[0] = sess_vt.addr();
        aux.words[0] = aux_vt.addr();
        tail.words[0] = tail_vt.addr();
        inner.words[0] = inner_vt.addr();
        probes[0].words[0] = probe_vt.addr();
        probes[1].words[0] = probe_vt.addr();
        view.words[0] = view_vt.addr();
        cfgsub.words[0] = cfgsub_vt.addr();
        obj.words[LIST_WORD] = list.addr();
        obj.words[AUX_WORD] = aux.addr();
        obj.words[CFG_WORD] = cfg.addr();
        obj.words[OUT_WORD] = out.addr();
        obj.words[VIEW_WORD] = view.addr();
        obj.words[SRC_WORD] = src.addr();
        // The trial's directed words overwrite the filler.
        obj.words[WANT_WORD] = trial.words[WANT_WORD];
        obj.words[DROPPED_WORD] = trial.words[DROPPED_WORD];
        obj.words[PENDING_WORD] = trial.words[PENDING_WORD];
        obj.words[CURSOR_WORD] = trial.words[CURSOR_WORD];
        obj.words[MODE_WORD] = (obj.words[MODE_WORD] & 0xFFFF_FF00) | u32::from(trial.mode);
        obj.words[PHASE_WORD] = (obj.words[PHASE_WORD] & 0xFFFF_FF00) | u32::from(trial.phase);
        // The links the rewrite reads through collaborator objects.
        list.words[0x1E0 / 4] = inner.addr();
        list.words[0x21C / 4] = tail.addr();
        cfg.words[0x1E4 / 4] = cfgsub.addr();
        state.words[0x200 / 4] = trial.state_flag;
        // The address answers both sides script from.
        push(&mut trial.script, "open_session", vec![sess.addr()]);
        push(&mut trial.script, "open_worker", vec![sess2.addr()]);
        push(&mut trial.script, "probe_open", vec![probes[0].addr(), probes[1].addr()]);
        push(&mut trial.script, "list_inner", vec![inner.addr(); 6]);
        push(&mut trial.script, "list_tail", vec![tail.addr()]);
        push(&mut trial.script, "cfg_sub", vec![cfgsub.addr()]);
        push(&mut trial.script, "resolve_output", vec![target.addr()]);
        push(&mut trial.script, "activate", vec![actives[0].addr(), actives[1].addr()]);
        // The walk answers cycle through the element pool; the element
        // words follow the same cycle, so the lift's scripted reads match
        // what the rewrite reads from memory.
        let order = [0usize, 1, 2, 3, 0, 1];
        push(
            &mut trial.script,
            "list_at",
            order.iter().map(|&k| elems[k].addr()).collect(),
        );
        push(
            &mut trial.script,
            "elem_word_a",
            order.iter().map(|&k| elems[k].words[0x310 / 4]).collect(),
        );
        push(
            &mut trial.script,
            "elem_word_b",
            order.iter().map(|&k| elems[k].words[0x314 / 4]).collect(),
        );
        Self {
            obj,
            obj_vt,
            list,
            list_vt,
            sess,
            sess_vt,
            sess2,
            aux,
            aux_vt,
            tail,
            tail_vt,
            inner,
            inner_vt,
            probes,
            probe_vt,
            view,
            view_vt,
            cfg,
            cfgsub,
            cfgsub_vt,
            src,
            out,
            elems,
            actives,
            target,
            state,
            stubs,
        }
    }

    fn ctx(&self) -> Ctx {
        Ctx {
            own: self.obj.addr(),
            list: self.list.addr(),
            aux: self.aux.addr(),
            cfg: self.cfg.addr(),
            out: self.out.addr(),
            view: self.view.addr(),
            src: self.src.addr(),
            sess: self.sess.addr(),
            sess2: self.sess2.addr(),
            tail: self.tail.addr(),
            inner: self.inner.addr(),
            probes: [self.probes[0].addr(), self.probes[1].addr()],
            cfgsub: self.cfgsub.addr(),
            elems: [
                self.elems[0].addr(),
                self.elems[1].addr(),
                self.elems[2].addr(),
                self.elems[3].addr(),
            ],
            actives: [self.actives[0].addr(), self.actives[1].addr()],
            target: self.target.addr(),
        }
    }
}

/// Decodes the object's modelled words into the lifted container.
fn decode(words: &[u32], own_addr: u32) -> MontageContainer {
    let mode = (words[MODE_WORD] & 0xFF) as u8;
    let phase = (words[PHASE_WORD] & 0xFF) as u8;
    MontageContainer {
        own: Handle32::<OwnTag>::new(own_addr).expect("the object has an address"),
        list: Handle32::<ListTag>::new(words[LIST_WORD]).expect("the list is planted"),
        aux: Handle32::<AuxTag>::new(words[AUX_WORD]).expect("the auxiliary is planted"),
        cfg: Handle32::<CfgTag>::new(words[CFG_WORD]).expect("the configuration is planted"),
        out: Handle32::<OutTag>::new(words[OUT_WORD]).expect("the output is planted"),
        view: Handle32::<ViewTag>::new(words[VIEW_WORD]).expect("the view is planted"),
        src: Handle32::<SrcTag>::new(words[SRC_WORD]).expect("the source is planted"),
        want: words[WANT_WORD],
        dropped: words[DROPPED_WORD],
        pending: words[PENDING_WORD],
        cursor: words[CURSOR_WORD],
        mode,
        phase,
    }
}

/// Writes the lifted container's modelled words back over the object's
/// bytes. Only the words the dispatcher writes can differ.
fn encode(container: &MontageContainer, words: &mut [u32]) {
    words[WANT_WORD] = container.want;
    words[DROPPED_WORD] = container.dropped;
    words[PENDING_WORD] = container.pending;
    words[CURSOR_WORD] = container.cursor;
    words[MODE_WORD] = (words[MODE_WORD] & 0xFFFF_FF00) | u32::from(container.mode);
    words[PHASE_WORD] = (words[PHASE_WORD] & 0xFFFF_FF00) | u32::from(container.phase);
}

fn dispatch_table() -> Vec<(CallSpec, Kind)> {
    let kinds = [
        Kind::T2,
        Kind::C1,
        Kind::T2,
        Kind::T3,
        Kind::T2,
        Kind::T3,
        Kind::T3,
        Kind::T1,
        Kind::C1,
        Kind::T1,
        Kind::T1,
        Kind::T2,
        Kind::C2,
        Kind::T4,
        Kind::T2,
        Kind::T2,
        Kind::T3,
        Kind::T1,
        Kind::T2,
        Kind::T2f32,
        Kind::T1,
        Kind::T2,
    ];
    DISPATCH_CALLS.iter().zip(kinds).map(|(spec, kind)| (*spec, kind)).collect()
}

/// Runs the verified dispatcher on the scene; returns the record and the
/// returned word.
fn run_rewrite(scene: &Scene, trial: &Trial, ctx: &Ctx) -> (Vec<String>, u32) {
    rt::install(&dispatch_table(), DISPATCH_VIRTUALS, &trial.script);
    rt::set_state_ptr(scene.state.addr());
    let eax = rewrites::rw_00dd46c0(scene.obj.addr());
    let log = rt::take_log();
    let events = normalise(ctx, DISPATCH_CALLS, DISPATCH_VIRTUALS, &log);
    (events, eax)
}

/// Runs the lifted dispatcher on a copy of the object's words; returns the
/// record, the returned word and the words afterwards.
fn run_lift(scene: &Scene, trial: &Trial, ctx: &Ctx) -> (Vec<String>, u32, Vec<u32>) {
    let mut lifted = scene.obj.words.clone();
    let mut container = decode(&lifted, scene.obj.addr());
    let mut world = FakeWorld::new(*ctx, trial.script.clone());
    let globals = MontageGlobals { state_flag: trial.state_flag };
    let returned = container.dispatch_state(&mut world, &globals);
    encode(&container, &mut lifted);
    (world.events, returned, lifted)
}

/// Runs one wrong dispatcher the same way.
fn run_mutant(
    scene: &Scene,
    trial: &Trial,
    ctx: &Ctx,
    m: DispatchMutant,
) -> (Vec<String>, u32, Vec<u32>) {
    let mut lifted = scene.obj.words.clone();
    let mut container = decode(&lifted, scene.obj.addr());
    let mut world = FakeWorld::new(*ctx, trial.script.clone());
    let globals = MontageGlobals { state_flag: trial.state_flag };
    let returned = mutants::dispatch(&mut container, &mut world, &globals, m);
    encode(&container, &mut lifted);
    (world.events, returned, lifted)
}

#[test]
fn dispatch_matches_its_rewrite() {
    let _guard = rt::case_lock();
    let mut rng = Rng(0xD150_47C0_0001);
    let mut caught = [false; 8];
    let mut path_count = [0usize; 8];
    let trials = TRIALS_PER_PATH * PATHS.len();
    for index in 0..trials {
        let path = PATHS[index % PATHS.len()];
        path_count[index % PATHS.len()] += 1;
        let mut trial = make_trial(&mut rng, path);
        let scene = Scene::build(&mut rng, &mut trial);
        let ctx = scene.ctx();
        let initial = scene.obj.words.clone();
        let (rewrite_events, eax) = run_rewrite(&scene, &trial, &ctx);
        let after_rewrite = scene.obj.words.clone();
        let (lift_events, returned, lifted) = run_lift(&scene, &trial, &ctx);

        assert_eq!(lift_events, rewrite_events, "trial {index} ({path:?}): calls differ");
        assert_eq!(returned, eax, "trial {index} ({path:?}): result differs");
        assert_eq!(lifted, after_rewrite, "trial {index} ({path:?}): object bytes differ");
        // The trial took its forced path.
        match path {
            Path::FlagZero => {
                assert!(rewrite_events.is_empty(), "trial {index}: flag-zero calls");
                assert_eq!(eax, 0, "trial {index}: flag-zero result");
                assert_eq!(after_rewrite[CURSOR_WORD], 0, "trial {index}: cursor cleared");
                assert_eq!(after_rewrite[PENDING_WORD], 0, "trial {index}: pending cleared");
            }
            Path::List => assert!(
                rewrite_events.iter().any(|e| e.starts_with("activate(tag_b)")),
                "trial {index}: list region not reached"
            ),
            Path::RefreshToggle => assert!(
                rewrite_events.iter().any(|e| e.starts_with("aux_set(")),
                "trial {index}: toggle path not reached"
            ),
            Path::RefreshProbe => assert!(
                rewrite_events.iter().any(|e| e.starts_with("probe_open(")),
                "trial {index}: probe path not reached"
            ),
            Path::Threshold => assert!(
                rewrite_events.iter().any(|e| e.starts_with("note_teardown_mark(")),
                "trial {index}: threshold path not reached"
            ),
            Path::ReconcileDrop => assert!(
                rewrite_events.iter().any(|e| e.starts_with("drop_pending(")),
                "trial {index}: drop path not reached"
            ),
            Path::ReconcileRearm => assert!(
                rewrite_events.iter().any(|e| e.starts_with("own_rearm(")),
                "trial {index}: re-arm path not reached"
            ),
            Path::Fallthrough => assert_eq!(
                rewrite_events.last().map(String::as_str),
                Some("tag_id(tag_j)"),
                "trial {index}: fallthrough shape differs"
            ),
        }
        // The wrong dispatchers run through the early trials; each must be
        // caught at least once.
        if index < 40 * PATHS.len() {
            for (k, m) in mutants::ALL.iter().enumerate() {
                if caught[k] {
                    continue;
                }
                let (mut_events, mut_ret, mut_words) = run_mutant(&scene, &trial, &ctx, *m);
                if mut_events != rewrite_events || mut_ret != eax || mut_words != after_rewrite {
                    caught[k] = true;
                }
                let _ = initial;
            }
        }
    }
    assert!(path_count.iter().all(|&c| c == TRIALS_PER_PATH), "every path ran");
    for (k, m) in mutants::ALL.iter().enumerate() {
        assert!(caught[k], "wrong dispatcher {m:?} was never caught");
    }
}

