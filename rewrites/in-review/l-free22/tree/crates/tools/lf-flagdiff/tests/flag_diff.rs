//! Differential tests: every lifted word test against its verified 32-bit rewrite.
//!
//! For each instance of `lf_core::word_test::instances::PREDICATES`, the test builds the same object (or the
//! same static words) and parameter for the lifted test and for the rewrite, and compares the answer. The
//! tested word is drawn from edge candidates around the instance's right-hand side, so that equal, unequal
//! and ordered outcomes are all reached: a random 32-bit word would almost never equal a small constant.
//! After each call the object and the static words are compared too, so a rewrite that wrote anything would
//! be seen. Each instance runs `CASES` cases.
//!
//! Deliberately wrong versions run through the same contract, and each must be caught by every instance: a
//! negated answer, a read from the wrong word, a right-hand side one away from the right one, and the wrong
//! comparison.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from a fixed seed.

#![cfg(target_arch = "x86")]

use lf_core::field_store::ObjectBytes;
use lf_core::static_list::{StaticWords, WordId};
use lf_core::word_test::instances::{GLOBAL_WORDS, NamedPredicate, PREDICATES};
use lf_core::word_test::{Compare, Predicate, Rhs, Source};
use lf_flagdiff::calls::CALLS;
use lf_flagdiff::rt;

/// Bytes in the generated object. Every field the instances read lies below this.
const OBJECT_BYTES: usize = 0x2000;
/// Cases per instance for the differential comparison.
const CASES: u32 = 1000;
/// Cases per instance and wrong version; a wrong version must be caught by one of them.
const WRONG_CASES: u32 = 64;

/// The object the rewrite reads. Aligned as in the original.
#[repr(align(16))]
#[derive(Clone)]
struct Object([u8; OBJECT_BYTES]);

/// A deterministic generator (xorshift32). The state is never zero.
struct Rng(u32);

impl Rng {
    fn new(seed: u32) -> Self {
        Self(seed | 1)
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
}

/// A tested word for one case, drawn around `pivot`: the pivot itself, its neighbours, zero, the extreme
/// values, and a random word, in turn.
fn candidate(pivot: u32, case: u32, rng: &mut Rng) -> u32 {
    match case % 8 {
        0 => pivot,
        1 => pivot.wrapping_add(1),
        2 => pivot.wrapping_sub(1),
        3 => 0,
        4 => u32::MAX,
        5 => 0x8000_0000,
        6 => 0x7FFF_FFFF,
        _ => rng.next_u32(),
    }
}

/// One case's inputs: the object (random, with the tested word placed in its field), the static words
/// (random, with the tested word placed in its word), and the parameter.
struct Case {
    object: Object,
    words: Vec<u32>,
    parameter: u32,
    tested: u32,
}

/// The inputs for one case of one instance. The right-hand side decides the pivot: the constant itself, or
/// the parameter, which is then chosen from the candidates so that the tested word can equal it.
fn case_inputs(predicate: &Predicate, instance: usize, case: u32) -> Case {
    let seed = ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    let mut object = Object([0; OBJECT_BYTES]);
    for chunk in object.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    let mut words: Vec<u32> = (0..GLOBAL_WORDS).map(|_| rng.next_u32()).collect();
    let (pivot, parameter) = match predicate.rhs {
        Rhs::Constant(value) => (value, rng.next_u32()),
        Rhs::Parameter => {
            let value = rng.next_u32();
            (value, value)
        }
    };
    let tested = candidate(pivot, case, &mut rng);
    match predicate.source {
        Source::Field(offset) => {
            object.0[offset..offset + 4].copy_from_slice(&tested.to_le_bytes());
        }
        Source::Global(WordId(number)) => {
            words[number] = tested;
        }
    }
    Case {
        object,
        words,
        parameter,
        tested,
    }
}

/// The answer and the effects of the rewrite on one case: the returned word, the object after the call and
/// the static words after the call.
fn run_rewrite(instance: usize, case: &Case) -> (u32, Object, Vec<u32>) {
    let mut object = case.object.clone();
    for (number, value) in case.words.iter().enumerate() {
        rt::set_word(number, *value);
    }
    let returned = CALLS[instance](object.0.as_mut_ptr(), case.parameter);
    let words = (0..GLOBAL_WORDS).map(rt::get_word).collect();
    (returned, object, words)
}

/// The answer and the effects of the lifted test on one case, for the predicate it is given.
fn run_lifted(predicate: &Predicate, case: &Case, parameter: u32) -> (u32, Object, Vec<u32>) {
    let object = ObjectBytes::from_bytes(&case.object.0);
    let mut words = StaticWords::new(GLOBAL_WORDS);
    for (number, value) in case.words.iter().enumerate() {
        words.set_word(WordId(number), *value);
    }
    let returned = predicate.evaluate(&object, &words, parameter);
    let mut after = Object([0; OBJECT_BYTES]);
    after.0.copy_from_slice(object.bytes());
    let after_words = (0..GLOBAL_WORDS).map(|number| words.word(WordId(number))).collect();
    (returned, after, after_words)
}

/// A deliberately wrong version of one instance's lifted test.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The answer is negated.
    Negated,
    /// The tested word is read from the wrong place: the next field, or the next static word.
    WrongSource,
    /// The right-hand side is one away: the constant plus one, or the parameter plus one.
    WrongOperand,
    /// The comparison is the wrong one: equal and unequal swap, and ordered becomes equal.
    WrongCompare,
}

