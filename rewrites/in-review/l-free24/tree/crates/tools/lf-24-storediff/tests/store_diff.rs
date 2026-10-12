//! Differential tests: every field-store routine of this crate against its verified 32-bit rewrite.
//!
//! Each instance is run through the rewrite and through the existing lifted `ObjectBytes::store` on the same
//! generated object and parameter. Every byte of the object is compared, and the returned value is compared
//! where the routine returns one. Deliberately wrong versions must each be caught by every instance: the store
//! at an offset four bytes on, and the store at the right offset with the field one width wider or narrower.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from a fixed seed.

#![cfg(target_arch = "x86")]

use lf_24_storediff::calls::CALLS;
use lf_24_storediff::instances::{INSTANCES, Named};
use lf_core::field_store::{FieldStore, FieldWidth, ObjectBytes, StoreResult};

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

/// The parameter of one case: random, so that every byte a wrong version writes differs.
fn parameter(rng: &mut Rng) -> u32 {
    rng.next_u32()
}

/// The rewrite on one object: its returned word, and the object's address as the rewrite saw it.
fn run_rewrite(instance: usize, object: &mut Object, value: u32) -> (u32, u32) {
    let this = object.0.as_mut_ptr() as usize as u32;
    let returned = CALLS[instance](object.0.as_mut_ptr(), value, 0);
    (returned, this)
}

/// The lifted store on a copy of the object: the result it hands back, when it returns one.
fn run_lifted(store: FieldStore, original: &Object, value: u32) -> (Option<u32>, ObjectBytes) {
    let mut object = ObjectBytes::from_bytes(&original.0);
    let returned = object.store(store, value);
    (returned, object)
}

/// The value the rewrite's word stands for, given the lifted result kind; `None` where the routine returns nothing.
fn comparable(result: StoreResult, returned: u32) -> Option<u32> {
    match result {
        StoreResult::Nothing => None,
        _ => Some(returned),
    }
}

/// A deliberately wrong version of one instance's store.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The store lands four bytes further on.
    ShiftedOffset,
    /// The field is one width wider or narrower than the rewrite writes it.
    OtherWidth,
}

const WRONG_KINDS: [Wrong; 2] = [Wrong::ShiftedOffset, Wrong::OtherWidth];

fn wrong_store(wrong: Wrong, store: FieldStore) -> FieldStore {
    match wrong {
        Wrong::ShiftedOffset => FieldStore { offset: store.offset + 4, ..store },
        Wrong::OtherWidth => FieldStore {
            width: match store.width {
                FieldWidth::Byte => FieldWidth::Word,
                FieldWidth::Word => FieldWidth::Byte,
            },
            ..store
        },
    }
}

/// The first case, if any, at which the lifted store and the rewrite disagree.
fn first_difference(named: &Named<FieldStore>, instance: usize, cases: u32) -> Option<String> {
    for case in 0..cases {
        let (original, mut rng) = random_object(instance, case);
        let value = parameter(&mut rng);
        let mut rewritten = Object(original.0.clone());
        let (want, this) = run_rewrite(instance, &mut rewritten, value);
        let _ = this;
        let (got, got_object) = run_lifted(named.spec, &original, value);
        if let Some(got_value) = got {
            if comparable(named.spec.result, want) != Some(got_value) {
                return Some(format!("case {case}: rewrite returned {want:#010x}, lifted returned {got_value:#010x}"));
            }
        } else if comparable(named.spec.result, want).is_some() {
            return Some(format!("case {case}: rewrite returned {want:#010x}, lifted returned nothing"));
        }
        if rewritten.0[..] != *got_object.bytes() {
            let at = rewritten.0.iter().zip(got_object.bytes().iter()).position(|(a, b)| a != b).unwrap_or(0);
            return Some(format!("case {case}: first differing byte at {at:#x}"));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &Named<FieldStore>, instance: usize) -> bool {
    let bad = wrong_store(wrong, named.spec);
    for case in 0..WRONG_CASES {
        let (original, mut rng) = random_object(instance, case);
        let value = parameter(&mut rng);
        let mut rewritten = Object(original.0.clone());
        let (want, _) = run_rewrite(instance, &mut rewritten, value);
        let (got, got_object) = run_lifted(bad, &original, value);
        let returns_differ = comparable(named.spec.result, want).is_some() && got != comparable(named.spec.result, want);
        if returns_differ || rewritten.0[..] != *got_object.bytes() {
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
