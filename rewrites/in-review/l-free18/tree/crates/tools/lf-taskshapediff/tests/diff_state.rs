//! Differential cases of the task state block's reset against its verified
//! 32-bit rewrite.
//!
//! The rewrite and the lifted reset run on the same generated blocks. The
//! effect compared is every one of the seventy-three words of the block
//! after the call. The rewrite's return value is not compared: the original
//! leaves no meaningful value there, and no caller of it reads one.
//!
//! The lifted side reads the planted words through the owned
//! [`TaskStateBlock`]: the test decodes the words into the type's fields,
//! calls [`TaskStateBlock::reset`] and compares the block's words in memory
//! order with the rewrite's words.
//!
//! The reset also has deliberately wrong versions. Each must differ from its
//! rewrite on at least one generated case.
//!
//! Runs on the 32-bit target only: the rewrite takes a raw address.

#![cfg(target_arch = "x86")]

use lf_peds_tasks::ped_task::{BLOCK_WORDS, StateRecord, TaskStateBlock, STATE_RECORD_COUNT, STATE_RECORD_WORDS};
use lf_taskshapediff::Xorshift;
use lf_taskshapediff::rewrites::fn_00cc2eb0;

/// Generated cases.
const CASES: u32 = 3000;
/// Seed of the case generator for every test in this program.
const SEED: u32 = 0x5EED_0cc2;

/// Generates one block of random words, a third of them boundary values.
fn generate(rng: &mut Xorshift) -> Vec<u32> {
    (0..BLOCK_WORDS).map(|_| rng.word()).collect()
}

/// Decodes planted words into the owned block: the header, then nine
/// records of two handle words and six further words.
fn decode(words: &[u32]) -> TaskStateBlock {
    let mut block = TaskStateBlock {
        header: words[0],
        records: [StateRecord::default(); STATE_RECORD_COUNT],
    };
    for (k, record) in block.records.iter_mut().enumerate() {
        let base = 1 + k * STATE_RECORD_WORDS;
        record.handle_a = words[base];
        record.handle_b = words[base + 1];
        record.words.copy_from_slice(&words[base + 2..base + STATE_RECORD_WORDS]);
    }
    block
}

/// Runs the rewrite on a planted copy of the words and returns the words
/// afterwards.
fn rewrite_reset(words: &[u32]) -> Vec<u32> {
    let mut planted = words.to_vec();
    let address = planted.as_mut_ptr() as usize as u32;
    let _ignored_return = fn_00cc2eb0::rw_00cc2eb0(address);
    planted
}

/// Counts the cases on which a candidate reset differs from the rewrite in
/// any of the block's words.
fn mismatches(candidate: impl Fn(&mut TaskStateBlock)) -> u32 {
    let mut rng = Xorshift::new(SEED);
    let mut differing = 0;
    for _ in 0..CASES {
        let words = generate(&mut rng);
        let expected = rewrite_reset(&words);
        let mut block = decode(&words);
        candidate(&mut block);
        if block.to_words().as_slice() != expected.as_slice() {
            differing += 1;
        }
    }
    differing
}

/// Asserts that a deliberately wrong reset differs from the rewrite on at
/// least one generated case.
fn assert_caught(name: &str, wrong: impl Fn(&mut TaskStateBlock)) {
    assert!(mismatches(wrong) > 0, "wrong version '{name}' is caught");
}

#[test]
fn reset_matches_its_rewrite_on_every_word() {
    assert_eq!(mismatches(TaskStateBlock::reset), 0);
}

#[test]
fn reset_wrong_versions_are_caught() {
    // Only the first handle word of each record is marked invalid.
    assert_caught("first handle only", |block| {
        block.reset();
        for record in &mut block.records {
            record.handle_b = 0;
        }
    });
    // The last record is left without its sentinels.
    assert_caught("last record unmarked", |block| {
        block.reset();
        block.records[STATE_RECORD_COUNT - 1] = StateRecord::default();
    });
    // The sentinel is one off.
    assert_caught("sentinel one off", |block| {
        block.reset();
        for record in &mut block.records {
            record.handle_a = 0xFFFF_FFFE;
        }
    });
    // The header is marked invalid too.
    assert_caught("header marked", |block| {
        block.reset();
        block.header = 0xFFFF_FFFF;
    });
    // Only the handle words are reset; the six further words keep their values.
    assert_caught("payload kept", |block| {
        let kept: Vec<[u32; STATE_RECORD_WORDS - 2]> = block.records.iter().map(|r| r.words).collect();
        block.reset();
        for (record, words) in block.records.iter_mut().zip(kept) {
            record.words = words;
        }
    });
}
