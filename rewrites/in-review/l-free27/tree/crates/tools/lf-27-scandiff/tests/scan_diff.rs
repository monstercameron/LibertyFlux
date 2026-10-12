//! Differential test of the lifted counted-run search against the eight
//! verified rewrites of the family.
//!
//! Each instance is one verified rewrite. For each instance, CASES generated
//! objects are built in a block of memory of this test program; the rewrite
//! runs on that block and the lifted method runs on a copy of the same bytes.
//! The returned value is compared, and so is every byte of the block: a
//! rewrite of this family must leave the object as it found it.
//!
//! The generated objects hold a count (fixed, unsigned or signed, as the
//! instance reads it), entries that are sometimes equal to the key and
//! sometimes zero, and noise everywhere else. Keys include zero. Each
//! instance is also run against four deliberately wrong versions of its own
//! parameters (a wrong stride, a wrong first offset, a wrong answer form, a
//! wrong zero guard); every one must be caught on every instance.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{Instance, INSTANCES};
use lf_core::counted_run::{Answer, Count, CountedRun};

/// Generated objects per instance.
const CASES: u32 = 1000;

/// The size of the object block. Every read the rewrites make for the largest
/// generated count stays inside it (the largest run ends near 0x1B44).
const OBJECT_LEN: usize = 0x2000;

/// The largest entry count the generator writes.
const MAX_ENTRIES: u32 = 40;

/// One lock across the whole test program, so that two tests never run their
/// rewrites at once.
static LOCK: Mutex<()> = Mutex::new(());

/// A block of object memory, aligned for the rewrites' word reads.
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

    /// A value in `0..n`; `n` is never zero here.
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    /// A 32-bit value, with a chance of a small value that collides often.
    fn word(&mut self) -> u32 {
        match self.below(4) {
            0 => self.next() as u32,
            _ => 1 + self.below(6) as u32,
        }
    }
}

/// Writes a little-endian 32-bit word into the block at `offset`.
fn put_word(object: &mut Object, offset: u32, value: u32) {
    let start = offset as usize;
    object.0[start..start + 4].copy_from_slice(&value.to_le_bytes());
}

/// Builds one generated object for `run` and returns the key to search for.
fn make_case(run: &CountedRun, rng: &mut Rng, object: &mut Object) -> u32 {
    for byte in object.0.iter_mut() {
        *byte = rng.next() as u8;
    }
    let count = match run.count {
        Count::Fixed(n) => n,
        Count::Unsigned { offset } => {
            let n = rng.below(MAX_ENTRIES as usize + 1) as u32;
            put_word(object, offset, n);
            n
        }
        Count::Signed { offset } => {
            // Zero and negative counts are part of the input space.
            let raw = rng.below(MAX_ENTRIES as usize + 4) as i32 - 3;
            put_word(object, offset, raw as u32);
            if raw > 0 { raw as u32 } else { 0 }
        }
    };
    let key = match rng.below(8) {
        0 => 0,
        1..=2 => rng.word(),
        _ => 1 + rng.below(6) as u32,
    };
    // One case in four has exactly one equal entry, at the first, the last or
    // a random position, so that a run which starts or ends in the wrong place
    // differs from the rewrite on reachable input.
    let single = count > 0 && rng.below(4) == 0;
    let lone = match rng.below(3) {
        0 => 0,
        1 => count.saturating_sub(1),
        _ => rng.below(count.max(1) as usize) as u32,
    };
    for index in 0..count {
        let offset = run.first.wrapping_add(index.wrapping_mul(run.stride));
        let value = if single {
            if index == lone {
                key
            } else {
                let other = rng.word();
                if other == key { key.wrapping_add(1) } else { other }
            }
        } else {
            match rng.below(4) {
                0 => key,
                1 => 0,
                _ => rng.word(),
            }
        };
        put_word(object, offset, value);
    }
    key
}

/// The outcome of one instance's cases against one parameter set.
struct Outcome {
    /// Cases run.
    cases: u32,
    /// Cases whose returned value differed, or where the rewrite wrote the
    /// object, or where the lifted method reported a truncated run.
    mismatches: u32,
    /// The first failing case, described for a reader.
    first: Option<String>,
}

/// Runs `CASES` generated objects through the rewrite and through the lifted
/// method built from `run`, and compares them.
fn diff(instance: &Instance, run: &CountedRun, seed: u64) -> Outcome {
    let mut rng = Rng(seed);
    let mut outcome = Outcome { cases: 0, mismatches: 0, first: None };
    let mut object = Box::new(Object([0; OBJECT_LEN]));
    for case in 0..CASES {
        let key = make_case(run, &mut rng, &mut object);
        let before = object.0.to_vec();
        let address = &*object as *const Object as usize as u32;
        let expected = (instance.call)(address, key);
        let wrote = object.0[..] != before[..];
        let lifted = run.evaluate(&before, address, key);
        outcome.cases += 1;
        let problem = match lifted {
            Err(_) => Some("the lifted method reported a truncated run".to_string()),
            Ok(_) if wrote => Some("the rewrite wrote the object".to_string()),
            Ok(got) if got != expected => Some(format!("rewrite {expected:#010x}, lifted {got:#010x}")),
            Ok(_) => None,
        };
        if let Some(text) = problem {
            outcome.mismatches += 1;
            if outcome.first.is_none() {
                outcome.first = Some(format!(
                    "case {case}: key {key:#010x}, count {:?}, {text}",
                    run.count
                ));
            }
        }
    }
    outcome
}

/// The four wrong versions of an instance's parameters, each a single change.
fn wrong_versions(run: &CountedRun) -> [(&'static str, CountedRun); 4] {
    let flipped = match run.answer {
        Answer::Flag => Answer::Index,
        Answer::Index => Answer::Flag,
        Answer::MaskedEntryAddress => Answer::MaskedKey,
        Answer::MaskedKey => Answer::MaskedEntryAddress,
    };
    [
        ("stride plus four", CountedRun { stride: run.stride.wrapping_add(4), ..*run }),
        ("first offset plus four", CountedRun { first: run.first.wrapping_add(4), ..*run }),
        ("answer form changed", CountedRun { answer: flipped, ..*run }),
        ("zero guard flipped", CountedRun { zero_key_misses: !run.zero_key_misses, ..*run }),
    ]
}

/// A seed per instance, from its position in the list.
fn seed_of(index: usize) -> u64 {
    0x9E37_79B9_7F4A_7C15 ^ ((index as u64 + 1) * 0x100_0000_01B3)
}

#[test]
fn lifted_search_matches_every_verified_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut total_mismatches = 0;
    for (index, instance) in INSTANCES.iter().enumerate() {
        let outcome = diff(instance, &instance.run, seed_of(index));
        eprintln!(
            "{} {} cases {} mismatches {} first {:?}",
            instance.address, instance.name, outcome.cases, outcome.mismatches, outcome.first
        );
        total_mismatches += outcome.mismatches;
    }
    assert_eq!(total_mismatches, 0, "the lifted search differs from a verified rewrite");
}

#[test]
fn wrong_versions_are_caught_on_every_instance() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    for (index, instance) in INSTANCES.iter().enumerate() {
        for (label, wrong) in wrong_versions(&instance.run) {
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
