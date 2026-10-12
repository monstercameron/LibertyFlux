//! Differential tests: every packed low-byte flag routine of this crate against its verified 32-bit rewrite.
//!
//! Each instance is run through the rewrite and through the new lifted `PackedFlag::read` on the same generated
//! object and parameter. The word is chosen from the instance's own condition, so that both answers occur: a
//! value inside the limit, a value equal to one of the constants, a word with the tested bits set and clear,
//! or a random word. Deliberately wrong versions must each be caught by every instance: the base one away from
//! the right one, the answer inverted, and the upper three bytes dropped.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from a fixed seed.

#![cfg(target_arch = "x86")]

use lf_24_packdiff::calls::CALLS;
use lf_24_packdiff::instances::{INSTANCES, Named};
use lf_core::field_store::ObjectBytes;
use lf_core::packed_flag::{Condition, PackedFlag, Source};

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

/// The word of one case, chosen from the condition so that the answers differ across cases.
fn candidate_word(flag: PackedFlag, case: u32, rng: &mut Rng) -> u32 {
    let random = rng.next_u32();
    match flag.condition {
        Condition::AtMost(limit) => {
            let span = u64::from(limit) + 3;
            let value = (u64::from(random) % span) as u32;
            if case % 2 == 0 {
                value.wrapping_add(flag.base)
            } else {
                random
            }
        }
        Condition::OneOf(first, second) => match case % 4 {
            0 => first,
            1 => second,
            2 => first.wrapping_add(1),
            _ => random,
        },
        Condition::SetAndClear { set, clear } => {
            if case % 2 == 0 {
                (random | set) & !clear
            } else {
                random
            }
        }
    }
}

/// The rewrite on one object: its returned word.
fn run_rewrite(instance: usize, object: &mut Object, parameter: u32) -> u32 {
    CALLS[instance](object.0.as_mut_ptr(), parameter, 0)
}

/// The lifted routine on a copy of the object.
fn run_lifted(flag: PackedFlag, original: &Object, parameter: u32) -> u32 {
    flag.read(&ObjectBytes::from_bytes(&original.0), parameter)
}

/// The object and the parameter of one case: the word goes where the routine reads it.
fn inputs(flag: PackedFlag, instance: usize, case: u32) -> (Object, u32) {
    let (mut object, mut rng) = random_object(instance, case);
    let word = candidate_word(flag, case, &mut rng);
    match flag.source {
        Source::Parameter => (object, word),
        Source::Field(offset) => {
            put_word(&mut object, offset, word);
            (object, 0)
        }
    }
}

/// A deliberately wrong version of one instance's routine.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The base is one away from the right one.
    WrongBase,
    /// The answer is inverted.
    InvertedAnswer,
    /// The upper three bytes are dropped.
    DroppedUpper,
}

const WRONG_KINDS: [Wrong; 3] = [Wrong::WrongBase, Wrong::InvertedAnswer, Wrong::DroppedUpper];

fn wrong_result(wrong: Wrong, flag: PackedFlag, original: &Object, parameter: u32) -> u32 {
    match wrong {
        Wrong::WrongBase => run_lifted(
            PackedFlag {
                base: flag.base.wrapping_add(1),
                ..flag
            },
            original,
            parameter,
        ),
        Wrong::InvertedAnswer => run_lifted(flag, original, parameter) ^ 1,
        Wrong::DroppedUpper => run_lifted(flag, original, parameter) & 1,
    }
}

/// The first case, if any, at which the lifted routine and the rewrite disagree.
fn first_difference(named: &Named<PackedFlag>, instance: usize, cases: u32) -> Option<String> {
    for case in 0..cases {
        let (mut object, parameter) = inputs(named.spec, instance, case);
        let original = Object(object.0.clone());
        let want = run_rewrite(instance, &mut object, parameter);
        let got = run_lifted(named.spec, &original, parameter);
        if want != got {
            return Some(format!(
                "case {case}: rewrite returned {want:#010x}, lifted returned {got:#010x}"
            ));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &Named<PackedFlag>, instance: usize) -> bool {
    for case in 0..WRONG_CASES {
        let (mut object, parameter) = inputs(named.spec, instance, case);
        let original = Object(object.0.clone());
        let want = run_rewrite(instance, &mut object, parameter);
        if wrong_result(wrong, named.spec, &original, parameter) != want {
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
