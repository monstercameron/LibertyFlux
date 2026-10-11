//! Differential cases of the two task predicates against their verified
//! 32-bit rewrites.
//!
//! Each lifted predicate runs on the same generated task objects as its
//! rewrite. The effects compared are the return value and every byte of the
//! object and of the resource it points to, which the predicates must leave
//! unchanged. The lifted side reads the same words through
//! [`PredicateTask`]: the test decodes the planted bytes at the offsets the
//! type names, and rebuilds the resource from the planted block the pointer
//! word addresses.
//!
//! Each predicate also has deliberately wrong versions. Each must differ
//! from its rewrite on at least one generated case.
//!
//! Runs on the 32-bit target only: the rewrites take raw addresses.

#![cfg(target_arch = "x86")]

use lf_peds_tasks::ped_task::{
    COUNT_OFFSET, ENABLED_OFFSET, FLAGS_OFFSET, KIND_OFFSET, PredicateTask, RESOURCE_OFFSET,
    RESOURCE_TAG_OFFSET, ResourceTag,
};
use lf_taskshapediff::Xorshift;
use lf_taskshapediff::rewrites::{fn_005c43e0, fn_005c4420};

/// Bytes planted for a task object: the largest offset read is the flag
/// byte, with room for the words around it.
const OBJECT_BYTES: usize = 0x280;
/// Bytes planted for a resource: the tag word sits at its offset.
const RESOURCE_BYTES: usize = 0x40;
/// Generated cases per routine.
const CASES: u32 = 3000;
/// Seed of the case generator for every test in this program.
const SEED: u32 = 0x5EED_0018;

/// One generated case: the task object, the resource block the pointer word
/// may address, and the address the resource block was planted at.
struct Case {
    object: Vec<u8>,
    resource: Vec<u8>,
    resource_addr: u32,
}

/// The address of a planted buffer, as the 32-bit rewrite sees it.
fn address_of(buffer: &[u8]) -> u32 {
    buffer.as_ptr() as usize as u32
}

/// Reads a little-endian word at a byte offset of a planted buffer.
fn word_at(buffer: &[u8], offset: u32) -> u32 {
    let at = offset as usize;
    u32::from_le_bytes([buffer[at], buffer[at + 1], buffer[at + 2], buffer[at + 3]])
}

