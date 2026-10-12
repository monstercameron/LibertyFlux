//! Differential test of the lifted constant-write routine against the verified
//! rewrites of its batch.
//!
//! Each instance writes constants into fields of the object it is given, in a
//! fixed order, and returns zero, one or its own address. For each instance,
//! CASES generated objects are filled with random bytes. The rewrite runs on the
//! object at its own address, the lifted routine (`ConstantWrites::apply`) runs
//! on a copy of the same bytes with the same address, and the returned value and
//! every byte of the object are compared.
//!
//! Wrong versions, each run against every instance and each required to fail:
//! the last write left out, the first write's constant with one bit changed,
//! every write four bytes further on, and a return one away from the right one.
//!
//! The test runs on the 32-bit target only, where the rewrites exist.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{CALLS, INSTANCES, NamedWrites};
use lf_core::constant_writes::{ConstantWrite, Returns, apply_writes};
use lf_core::field_store::ObjectBytes;

/// Bytes in the generated object. Every field the instances write lies below this.
const OBJECT_BYTES: usize = 0x200;
/// Cases per instance for the differential comparison.
const CASES: u32 = 1000;
/// Cases per instance and wrong version; a wrong version must be caught by one of them.
const WRONG_CASES: u32 = 64;
/// The number of instances the generated table lists.
const INSTANCE_COUNT: usize = //@COUNT@;

/// One lock across the whole test program.
static LOCK: Mutex<()> = Mutex::new(());

/// The object the rewrite writes into. Aligned, so word fields are aligned as in
/// the original.
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

/// An object filled with random bytes, so that every byte a wrong version writes
/// differs from what the right version leaves.
fn case_object(seed: u32) -> Object {
    let mut rng = Rng::new(seed);
    let mut object = Object([0; OBJECT_BYTES]);
    for chunk in object.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    object
}

/// The first mismatch of a lifted candidate against the rewrite, if any.
fn first_mismatch(
    instance: &NamedWrites,
    call: fn(u32) -> u32,
    lifted: &dyn Fn(&mut ObjectBytes, u32) -> u32,
    cases: u32,
    seed: u32,
) -> Option<String> {
    for case in 0..cases {
        let mut object = case_object(
            seed.wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B),
        );
        let original = object.clone();
        let this = object.0.as_mut_ptr() as usize as u32;
        let returned = call(this);
        let mut copy = ObjectBytes::from_bytes(&original.0);
        let lifted_return = lifted(&mut copy, this);
        if returned != lifted_return {
            return Some(format!(
                "{} case {}: rewrite returned {:#x}, lift returned {:#x}",
                instance.name, case, returned, lifted_return
            ));
        }
        if let Some(offset) = (0..OBJECT_BYTES).find(|&k| object.0[k] != copy.byte(k)) {
            return Some(format!(
                "{} case {}: byte {:#x} of the object is {:#x} in the rewrite and {:#x} in the lift",
                instance.name,
                case,
                offset,
                object.0[offset],
                copy.byte(offset)
            ));
        }
    }
    None
}

/// A deliberately wrong version of a routine's writes and return.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The last write is left out.
    DroppedWrite,
    /// The first write's constant has one bit flipped.
    ChangedValue,
    /// Every write lands four bytes further on.
    ShiftedOffset,
    /// The return differs.
    WrongReturn,
}

const WRONG_KINDS: [Wrong; 4] = [
    Wrong::DroppedWrite,
    Wrong::ChangedValue,
    Wrong::ShiftedOffset,
    Wrong::WrongReturn,
];

/// Runs a wrong version of an instance's routine.
fn run_wrong(instance: &NamedWrites, wrong: Wrong, object: &mut ObjectBytes, this: u32) -> u32 {
    let writes = instance.writes.writes;
    let returns = instance.writes.returns;
    match wrong {
        Wrong::DroppedWrite => {
            let kept = &writes[..writes.len() - 1];
            apply_writes(kept, returns, object, this)
        }
        Wrong::ChangedValue => {
            let mut changed: Vec<ConstantWrite> = writes.to_vec();
            if let Some(first) = changed.first_mut() {
                first.value ^= 1;
            }
            apply_writes(&changed, returns, object, this)
        }
        Wrong::ShiftedOffset => {
            let shifted: Vec<ConstantWrite> = writes
                .iter()
                .map(|write| ConstantWrite {
                    offset: write.offset + 4,
                    ..*write
                })
                .collect();
            apply_writes(&shifted, returns, object, this)
        }
        Wrong::WrongReturn => {
            let other = match returns {
                Returns::This => Returns::Zero,
                Returns::Zero | Returns::One => Returns::One,
            };
            apply_writes(writes, other, object, this)
        }
    }
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
            let call = CALLS[$index];
            let seed = 0x5851_F42D ^ ($index as u32 + 1).wrapping_mul(0x9E37_79B9);
            let lifted = |object: &mut ObjectBytes, this: u32| instance.writes.apply(object, this);
            if let Some(message) = first_mismatch(instance, call, &lifted, CASES, seed) {
                panic!("lifted routine differs from the rewrite: {message}");
            }
            for wrong in WRONG_KINDS {
                let wrong_lifted = |object: &mut ObjectBytes, this: u32| {
                    run_wrong(instance, wrong, object, this)
                };
                assert!(
                    first_mismatch(instance, call, &wrong_lifted, WRONG_CASES, seed).is_some(),
                    "wrong version ({wrong:?}) was not caught for {}",
                    instance.name
                );
            }
        }
    };
}

//@INSTANCE_TESTS@
