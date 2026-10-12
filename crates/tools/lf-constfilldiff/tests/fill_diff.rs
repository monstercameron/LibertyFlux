//! Differential test of the lifted constant fill against the verified rewrites
//! of its family.
//!
//! Each rewrite writes one fixed constant into three globals, in order, and
//! returns zero. The lifted method is `WordFill::apply` in `lf-core`, with the
//! constant of the instance. For each instance, CASES cases plant every global
//! of the family with generated values; the rewrite is called; and the lifted
//! method runs on a copy of the same words, with the instance's targets named
//! by their index in the test's table. Every global must match afterwards, and
//! the rewrite's result must be zero.
//!
//! Wrong versions, each of which every instance must catch: the constant with
//! its low bit flipped, the last target left out, and the first target left out.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{INSTANCES, Instance};
use lf_constfilldiff::rt::{SLOT_VAS, plant, read_back};
use lf_core::const_fill::{FILL_VALUES, WordFill};

/// Generated cases per instance.
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
/// constant and targets given as table indices.
type Candidate = fn(u32, &mut [u32], [usize; 3]);

fn lifted(value: u32, words: &mut [u32], targets: [usize; 3]) {
    WordFill::new(value).apply(words, &targets);
}

fn wrong_constant(value: u32, words: &mut [u32], targets: [usize; 3]) {
    WordFill::new(value ^ 1).apply(words, &targets);
}

fn wrong_last_target_left_out(value: u32, words: &mut [u32], targets: [usize; 3]) {
    WordFill::new(value).apply(words, &targets[..2]);
}

fn wrong_first_target_left_out(value: u32, words: &mut [u32], targets: [usize; 3]) {
    WordFill::new(value).apply(words, &targets[1..]);
}

/// The first mismatch of a candidate against the instance's rewrite, if any.
fn first_mismatch(
    instance: &Instance,
    value: u32,
    candidate: Candidate,
    seed: u64,
) -> Option<String> {
    let targets = instance.target_vas.map(index_of);
    let mut rng = Rng(seed);
    for case in 0..CASES {
        let mut state = vec![0u32; SLOT_VAS.len()];
        for word in state.iter_mut() {
            *word = rng.word();
        }
        for (&va, &word) in SLOT_VAS.iter().zip(state.iter()) {
            plant(va, word);
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
        candidate(value, &mut words, targets);
        if words != after {
            let k = (0..words.len())
                .find(|&k| words[k] != after[k])
                .unwrap_or(0);
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
            let value = FILL_VALUES[$index];
            let seed = 0x5851_F42D_4C95_7F2D ^ ($index as u64 + 1);
            if let Some(message) = first_mismatch(instance, value, lifted, seed) {
                panic!("lifted method differs from the rewrite: {message}");
            }
            assert!(
                first_mismatch(instance, value, wrong_constant, seed).is_some(),
                "wrong version (constant changed) was not caught for {}",
                instance.address
            );
            assert!(
                first_mismatch(instance, value, wrong_last_target_left_out, seed).is_some(),
                "wrong version (last target left out) was not caught for {}",
                instance.address
            );
            assert!(
                first_mismatch(instance, value, wrong_first_target_left_out, seed).is_some(),
                "wrong version (first target left out) was not caught for {}",
                instance.address
            );
        }
    };
}

instance_test!(t_fn_00967120, 0);
instance_test!(t_fn_009f9600, 1);
instance_test!(t_fn_00b819b0, 2);
instance_test!(t_fn_00b819d0, 3);
