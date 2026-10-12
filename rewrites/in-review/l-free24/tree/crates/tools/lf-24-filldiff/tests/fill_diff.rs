//! Differential tests: every table fill of this crate against its verified 32-bit rewrite, through the existing
//! lifted `WordFill::apply` of `lf_core::const_fill`.
//!
//! Each instance's table is the test region: the region is filled with random words, the rewrite runs, and the
//! lifted routine runs on a copy of the same words with the instance's word indices as targets. Every word of
//! the region is compared, and the returned word where the routine returns one. Deliberately wrong versions
//! must each be caught by every instance: one word fewer filled, the fill constant with one bit flipped, and
//! the table one word further on.
//!
//! The test runs on the 32-bit target only. Its inputs come from a fixed seed, and one lock serialises the
//! tests, since they share the region.

#![cfg(target_arch = "x86")]

use std::sync::Mutex;

use lf_24_filldiff::calls::CALLS;
use lf_24_filldiff::instances::{Fill, INSTANCES, Named, Returns};
use lf_24_filldiff::rt::{REGION_WORDS, fill_region, region_words, set_region};
use lf_core::const_fill::WordFill;

/// Cases per instance for the differential comparison.
const CASES: u32 = 1000;
/// Cases per instance and wrong version; a wrong version must be caught by one of them.
const WRONG_CASES: u32 = 64;

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

/// The region's starting words for one case: random, so that every word a wrong version leaves alone differs.
fn starting_words(instance: usize, case: u32) -> Vec<u32> {
    let seed = ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    (0..REGION_WORDS).map(|_| rng.next_u32()).collect()
}

/// The rewrite on the region: its returned word, and the region afterwards.
fn run_rewrite(instance: usize, base: u32, words: &[u32]) -> (u32, Vec<u32>) {
    set_region(base);
    fill_region(words);
    let returned = CALLS[instance](core::ptr::null_mut(), 0, 0);
    (returned, region_words())
}

/// The lifted fill on a copy of the starting words, over the instance's word indices.
fn run_lifted(words: &[u32], first: usize, count: usize, value: u32) -> (u32, Vec<u32>) {
    let mut table = words.to_vec();
    let targets: Vec<usize> = (first..first + count).collect();
    WordFill::new(value).apply(&mut table, &targets);
    (0, table)
}

/// The value the lifted routine returns for an instance's return kind.
fn lifted_return(returns: Returns, value: u32, lifted: u32) -> Option<u32> {
    match returns {
        Returns::Zero => Some(lifted),
        Returns::Fill => Some(value),
        Returns::Unit => None,
    }
}

/// A deliberately wrong version of one instance's fill.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// One word fewer is filled: the last one is left alone.
    DroppedLast,
    /// The fill constant has one bit flipped.
    ChangedValue,
    /// The table starts one word further on.
    ShiftedBase,
}

const WRONG_KINDS: [Wrong; 3] = [Wrong::DroppedLast, Wrong::ChangedValue, Wrong::ShiftedBase];

fn run_wrong(wrong: Wrong, spec: Fill, words: &[u32]) -> Vec<u32> {
    let s = spec;
    match wrong {
        Wrong::DroppedLast => run_lifted(words, 0, s.count - 1, s.value).1,
        Wrong::ChangedValue => run_lifted(words, 0, s.count, s.value ^ 1).1,
        Wrong::ShiftedBase => run_lifted(words, 1, s.count, s.value).1,
    }
}

/// The first case, if any, at which the lifted fill and the rewrite disagree.
fn first_difference(named: &Named<Fill>, instance: usize, cases: u32) -> Option<String> {
    let s = named.spec;
    for case in 0..cases {
        let words = starting_words(instance, case);
        let (want_return, want_words) = run_rewrite(instance, s.base, &words);
        let (lifted_ret, got_words) = run_lifted(&words, 0, s.count, s.value);
        if let Some(expected) = lifted_return(s.returns, s.value, lifted_ret) {
            if want_return != expected {
                return Some(format!("case {case}: rewrite returned {want_return:#010x}, lifted returned {expected:#010x}"));
            }
        }
        if want_words != got_words {
            let at = want_words.iter().zip(got_words.iter()).position(|(a, b)| a != b).unwrap_or(0);
            return Some(format!("case {case}: first differing word at index {at}"));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &Named<Fill>, instance: usize) -> bool {
    for case in 0..WRONG_CASES {
        let words = starting_words(instance, case);
        let (_, want_words) = run_rewrite(instance, named.spec.base, &words);
        if run_wrong(wrong, named.spec, &words) != want_words {
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
        if let Some(detail) = first_difference(named, instance, CASES) {
            failures.push(format!("{} (instance {instance}): {detail}", named.name));
        }
    }
    assert!(failures.is_empty(), "instances that differ from their rewrite:\n{}", failures.join("\n"));
}

#[test]
fn every_instance_catches_every_wrong_version() {
    let _guard = LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
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
