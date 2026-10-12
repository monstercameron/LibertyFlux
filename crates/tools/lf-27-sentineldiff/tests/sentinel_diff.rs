//! Differential test of the lifted sentinel-table reset against the four
//! verified rewrites of the family.
//!
//! Each instance is one verified rewrite. For each instance, CASES generated
//! images are built in a block of test memory, which the runtime maps as the
//! window of file addresses around the run. The rewrite runs on the block and
//! `SentinelTable::fill` runs on a copy of the same bytes. The returned end of
//! the run is compared (the rewrite's address against the lifted index mapped
//! into the block), and so is every byte of the block: the records' marker and
//! half-word, the flag byte when there is one, and every byte the rewrite must
//! leave alone.
//!
//! Each instance is also run against four deliberately wrong versions of its
//! own parameters (one record too few, a wrong record size, a flag added or
//! dropped, a flag one byte off); every one must be caught on every instance.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{INSTANCES, Instance};
use lf_27_sentineldiff::rt::set_window;
use lf_core::sentinel_table::SentinelTable;

/// Generated images per instance.
const CASES: u32 = 1000;

/// The block of test memory the window maps.
const IMAGE_LEN: usize = 4096;

/// File addresses below the run's first record that the window also covers,
/// so that a flag byte before the run is inside it.
const BEFORE_RUN: u32 = 16;

/// One lock across the whole test program: the window is process-wide state.
static LOCK: Mutex<()> = Mutex::new(());

/// A block of test memory, aligned for the rewrites' word writes.
#[repr(C, align(8))]
struct Image([u8; IMAGE_LEN]);

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

/// The outcome of one instance's cases against one set of parameters.
struct Outcome {
    /// Cases run.
    cases: u32,
    /// Cases whose returned end or image bytes differed, or where the lifted
    /// routine faulted on an image the rewrite handled.
    mismatches: u32,
    /// The first failing case, described for a reader.
    first: Option<String>,
}

/// Runs `CASES` generated images through the rewrite and through the lifted
/// routine built from `table`, and compares them.
fn diff(instance: &Instance, table: &SentinelTable, seed: u64) -> Outcome {
    let mut rng = Rng(seed);
    let mut outcome = Outcome {
        cases: 0,
        mismatches: 0,
        first: None,
    };
    let mut image = Box::new(Image([0; IMAGE_LEN]));
    let window_va = instance.base - BEFORE_RUN;
    let table_index = BEFORE_RUN as usize;
    for case in 0..CASES {
        for byte in image.0.iter_mut() {
            *byte = rng.next() as u8;
        }
        let before = image.0.to_vec();
        let address = &*image as *const Image as usize as u32;
        set_window(window_va, address, IMAGE_LEN as u32);
        let returned = (instance.call)();
        let rewrite_bytes = image.0.to_vec();

        let mut lifted_bytes = before;
        let lifted = table.fill(&mut lifted_bytes, table_index);
        outcome.cases += 1;
        let problem = match lifted {
            Err(_) => Some("the lifted routine faulted inside the window".to_string()),
            Ok(end) => {
                let expected_end = address.wrapping_add(end as u32);
                if returned != expected_end {
                    Some(format!(
                        "rewrite end {returned:#010x}, lifted end {expected_end:#010x}"
                    ))
                } else if lifted_bytes != rewrite_bytes {
                    let at = lifted_bytes
                        .iter()
                        .zip(rewrite_bytes.iter())
                        .position(|(a, b)| a != b)
                        .unwrap_or(0);
                    Some(format!("the image differs first at window byte {at}"))
                } else {
                    None
                }
            }
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

/// The four wrong versions of an instance's parameters, each a single change.
fn wrong_versions(table: &SentinelTable) -> [(&'static str, SentinelTable); 4] {
    let flag_added_or_dropped = SentinelTable {
        flag: match table.flag {
            None => Some(-7),
            Some(_) => None,
        },
        ..*table
    };
    let flag_one_off = SentinelTable {
        flag: Some(table.flag.map_or(-7, |f| f - 1)),
        ..*table
    };
    [
        (
            "one record too few",
            SentinelTable {
                records: table.records - 1,
                ..*table
            },
        ),
        (
            "record size plus two",
            SentinelTable {
                stride: table.stride + 2,
                ..*table
            },
        ),
        ("flag added or dropped", flag_added_or_dropped),
        ("flag one byte off", flag_one_off),
    ]
}

/// A seed per instance, from its position in the list.
fn seed_of(index: usize) -> u64 {
    0x5851_F42D_4C95_7F2D ^ ((index as u64 + 1) * 0x100_0000_01B3)
}

#[test]
fn lifted_reset_matches_every_verified_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut total = 0;
    for (index, instance) in INSTANCES.iter().enumerate() {
        let outcome = diff(instance, &instance.table, seed_of(index));
        eprintln!(
            "{} {} cases {} mismatches {} first {:?}",
            instance.address, instance.name, outcome.cases, outcome.mismatches, outcome.first
        );
        total += outcome.mismatches;
    }
    assert_eq!(
        total, 0,
        "the lifted table reset differs from a verified rewrite"
    );
}

#[test]
fn wrong_versions_are_caught_on_every_instance() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    for (index, instance) in INSTANCES.iter().enumerate() {
        for (label, wrong) in wrong_versions(&instance.table) {
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
