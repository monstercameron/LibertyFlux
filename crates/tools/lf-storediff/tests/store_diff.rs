//! Differential test of the lifted constant-store method against the
//! verified rewrites of its family.
//!
//! Each instance writes two constants into two words and returns zero (or
//! nothing). The lifted method is `StaticWords::store_pair`. For each
//! instance, CASES generated states are planted in every word of this crate's
//! table; the rewrite runs on the planted memory and the lifted method runs
//! on the same state, and every word is compared. The rewrite's return value
//! is checked to be the constant it is verified to return, which the lifted
//! method does not carry.
//!
//! Wrong versions: the second store left out, and the first store left out.
//! The store-order version is equivalent for every instance (the two words of
//! each instance are distinct), and the test shows that rather than claiming
//! a catch.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{INSTANCES, Instance, WORDS};
use lf_core::static_list::{StaticWords, WordId};
use lf_storediff::{plant, read_back};

/// Generated states per instance.
const CASES: u32 = 1000;

/// One lock across the whole test program: the rewrites share slots.
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

/// A lifted method under test: the same contract as `store_pair` for the
/// instance's two words and values.
type Candidate = fn(&mut StaticWords, WordId, WordId, u32, u32);

fn lifted(
    words: &mut StaticWords,
    first: WordId,
    second: WordId,
    first_value: u32,
    second_value: u32,
) {
    words.store_pair(first, second, first_value, second_value);
}

fn wrong_second_left_out(
    words: &mut StaticWords,
    first: WordId,
    _second: WordId,
    first_value: u32,
    _second_value: u32,
) {
    words.set_word(first, first_value);
}

fn wrong_first_left_out(
    words: &mut StaticWords,
    _first: WordId,
    second: WordId,
    _first_value: u32,
    second_value: u32,
) {
    words.set_word(second, second_value);
}

fn wrong_store_order(
    words: &mut StaticWords,
    first: WordId,
    second: WordId,
    first_value: u32,
    second_value: u32,
) {
    words.set_word(second, second_value);
    words.set_word(first, first_value);
}

fn index_of(address: u32) -> usize {
    WORDS
        .iter()
        .position(|&w| w == address)
        .expect("test bug: an instance names a word outside the table")
}

/// The first mismatch of a candidate against the instance's rewrite, if any.
fn first_mismatch(instance: &Instance, candidate: Candidate, seed: u64) -> Option<String> {
    let first = index_of(instance.first);
    let second = index_of(instance.second);
    let mut rng = Rng(seed);
    for case in 0..CASES {
        let mut state = vec![0u32; WORDS.len()];
        for word in state.iter_mut() {
            *word = rng.word();
        }
        for (word, &value) in WORDS.iter().zip(state.iter()) {
            plant(*word, value);
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
        let after: Vec<u32> = WORDS.iter().map(|&w| read_back(w)).collect();

        let mut words = StaticWords::new(WORDS.len());
        for (k, &value) in state.iter().enumerate() {
            words.set_word(WordId(k), value);
        }
        candidate(
            &mut words,
            WordId(first),
            WordId(second),
            instance.first_value,
            instance.second_value,
        );
        let got: Vec<u32> = (0..WORDS.len()).map(|k| words.word(WordId(k))).collect();
        if got != after {
            let k = (0..WORDS.len()).find(|&k| got[k] != after[k]).unwrap_or(0);
            return Some(format!(
                "{} case {}: word {:#x} is {:#x} in the rewrite and {:#x} in the lift",
                instance.name, case, WORDS[k], after[k], got[k]
            ));
        }
    }
    None
}

macro_rules! instance_test {
    ($name:ident, $index:expr) => {
        #[test]
        fn $name() {
            let _guard = LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            let instance = &INSTANCES[$index];
            let seed = 0x5851_F42D_4C95_7F2D ^ ($index as u64 + 1);
            if let Some(message) = first_mismatch(instance, lifted, seed) {
                panic!("lifted method differs from the rewrite: {message}");
            }
            assert!(
                first_mismatch(instance, wrong_second_left_out, seed).is_some(),
                "wrong version (second store left out) was not caught for {}",
                instance.name
            );
            assert!(
                first_mismatch(instance, wrong_first_left_out, seed).is_some(),
                "wrong version (first store left out) was not caught for {}",
                instance.name
            );
            assert!(
                first_mismatch(instance, wrong_store_order, seed).is_none(),
                "store-order version differs for {}; the equivalence claim is wrong",
                instance.name
            );
        }
    };
}

#[test]
fn every_instance_stores_into_two_distinct_words() {
    for instance in INSTANCES {
        assert_ne!(instance.first, instance.second, "{}", instance.name);
    }
}

instance_test!(t_fn_006a33f0, 0);
instance_test!(t_fn_008f9260, 1);
instance_test!(t_fn_009bba00, 2);
instance_test!(t_fn_00b02d10, 3);
instance_test!(t_fn_00db1920, 4);
instance_test!(t_fn_00e607c0, 5);
instance_test!(t_fn_00e60880, 6);
instance_test!(t_fn_00e60e50, 7);
