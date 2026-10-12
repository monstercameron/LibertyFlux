//! Differential tests: every indexed-read routine of this crate against its verified 32-bit rewrite.
//!
//! Each instance is run through the rewrite and through the existing lifted `IndexedRead::read` on the same
//! generated object and index. Indices run over the first 64 entries, so every entry lies inside the generated
//! object. The returned word is compared bit for bit. Deliberately wrong versions must each be caught by every
//! instance: the table based four bytes on, the stride four bytes wider, and the entry read at the other width.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from a fixed seed.

#![cfg(target_arch = "x86")]

use lf_24_indexdiff::calls::CALLS;
use lf_24_indexdiff::instances::{INSTANCES, Named};
use lf_core::field_store::ObjectBytes;
use lf_core::indexed_read::{EntryWidth, IndexedRead};

/// Bytes in the generated object. Every field and table entry the instances touch lies below this.
const OBJECT_BYTES: usize = 0x2000;
/// Cases per instance for the differential comparison.
const CASES: u32 = 1000;
/// Cases per instance and wrong version; a wrong version must be caught by one of them.
const WRONG_CASES: u32 = 64;

/// The object the rewrite works on, aligned so word fields at 4-byte offsets are aligned as in the original.
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

/// The object for one case of one instance, filled with random bytes.
fn random_object(instance: usize, case: u32) -> (Object, Rng) {
    let seed =
        ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    let mut object = Object([0; OBJECT_BYTES]);
    for chunk in object.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    (object, rng)
}

/// Writes a word into an object at a byte offset, little endian.
#[allow(dead_code)]
fn put_word(object: &mut Object, offset: usize, value: u32) {
    object.0[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

/// Entries per table the cases read: every one lies inside the object for every instance.
const ENTRIES: u32 = 64;

/// The index of one case.
fn index_of(rng: &mut Rng) -> u32 {
    rng.next_u32() % ENTRIES
}

/// The rewrite on one object: its returned word.
fn run_rewrite(instance: usize, object: &mut Object, index: u32) -> u32 {
    CALLS[instance](object.0.as_mut_ptr(), index, 0)
}

/// The lifted read on a copy of the object.
fn run_lifted(read: IndexedRead, original: &Object, index: u32) -> u32 {
    let object = ObjectBytes::from_bytes(&original.0);
    read.read(&object, index)
}

/// A deliberately wrong version of one instance's read.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The table starts four bytes further on.
    ShiftedBase,
    /// Each entry is four bytes further on than the rewrite's.
    WiderStride,
    /// The entry is read at the other width.
    OtherWidth,
}

const WRONG_KINDS: [Wrong; 3] = [Wrong::ShiftedBase, Wrong::WiderStride, Wrong::OtherWidth];

fn wrong_read(wrong: Wrong, read: IndexedRead) -> IndexedRead {
    match wrong {
        Wrong::ShiftedBase => IndexedRead {
            base: read.base + 4,
            ..read
        },
        Wrong::WiderStride => IndexedRead {
            stride: read.stride + 4,
            ..read
        },
        Wrong::OtherWidth => IndexedRead {
            width: match read.width {
                EntryWidth::Byte => EntryWidth::Word,
                EntryWidth::Word => EntryWidth::Byte,
            },
            ..read
        },
    }
}

/// The first case, if any, at which the lifted read and the rewrite disagree.
fn first_difference(named: &Named<IndexedRead>, instance: usize, cases: u32) -> Option<String> {
    for case in 0..cases {
        let (original, mut rng) = random_object(instance, case);
        let index = index_of(&mut rng);
        let mut rewritten = Object(original.0.clone());
        let want = run_rewrite(instance, &mut rewritten, index);
        let got = run_lifted(named.spec, &original, index);
        if want != got {
            return Some(format!(
                "case {case}: rewrite returned {want:#010x}, lifted returned {got:#010x}"
            ));
        }
    }
    None
}

/// True when a wrong version differs from the rewrite on at least one case.
fn wrong_is_caught(wrong: Wrong, named: &Named<IndexedRead>, instance: usize) -> bool {
    let bad = wrong_read(wrong, named.spec);
    for case in 0..WRONG_CASES {
        let (original, mut rng) = random_object(instance, case);
        let index = index_of(&mut rng);
        let mut rewritten = Object(original.0.clone());
        let want = run_rewrite(instance, &mut rewritten, index);
        if run_lifted(bad, &original, index) != want {
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
    assert!(
        failures.is_empty(),
        "instances that differ from their rewrite:\n{}",
        failures.join("\n")
    );
}

#[test]
fn every_instance_catches_every_wrong_version() {
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
