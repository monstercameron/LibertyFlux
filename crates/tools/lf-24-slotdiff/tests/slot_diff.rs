//! Differential tests: every object-to-global copy of this crate against its verified 32-bit rewrite, through the
//! new lifted `FieldSlotCopy::apply`.
//!
//! Each instance is run through the rewrite and through the lifted routine on the same generated object and the
//! same region. The whole region is compared, and the returned word. Deliberately wrong versions must each be
//! caught by every instance: the field four bytes on, the slot one word on, and the return one.
//!
//! The test runs on the 32-bit target only. Its inputs come from a fixed seed, and one lock serialises the
//! tests, since they share the region.

#![cfg(target_arch = "x86")]

use std::sync::Mutex;

use lf_24_slotdiff::calls::CALLS;
use lf_24_slotdiff::instances::{INSTANCES, Named};
use lf_24_slotdiff::rt::{REGION_WORDS, fill_region, region_words, set_region};
use lf_core::field_slot_copy::FieldSlotCopy;
use lf_core::field_store::ObjectBytes;

/// Cases per instance for the differential comparison.
const CASES: u32 = 1000;
/// Cases per instance and wrong version; a wrong version must be caught by one of them.
const WRONG_CASES: u32 = 64;
/// Bytes in the generated object.
const OBJECT_BYTES: usize = 0x2000;

/// Serialises the tests: they share the region.
static LOCK: Mutex<()> = Mutex::new(());

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

/// The object and the region's starting words for one case: random, so every wrong write differs.
fn case_inputs(instance: usize, case: u32) -> (Vec<u8>, Vec<u32>) {
    let seed =
        ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    let object = (0..OBJECT_BYTES).map(|_| rng.next_u32() as u8).collect();
    let words = (0..REGION_WORDS).map(|_| rng.next_u32()).collect();
    (object, words)
}

/// The rewrite on the region with the object: its returned word, and the region afterwards.
fn run_rewrite(instance: usize, base: u32, object: &mut [u8], words: &[u32]) -> (u32, Vec<u32>) {
    set_region(base);
    fill_region(words);
    let returned = CALLS[instance](object.as_mut_ptr(), 0, 0);
    (returned, region_words())
}

/// The lifted copy on a copy of the object and the starting words.
fn run_lifted(copy: FieldSlotCopy, object: &[u8], words: &[u32]) -> (u32, Vec<u32>) {
    let mut table = words.to_vec();
    let returned = copy.apply(&ObjectBytes::from_bytes(object), &mut table);
    (returned, table)
}

/// A deliberately wrong version of one instance's copy.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The field is four bytes further on.
    ShiftedField,
    /// The word lands one slot further on.
    ShiftedSlot,
    /// The return is one.
    OtherReturn,
}

const WRONG_KINDS: [Wrong; 3] = [Wrong::ShiftedField, Wrong::ShiftedSlot, Wrong::OtherReturn];

fn run_wrong(wrong: Wrong, copy: FieldSlotCopy, object: &[u8], words: &[u32]) -> (u32, Vec<u32>) {
    match wrong {
        Wrong::ShiftedField => run_lifted(
            FieldSlotCopy {
                offset: copy.offset + 4,
                ..copy
            },
            object,
            words,
        ),
        Wrong::ShiftedSlot => run_lifted(
            FieldSlotCopy {
                slot: copy.slot + 1,
                ..copy
            },
            object,
            words,
        ),
        Wrong::OtherReturn => {
            let (_, table) = run_lifted(copy, object, words);
            (1, table)
        }
    }
}

/// The first case, if any, at which the lifted copy and the rewrite disagree.
fn first_difference(
    named: &Named<FieldSlotCopy>,
    base: u32,
    instance: usize,
    cases: u32,
) -> Option<String> {
    for case in 0..cases {
        let (mut object, words) = case_inputs(instance, case);
        let (want_return, want_words) = run_rewrite(instance, base, &mut object, &words);
        let (got_return, got_words) = run_lifted(named.spec, &object, &words);
        if want_return != got_return {
            return Some(format!(
                "case {case}: rewrite returned {want_return:#010x}, lifted returned {got_return:#010x}"
            ));
        }
        if want_words != got_words {
            let at = want_words
                .iter()
                .zip(got_words.iter())
                .position(|(a, b)| a != b)
                .unwrap_or(0);
            return Some(format!("case {case}: first differing word at index {at}"));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &Named<FieldSlotCopy>, base: u32, instance: usize) -> bool {
    for case in 0..WRONG_CASES {
        let (mut object, words) = case_inputs(instance, case);
        let (want_return, want_words) = run_rewrite(instance, base, &mut object, &words);
        if run_wrong(wrong, named.spec, &object, &words) != (want_return, want_words) {
            return true;
        }
    }
    false
}

#[test]
fn every_instance_matches_its_verified_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut failures = Vec::new();
    for (instance, named) in INSTANCES.iter().enumerate() {
        if let Some(detail) = first_difference(named, REGION_BASE, instance, CASES) {
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
    let _guard = LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut escaped = Vec::new();
    for (instance, named) in INSTANCES.iter().enumerate() {
        for wrong in WRONG_KINDS {
            if !wrong_is_caught(wrong, named, REGION_BASE, instance) {
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

/// The first global address of the family's region, which every instance's global lies after.
const REGION_BASE: u32 = 0x0103_B760;
