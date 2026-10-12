//! Differential test of the existing lifted block copy (`lf_core::word_block`) against one verified 32-bit rewrite
//! that copies six words from the caller's buffer into an object and returns the buffer's address.
//!
//! The module documents its family as three or four words; the routine takes the word count as a parameter, and
//! this test proves the six-word instance that the rewrite is. For `CASES` generated cases, the rewrite runs on the
//! object and the buffer in memory and the lifted copy runs on copies of the same two. The returned address and
//! every byte of the object and of the buffer are compared. The rewrite's returned address is the buffer's own
//! address, so the lifted outcome is mapped to that address before comparing.
//!
//! Deliberately wrong versions of the copy run through the same contract, and each must be caught by at least one
//! case: five words instead of six, a copy in the wrong direction, the object offset moved four bytes, and a
//! zero return instead of the buffer.
//!
//! The test runs on the 32-bit target only, where the rewrite exists. Its inputs come from fixed seeds.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

#[path = "support/instances_gen.rs"]
mod instances;

use instances::INSTANCES;
use lf_core::word_block::{CopyDirection, CopyOutcome, CopyReturn, WordBlockCopy};

/// Generated cases per instance.
const CASES: u32 = 1000;

/// Bytes in the generated object. The copy writes six words at offset 0x10, so it ends at 0x28.
const OBJECT_BYTES: usize = 0x80;

/// Bytes in the generated buffer: six words.
const BUFFER_BYTES: usize = 0x18;

/// The object the rewrite writes into. Aligned as in the original.
#[repr(align(16))]
#[derive(Clone)]
struct Object([u8; OBJECT_BYTES]);

/// The buffer the rewrite reads from. Aligned as in the original.
#[repr(align(16))]
#[derive(Clone)]
struct Buffer([u8; BUFFER_BYTES]);

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

/// One generated case: the object and the buffer before the call.
struct Case {
    object: Object,
    buffer: Buffer,
}

/// The case `case`, from a seed that depends on it.
fn make_case(case: u32) -> Case {
    let mut rng = Rng::new(0x5EED_0000 ^ (case + 1).wrapping_mul(0x85EB_CA6B));
    let mut object = Object([0; OBJECT_BYTES]);
    for chunk in object.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    let mut buffer = Buffer([0; BUFFER_BYTES]);
    for chunk in buffer.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    Case { object, buffer }
}

/// The verified rewrite on one case: its result, the buffer's address it was given, and the object and buffer
/// afterwards.
fn run_rewrite(instance: usize, case: &Case) -> (u32, u32, Vec<u8>, Vec<u8>) {
    let mut object = case.object.clone();
    let buffer = case.buffer.clone();
    let object_address = object.0.as_mut_ptr();
    let buffer_address = buffer.0.as_ptr();
    let returned = (INSTANCES[instance].rewrite)(object_address, buffer_address);
    (
        returned,
        buffer_address as usize as u32,
        object.0.to_vec(),
        buffer.0.to_vec(),
    )
}

/// Maps a copy outcome to the word the original returns, given the buffer's address.
fn outcome_word(outcome: CopyOutcome, buffer_address: u32) -> u32 {
    match outcome {
        CopyOutcome::Null => 0,
        CopyOutcome::Buffer => buffer_address,
        CopyOutcome::Word(word) => word,
    }
}

/// The lifted copy on copies of the case's object and buffer, with the buffer's address the rewrite saw.
fn run_lifted(copy: WordBlockCopy, case: &Case, buffer_address: u32) -> (u32, Vec<u8>, Vec<u8>) {
    let mut object = case.object.0.to_vec();
    let mut buffer = case.buffer.0.to_vec();
    let outcome = copy.run(&mut object, &mut buffer);
    (outcome_word(outcome, buffer_address), object, buffer)
}

/// A deliberately wrong version of the lifted copy.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// Five words are copied instead of six.
    FiveWords,
    /// The copy runs from the object to the buffer.
    FromObject,
    /// The run starts four bytes further into the object.
    OffsetPlusFour,
    /// Zero is returned instead of the buffer's address.
    ReturnNull,
}

fn wrong_copy(wrong: Wrong) -> WordBlockCopy {
    let right = INSTANCES[0].copy;
    match wrong {
        Wrong::FiveWords => {
            WordBlockCopy::new(right.direction, right.object_offset, 5, right.returns)
        }
        Wrong::FromObject => WordBlockCopy::new(
            CopyDirection::FromObject,
            right.object_offset,
            right.words,
            right.returns,
        ),
        Wrong::OffsetPlusFour => WordBlockCopy::new(
            right.direction,
            right.object_offset + 4,
            right.words,
            right.returns,
        ),
        Wrong::ReturnNull => WordBlockCopy::new(
            right.direction,
            right.object_offset,
            right.words,
            CopyReturn::Null,
        ),
    }
}

#[test]
fn lifted_copy_matches_the_verified_rewrite() {
    let copy = INSTANCES[0].copy;
    for case in 0..CASES {
        let generated = make_case(case);
        let (rewrite_result, buffer_address, rewrite_object, rewrite_buffer) =
            run_rewrite(0, &generated);
        let (lifted_result, lifted_object, lifted_buffer) =
            run_lifted(copy, &generated, buffer_address);
        assert_eq!(
            lifted_result, rewrite_result,
            "{} case {case}: returned address differs",
            INSTANCES[0].name
        );
        assert_eq!(
            lifted_object, rewrite_object,
            "{} case {case}: object bytes differ",
            INSTANCES[0].name
        );
        assert_eq!(
            lifted_buffer, rewrite_buffer,
            "{} case {case}: buffer bytes differ",
            INSTANCES[0].name
        );
    }
}

#[test]
fn every_wrong_version_is_caught() {
    for wrong in [
        Wrong::FiveWords,
        Wrong::FromObject,
        Wrong::OffsetPlusFour,
        Wrong::ReturnNull,
    ] {
        let mut caught = 0u32;
        for case in 0..CASES {
            let generated = make_case(case);
            let (rewrite_result, buffer_address, rewrite_object, rewrite_buffer) =
                run_rewrite(0, &generated);
            let (wrong_result, wrong_object, wrong_buffer) =
                run_lifted(wrong_copy(wrong), &generated, buffer_address);
            if wrong_result != rewrite_result
                || wrong_object != rewrite_object
                || wrong_buffer != rewrite_buffer
            {
                caught += 1;
            }
        }
        assert!(
            caught > 0,
            "{} wrong version {wrong:?} was not caught by any case",
            INSTANCES[0].name
        );
    }
}
