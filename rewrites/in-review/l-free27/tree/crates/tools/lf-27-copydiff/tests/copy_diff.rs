//! Differential test of the existing lifted word-block copy against the four
//! verified block copies of this family.
//!
//! Each instance is one verified rewrite. For each instance, CASES generated
//! pairs of blocks are built in this test program: an object block, which is
//! the rewrite's `this`, and a caller buffer. The rewrite runs on the blocks
//! and `WordBlockCopy::run` runs on copies of the same bytes. Every byte of
//! both blocks is compared, so the copy must write the same words in the same
//! place and nothing else may change. The return is compared where the
//! rewrite's own contract compares one (the zero result, and the last word of
//! the copy); the void routines are checked by their bytes alone.
//!
//! Each instance is also run against three deliberately wrong versions of its
//! own copy (the direction flipped, the object offset moved by four bytes, one
//! word short); every one must be caught on every instance.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{Call, Compared, Instance, INSTANCES};
use lf_core::word_block::{CopyDirection, CopyOutcome, CopyReturn, WordBlockCopy};

/// Generated block pairs per instance.
const CASES: u32 = 1000;

/// The object block: large enough for every run's offset plus four words.
const OBJECT_LEN: usize = 0x400;

/// The caller buffer: four words.
const BUFFER_LEN: usize = 0x10;

/// One lock across the whole test program.
static LOCK: Mutex<()> = Mutex::new(());

/// A block of object memory, aligned for the rewrites' word accesses.
#[repr(C, align(8))]
struct Object([u8; OBJECT_LEN]);

/// A block of buffer memory, aligned the same way.
#[repr(C, align(8))]
struct Buffer([u8; BUFFER_LEN]);

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

/// The outcome of one instance's cases against one copy.
struct Outcome {
    /// Cases run.
    cases: u32,
    /// Cases whose bytes or compared return differed.
    mismatches: u32,
    /// The first failing case, described for a reader.
    first: Option<String>,
}

/// Calls the rewrite with the object and the buffer, in its own call form. In
/// the four-argument form the buffer is the second stack argument, which is the
/// third word after `this`; the first and third stack arguments are unused.
fn call_rewrite(call: Call, object: u32, buffer: u32) -> Option<u32> {
    match call {
        Call::A(f) => Some(f(object, buffer)),
        Call::B(f) => Some(f(object, 0, buffer, 0)),
        Call::U(f) => {
            f(object, 0, buffer, 0);
            None
        }
    }
}

/// Runs `CASES` generated block pairs through the rewrite and through the
/// lifted copy built from `copy`, and compares them.
fn diff(instance: &Instance, copy: &WordBlockCopy, compared: Compared, seed: u64) -> Outcome {
    let mut rng = Rng(seed);
    let mut outcome = Outcome { cases: 0, mismatches: 0, first: None };
    let mut object = Box::new(Object([0; OBJECT_LEN]));
    let mut buffer = Box::new(Buffer([0; BUFFER_LEN]));
    for case in 0..CASES {
        for byte in object.0.iter_mut() {
            *byte = rng.next() as u8;
        }
        for byte in buffer.0.iter_mut() {
            *byte = rng.next() as u8;
        }
        let object_before = object.0.to_vec();
        let buffer_before = buffer.0.to_vec();
        let object_address = &*object as *const Object as usize as u32;
        let buffer_address = &*buffer as *const Buffer as usize as u32;
        let returned = call_rewrite(instance.call, object_address, buffer_address);

        let mut lifted_object = object_before;
        let mut lifted_buffer = buffer_before;
        let lifted = copy.run(&mut lifted_object, &mut lifted_buffer);
        outcome.cases += 1;

        let problem = if lifted_object[..] != object.0[..] {
            Some("the object bytes differ".to_string())
        } else if lifted_buffer[..] != buffer.0[..] {
            Some("the buffer bytes differ".to_string())
        } else {
            match (compared, lifted, returned) {
                (Compared::No, _, _) => None,
                (Compared::Yes, CopyOutcome::Null, Some(got)) if got == 0 => None,
                (Compared::Yes, CopyOutcome::Word(word), Some(got)) if got == word => None,
                (Compared::Yes, outcome_value, got) => Some(format!(
                    "rewrite returned {got:?}, lifted outcome {outcome_value:?}"
                )),
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

/// The three wrong versions of an instance's copy, each a single change.
fn wrong_versions(copy: &WordBlockCopy) -> [(&'static str, WordBlockCopy); 3] {
    let flipped = match copy.direction {
        CopyDirection::FromObject => CopyDirection::IntoObject,
        CopyDirection::IntoObject => CopyDirection::FromObject,
    };
    [
        ("direction flipped", WordBlockCopy { direction: flipped, ..*copy }),
        ("object offset plus four", WordBlockCopy { object_offset: copy.object_offset + 4, ..*copy }),
        ("one word short", WordBlockCopy { words: copy.words - 1, returns: CopyReturn::Null, ..*copy }),
    ]
}

/// A seed per instance, from its position in the list.
fn seed_of(index: usize) -> u64 {
    0x2545_F491_4F6C_DD1D ^ ((index as u64 + 1) * 0x100_0000_01B3)
}

#[test]
fn existing_block_copy_matches_every_verified_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut total = 0;
    for (index, instance) in INSTANCES.iter().enumerate() {
        let outcome = diff(instance, &instance.copy, instance.compared, seed_of(index));
        eprintln!(
            "{} {} cases {} mismatches {} first {:?}",
            instance.address, instance.name, outcome.cases, outcome.mismatches, outcome.first
        );
        total += outcome.mismatches;
    }
    assert_eq!(total, 0, "the existing block copy differs from a verified rewrite");
}

#[test]
fn wrong_versions_are_caught_on_every_instance() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    for (index, instance) in INSTANCES.iter().enumerate() {
        for (label, wrong) in wrong_versions(&instance.copy) {
            let outcome = diff(instance, &wrong, instance.compared, seed_of(index));
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
