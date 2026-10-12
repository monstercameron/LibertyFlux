//! Differential test of the lifted one-source, three-target word fan-out
//! against the verified rewrites of its family.
//!
//! Each rewrite reads one global and writes its value into three other
//! globals, in order, and returns zero. The lifted method is `WordFanout::apply`
//! in `lf-core`. For each instance, every global the family touches is planted
//! with generated values; the rewrite is called; and the lifted method runs on
//! a copy of the same words, with the instance's source and targets named by
//! their index in the test's table. Every global must match afterwards and the
//! rewrite's result must be zero.
//!
//! Wrong versions, each of which every instance must catch: the source moved to
//! another global, the last target left out, and the first target left out.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{INSTANCES, Instance};
use lf_core::word_fanout::WordFanout;
use lf_fanoutdiff::rt::{SLOT_VAS, plant, read_back};

/// Generated states per instance.
const CASES: u32 = 1000;

/// One lock across the whole test program: the rewrites share globals.
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

/// Index of a global's address in the test's table.
fn index_of(address: u32) -> usize {
    SLOT_VAS
        .iter()
        .position(|&va| va == address)
        .expect("test bug: an instance names a global outside the table")
}

/// A candidate: the same contract as the lifted method, for one instance's
/// source and targets given as table indices.
type Candidate = fn(&mut [u32], usize, [usize; 3]);

fn lifted(words: &mut [u32], source: usize, targets: [usize; 3]) {
    WordFanout::new(source, &targets).apply(words);
}

fn wrong_source_moved(words: &mut [u32], source: usize, targets: [usize; 3]) {
    let other = (source + 1) % words.len();
    WordFanout::new(other, &targets).apply(words);
}

fn wrong_last_target_left_out(words: &mut [u32], source: usize, targets: [usize; 3]) {
    WordFanout::new(source, &targets[..2]).apply(words);
}

fn wrong_first_target_left_out(words: &mut [u32], source: usize, targets: [usize; 3]) {
    WordFanout::new(source, &targets[1..]).apply(words);
}

/// The first mismatch of a candidate against the instance's rewrite, if any.
fn first_mismatch(instance: &Instance, candidate: Candidate, seed: u64) -> Option<String> {
    let source = index_of(instance.source_va);
    let targets = instance.target_vas.map(index_of);
    let mut rng = Rng(seed);
    for case in 0..CASES {
        let mut state = vec![0u32; SLOT_VAS.len()];
        for word in state.iter_mut() {
            *word = rng.word();
        }
        for (&va, &value) in SLOT_VAS.iter().zip(state.iter()) {
            plant(va, value);
        }
        let returned = (instance.call)();
        if returned != 0 {
            return Some(format!(
                "{} case {}: rewrite returned {:#x}, the verified result is zero",
                instance.address, case, returned
            ));
        }
        let after: Vec<u32> = SLOT_VAS.iter().map(|&va| read_back(va)).collect();

        let mut words = state.clone();
        candidate(&mut words, source, targets);
        if words != after {
            let k = (0..words.len()).find(|&k| words[k] != after[k]).unwrap_or(0);
            return Some(format!(
                "{} case {}: global {:#x} is {:#x} in the rewrite and {:#x} in the lift",
                instance.address, case, SLOT_VAS[k], after[k], words[k]
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
                first_mismatch(instance, wrong_source_moved, seed).is_some(),
                "wrong version (source moved) was not caught for {}",
                instance.address
            );
            assert!(
                first_mismatch(instance, wrong_last_target_left_out, seed).is_some(),
                "wrong version (last target left out) was not caught for {}",
                instance.address
            );
            assert!(
                first_mismatch(instance, wrong_first_target_left_out, seed).is_some(),
                "wrong version (first target left out) was not caught for {}",
                instance.address
            );
        }
    };
}

//@INSTANCE_TESTS@
