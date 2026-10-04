//! Forwarders: functions whose behaviour is the calls they make.
//!
//! Each original here reaches other code only through callee slots. The
//! lift turns every slot into a method of a small trait named for what the
//! callee does, so a caller supplies the real implementation and a test
//! supplies a recording fake. Object pointers become [`Handle`]s; ranges of
//! elements become `Range<usize>` in elements, not bytes; flag words that
//! are tested for one bit become `bool`s at the boundary.
//!
//! Three families and four singles:
//!
//! - **Deleting destructors** ([`deleting_destructor`], seven originals):
//!   run the destructor, release the memory when flag bit 0 is set,
//!   return the object.
//! - **Final insertion sort** ([`final_insertion_sort`], seven
//!   originals): the tail of an introspective sort, one instantiation per
//!   element type. Ranges longer than [`SORT_THRESHOLD`] elements are
//!   split into a guarded head and an unguarded tail.
//! - **Sort entry** ([`sort`]): depth budget then the two sort phases.
//! - Singles: [`any_gate_open`], [`dual_rank_test`], [`probe_nonzero`],
//!   [`effect_process`].
//!
//! What every function here narrows (also listed per item):
//!
//! - Inferred: callees reached by the sort families return nothing; the
//!   originals pass the last callee's return register through, and the
//!   lift drops it.
//! - The originals take ranges as two addresses and compare their byte
//!   span as a signed word. The lifted domain is a valid range (start not
//!   after end, span below 2^31 bytes, a whole number of elements); outside
//!   it the original computes from garbage, and the lift cannot be called.

use core::ops::Range;

use lf_core::Handle;

/// What a deleting destructor calls.
pub trait Lifecycle<T> {
    /// Runs the object's destructor (the original's first callee).
    fn destruct(&mut self, object: Handle<T>);
    /// Returns the object's memory to its allocator (the second callee).
    fn release(&mut self, object: Handle<T>);
}

/// The deleting-destructor routine: destruct, then release when
/// `release_memory` is set, then hand the object back.
///
/// Narrowed: the original takes a flag word and tests bit 0; the boundary
/// passes `flags & 1 != 0`. It returns the object's address, which is the
/// handle here.
///
/// Originals (one per class, identical shape; slot 1 destructs, slot 2
/// releases): `rw_009815f0` (0x009815F0, `audAmbientAudioEntity`),
/// `rw_009856a0` (0x009856A0, `audEmitterAudioEntity`), `rw_00add1a0`
/// (0x00ADD1A0, `CRenderPhaseDrawScene`), `rw_00add1c0` (0x00ADD1C0,
/// `CRenderPhaseScript2d`), `rw_00c6e1c0` (0x00C6E1C0, `CAnimAssociations`),
/// `rw_00ca4e40` (0x00CA4E40, `CEventHandler`), `rw_00d8c690` (0x00D8C690,
/// `audFrontendAudioEntity`).
pub fn deleting_destructor<T, L: Lifecycle<T> + ?Sized>(
    life: &mut L,
    object: Handle<T>,
    release_memory: bool,
) -> Handle<T> {
    life.destruct(object);
    if release_memory {
        life.release(object);
    }
    object
}

/// Ranges longer than this many elements are split by
/// [`final_insertion_sort`].
pub const SORT_THRESHOLD: usize = 16;

/// The two insertion passes a final insertion sort calls.
pub trait InsertionSort {
    /// Guarded insertion sort of `range` (the original's first callee).
    fn insertion_sort(&mut self, range: Range<usize>);
    /// Unguarded insertion sort of `range`, which relies on a smaller
    /// element sitting before it (the original's second callee).
    fn unguarded_insertion_sort(&mut self, range: Range<usize>);
}

/// Final phase of an introspective sort over `len` elements.
///
/// More than [`SORT_THRESHOLD`] elements: guarded pass over the first
/// [`SORT_THRESHOLD`], unguarded pass over the rest. Otherwise one guarded
/// pass over everything.
///
/// Narrowed: the originals also pass a placeholder zero and the
/// comparator to each callee; both live in the `InsertionSort`
/// implementation here. One original (`rw_00b05100`) passes zero instead
/// of the comparator to its unguarded pass; its differential test checks
/// that argument on the 32-bit side.
///
/// Originals (element size; slot numbers guarded/unguarded):
/// `rw_00abbda0` (0x00ABBDA0; 4 bytes; 1/2), `rw_00ade7a0` (0x00ADE7A0;
/// 4 bytes; 2/3; compares the span unsigned), `rw_00b05100` (0x00B05100;
/// 8 bytes; 1/2), `rw_00b33d00` and `rw_00b33d70` (0x00B33D00,
/// 0x00B33D70; 28 bytes; 1/2; count by signed division), `rw_00b33de0`
/// (0x00B33DE0; 16 bytes; 1/2), `rw_00c6dbb0` (0x00C6DBB0; 8 bytes; 1/2).
pub fn final_insertion_sort<S: InsertionSort + ?Sized>(sorter: &mut S, len: usize) {
    if len > SORT_THRESHOLD {
        sorter.insertion_sort(0..SORT_THRESHOLD);
        sorter.unguarded_insertion_sort(SORT_THRESHOLD..len);
    } else {
        sorter.insertion_sort(0..len);
    }
}

