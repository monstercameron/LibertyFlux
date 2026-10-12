//! Differential test of the lifted static-row method against the verified
//! rewrites of its family.
//!
//! Each instance writes one four-word block: its first constant, two zeros and
//! its second constant. For each instance, CASES generated states are planted
//! in every word of the table. The rewrite runs on the planted table and the
//! lifted method runs on the same state, and every word of the table is
//! compared, so a store the rewrite makes outside its block is caught too. Where
//! the rewrite's contract has a return, the rewrite's return is compared with
//! it.
//!
//! Wrong versions, each run against every instance and each required to fail:
//! the first constant changed, the last store left out, and a zero word written
//! as one.
//!
//! One rewrite writes a 64-bit word four bytes past an eight-byte boundary. The
//! test crate's profile switches the debug build's pointer-alignment check off,
//! as the workspace manifest says, so the test runs in an ordinary debug build.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{INSTANCES, Instance};
use lf_25_rowdiff::rt::{MEM_BASE, MEM_WORDS};
use lf_25_rowdiff::{plant, read_back};
use lf_core::static_list::{StaticWords, WordId};
use lf_core::static_row::StaticRow;

/// Generated states per instance.
const CASES: u32 = 1000;

/// The number of instances the generated table lists.
const INSTANCE_COUNT: usize = //@COUNT@;

/// One lock across the whole test program: the rewrites share the table.
static LOCK: Mutex<()> = Mutex::new(());

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

    fn word(&mut self) -> u32 {
        self.next() as u32
    }
}

/// A lifted method under test: given the index of the block's first word and
/// the instance's row, it writes the block.
type Candidate = fn(&mut StaticWords, WordId, StaticRow);

/// The lifted method itself.
fn lifted(words: &mut StaticWords, base: WordId, row: StaticRow) {
    row.write(words, base);
}

/// Wrong version: the first constant is changed.
fn wrong_first_constant(words: &mut StaticWords, base: WordId, row: StaticRow) {
    StaticRow::new(row.first ^ 1, row.second).write(words, base);
}

/// Wrong version: the last store is left out.
fn wrong_last_store_left_out(words: &mut StaticWords, base: WordId, row: StaticRow) {
    words.set_word(base, row.first);
    words.set_word(WordId(base.0 + 1), 0);
    words.set_word(WordId(base.0 + 2), 0);
}

/// Wrong version: a zero word receives one.
fn wrong_zero_word_is_one(words: &mut StaticWords, base: WordId, row: StaticRow) {
    row.write(words, base);
    words.set_word(WordId(base.0 + 2), 1);
}

/// The index, in the table, of the word at an image address.
fn index_of(address: u32) -> usize {
    assert!(
        address >= MEM_BASE && (address - MEM_BASE) % 4 == 0,
        "test bug: a block lies outside the table or is not word aligned"
    );
    let index = ((address - MEM_BASE) / 4) as usize;
    assert!(index + 4 <= MEM_WORDS, "test bug: a block runs past the table");
    index
}

/// The first mismatch of a candidate against the instance's rewrite, if any.
fn first_mismatch(instance: &Instance, candidate: Candidate, seed: u64) -> Option<String> {
    let base = WordId(index_of(instance.base));
    let row = StaticRow::new(instance.first, instance.second);
    let mut rng = Rng(seed);
    for case in 0..CASES {
        let state: Vec<u32> = (0..MEM_WORDS).map(|_| rng.word()).collect();
        for (k, &value) in state.iter().enumerate() {
            plant(MEM_BASE + 4 * (k as u32), value);
        }
        let returned = (instance.call)();
        if let Some(expected) = instance.returns {
            if returned != expected {
                return Some(format!(
                    "{} case {}: rewrite returned {:#x}, verified constant is {:#x}",
                    instance.name, case, returned, expected
                ));
            }
        }
        let after: Vec<u32> = (0..MEM_WORDS)
            .map(|k| read_back(MEM_BASE + 4 * (k as u32)))
            .collect();

        let mut words = StaticWords::new(MEM_WORDS);
        for (k, &value) in state.iter().enumerate() {
            words.set_word(WordId(k), value);
        }
        candidate(&mut words, base, row);
        if let Some(k) = (0..MEM_WORDS).find(|&k| words.word(WordId(k)) != after[k]) {
            return Some(format!(
                "{} case {}: word {:#x} is {:#x} in the rewrite and {:#x} in the lift",
                instance.name,
                case,
                MEM_BASE + 4 * (k as u32),
                after[k],
                words.word(WordId(k))
            ));
        }
    }
    None
}

#[test]
fn the_instance_table_has_every_member() {
    assert_eq!(INSTANCES.len(), INSTANCE_COUNT);
}

macro_rules! instance_test {
    ($name:ident, $index:expr, $export:expr) => {
        #[test]
        fn $name() {
            let _guard = LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            let instance = &INSTANCES[$index];
            assert_eq!(instance.name, $export);
            let seed = 0x5851_F42D_4C95_7F2D ^ ($index as u64 + 1);
            if let Some(message) = first_mismatch(instance, lifted, seed) {
                panic!("lifted method differs from the rewrite: {message}");
            }
            let wrong: [(&str, Candidate); 3] = [
                ("first constant changed", wrong_first_constant),
                ("last store left out", wrong_last_store_left_out),
                ("zero word written as one", wrong_zero_word_is_one),
            ];
            for (label, candidate) in wrong {
                assert!(
                    first_mismatch(instance, candidate, seed).is_some(),
                    "wrong version ({label}) was not caught for {}",
                    instance.name
                );
            }
        }
    };
}

//@INSTANCE_TESTS@
