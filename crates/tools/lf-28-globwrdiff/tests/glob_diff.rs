//! Differential tests: every global constant-write routine of this crate against its verified 32-bit rewrite, through
//! the existing lifted `ConstantWrites::apply`.
//!
//! Each instance's globals are the test region, which starts at the instance's lowest global. The region is filled
//! with random bytes, the rewrite runs, and the lifted routine runs on a copy of the same bytes with the writes
//! placed at their offsets from that base. Every byte of the region is compared, and the returned word. Deliberately
//! wrong versions must each be caught by every instance: the last write left out, a value with one bit flipped, and
//! every write one byte further on.
//!
//! The test runs on the 32-bit target only. Its inputs come from a fixed seed, and one lock serialises the tests,
//! since they share the region.

#![cfg(target_arch = "x86")]

use std::sync::Mutex;

use lf_28_globwrdiff::calls::CALLS;
use lf_28_globwrdiff::instances::{INSTANCES, Named};
use lf_28_globwrdiff::rt::{REGION_BYTES, fill_region, region_bytes, set_region};
use lf_core::constant_writes::{ConstantWrite, Returns, apply_writes};
use lf_core::field_store::ObjectBytes;

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

/// The region's starting bytes for one case: random, so every byte a wrong version leaves alone differs.
fn starting_bytes(instance: usize, case: u32) -> Vec<u8> {
    let seed =
        ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    (0..REGION_BYTES).map(|_| rng.next_u32() as u8).collect()
}

/// The rewrite on the region: its returned word, and the region afterwards.
fn run_rewrite(instance: usize, base: u32, bytes: &[u8]) -> (u32, Vec<u8>) {
    set_region(base);
    fill_region(bytes);
    let returned = CALLS[instance](core::ptr::null_mut(), 0, 0);
    (returned, region_bytes())
}

/// The lifted writes on a copy of the starting bytes, at the instance's offsets from its base.
fn run_lifted(
    named: &Named,
    bytes: &[u8],
    writes: &[ConstantWrite],
    returns: Returns,
    base: u32,
) -> (u32, Vec<u8>) {
    let _ = named;
    let mut object = ObjectBytes::from_bytes(bytes);
    let returned = apply_writes(writes, returns, &mut object, base);
    (returned, object.bytes().to_vec())
}

/// A deliberately wrong version of one instance's writes.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The last write is left out.
    DroppedWrite,
    /// The first write's value has one bit flipped.
    ChangedValue,
    /// Every write lands one byte further on.
    ShiftedOffset,
}

const WRONG_KINDS: [Wrong; 3] = [
    Wrong::DroppedWrite,
    Wrong::ChangedValue,
    Wrong::ShiftedOffset,
];

fn run_wrong(wrong: Wrong, named: &Named, bytes: &[u8]) -> (u32, Vec<u8>) {
    let writes = named.spec.writes;
    let returns = named.spec.returns;
    match wrong {
        Wrong::DroppedWrite => run_lifted(named, bytes, &writes[..writes.len() - 1], returns, 0),
        Wrong::ChangedValue => {
            let mut changed = writes.to_vec();
            changed[0].value ^= 1;
            run_lifted(named, bytes, &changed, returns, 0)
        }
        Wrong::ShiftedOffset => {
            let shifted: Vec<ConstantWrite> = writes
                .iter()
                .map(|w| ConstantWrite {
                    offset: w.offset + 1,
                    ..*w
                })
                .collect();
            run_lifted(named, bytes, &shifted, returns, 0)
        }
    }
}

/// The first case, if any, at which the lifted writes and the rewrite disagree.
fn first_difference(named: &Named, instance: usize, cases: u32) -> Option<String> {
    let base = named.base;
    for case in 0..cases {
        let bytes = starting_bytes(instance, case);
        let (want_return, want_bytes) = run_rewrite(instance, base, &bytes);
        let (got_return, got_bytes) =
            run_lifted(named, &bytes, named.spec.writes, named.spec.returns, 0);
        if want_return != got_return {
            return Some(format!(
                "case {case}: rewrite returned {want_return:#010x}, lifted returned {got_return:#010x}"
            ));
        }
        if want_bytes != got_bytes {
            let at = want_bytes
                .iter()
                .zip(got_bytes.iter())
                .position(|(a, b)| a != b)
                .unwrap_or(0);
            return Some(format!("case {case}: first differing byte at {at:#x}"));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &Named, instance: usize) -> bool {
    for case in 0..WRONG_CASES {
        let bytes = starting_bytes(instance, case);
        let (want_return, want_bytes) = run_rewrite(instance, named.base, &bytes);
        if run_wrong(wrong, named, &bytes) != (want_return, want_bytes) {
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
