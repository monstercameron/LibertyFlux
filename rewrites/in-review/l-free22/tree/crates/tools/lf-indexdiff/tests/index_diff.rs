//! Differential tests: every lifted indexed read against its verified 32-bit rewrite.
//!
//! For each instance of `lf_core::indexed_read::instances::READS`, the test gives the rewrite and the lifted read
//! the same generated object and index, and compares the returned word and the object after the call. The
//! indexes cover the first and last entries of the table the tests use, and random entries between them. Each
//! instance runs `CASES` cases. The tests keep every index below `MAX_INDEX`, so that every entry read lies
//! inside the generated object; an index that wraps the 32-bit offset is covered by the host tests of the
//! lifted method, not here.
//!
//! Deliberately wrong versions run through the same contract, and each must be caught by every instance: a
//! stride one too large, a base one too large, the other width, and an index that is ignored.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from a fixed seed.

#![cfg(target_arch = "x86")]

use lf_core::field_store::ObjectBytes;
use lf_core::indexed_read::instances::{NamedRead, READS};
use lf_core::indexed_read::{EntryWidth, IndexedRead};
use lf_indexdiff::calls::CALLS;

/// Bytes in the generated object. Every entry the tests read lies below this.
const OBJECT_BYTES: usize = 0x2000;
/// Indexes used by the tests are below this, so every entry read lies inside the object.
const MAX_INDEX: u32 = 256;
/// Cases per instance for the differential comparison.
const CASES: u32 = 1000;
/// Cases per instance and wrong version; a wrong version must be caught by one of them.
const WRONG_CASES: u32 = 64;

/// The object the rewrite reads. Aligned, as in the original.
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

/// One case's inputs: a random object and an index below `MAX_INDEX`. Cases 0 to 2 read the first entry, the
/// second entry and the last entry.
fn case_inputs(instance: usize, case: u32) -> (Object, u32) {
    let seed = ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    let mut object = Object([0; OBJECT_BYTES]);
    for chunk in object.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    let index = match case {
        0 => 0,
        1 => 1,
        2 => MAX_INDEX - 1,
        _ => rng.next_u32() % MAX_INDEX,
    };
    (object, index)
}

/// The rewrite on one case: the returned word and the object after the call.
fn run_rewrite(instance: usize, object: &Object, index: u32) -> (u32, Object) {
    let mut after = object.clone();
    let returned = CALLS[instance](after.0.as_mut_ptr(), index);
    (returned, after)
}

/// The lifted read on one case: the returned word and the object after the call (the read changes nothing).
fn run_lifted(read: IndexedRead, object: &Object, index: u32) -> (u32, Object) {
    let lifted = ObjectBytes::from_bytes(&object.0);
    let returned = read.read(&lifted, index);
    let mut after = Object([0; OBJECT_BYTES]);
    after.0.copy_from_slice(lifted.bytes());
    (returned, after)
}

/// A deliberately wrong version of one instance's lifted read.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The stride is one too large.
    StrideOffByOne,
    /// The base is one too large.
    BaseOffByOne,
    /// The entry is read at the other width.
    OtherWidth,
    /// The index is ignored: the first entry is always read.
    IndexIgnored,
}

const WRONG_KINDS: [Wrong; 4] = [Wrong::StrideOffByOne, Wrong::BaseOffByOne, Wrong::OtherWidth, Wrong::IndexIgnored];

/// The lifted read run as the wrong version describes.
fn run_wrong(wrong: Wrong, named: &NamedRead, object: &Object, index: u32) -> (u32, Object) {
    let read = named.read;
    match wrong {
        Wrong::StrideOffByOne => run_lifted(IndexedRead { stride: read.stride + 1, ..read }, object, index),
        Wrong::BaseOffByOne => run_lifted(IndexedRead { base: read.base + 1, ..read }, object, index),
        Wrong::OtherWidth => {
            let width = match read.width {
                EntryWidth::Byte => EntryWidth::Word,
                EntryWidth::Word => EntryWidth::Byte,
            };
            run_lifted(IndexedRead { width, ..read }, object, index)
        }
        Wrong::IndexIgnored => run_lifted(read, object, 0),
    }
}

/// The first case, if any, at which the lifted read and the rewrite disagree.
fn first_difference(named: &NamedRead, instance: usize, cases: u32) -> Option<String> {
    for case in 0..cases {
        let (object, index) = case_inputs(instance, case);
        let (want, want_object) = run_rewrite(instance, &object, index);
        let (got, got_object) = run_lifted(named.read, &object, index);
        if want != got {
            return Some(format!("case {case}: index {index}, rewrite returned {want:#010x}, lifted returned {got:#010x}"));
        }
        if want_object.0 != got_object.0 {
            return Some(format!("case {case}: index {index}, the rewrite changed the object"));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &NamedRead, instance: usize) -> bool {
    for case in 0..WRONG_CASES {
        let (object, index) = case_inputs(instance, case);
        let (want, want_object) = run_rewrite(instance, &object, index);
        let (got, got_object) = run_wrong(wrong, named, &object, index);
        if want != got || want_object.0 != got_object.0 {
            return true;
        }
    }
    false
}

#[test]
fn every_instance_matches_its_verified_rewrite() {
    let mut failures = Vec::new();
    for (instance, named) in READS.iter().enumerate() {
        if let Some(detail) = first_difference(named, instance, CASES) {
            failures.push(format!("{} (instance {instance}): {detail}", named.name));
        }
    }
    assert!(failures.is_empty(), "instances that differ from their rewrite:\n{}", failures.join("\n"));
}

#[test]
fn every_instance_catches_every_wrong_version() {
    let mut escaped = Vec::new();
    for (instance, named) in READS.iter().enumerate() {
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
    assert_eq!(CALLS.len(), READS.len());
}