/// The two phases a sort entry calls.
pub trait IntroSort {
    /// Partitioning loop over `range` with `depth_budget` levels before it
    /// falls back to heap sort (the original's first callee).
    fn introsort_loop(&mut self, range: Range<usize>, depth_budget: u32);
    /// Final insertion sort of `range` (the second callee).
    fn final_insertion_sort(&mut self, range: Range<usize>);
}

/// Sorts `len` elements: nothing for an empty range; otherwise the
/// partitioning loop with a depth budget of `2 × floor(log2(len))`, then
/// the final insertion sort.
///
/// Domain: the original divides the byte span by four and loops until the
/// count halves down to one, so a span of one to three bytes never ends.
/// Element counts carry no such case.
///
/// Original: `rw_00adebd0` (0x00ADEBD0, `std_sort`; 4-byte elements).
pub fn sort<S: IntroSort + ?Sized>(sorter: &mut S, len: usize) {
    if len == 0 {
        return;
    }
    let depth = len.ilog2();
    sorter.introsort_loop(0..len, depth + depth);
    sorter.final_insertion_sort(0..len);
}

/// The three gates [`any_gate_open`] consults.
pub trait Gates {
    /// First gate, asked about `subject` (slot 1).
    fn first(&mut self, subject: u32) -> bool;
    /// Second gate (slot 2).
    fn second(&mut self) -> bool;
    /// Third gate (slot 3).
    fn third(&mut self) -> bool;
}

/// True when any gate is open, asking them in order and stopping at the
/// first open one.
///
/// Narrowed (Inferred): each gate's answer is the low byte of its return
/// register in the original; the boundary passes `answer as u8 != 0`.
///
/// Original: `rw_00a72820` (0x00A72820, `any_gate_true`).
pub fn any_gate_open<G: Gates + ?Sized>(gates: &mut G, subject: u32) -> bool {
    gates.first(subject) || gates.second() || gates.third()
}

/// The two rank lookups [`dual_rank_test`] makes.
pub trait RankLookup {
    /// Rank of the pair `(a, b)` (slot 1; the low byte of its answer).
    fn primary(&mut self, a: u32, b: u32) -> u8;
    /// Rank of the pair under `level` (slot 2; the low byte of its answer).
    fn secondary(&mut self, a: u32, b: u32, level: i32) -> u8;
}

/// The two-stage rank test.
///
/// The sentinel pair `level == -1, sub == 0` passes outright. Otherwise
/// `level` must be below the primary rank (signed compare) or the test
/// fails, and then `sub` must be below the secondary rank.
///
/// Original: `rw_00aba180` (0x00ABA180, `dual_lookup_compare`).
pub fn dual_rank_test<R: RankLookup + ?Sized>(
    ranks: &mut R,
    a: u32,
    b: u32,
    level: i32,
    sub: i32,
) -> bool {
    if level == -1 && sub == 0 {
        return true;
    }
    if level >= i32::from(ranks.primary(a, b)) {
        return false;
    }
    sub < i32::from(ranks.secondary(a, b, level))
}

/// A helper whose answer is tested for zero.
pub trait Probe {
    /// The helper's answer for `subject` (slot 1).
    fn probe(&mut self, subject: u32) -> u32;
}

/// True when the helper's answer for `subject` is non-zero.
///
/// Original: `rw_009a3ea0` (0x009A3EA0, `audio_helper_test_nonzero`).
pub fn probe_nonzero<P: Probe + ?Sized>(helper: &mut P, subject: u32) -> bool {
    helper.probe(subject) != 0
}

/// Marker type for an audio effect object.
#[derive(Debug)]
pub enum Effect {}

/// The base implementation an effect's first virtual forwards to.
pub trait EffectBase {
    /// The base class's step for `effect` with two arguments (slot 1).
    fn process(&mut self, effect: Handle<Effect>, a: u32, b: u32) -> bool;
}

/// The compressor effect's first virtual: the base step's answer,
/// normalised to a boolean.
///
/// Narrowed (Inferred: the base step returns a boolean): the original
/// keeps bits 8-31 of the base step's return register and normalises
/// only the low byte to 0 or 1; the lift returns the boolean, and the
/// differential test compares the low byte and pins the residue.
///
/// Original: `rw_008ac690` (0x008AC690, `rage::audCompressorEffect::vf1`).
pub fn effect_process<B: EffectBase + ?Sized>(
    base: &mut B,
    effect: Handle<Effect>,
    a: u32,
    b: u32,
) -> bool {
    base.process(effect, a, b)
}