const WRONG_KINDS: [Wrong; 4] = [Wrong::Negated, Wrong::WrongSource, Wrong::WrongOperand, Wrong::WrongCompare];

/// The lifted test run as the wrong version describes.
fn run_wrong(wrong: Wrong, predicate: &Predicate, case: &Case) -> (u32, Object, Vec<u32>) {
    match wrong {
        Wrong::Negated => {
            let (returned, object, words) = run_lifted(predicate, case, case.parameter);
            (returned ^ 1, object, words)
        }
        Wrong::WrongSource => {
            let source = match predicate.source {
                Source::Field(offset) => Source::Field(offset + 4),
                Source::Global(WordId(number)) => Source::Global(WordId((number + 1) % GLOBAL_WORDS)),
            };
            let moved = Predicate {
                source,
                ..*predicate
            };
            run_lifted(&moved, case, case.parameter)
        }
        Wrong::WrongOperand => {
            let (rhs, parameter) = match predicate.rhs {
                Rhs::Constant(value) => (Rhs::Constant(value.wrapping_add(1)), case.parameter),
                Rhs::Parameter => (Rhs::Parameter, case.parameter.wrapping_add(1)),
            };
            let moved = Predicate { rhs, ..*predicate };
            run_lifted(&moved, case, parameter)
        }
        Wrong::WrongCompare => {
            let compare = match predicate.compare {
                Compare::Equal => Compare::NotEqual,
                Compare::NotEqual => Compare::Equal,
                Compare::SignedGreater => Compare::Equal,
            };
            let moved = Predicate {
                compare,
                ..*predicate
            };
            run_lifted(&moved, case, case.parameter)
        }
    }
}

/// The first case, if any, at which the lifted test and the rewrite disagree in the answer or the effects.
fn first_difference(named: &NamedPredicate, instance: usize, cases: u32) -> Option<String> {
    for case_number in 0..cases {
        let case = case_inputs(&named.predicate, instance, case_number);
        let (want, want_object, want_words) = run_rewrite(instance, &case);
        let (got, got_object, got_words) = run_lifted(&named.predicate, &case, case.parameter);
        if want != got {
            return Some(format!(
                "case {case_number}: tested word {:#010x}, rewrite answered {want}, lifted answered {got}",
                case.tested
            ));
        }
        if want_object.0 != got_object.0 || want_words != got_words {
            return Some(format!("case {case_number}: the rewrite changed state the lifted test leaves alone"));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &NamedPredicate, instance: usize) -> bool {
    for case_number in 0..WRONG_CASES {
        let case = case_inputs(&named.predicate, instance, case_number);
        let (want, want_object, want_words) = run_rewrite(instance, &case);
        let (got, got_object, got_words) = run_wrong(wrong, &named.predicate, &case);
        if want != got || want_object.0 != got_object.0 || want_words != got_words {
            return true;
        }
    }
    false
}

#[test]
fn every_instance_matches_its_verified_rewrite() {
    let mut failures = Vec::new();
    for (instance, named) in PREDICATES.iter().enumerate() {
        if let Some(detail) = first_difference(named, instance, CASES) {
            failures.push(format!("{} (instance {instance}): {detail}", named.name));
        }
    }
    assert!(failures.is_empty(), "instances that differ from their rewrite:\n{}", failures.join("\n"));
}

#[test]
fn every_instance_catches_every_wrong_version() {
    let mut escaped = Vec::new();
    for (instance, named) in PREDICATES.iter().enumerate() {
        for wrong in WRONG_KINDS {
            if !wrong_is_caught(wrong, named, instance) {
                escaped.push(format!("{} (instance {instance}): {wrong:?} was not caught", named.name));
            }
        }
    }
    assert!(escaped.is_empty(), "wrong versions that escaped:\n{}", escaped.join("\n"));
}

#[test]
fn the_call_table_and_the_instance_table_have_the_same_length() {
    assert_eq!(CALLS.len(), PREDICATES.len());
    assert_eq!(rt::WORD_COUNT, GLOBAL_WORDS);
}
