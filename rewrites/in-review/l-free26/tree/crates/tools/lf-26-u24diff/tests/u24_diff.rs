//! Differential test of the lifted three-byte store (`lf_core::u24_store`) against the four verified 32-bit
//! rewrites of its family.
//!
//! For each instance, `CASES` generated cases run. A case is a generated object and a generated parameter. The
//! rewrite runs on the object in memory and the lifted routine on a copy of the same object. The returned word
//! and every byte of the object are compared. Two of the rewrites take the object as an address and two as a
//! byte pointer; both forms are called with the object's address, and the test runs each in its own form.
//!
//! Deliberately wrong versions of the lifted routine run through the same contract, and each must be caught by
//! at least one case of every instance: only two bytes stored, the parameter returned unshifted, the middle byte
//! taken from the wrong bits, the bytes written one byte further on, and a fourth byte written too.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from fixed seeds.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

#[path = "support/instances_gen.rs"]
mod instances;

use instances::{Form, INSTANCES};
use lf_core::field_store::ObjectBytes;
use lf_core::u24_store::U24Store;

/// Generated cases per instance.
const CASES: u32 = 1000;

/// Bytes in the generated object. Every byte the instances write lies below this.
const OBJECT_BYTES: usize = 0x80;

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

/// One generated case: the object before the call and the parameter.
struct Case {
    object: Object,
    value: u32,
}

/// The case `case` of instance `instance`, from a seed that depends on both.
fn make_case(instance: usize, case: u32) -> Case {
    let seed = ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    let mut object = Object([0; OBJECT_BYTES]);
    for chunk in object.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    Case {
        object,
        value: rng.next_u32(),
    }
}

/// The verified rewrite on one case: its result, and the object afterwards.
fn run_rewrite(instance: usize, case: &Case) -> (u32, Object) {
    let mut object = case.object.clone();
    let this = object.0.as_mut_ptr();
    let returned = match INSTANCES[instance].form {
        Form::Address(rewrite) => rewrite(this as usize as u32, case.value),
        Form::Pointer(rewrite) => rewrite(this, case.value),
    };
    (returned, object)
}

/// The lifted routine on a copy of the case's object.
fn run_lifted(instance: usize, case: &Case) -> (u32, Vec<u8>) {
    let routine = U24Store::new(INSTANCES[instance].offset);
    let mut object = ObjectBytes::from_bytes(&case.object.0);
    let returned = routine.store(&mut object, case.value);
    (returned, object.bytes().to_vec())
}

/// A deliberately wrong version of the lifted routine.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// Only the two low bytes are stored.
    TwoBytesOnly,
    /// The parameter is returned, not shifted.
    ReturnUnshifted,
    /// The middle byte is taken one bit further up.
    MiddleShifted,
    /// The three bytes land one byte further on.
    OffsetPlusOne,
    /// The top byte is stored as well, four bytes in all.
    FourBytes,
}

/// The wrong version on one case: its result, and the object afterwards.
fn run_wrong(wrong: Wrong, instance: usize, case: &Case) -> (u32, Vec<u8>) {
    let offset = INSTANCES[instance].offset;
    let mut object = ObjectBytes::from_bytes(&case.object.0);
    let value = case.value.to_le_bytes();
    let returned = match wrong {
        Wrong::TwoBytesOnly => {
            object.write(offset, &value[..2]);
            case.value >> 8
        }
        Wrong::ReturnUnshifted => {
            object.write(offset, &value[..3]);
            case.value
        }
        Wrong::MiddleShifted => {
            object.write(offset, &[value[0], (case.value >> 9) as u8, value[2]]);
            case.value >> 8
        }
        Wrong::OffsetPlusOne => {
            object.write(offset + 1, &value[..3]);
            case.value >> 8
        }
        Wrong::FourBytes => {
            object.write(offset, &value);
            case.value >> 8
        }
    };
    (returned, object.bytes().to_vec())
}

#[test]
fn lifted_routine_matches_every_verified_rewrite() {
    for instance in 0..INSTANCES.len() {
        for case in 0..CASES {
            let generated = make_case(instance, case);
            let (rewrite_result, rewrite_object) = run_rewrite(instance, &generated);
            let (lifted_result, lifted_object) = run_lifted(instance, &generated);
            assert_eq!(
                lifted_result, rewrite_result,
                "{} case {case}: returned word differs (value {:#010x})",
                INSTANCES[instance].name, generated.value
            );
            assert_eq!(
                lifted_object,
                rewrite_object.0.to_vec(),
                "{} case {case}: object bytes differ (value {:#010x})",
                INSTANCES[instance].name,
                generated.value
            );
        }
    }
}

#[test]
fn every_wrong_version_is_caught_by_every_instance() {
    let wrongs = [
        Wrong::TwoBytesOnly,
        Wrong::ReturnUnshifted,
        Wrong::MiddleShifted,
        Wrong::OffsetPlusOne,
        Wrong::FourBytes,
    ];
    for instance in 0..INSTANCES.len() {
        for wrong in wrongs {
            let mut caught = 0u32;
            for case in 0..CASES {
                let generated = make_case(instance, case);
                let (rewrite_result, rewrite_object) = run_rewrite(instance, &generated);
                let (wrong_result, wrong_object) = run_wrong(wrong, instance, &generated);
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
