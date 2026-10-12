//! Differential tests: every row-address routine of this crate against its verified 32-bit rewrite, through the
//! new lifted `RowAddress::apply`.
//!
//! The table's base is resolved the way the routine resolves it: the global's own address, or the pointer stored
//! in the global. Each instance runs the rewrite and the lifted routine on the same generated object, parameter
//! and base, and compares the address each one returns. Deliberately wrong versions must each be caught by every
//! instance: the stride one more, the offset four bytes more, and the base four bytes further on.
//!
//! The test runs on the 32-bit target only. Its inputs come from a fixed seed, and one lock serialises the
//! tests, since they share the region.

#![cfg(target_arch = "x86")]

use std::sync::Mutex;

use lf_24_rowdiff::calls::CALLS;
use lf_24_rowdiff::instances::{INSTANCES, Named};
use lf_24_rowdiff::rt::{read_word, relocated, set_region, write_word};
use lf_core::field_store::ObjectBytes;
use lf_core::row_address::{BaseSource, RowAddress};

/// Bytes in the generated object: large enough for every field an instance reads.
const OBJECT_BYTES: usize = 0x3400;
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

/// The object, the parameter and the table base of one case. The object is random, so an index read from it is too.
fn case_inputs(instance: usize, case: u32) -> (Vec<u8>, u32) {
    let seed =
        ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    let object = (0..OBJECT_BYTES).map(|_| rng.next_u32() as u8).collect();
    (object, rng.next_u32() % 64)
}

/// Resolves the base of an instance: the global's address, or the pointer the test stores in the global.
fn resolve_base(named: &Named, global_va: u32, case: u32) -> u32 {
    match named.spec.base {
        BaseSource::Global => relocated(global_va),
        BaseSource::PointerInGlobal => {
            let target = relocated(global_va.wrapping_add(0x800 + 0x40 * (case % 8)));
            write_word(global_va, target);
            read_word(global_va)
        }
    }
}

/// The rewrite on the object: the address it returns.
fn run_rewrite(instance: usize, object: &mut [u8], parameter: u32) -> u32 {
    CALLS[instance](object.as_mut_ptr(), parameter, 0)
}

/// The lifted routine on a copy of the object, from the resolved base.
fn run_lifted(row: RowAddress, base: u32, object: &[u8], parameter: u32) -> u32 {
    row.apply(base, &ObjectBytes::from_bytes(object), parameter)
}

/// The global of an instance: its address is the one the rewrite names, the region's first word.
fn global_of(named: &Named) -> u32 {
    named.base_va
}

/// A deliberately wrong version of one instance's routine.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The stride is one row bigger.
    WiderStride,
    /// The returned address is four bytes further on.
    ShiftedOffset,
    /// The base is four bytes further on.
    ShiftedBase,
}

const WRONG_KINDS: [Wrong; 3] = [Wrong::WiderStride, Wrong::ShiftedOffset, Wrong::ShiftedBase];

fn run_wrong(wrong: Wrong, row: RowAddress, base: u32, object: &[u8], parameter: u32) -> u32 {
    match wrong {
        Wrong::WiderStride => run_lifted(
            RowAddress {
                stride: row.stride + 4,
                ..row
            },
            base,
            object,
            parameter,
        ),
        Wrong::ShiftedOffset => run_lifted(
            RowAddress {
                offset: row.offset + 4,
                ..row
            },
            base,
            object,
            parameter,
        ),
        Wrong::ShiftedBase => run_lifted(row, base.wrapping_add(4), object, parameter),
    }
}

/// The first case, if any, at which the lifted routine and the rewrite disagree.
fn first_difference(named: &Named, instance: usize, cases: u32) -> Option<String> {
    set_region(global_of(named));
    for case in 0..cases {
        let (mut object, parameter) = case_inputs(instance, case);
        let base = resolve_base(named, global_of(named), case);
        let want = run_rewrite(instance, &mut object, parameter);
        let got = run_lifted(named.spec, base, &object, parameter);
        if want != got {
            return Some(format!(
                "case {case}: rewrite returned {want:#010x}, lifted returned {got:#010x}"
            ));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &Named, instance: usize) -> bool {
    set_region(global_of(named));
    for case in 0..WRONG_CASES {
        let (mut object, parameter) = case_inputs(instance, case);
        let base = resolve_base(named, global_of(named), case);
        let want = run_rewrite(instance, &mut object, parameter);
        if run_wrong(wrong, named.spec, base, &object, parameter) != want {
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
