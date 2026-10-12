//! Differential tests: every byte-flag routine of this crate against its verified 32-bit rewrite.
//!
//! Each instance is run through the rewrite and through the new lifted `ByteFlagTest::evaluate` on the same
//! generated object. The tested byte is set to the mask, to zero, or left random, so that both answers occur.
//! Deliberately wrong versions must each be caught by every instance: the tested byte one further on, the
//! answer inverted, and the mask shifted one bit up.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from a fixed seed.

#![cfg(target_arch = "x86")]

use lf_24_maskdiff::calls::CALLS;
use lf_24_maskdiff::instances::{INSTANCES, Named};
use lf_core::byte_flag_test::ByteFlagTest;
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

/// Sets the tested byte of one case: the mask (answer yes), zero (answer no), or the random byte.
fn set_tested_byte(object: &mut Object, test: ByteFlagTest, case: u32) {
    object.0[test.offset] = match case % 3 {
        0 => test.mask,
        1 => 0,
        _ => object.0[test.offset],
    };
}

/// The rewrite on one object: its answer as a word.
fn run_rewrite(instance: usize, object: &mut Object) -> u32 {
    CALLS[instance](object.0.as_mut_ptr(), 0, 0)
}

/// The lifted test on a copy of the object.
fn run_lifted(test: ByteFlagTest, original: &Object) -> u32 {
    test.evaluate(&ObjectBytes::from_bytes(&original.0))
}

/// A deliberately wrong version of one instance's test.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The tested byte is one further on.
    ShiftedOffset,
    /// The answer is inverted.
    InvertedAnswer,
    /// The mask is shifted one bit up.
    ShiftedMask,
}

const WRONG_KINDS: [Wrong; 3] = [
    Wrong::ShiftedOffset,
    Wrong::InvertedAnswer,
    Wrong::ShiftedMask,
];

fn wrong_test(wrong: Wrong, test: ByteFlagTest) -> ByteFlagTest {
    match wrong {
        Wrong::ShiftedOffset => ByteFlagTest {
            offset: test.offset + 1,
            ..test
        },
        Wrong::InvertedAnswer => test,
        Wrong::ShiftedMask => ByteFlagTest {
            mask: test.mask.wrapping_shl(1),
            ..test
        },
    }
}

fn wrong_answer(wrong: Wrong, answer: u32) -> u32 {
    match wrong {
        Wrong::InvertedAnswer => 1 - answer,
        _ => answer,
    }
}

/// The first case, if any, at which the lifted test and the rewrite disagree.
fn first_difference(named: &Named<ByteFlagTest>, instance: usize, cases: u32) -> Option<String> {
    for case in 0..cases {
        let (mut object, _) = random_object(instance, case);
        set_tested_byte(&mut object, named.spec, case);
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
fn wrong_is_caught(wrong: Wrong, named: &Named<ByteFlagTest>, instance: usize) -> bool {
    let bad = wrong_test(wrong, named.spec);
    for case in 0..WRONG_CASES {
        let (mut object, _) = random_object(instance, case);
        set_tested_byte(&mut object, named.spec, case);
        let original = Object(object.0.clone());
        let want = run_rewrite(instance, &mut object);
        if wrong_answer(wrong, run_lifted(bad, &original)) != want {
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
