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
                            let cursor = rng.pick_word();
                            // Adjacent, sometimes across the wrap.
                            let want = if rng.below(4) == 0 {
                                cursor.wrapping_sub(1)
                            } else {
                                cursor
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
