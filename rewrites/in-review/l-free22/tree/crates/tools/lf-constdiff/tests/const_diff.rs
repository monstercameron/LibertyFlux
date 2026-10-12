//! Differential tests: every lifted constant-write routine against its verified 32-bit rewrite.
//!
//! For each instance of `lf_core::constant_writes::instances::INSTANCES`, the test gives the rewrite and the
//! lifted routine the same generated object, and compares the returned value and every byte of the object
//! after the call. The lifted routine receives the object's address as the rewrite saw it, so a routine that
//! returns its own address is compared exactly. Each instance runs `CASES` cases.
//!
//! Deliberately wrong versions run through the same contract, and each must be caught by every instance: the
//! last write left out, a constant with one bit flipped, every write shifted four bytes on, and a return one
//! away from the right one.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from a fixed seed.

#![cfg(target_arch = "x86")]

use lf_constdiff::calls::CALLS;
use lf_core::constant_writes::instances::{INSTANCES, NamedWrites};
use lf_core::constant_writes::{ConstantWrite, Returns, apply_writes};
use lf_core::field_store::ObjectBytes;

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

/// The object for one case of one instance, filled with random bytes so that every byte a wrong version
/// writes differs from what the right version leaves.
fn case_object(instance: usize, case: u32) -> Object {
    let seed = ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    let mut object = Object([0; OBJECT_BYTES]);
    for chunk in object.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    object
}

/// The rewrite on one object: its result, and the object's address as the rewrite saw it.
fn run_rewrite(instance: usize, object: &mut Object) -> (u32, u32) {
    let this = object.0.as_mut_ptr() as usize as u32;
    let returned = CALLS[instance](object.0.as_mut_ptr());
    (returned, this)
}

/// The lifted routine on a copy of the object, with the address the rewrite saw.
fn run_lifted(named: &NamedWrites, original: &Object, this: u32) -> (u32, ObjectBytes) {
    let mut object = ObjectBytes::from_bytes(&original.0);
    let returned = named.writes.apply(&mut object, this);
    (returned, object)
}

/// A deliberately wrong version of one instance's lifted routine.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The last write is left out.
    DroppedWrite,
    /// The first write's constant has one bit flipped.
    ChangedValue,
    /// Every write lands four bytes further on.
    ShiftedOffset,
    /// The return differs: zero becomes one, one becomes zero, and the object's address becomes one more.
    WrongReturn,
}

const WRONG_KINDS: [Wrong; 4] = [Wrong::DroppedWrite, Wrong::ChangedValue, Wrong::ShiftedOffset, Wrong::WrongReturn];

/// The lifted routine run as the wrong version describes.
fn run_wrong(wrong: Wrong, named: &NamedWrites, original: &Object, this: u32) -> (u32, ObjectBytes) {
    let writes = named.writes.writes;
    let returns = named.writes.returns;
    match wrong {
        Wrong::DroppedWrite => {
            let kept = &writes[..writes.len() - 1];
            let mut object = ObjectBytes::from_bytes(&original.0);
            let returned = apply_writes(kept, returns, &mut object, this);
            (returned, object)
        }
        Wrong::ChangedValue => {
            let mut changed: Vec<ConstantWrite> = writes.to_vec();
            if let Some(first) = changed.first_mut() {
                first.value ^= 1;
            }
            let mut object = ObjectBytes::from_bytes(&original.0);
            let returned = apply_writes(&changed, returns, &mut object, this);
            (returned, object)
        }
        Wrong::ShiftedOffset => {
            let shifted: Vec<ConstantWrite> = writes
                .iter()
                .map(|write| ConstantWrite {
                    offset: write.offset + 4,
                    ..*write
                })
                .collect();
            let mut object = ObjectBytes::from_bytes(&original.0);
            let returned = apply_writes(&shifted, returns, &mut object, this);
            (returned, object)
        }
        Wrong::WrongReturn => {
            let (returned, object) = run_lifted(named, original, this);
            let changed = match returns {
                Returns::Zero => 1,
                Returns::One => 0,
                Returns::This => this.wrapping_add(1),
            };
            let _ = returned;
            (changed, object)
        }
    }
}

/// The first case, if any, at which the lifted routine and the rewrite disagree.
fn first_difference(named: &NamedWrites, instance: usize, cases: u32) -> Option<String> {
    for case in 0..cases {
        let original = case_object(instance, case);
        let mut rewritten = original.clone();
        let (want_result, this) = run_rewrite(instance, &mut rewritten);
        let (got_result, got_object) = run_lifted(named, &original, this);
        if want_result != got_result {
            return Some(format!("case {case}: rewrite returned {want_result:#010x}, lifted returned {got_result:#010x}"));
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
fn wrong_is_caught(wrong: Wrong, named: &NamedWrites, instance: usize) -> bool {
    for case in 0..WRONG_CASES {
        let original = case_object(instance, case);
        let mut rewritten = original.clone();
        let (want_result, this) = run_rewrite(instance, &mut rewritten);
        let (got_result, got_object) = run_wrong(wrong, named, &original, this);
        if want_result != got_result || rewritten.0[..] != *got_object.bytes() {
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
