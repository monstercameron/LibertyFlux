//! Differential tests: every lifted field store against its verified 32-bit rewrite.
//!
//! For each instance of `lf_core::field_store::instances::STORES`, the test runs the lifted
//! `ObjectBytes::store` and the verified rewrite on the same generated object and the same value. The
//! objects are compared byte for byte after the call, and the returned values are compared (`None` when
//! the rewrite returns nothing). Every instance runs `CASES` cases; the first few cases use edge values.
//!
//! Deliberately wrong versions run through the same contract. Each one must be caught by every instance:
//! the store at an offset four bytes further on, the store left out, a result that differs from the
//! rewrite's, and the store at the other width.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs are generated from a
//! fixed seed, so every run sees the same cases.

#![cfg(target_arch = "x86")]

use lf_core::field_store::instances::{NamedStore, STORES};
use lf_core::field_store::{FieldStore, FieldWidth, ObjectBytes};
use lf_setdiff::calls::CALLS;

/// Bytes in the generated object. Every field the instances write lies below this.
const OBJECT_BYTES: usize = 0x2000;
/// Cases per instance for the differential comparison.
const CASES: u32 = 1000;
/// Cases per instance and wrong version; a wrong version must be caught by one of them.
const WRONG_CASES: u32 = 64;

/// The object the rewrite writes into. Aligned, so word fields at 4-byte offsets are aligned as in the
/// original.
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

/// The object and the value for one case of one instance. Cases 0 to 3 use edge values; the rest are
/// random. The object is filled with random bytes so that every byte a wrong version writes differs.
fn case_inputs(instance: usize, case: u32) -> (Object, u32) {
    let seed =
        ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    let mut object = Object([0; OBJECT_BYTES]);
    for chunk in object.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    let value = match case {
        0 => 0,
        1 => u32::MAX,
        2 => 0x80,
        3 => 0x0000_0001,
        _ => rng.next_u32(),
    };
    (object, value)
}

/// The rewrite's result on one case: the object after the call and what it returned.
fn run_rewrite(instance: usize, object: &Object, value: u32) -> (Object, Option<u32>) {
    let mut after = object.clone();
    let returned = CALLS[instance](after.0.as_mut_ptr(), value);
    (after, returned)
}

/// The lifted result on one case, with the object after the call.
fn run_lifted(store: FieldStore, object: &Object, value: u32) -> (Object, Option<u32>) {
    let mut lifted = ObjectBytes::from_bytes(&object.0);
    let returned = lifted.store(store, value);
    let mut after = Object([0; OBJECT_BYTES]);
    after.0.copy_from_slice(lifted.bytes());
    (after, returned)
}

/// The field's byte range for a store.
fn field_range(store: FieldStore) -> std::ops::Range<usize> {
    let width = match store.width {
        FieldWidth::Byte => 1,
        FieldWidth::Word => 4,
    };
    store.offset..store.offset + width
}

/// A deliberately wrong version of one instance's lifted store.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The store lands four bytes further on than the field.
    OffsetPlusFour,
    /// The store is left out: the field keeps the bytes it had.
    MissingStore,
    /// The result differs from the rewrite's: a returned value is changed, and a routine that returns
    /// nothing returns the value.
    WrongResult,
    /// The store writes the other width.
    OtherWidth,
}

const WRONG_KINDS: [Wrong; 4] = [
    Wrong::OffsetPlusFour,
    Wrong::MissingStore,
    Wrong::WrongResult,
    Wrong::OtherWidth,
];

/// The lifted store run as the wrong version describes.
fn run_wrong(
    wrong: Wrong,
    store: FieldStore,
    object: &Object,
    value: u32,
) -> (Object, Option<u32>) {
    match wrong {
        Wrong::OffsetPlusFour => run_lifted(
            FieldStore {
                offset: store.offset + 4,
                ..store
            },
            object,
            value,
        ),
        Wrong::MissingStore => {
            let (mut after, returned) = run_lifted(store, object, value);
            let range = field_range(store);
            after.0[range.clone()].copy_from_slice(&object.0[range]);
            (after, returned)
        }
        Wrong::WrongResult => {
            let (after, returned) = run_lifted(store, object, value);
            let changed = match returned {
                Some(result) => Some(result ^ 1),
                None => Some(value),
            };
            (after, changed)
        }
        Wrong::OtherWidth => {
            let width = match store.width {
                FieldWidth::Byte => FieldWidth::Word,
                FieldWidth::Word => FieldWidth::Byte,
            };
            run_lifted(FieldStore { width, ..store }, object, value)
        }
    }
}

/// The first case, if any, at which the lifted store and the rewrite disagree.
fn first_difference(named: &NamedStore, instance: usize, cases: u32) -> Option<String> {
    for case in 0..cases {
        let (object, value) = case_inputs(instance, case);
        let (want_object, want_result) = run_rewrite(instance, &object, value);
        let (got_object, got_result) = run_lifted(named.store, &object, value);
        if want_result != got_result {
            return Some(format!(
                "case {case}: value {value:#010x}, rewrite returned {want_result:?}, lifted returned {got_result:?}"
            ));
        }
        if want_object.0 != got_object.0 {
            let at = want_object
                .0
                .iter()
                .zip(got_object.0.iter())
                .position(|(a, b)| a != b)
                .unwrap_or(0);
            return Some(format!(
                "case {case}: value {value:#010x}, first differing byte at {at:#x}"
            ));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &NamedStore, instance: usize) -> bool {
    for case in 0..WRONG_CASES {
        let (object, value) = case_inputs(instance, case);
        let (want_object, want_result) = run_rewrite(instance, &object, value);
        let (got_object, got_result) = run_wrong(wrong, named.store, &object, value);
        if want_result != got_result || want_object.0 != got_object.0 {
            return true;
        }
    }
    false
}

#[test]
fn every_instance_matches_its_verified_rewrite() {
    let mut failures = Vec::new();
    for (instance, named) in STORES.iter().enumerate() {
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
    for (instance, named) in STORES.iter().enumerate() {
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
    assert_eq!(CALLS.len(), STORES.len());
}