/// Writes a little-endian word at a byte offset of a planted buffer.
fn set_word(buffer: &mut [u8], offset: u32, value: u32) {
    let at = offset as usize;
    buffer[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

/// A byte drawn from boundary values or at random, so the masked
/// comparisons hit both sides often.
fn byte_pick(rng: &mut Xorshift) -> u8 {
    const EDGES: [u8; 8] = [0, 1, 2, 3, 4, 0x0F, 0x80, 0xFF];
    let pick = rng.next_u32();
    if pick % 2 == 0 {
        EDGES[(pick >> 8) as usize % EDGES.len()]
    } else {
        (rng.next_u32() & 0xFF) as u8
    }
}

/// Generates one case. The object's words and bytes are random first, then
/// the words the predicates read are set from the boundary-biased pickers.
/// One case in four binds the task to no resource.
fn generate(rng: &mut Xorshift) -> Case {
    let mut object: Vec<u8> = (0..OBJECT_BYTES)
        .map(|_| (rng.next_u32() & 0xFF) as u8)
        .collect();
    let mut resource: Vec<u8> = (0..RESOURCE_BYTES)
        .map(|_| (rng.next_u32() & 0xFF) as u8)
        .collect();
    let resource_addr = address_of(&resource);
    set_word(&mut object, ENABLED_OFFSET, rng.word());
    set_word(&mut object, COUNT_OFFSET, rng.word());
    object[FLAGS_OFFSET as usize] = byte_pick(rng);
    object[KIND_OFFSET as usize] = byte_pick(rng);
    let bound = rng.next_u32() % 4 != 0;
    set_word(
        &mut object,
        RESOURCE_OFFSET,
        if bound { resource_addr } else { 0 },
    );
    set_word(&mut resource, RESOURCE_TAG_OFFSET, rng.word());
    Case {
        object,
        resource,
        resource_addr,
    }
}

/// Decodes a case into the owned task the lifted predicates take. The
/// resource pointer word is followed to the planted resource block, and
/// the test checks it addresses that block.
fn decode(case: &Case) -> PredicateTask {
    let pointer = word_at(&case.object, RESOURCE_OFFSET);
    let resource = if pointer == 0 {
        None
    } else {
        assert_eq!(
            pointer, case.resource_addr,
            "the pointer names the planted resource"
        );
        Some(ResourceTag {
            tag: word_at(&case.resource, RESOURCE_TAG_OFFSET),
        })
    };
    PredicateTask {
        enabled: word_at(&case.object, ENABLED_OFFSET),
        count: word_at(&case.object, COUNT_OFFSET) as i32,
        flags: case.object[FLAGS_OFFSET as usize],
        kind: case.object[KIND_OFFSET as usize],
        resource,
    }
}

/// Runs the rewrite of the enabled-count predicate on a case and returns
/// its answer, after checking that the rewrite changed no byte.
fn rewrite_enabled(case: &mut Case) -> u32 {
    let before_object = case.object.clone();
    let before_resource = case.resource.clone();
    let answer = fn_005c43e0::rw_005c43e0(address_of(&case.object));
    assert_eq!(
        case.object, before_object,
        "the predicate writes no object byte"
    );
    assert_eq!(
        case.resource, before_resource,
        "the predicate writes no resource byte"
    );
    answer
}

/// Runs the rewrite of the resource-tag predicate on a case and returns its
/// answer, after checking that the rewrite changed no byte.
fn rewrite_tag(case: &mut Case) -> u32 {
    let before_object = case.object.clone();
    let before_resource = case.resource.clone();
    let answer = fn_005c4420::rw_005c4420(address_of(&case.object));
    assert_eq!(
        case.object, before_object,
        "the predicate writes no object byte"
    );
    assert_eq!(
        case.resource, before_resource,
        "the predicate writes no resource byte"
    );
    answer
}

/// Counts the cases on which a candidate answer differs from its rewrite.
/// The rewrite's answer is the reference; the candidate sees the decoded
/// task only.
fn mismatches(candidate: impl Fn(&PredicateTask) -> bool, rewrite: fn(&mut Case) -> u32) -> u32 {
    let mut rng = Xorshift::new(SEED);
    let mut differing = 0;
    let mut answered_one = 0;
    for _ in 0..CASES {
        let mut case = generate(&mut rng);
        let task = decode(&case);
        let expected = rewrite(&mut case);
        let got = u32::from(candidate(&task));
        answered_one += expected;
        if got != expected {
            differing += 1;
        }
    }
    // Coverage line for the run log: how many cases the rewrite answered one.
    eprintln!("cases {CASES}: rewrite answered one {answered_one}, differing {differing}");
    differing
}

#[test]
fn enabled_positive_count_matches_its_rewrite() {
    assert_eq!(
        mismatches(PredicateTask::enabled_positive_count, rewrite_enabled),
        0
    );
}

#[test]
fn resource_tag_matches_its_rewrite() {
    assert_eq!(mismatches(PredicateTask::resource_tag, rewrite_tag), 0);
}

#[test]
fn enabled_positive_count_wrong_versions_are_caught() {
    // Counts that accept zero.
    let zero_counts = |t: &PredicateTask| t.enabled != 0 && t.count >= 0;
    // An enable test on the low bit only.
    let low_bit = |t: &PredicateTask| t.enabled & 1 != 0 && t.count > 0;
    // A count compared as unsigned: a negative count looks large and passes.
    let unsigned_count = |t: &PredicateTask| t.enabled != 0 && (t.count as u32) > 0;
    // The enable test dropped.
    let count_only = |t: &PredicateTask| t.count > 0;
    assert_caught("zero counts", zero_counts, rewrite_enabled);
    assert_caught("low bit only", low_bit, rewrite_enabled);
    assert_caught("unsigned count", unsigned_count, rewrite_enabled);
    assert_caught("count only", count_only, rewrite_enabled);
}

/// Asserts that a deliberately wrong version differs from its rewrite on at
/// least one generated case.
fn assert_caught(
    name: &str,
    wrong: impl Fn(&PredicateTask) -> bool,
    rewrite: fn(&mut Case) -> u32,
) {
    assert!(
        mismatches(wrong, rewrite) > 0,
        "wrong version '{name}' is caught"
    );
}

#[test]
fn resource_tag_wrong_versions_are_caught() {
    // The flag shortcut ignored: a set flag still needs the resource.
    let no_shortcut = |t: &PredicateTask| {
        if t.kind & 0x0F < 2 {
            return false;
        }
        match t.resource {
            None => false,
            Some(r) => r.tag & 0x3C0 == 0x80,
        }
    };
    // The kind compared without its mask.
    let unmasked_kind = |t: &PredicateTask| {
        if t.flags & 0x04 != 0 {
            return true;
        }
        if t.kind < 2 {
            return false;
        }
        match t.resource {
            None => false,
            Some(r) => r.tag & 0x3C0 == 0x80,
        }
    };
    // The tag mask one bit wider.
    let wider_mask = |t: &PredicateTask| {
        if t.flags & 0x04 != 0 {
            return true;
        }
        if t.kind & 0x0F < 2 {
            return false;
        }
        match t.resource {
            None => false,
            Some(r) => r.tag & 0x3F0 == 0x80,
        }
    };
    // The flag bit one position off.
    let other_flag = |t: &PredicateTask| {
        if t.flags & 0x08 != 0 {
            return true;
        }
        if t.kind & 0x0F < 2 {
            return false;
        }
        match t.resource {
            None => false,
            Some(r) => r.tag & 0x3C0 == 0x80,
        }
    };
    // The minimum kind one too high.
    let kind_three = |t: &PredicateTask| {
        if t.flags & 0x04 != 0 {
            return true;
        }
        if t.kind & 0x0F < 3 {
            return false;
        }
        match t.resource {
            None => false,
            Some(r) => r.tag & 0x3C0 == 0x80,
        }
    };
    assert_caught("no flag shortcut", no_shortcut, rewrite_tag);
    assert_caught("unmasked kind", unmasked_kind, rewrite_tag);
    assert_caught("wider tag mask", wider_mask, rewrite_tag);
    assert_caught("other flag bit", other_flag, rewrite_tag);
    assert_caught("kind minimum three", kind_three, rewrite_tag);
}