#[cfg(test)]
#[allow(clippy::cast_possible_truncation)] // fakes truncate answers on purpose
mod tests {
    use super::*;
    use lf_core::Arena;

    /// Records every call as text, the way a test fake does.
    #[derive(Default)]
    struct Recorder {
        calls: Vec<String>,
        answers: Vec<u32>,
    }

    impl Recorder {
        fn answer(&mut self) -> u32 {
            if self.answers.is_empty() {
                0
            } else {
                self.answers.remove(0)
            }
        }
    }

    impl<T> Lifecycle<T> for Recorder {
        fn destruct(&mut self, object: Handle<T>) {
            self.calls.push(format!("destruct {}", object.index()));
        }
        fn release(&mut self, object: Handle<T>) {
            self.calls.push(format!("release {}", object.index()));
        }
    }

    impl InsertionSort for Recorder {
        fn insertion_sort(&mut self, range: Range<usize>) {
            self.calls.push(format!("guarded {range:?}"));
        }
        fn unguarded_insertion_sort(&mut self, range: Range<usize>) {
            self.calls.push(format!("unguarded {range:?}"));
        }
    }

    impl IntroSort for Recorder {
        fn introsort_loop(&mut self, range: Range<usize>, depth_budget: u32) {
            self.calls.push(format!("loop {range:?} {depth_budget}"));
        }
        fn final_insertion_sort(&mut self, range: Range<usize>) {
            self.calls.push(format!("final {range:?}"));
        }
    }

    impl Gates for Recorder {
        fn first(&mut self, subject: u32) -> bool {
            self.calls.push(format!("first {subject}"));
            self.answer() != 0
        }
        fn second(&mut self) -> bool {
            self.calls.push("second".into());
            self.answer() != 0
        }
        fn third(&mut self) -> bool {
            self.calls.push("third".into());
            self.answer() != 0
        }
    }

    impl RankLookup for Recorder {
        fn primary(&mut self, a: u32, b: u32) -> u8 {
            self.calls.push(format!("primary {a} {b}"));
            self.answer() as u8
        }
        fn secondary(&mut self, a: u32, b: u32, level: i32) -> u8 {
            self.calls.push(format!("secondary {a} {b} {level}"));
            self.answer() as u8
        }
    }

    #[test]
    fn deleting_destructor_releases_only_on_request() {
        let mut arena = Arena::new();
        let h = arena.insert(());
        let mut rec = Recorder::default();
        assert_eq!(deleting_destructor(&mut rec, h, false), h);
        assert_eq!(rec.calls, ["destruct 0"]);
        rec.calls.clear();
        deleting_destructor(&mut rec, h, true);
        assert_eq!(rec.calls, ["destruct 0", "release 0"]);
    }

    #[test]
    fn final_insertion_sort_splits_above_threshold() {
        let mut rec = Recorder::default();
        final_insertion_sort(&mut rec, 16);
        final_insertion_sort(&mut rec, 17);
        final_insertion_sort(&mut rec, 0);
        assert_eq!(
            rec.calls,
            [
                "guarded 0..16",
                "guarded 0..16",
                "unguarded 16..17",
                "guarded 0..0"
            ]
        );
    }

    #[test]
    fn sort_budget_is_twice_the_floor_log() {
        let mut rec = Recorder::default();
        sort(&mut rec, 0);
        sort(&mut rec, 1);
        sort(&mut rec, 1000);
        assert_eq!(
            rec.calls,
            [
                "loop 0..1 0",
                "final 0..1",
                "loop 0..1000 18",
                "final 0..1000"
            ]
        );
    }

    #[test]
    fn gates_stop_at_first_open() {
        let mut rec = Recorder {
            answers: vec![0, 0x100, 7],
            ..Recorder::default()
        };
        // This fake tests its whole answer word, so 0x100 opens the second
        // gate (the boundary for the original would test only the low
        // byte: narrowing the answer is the trait implementor's job).
        assert!(any_gate_open(&mut rec, 9));
        assert_eq!(rec.calls, ["first 9", "second"]);
        let mut closed = Recorder::default();
        assert!(!any_gate_open(&mut closed, 1));
        assert_eq!(closed.calls.len(), 3);
    }

    #[test]
    fn rank_test_paths() {
        let mut rec = Recorder::default();
        assert!(dual_rank_test(&mut rec, 1, 2, -1, 0));
        assert!(rec.calls.is_empty());
        let mut rec = Recorder {
            answers: vec![3, 5],
            ..Recorder::default()
        };
        assert!(dual_rank_test(&mut rec, 1, 2, 2, 4));
        assert_eq!(rec.calls, ["primary 1 2", "secondary 1 2 2"]);
        let mut rec = Recorder {
            answers: vec![3],
            ..Recorder::default()
        };
        assert!(!dual_rank_test(&mut rec, 1, 2, 3, 0));
        assert_eq!(rec.calls.len(), 1);
    }
}
