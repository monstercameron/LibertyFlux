//! Differential tests: every word-test routine of this crate against its verified 32-bit rewrite.
//!
//! Each instance is run through the rewrite and through the existing lifted `Predicate::evaluate` on the same
//! generated object and parameter. The tested word is set to the constant, to the parameter, to one more than
//! either, or left random, so that the equal and unequal answers both occur. The answer is compared as a word.
//! Deliberately wrong versions must each be caught by every instance: the tested field four bytes on, the
//! comparison reversed, and the right-hand side one away from the right one.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from a fixed seed.

#![cfg(target_arch = "x86")]

use lf_24_testdiff::calls::CALLS;
use lf_24_testdiff::instances::{INSTANCES, Named};
use lf_core::field_store::ObjectBytes;
use lf_core::static_list::StaticWords;
use lf_core::word_test::{Compare, Predicate, Rhs, Source};

/// Bytes in the generated object. Every field and table entry the instances touch lies below this.
const OBJECT_BYTES: usize = 0x2000;
/// Cases per instance for the differential comparison.
const CASES: u32 = 1000;
/// Cases per instance and wrong version; a wrong version must be caught by one of them.
const WRONG_CASES: u32 = 64;

/// The object the rewrite works on, aligned so word fields at 4-byte offsets are aligned as in the original.
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

/// The object for one case of one instance, filled with random bytes.
fn random_object(instance: usize, case: u32) -> (Object, Rng) {
    let seed =
        ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    let mut object = Object([0; OBJECT_BYTES]);
    for chunk in object.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    (object, rng)
}

/// Writes a word into an object at a byte offset, little endian.
#[allow(dead_code)]
fn put_word(object: &mut Object, offset: usize, value: u32) {
    object.0[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

/// The parameter and the tested word of one case, chosen so that the answers differ across cases.
fn inputs(named: &Named<Predicate>, object: &mut Object, rng: &mut Rng, case: u32) -> u32 {
    let parameter = rng.next_u32();
    let constant = match named.spec.rhs {
        Rhs::Constant(value) => value,
        Rhs::Parameter => parameter,
    };
    let field = match named.spec.source {
        Source::Field(offset) => offset,
        Source::Global(_) => unreachable!("object-based instances only"),
    };
    let word = match case % 3 {
        0 => constant,
        1 => constant.wrapping_add(1),
        _ => rng.next_u32(),
    };
    put_word(object, field, word);
    parameter
}

/// The rewrite on one object: its answer as a word.
fn run_rewrite(instance: usize, object: &mut Object, parameter: u32) -> u32 {
    CALLS[instance](object.0.as_mut_ptr(), parameter, 0)
}

/// The lifted predicate on a copy of the object.
fn run_lifted(predicate: Predicate, original: &Object, parameter: u32) -> u32 {
    let object = ObjectBytes::from_bytes(&original.0);
    predicate.evaluate(&object, &StaticWords::new(0), parameter)
}

/// A deliberately wrong version of one instance's predicate.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The tested field is four bytes further on.
    ShiftedField,
    /// The comparison is reversed.
    ReversedCompare,
    /// The right-hand side is one away from the right one.
    WrongRhs,
}

const WRONG_KINDS: [Wrong; 3] = [Wrong::ShiftedField, Wrong::ReversedCompare, Wrong::WrongRhs];

fn wrong_predicate(wrong: Wrong, predicate: Predicate) -> Predicate {
    match wrong {
        Wrong::ShiftedField => match predicate.source {
            Source::Field(offset) => Predicate { source: Source::Field(offset + 4), ..predicate },
            Source::Global(_) => predicate,
        },
        Wrong::ReversedCompare => Predicate {
            compare: match predicate.compare {
                Compare::Equal => Compare::NotEqual,
                Compare::NotEqual => Compare::Equal,
                Compare::SignedGreater => Compare::Equal,
            },
            ..predicate
        },
        Wrong::WrongRhs => Predicate {
            rhs: match predicate.rhs {
                Rhs::Constant(value) => Rhs::Constant(value.wrapping_add(1)),
                Rhs::Parameter => Rhs::Constant(1),
            },
            ..predicate
        },
    }
}

/// The first case, if any, at which the lifted predicate and the rewrite disagree.
fn first_difference(named: &Named<Predicate>, instance: usize, cases: u32) -> Option<String> {
    for case in 0..cases {
        let (mut object, mut rng) = random_object(instance, case);
        let parameter = inputs(named, &mut object, &mut rng, case);
        let original = Object(object.0.clone());
        let mut rewritten = Object(object.0.clone());
        let want = run_rewrite(instance, &mut rewritten, parameter);
        let got = run_lifted(named.spec, &original, parameter);
        if want != got {
            return Some(format!("case {case}: rewrite returned {want}, lifted returned {got}"));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &Named<Predicate>, instance: usize) -> bool {
    let bad = wrong_predicate(wrong, named.spec);
    for case in 0..WRONG_CASES {
        let (mut object, mut rng) = random_object(instance, case);
        let parameter = inputs(named, &mut object, &mut rng, case);
        let original = Object(object.0.clone());
        let mut rewritten = Object(object.0.clone());
        let want = run_rewrite(instance, &mut rewritten, parameter);
        if run_lifted(bad, &original, parameter) != want {
            return true;
        }
    }
    false
}

#[test]
fn every_instance_matches_its_verified_rewrite() {
    let mut failures = Vec::new();
    for (instance, named) in INSTANCES.iter().enumerate() {
        if let Some(detail) = first_difference(named, instance, CASES) {
            failures.push(format!("{} (instance {instance}): {detail}", named.name));
        }
    }
    assert!(failures.is_empty(), "instances that differ from their rewrite:\n{}", failures.join("\n"));
}

#[test]
fn every_instance_catches_every_wrong_version() {
    let mut escaped = Vec::new();
    for (instance, named) in INSTANCES.iter().enumerate() {
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
    assert_eq!(CALLS.len(), INSTANCES.len());
}
