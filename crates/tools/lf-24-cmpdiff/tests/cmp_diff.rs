//! Differential tests: every field comparison of this crate against its verified 32-bit rewrite, through the new
//! lifted `FieldCompare::evaluate`.
//!
//! Each instance is run through the rewrite and through the lifted routine on the same generated object. The
//! compared field is set to the constant, to its neighbours in the type's order, to a not-a-number, to the
//! negated constant, or left random, so that every answer occurs. The answer is compared as a word.
//! Deliberately wrong versions must each be caught by every instance: the field four bytes on, the comparison
//! reversed to its opposite, and the constant one unit away.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from a fixed seed.

#![cfg(target_arch = "x86")]

use lf_24_cmpdiff::calls::CALLS;
use lf_24_cmpdiff::instances::{INSTANCES, Named};
use lf_core::field_compare::{FieldCompare, Operand};
use lf_core::field_store::ObjectBytes;

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

/// The field value of one case, chosen so that the comparison answers both ways across cases.
fn field_word(test: FieldCompare, case: u32, rng: &mut Rng) -> u32 {
    let random = rng.next_u32();
    let constant = test.constant;
    match test.operand {
        Operand::Float => match case % 6 {
            0 => constant,
            1 => constant.wrapping_add(1),
            2 => constant.wrapping_sub(1),
            3 => 0x7FC0_0000,
            4 => constant ^ 0x8000_0000,
            _ => random,
        },
        Operand::Signed => match case % 4 {
            0 => constant,
            1 => constant.wrapping_add(1),
            2 => constant.wrapping_sub(1),
            _ => random,
        },
    }
}

/// The rewrite on one object: its answer as a word.
fn run_rewrite(instance: usize, object: &mut Object) -> u32 {
    CALLS[instance](object.0.as_mut_ptr(), 0, 0)
}

/// The lifted comparison on a copy of the object.
fn run_lifted(test: FieldCompare, original: &Object) -> u32 {
    test.evaluate(&ObjectBytes::from_bytes(&original.0))
}

/// The object of one case, with the compared field set.
fn inputs(test: FieldCompare, instance: usize, case: u32) -> Object {
    let (mut object, mut rng) = random_object(instance, case);
    let word = field_word(test, case, &mut rng);
    put_word(&mut object, test.offset, word);
    object
}

/// A deliberately wrong version of one instance's comparison.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The field is four bytes further on.
    ShiftedField,
    /// The comparison is the opposite one.
    ReversedAnswer,
    /// The constant is one unit away.
    ShiftedConstant,
}

const WRONG_KINDS: [Wrong; 3] = [
    Wrong::ShiftedField,
    Wrong::ReversedAnswer,
    Wrong::ShiftedConstant,
];

fn run_wrong(wrong: Wrong, test: FieldCompare, original: &Object) -> u32 {
    match wrong {
        Wrong::ShiftedField => run_lifted(
            FieldCompare {
                offset: test.offset + 4,
                ..test
            },
            original,
        ),
        Wrong::ReversedAnswer => run_lifted(
            FieldCompare {
                negate: !test.negate,
                ..test
            },
            original,
        ),
        Wrong::ShiftedConstant => run_lifted(
            FieldCompare {
                constant: test.constant.wrapping_add(1),
                ..test
            },
            original,
        ),
    }
}

/// The first case, if any, at which the lifted comparison and the rewrite disagree.
fn first_difference(named: &Named<FieldCompare>, instance: usize, cases: u32) -> Option<String> {
    for case in 0..cases {
        let mut object = inputs(named.spec, instance, case);
        let original = Object(object.0.clone());
        let want = run_rewrite(instance, &mut object);
        let got = run_lifted(named.spec, &original);
        if want != got {
            return Some(format!(
                "case {case}: rewrite returned {want}, lifted returned {got}"
            ));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &Named<FieldCompare>, instance: usize) -> bool {
    for case in 0..WRONG_CASES {
        let mut object = inputs(named.spec, instance, case);
        let original = Object(object.0.clone());
        let want = run_rewrite(instance, &mut object);
        if run_wrong(wrong, named.spec, &original) != want {
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
    assert!(
        failures.is_empty(),
        "instances that differ from their rewrite:\n{}",
        failures.join("\n")
    );
}

#[test]
fn every_instance_catches_every_wrong_version() {
    let mut escaped = Vec::new();
    for (instance, named) in INSTANCES.iter().enumerate() {
        for wrong in WRONG_KINDS {
            if !wrong_is_caught(wrong, named, instance) {
                escaped.push(format!(
                    "{} (instance {instance}): {wrong:?} was not caught",
                    named.name
                ));
            }
        }
    }
    assert!(
        escaped.is_empty(),
        "wrong versions that escaped:\n{}",
        escaped.join("\n")
    );
}

#[test]
fn the_call_table_and_the_instance_table_have_the_same_length() {
    assert_eq!(CALLS.len(), INSTANCES.len());
}
