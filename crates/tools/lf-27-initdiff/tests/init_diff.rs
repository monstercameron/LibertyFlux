//! Differential test of the existing lifted constant-write routine against the
//! four verified record initialisers of this family.
//!
//! Each instance is one verified rewrite: it writes constants into fixed
//! fields of its object and returns the object's address. For each instance,
//! CASES generated objects are built in a block of this test program; the
//! rewrite runs on the block and `ConstantWrites::apply` runs on a copy of the
//! same bytes. The returned value is compared and so is every byte of the
//! block, so every write the rewrite makes must be made by the lifted routine
//! at the same place with the same value, and nothing else may change.
//!
//! Each instance is also run against four deliberately wrong versions of its
//! own writes (a wrong constant, a wrong word, a missing store, a wrong
//! return); every one must be caught on every instance.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{INSTANCES, Instance};
use lf_core::constant_writes::{ConstantWrite, ConstantWrites, Returns};
use lf_core::field_store::ObjectBytes;

/// Generated objects per instance.
const CASES: u32 = 1000;

/// The size of the object block: every write of the four rewrites lies inside
/// its first 0x28 bytes.
const BLOCK_LEN: usize = 0x200;

/// One lock across the whole test program.
static LOCK: Mutex<()> = Mutex::new(());

/// A block of object memory, aligned for the rewrites' word writes.
#[repr(C, align(8))]
struct Block([u8; BLOCK_LEN]);

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

/// The outcome of one instance's cases against one set of writes.
struct Outcome {
    /// Cases run.
    cases: u32,
    /// Cases whose returned value or block bytes differed.
    mismatches: u32,
    /// The first failing case, described for a reader.
    first: Option<String>,
}

/// Runs `CASES` generated blocks through the rewrite and through the lifted
/// routine built from `writes`, and compares them.
fn diff(instance: &Instance, writes: &ConstantWrites, seed: u64) -> Outcome {
    let mut rng = Rng(seed);
    let mut outcome = Outcome {
        cases: 0,
        mismatches: 0,
        first: None,
    };
    let mut block = Box::new(Block([0; BLOCK_LEN]));
    for case in 0..CASES {
        for byte in block.0.iter_mut() {
            *byte = rng.next() as u8;
        }
        let before = block.0.to_vec();
        let address = &*block as *const Block as usize as u32;
        let expected = (instance.call)(address);
        let mut lifted_block = ObjectBytes::from_bytes(&before);
        let got = writes.apply(&mut lifted_block, address);
        outcome.cases += 1;
        let problem = if got != expected {
            Some(format!(
                "rewrite returned {expected:#010x}, lifted {got:#010x}"
            ))
        } else if lifted_block.bytes() != &block.0[..] {
            Some("the block bytes differ after the write".to_string())
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

/// The four wrong versions of an instance's writes, each a single change.
fn wrong_versions(writes: &ConstantWrites) -> [(&'static str, ConstantWrites); 4] {
    let list = writes.writes;
    let first = list[0];
    let mut wrong_constant = list.to_vec();
    wrong_constant[0] = ConstantWrite {
        value: first.value ^ 1,
        ..first
    };
    let mut wrong_word = list.to_vec();
    wrong_word[0] = ConstantWrite {
        offset: first.offset + 4,
        ..first
    };
    let missing_last = &list[..list.len() - 1];
    let returns_zero = Returns::Zero;
    let leak =
        |v: Vec<ConstantWrite>| -> &'static [ConstantWrite] { Box::leak(v.into_boxed_slice()) };
    [
        (
            "wrong constant",
            ConstantWrites {
                writes: leak(wrong_constant),
                returns: writes.returns,
            },
        ),
        (
            "wrong word",
            ConstantWrites {
                writes: leak(wrong_word),
                returns: writes.returns,
            },
        ),
        (
            "missing last store",
            ConstantWrites {
                writes: missing_last,
                returns: writes.returns,
            },
        ),
        (
            "wrong return",
            ConstantWrites {
                writes: list,
                returns: returns_zero,
            },
        ),
    ]
}

/// A seed per instance, from its position in the list.
fn seed_of(index: usize) -> u64 {
    0xA076_1D64_78BD_642F ^ ((index as u64 + 1) * 0x100_0000_01B3)
}

#[test]
fn existing_constant_writes_match_every_verified_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut total = 0;
    for (index, instance) in INSTANCES.iter().enumerate() {
        let outcome = diff(instance, &instance.writes, seed_of(index));
        eprintln!(
            "{} {} cases {} mismatches {} first {:?}",
            instance.address, instance.name, outcome.cases, outcome.mismatches, outcome.first
        );
        total += outcome.mismatches;
    }
    assert_eq!(
        total, 0,
        "the existing constant-write routine differs from a verified rewrite"
    );
}

#[test]
fn wrong_versions_are_caught_on_every_instance() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    for (index, instance) in INSTANCES.iter().enumerate() {
        for (label, wrong) in wrong_versions(&instance.writes) {
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
