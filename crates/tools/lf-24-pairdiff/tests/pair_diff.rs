//! Differential tests: every field-pair store of this crate against its verified 32-bit rewrite.
//!
//! Each instance is run through the rewrite and through the new lifted `FieldPairStore::apply` on the same
//! generated object and parameters. Every byte of the object is compared, and the returned word too. Deliberately
//! wrong versions must each be caught by every instance: the two parameters swapped, the first field one word
//! further on, and the return replaced by the other possible result.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from a fixed seed.

#![cfg(target_arch = "x86")]

use lf_24_pairdiff::calls::CALLS;
use lf_24_pairdiff::instances::{INSTANCES, Named};
use lf_core::field_pair_store::{FieldPairStore, PairResult};
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

/// The two parameters of one case: random, so that every byte a wrong version writes differs.
fn parameters(rng: &mut Rng) -> (u32, u32) {
    (rng.next_u32(), rng.next_u32())
}

/// The rewrite on one object: its returned word.
fn run_rewrite(instance: usize, object: &mut Object, first: u32, second: u32) -> u32 {
    CALLS[instance](object.0.as_mut_ptr(), first, second)
}

/// The lifted store on a copy of the object: its returned word and the object after the call.
fn run_lifted(
    store: FieldPairStore,
    original: &Object,
    first: u32,
    second: u32,
) -> (u32, ObjectBytes) {
    let mut object = ObjectBytes::from_bytes(&original.0);
    let returned = store.apply(&mut object, first, second);
    (returned, object)
}

/// A deliberately wrong version of one instance's store.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The parameters are written the other way round.
    SwappedValues,
    /// The first field is four bytes further on.
    ShiftedFirst,
    /// The return is the other possible result.
    OtherReturn,
}

const WRONG_KINDS: [Wrong; 3] = [
    Wrong::SwappedValues,
    Wrong::ShiftedFirst,
    Wrong::OtherReturn,
];

fn run_wrong(
    wrong: Wrong,
    store: FieldPairStore,
    original: &Object,
    first: u32,
    second: u32,
) -> (u32, ObjectBytes) {
    match wrong {
        Wrong::SwappedValues => run_lifted(store, original, second, first),
        Wrong::ShiftedFirst => run_lifted(
            FieldPairStore {
                first_offset: store.first_offset + 4,
                ..store
            },
            original,
            first,
            second,
        ),
        Wrong::OtherReturn => {
            let (returned, object) = run_lifted(store, original, first, second);
            let other = match store.result {
                PairResult::Second => 0,
                PairResult::Zero => second,
            };
            let _ = returned;
            (other, object)
        }
    }
}

/// The first case, if any, at which the lifted store and the rewrite disagree.
fn first_difference(named: &Named<FieldPairStore>, instance: usize, cases: u32) -> Option<String> {
    for case in 0..cases {
        let (original, mut rng) = random_object(instance, case);
        let (first, second) = parameters(&mut rng);
        let mut rewritten = Object(original.0.clone());
        let want = run_rewrite(instance, &mut rewritten, first, second);
        let (got, got_object) = run_lifted(named.spec, &original, first, second);
        if want != got {
            return Some(format!(
                "case {case}: rewrite returned {want:#010x}, lifted returned {got:#010x}"
            ));
        }
        if rewritten.0[..] != *got_object.bytes() {
            let at = rewritten
                .0
                .iter()
                .zip(got_object.bytes().iter())
                .position(|(a, b)| a != b)
                .unwrap_or(0);
            return Some(format!("case {case}: first differing byte at {at:#x}"));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &Named<FieldPairStore>, instance: usize) -> bool {
    for case in 0..WRONG_CASES {
        let (original, mut rng) = random_object(instance, case);
        let (first, second) = parameters(&mut rng);
        let mut rewritten = Object(original.0.clone());
        let want = run_rewrite(instance, &mut rewritten, first, second);
        let (got, got_object) = run_wrong(wrong, named.spec, &original, first, second);
        if want != got || rewritten.0[..] != *got_object.bytes() {
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
