//! Differential test of the lifted flagged pair store against the four verified
//! rewrites of the family.
//!
//! Each instance is one verified rewrite. For each instance, CASES generated
//! objects are built in a block of this test program, and two parameter words
//! are drawn. The rewrite runs on the block and `FlaggedPairStore::apply` runs
//! on a copy of the same bytes. The returned value and every byte of the block
//! are compared, so every store (the two parameter words and the flag byte) must
//! be made at the same place with the same value, and nothing else may change.
//!
//! Each instance is also run against three deliberately wrong versions of its
//! own parameters (the first field moved by four bytes, the flag's constant
//! changed, the returned value changed); every one must be caught on every
//! instance.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{Call, Instance, INSTANCES};
use lf_core::flagged_pair_store::{FlagAction, FlaggedPairStore};

/// Generated objects per instance.
const CASES: u32 = 1000;

/// The object block: large enough for every field and flag of the four rewrites.
const OBJECT_LEN: usize = 0x400;

/// One lock across the whole test program.
static LOCK: Mutex<()> = Mutex::new(());

/// A block of object memory, aligned for the rewrites' word stores.
#[repr(C, align(8))]
struct Object([u8; OBJECT_LEN]);

/// A deterministic generator (xorshift64), seeded per instance.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

/// The outcome of one instance's cases against one store.
struct Outcome {
    /// Cases run.
    cases: u32,
    /// Cases whose returned value or block bytes differed.
    mismatches: u32,
    /// The first failing case, described for a reader.
    first: Option<String>,
}

/// Calls the rewrite with the object and the two parameters, in its own form.
fn call_rewrite(call: Call, object: u32, first: u32, second: u32) -> u32 {
    match call {
        Call::Two(f) => f(object, first, second),
        Call::Four(f) => f(object, first, second, 0, 0),
    }
}

/// Runs `CASES` generated objects through the rewrite and through the lifted
/// store built from `store`, and compares them.
fn diff(instance: &Instance, store: &FlaggedPairStore, seed: u64) -> Outcome {
    let mut rng = Rng(seed);
    let mut outcome = Outcome { cases: 0, mismatches: 0, first: None };
    let mut object = Box::new(Object([0; OBJECT_LEN]));
    for case in 0..CASES {
        for byte in object.0.iter_mut() {
            *byte = rng.next() as u8;
        }
        let first = rng.next() as u32;
        let second = rng.next() as u32;
        let before = object.0.to_vec();
        let address = &*object as *const Object as usize as u32;
        let expected = call_rewrite(instance.call, address, first, second);

        let mut lifted_block = lf_core::field_store::ObjectBytes::from_bytes(&before);
        let got = store.apply(&mut lifted_block, first, second);
        outcome.cases += 1;

        let problem = if got != expected {
            Some(format!("rewrite returned {expected:#010x}, lifted {got:#010x}"))
        } else if lifted_block.bytes() != &object.0[..] {
            let at = lifted_block
                .bytes()
                .iter()
                .zip(object.0.iter())
                .position(|(a, b)| a != b)
                .unwrap_or(0);
            Some(format!("the object differs first at byte {at:#x}"))
        } else {
            None
        };
        if let Some(text) = problem {
            outcome.mismatches += 1;
            if outcome.first.is_none() {
                outcome.first = Some(format!("case {case}: {text}"));
            }
        }
    }
    outcome
}

/// The three wrong versions of an instance's store, each a single change.
fn wrong_versions(store: &FlaggedPairStore) -> [(&'static str, FlaggedPairStore); 3] {
    let flag = match store.flag {
        FlagAction::Set { offset, value } => FlagAction::Set { offset, value: value ^ 1 },
        FlagAction::Or { offset, mask } => FlagAction::Or { offset, mask: mask ^ 0x10 },
    };
    [
        ("first field plus four", FlaggedPairStore { first_offset: store.first_offset + 4, ..*store }),
        ("flag constant changed", FlaggedPairStore { flag, ..*store }),
        ("returned value changed", FlaggedPairStore { returns: store.returns ^ 1, ..*store }),
    ]
}

/// A seed per instance, from its position in the list.
fn seed_of(index: usize) -> u64 {
    0x7F4A_7C15_9E37_79B9 ^ ((index as u64 + 1) * 0x100_0000_01B3)
}

#[test]
fn lifted_flagged_store_matches_every_verified_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut total = 0;
    for (index, instance) in INSTANCES.iter().enumerate() {
        let outcome = diff(instance, &instance.store, seed_of(index));
        eprintln!(
            "{} {} cases {} mismatches {} first {:?}",
            instance.address, instance.name, outcome.cases, outcome.mismatches, outcome.first
        );
        total += outcome.mismatches;
    }
    assert_eq!(total, 0, "the lifted flagged store differs from a verified rewrite");
}

#[test]
fn wrong_versions_are_caught_on_every_instance() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    for (index, instance) in INSTANCES.iter().enumerate() {
        for (label, wrong) in wrong_versions(&instance.store) {
            let outcome = diff(instance, &wrong, seed_of(index));
            eprintln!(
                "teeth {} {} [{}] mismatches {} of {}",
                instance.address, instance.name, label, outcome.mismatches, outcome.cases
            );
            assert!(
                outcome.mismatches > 0,
                "{} {}: wrong version [{}] was not caught",
                instance.address,
                instance.name,
                label
            );
        }
    }
}
