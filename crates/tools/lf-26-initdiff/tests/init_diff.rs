//! Differential test of the existing constant-write routine (`lf_core::constant_writes`) against three verified
//! 32-bit rewrites whose bodies are constant writes into one object followed by a fixed return.
//!
//! For each instance, `CASES` generated cases run. A case is a generated object. The rewrite runs on the object in
//! memory, and the lifted routine runs on a copy of the same object at the same address. The returned word and
//! every byte of the object are compared.
//!
//! Deliberately wrong versions of the lifted routine run through the same contract, and each must be caught by at
//! least one case of every instance: the last write left out, the first write's constant with one bit flipped,
//! every write moved four bytes on, and the return one away from the right one.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from fixed seeds.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::INSTANCES;
use lf_core::constant_writes::{ConstantWrite, Returns, apply_writes};
use lf_core::field_store::ObjectBytes;

/// Generated cases per instance.
const CASES: u32 = 1000;

/// Bytes in the generated object. Every field the instances write lies below this.
const OBJECT_BYTES: usize = 0x200;

/// One lock across the whole test program, as the other differential programs keep.
static LOCK: Mutex<()> = Mutex::new(());

/// The object the rewrite writes into. Aligned as in the original.
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

/// The object for one case, filled with random bytes so that every byte a wrong version changes differs.
fn make_object(instance: usize, case: u32) -> Object {
    let seed =
        ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    let mut object = Object([0; OBJECT_BYTES]);
    for chunk in object.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    object
}

/// The verified rewrite on one object: its result, the address the rewrite saw, and the object afterwards.
fn run_rewrite(instance: usize, object: &Object) -> (u32, u32, Object) {
    let mut object = object.clone();
    let this = object.0.as_mut_ptr() as usize as u32;
    let returned = (INSTANCES[instance].rewrite)(this);
    (returned, this, object)
}

/// The existing lifted routine on a copy of the object, at the address the rewrite saw.
fn run_lifted(instance: usize, object: &Object, this: u32) -> (u32, Vec<u8>) {
    let mut lifted = ObjectBytes::from_bytes(&object.0);
    let returned = INSTANCES[instance].writes.apply(&mut lifted, this);
    (returned, lifted.bytes().to_vec())
}

/// A deliberately wrong version: the same writes and return as the lifted routine, altered as named.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The last write is left out.
    DroppedWrite,
    /// The first write's constant has its lowest bit flipped.
    ChangedValue,
    /// Every write lands four bytes further on.
    ShiftedOffset,
    /// The return is one away from the right one.
    WrongReturn,
}

/// The wrong version on one object: its result, and the object afterwards.
fn run_wrong(wrong: Wrong, instance: usize, object: &Object, this: u32) -> (u32, Vec<u8>) {
    let named = &INSTANCES[instance].writes;
    let mut writes: Vec<ConstantWrite> = named.writes.to_vec();
    let mut returns = named.returns;
    match wrong {
        Wrong::DroppedWrite => {
            writes.pop();
        }
        Wrong::ChangedValue => {
            writes[0].value ^= 1;
        }
        Wrong::ShiftedOffset => {
            for write in &mut writes {
                write.offset += 4;
            }
        }
        Wrong::WrongReturn => {
            returns = match returns {
                Returns::Zero => Returns::One,
                Returns::One | Returns::This => Returns::Zero,
            };
        }
    }
    let mut lifted = ObjectBytes::from_bytes(&object.0);
    let returned = apply_writes(&writes, returns, &mut lifted, this);
    (returned, lifted.bytes().to_vec())
}

#[test]
fn lifted_routine_matches_every_verified_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    for instance in 0..INSTANCES.len() {
        for case in 0..CASES {
            let object = make_object(instance, case);
            let (rewrite_result, this, rewrite_object) = run_rewrite(instance, &object);
            let (lifted_result, lifted_object) = run_lifted(instance, &object, this);
            assert_eq!(
                lifted_result, rewrite_result,
                "{} case {case}: returned word differs",
                INSTANCES[instance].name
            );
            assert_eq!(
                lifted_object,
                rewrite_object.0.to_vec(),
                "{} case {case}: object bytes differ",
                INSTANCES[instance].name
            );
        }
    }
}

#[test]
fn every_wrong_version_is_caught_by_every_instance() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    let wrongs = [
        Wrong::DroppedWrite,
        Wrong::ChangedValue,
        Wrong::ShiftedOffset,
        Wrong::WrongReturn,
    ];
    for instance in 0..INSTANCES.len() {
        for wrong in wrongs {
            let mut caught = 0u32;
            for case in 0..CASES {
                let object = make_object(instance, case);
                let (rewrite_result, this, rewrite_object) = run_rewrite(instance, &object);
                let (wrong_result, wrong_object) = run_wrong(wrong, instance, &object, this);
                if wrong_result != rewrite_result || wrong_object != rewrite_object.0.to_vec() {
                    caught += 1;
                }
            }
            assert!(
                caught > 0,
                "{} wrong version {wrong:?} was not caught by any case",
                INSTANCES[instance].name
            );
        }
    }
}
