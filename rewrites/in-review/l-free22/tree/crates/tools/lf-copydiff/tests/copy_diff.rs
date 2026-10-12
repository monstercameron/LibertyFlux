//! Differential tests: every lifted word copy against its verified 32-bit rewrite.
//!
//! For each instance of `lf_core::word_copy::instances::COPIES`, the test sets the same generated values in
//! the runtime's words and in a lifted table, runs the lifted copy and the verified rewrite, and compares
//! every word of the table and the returned value. Each instance runs `CASES` cases; the first few cases use
//! edge values.
//!
//! Deliberately wrong versions run through the same contract, and each must be caught by every instance: the
//! copy into the wrong word, the copy in the wrong direction, a copy that is left out, and a result of one.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from a fixed seed.

#![cfg(target_arch = "x86")]

use lf_copydiff::calls::CALLS;
use lf_copydiff::rt;
use lf_core::static_list::{StaticWords, WordId};
use lf_core::word_copy::WordCopy;
use lf_core::word_copy::instances::{COPIES, NamedCopy, WORD_COUNT};

/// Cases per instance for the differential comparison.
const CASES: u32 = 1000;
/// Cases per instance and wrong version; a wrong version must be caught by one of them.
const WRONG_CASES: u32 = 64;

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

/// The starting value of every word for one case of one instance. Cases 0 to 3 use edge values.
fn case_values(instance: usize, case: u32) -> Vec<u32> {
    let seed = ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    let mut values = Vec::with_capacity(rt::WORD_COUNT);
    for _ in 0..rt::WORD_COUNT {
        values.push(rng.next_u32());
    }
    match case {
        0 => values.iter_mut().for_each(|value| *value = 0),
        1 => values.iter_mut().for_each(|value| *value = u32::MAX),
        2 => values.iter_mut().for_each(|value| *value = 0x7FC0_0001),
        3 => values.iter_mut().for_each(|value| *value = 0x0000_0001),
        _ => {}
    }
    values
}

/// The runtime's words after the rewrite ran on `values`, and what it returned.
fn run_rewrite(instance: usize, values: &[u32]) -> (Vec<u32>, u32) {
    for (number, value) in values.iter().enumerate() {
        rt::set_word(number, *value);
    }
    let returned = CALLS[instance]();
    let after = (0..rt::WORD_COUNT).map(rt::get_word).collect();
    (after, returned)
}

/// The lifted table's words after the copy ran on `values`, and what it returned.
fn run_lifted(copy: WordCopy, values: &[u32]) -> (Vec<u32>, u32) {
    let mut words = StaticWords::new(WORD_COUNT);
    for (number, value) in values.iter().enumerate() {
        words.set_word(WordId(number), *value);
    }
    let returned = copy.apply(&mut words);
    let after = (0..WORD_COUNT).map(|number| words.word(WordId(number))).collect();
    (after, returned)
}

/// A deliberately wrong version of one instance's lifted copy.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The value lands in the word after the destination.
    WrongDestination,
    /// The copy runs in the wrong direction: destination to source.
    Reversed,
    /// The copy is left out: the destination keeps its value.
    MissingCopy,
    /// The result is one, not zero.
    WrongResult,
}

const WRONG_KINDS: [Wrong; 4] = [
    Wrong::WrongDestination,
    Wrong::Reversed,
    Wrong::MissingCopy,
    Wrong::WrongResult,
];

/// The lifted copy run as the wrong version describes.
fn run_wrong(wrong: Wrong, copy: WordCopy, values: &[u32]) -> (Vec<u32>, u32) {
    match wrong {
        Wrong::WrongDestination => {
            let shifted = WordCopy {
                destination: WordId((copy.destination.0 + 1) % WORD_COUNT),
                ..copy
            };
            run_lifted(shifted, values)
        }
        Wrong::Reversed => run_lifted(
            WordCopy {
                source: copy.destination,
                destination: copy.source,
            },
            values,
        ),
        Wrong::MissingCopy => {
            let (mut after, returned) = run_lifted(copy, values);
            after[copy.destination.0] = values[copy.destination.0];
            (after, returned)
        }
        Wrong::WrongResult => {
            let (after, _) = run_lifted(copy, values);
            (after, 1)
        }
    }
}

/// The first case, if any, at which the lifted copy and the rewrite disagree.
fn first_difference(named: &NamedCopy, instance: usize, cases: u32) -> Option<String> {
    for case in 0..cases {
        let values = case_values(instance, case);
        let (want_words, want_result) = run_rewrite(instance, &values);
        let (got_words, got_result) = run_lifted(named.copy, &values);
        if want_result != got_result {
            return Some(format!("case {case}: rewrite returned {want_result:#x}, lifted returned {got_result:#x}"));
        }
        if want_words != got_words {
            let at = want_words
                .iter()
                .zip(got_words.iter())
                .position(|(a, b)| a != b)
                .unwrap_or(0);
            return Some(format!("case {case}: first differing word number {at}"));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &NamedCopy, instance: usize) -> bool {
    for case in 0..WRONG_CASES {
        let values = case_values(instance, case);
        let (want_words, want_result) = run_rewrite(instance, &values);
        let (got_words, got_result) = run_wrong(wrong, named.copy, &values);
        if want_result != got_result || want_words != got_words {
            return true;
        }
    }
    false
}

#[test]
fn every_instance_matches_its_verified_rewrite() {
    let mut failures = Vec::new();
    for (instance, named) in COPIES.iter().enumerate() {
        if let Some(detail) = first_difference(named, instance, CASES) {
            failures.push(format!("{} (instance {instance}): {detail}", named.name));
        }
    }
    assert!(failures.is_empty(), "instances that differ from their rewrite:\n{}", failures.join("\n"));
}

#[test]
fn every_instance_catches_every_wrong_version() {
    let mut escaped = Vec::new();
    for (instance, named) in COPIES.iter().enumerate() {
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
    assert_eq!(CALLS.len(), COPIES.len());
    assert_eq!(rt::WORD_COUNT, WORD_COUNT);
}
