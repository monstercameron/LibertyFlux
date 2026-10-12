//! Differential test of the lifted indexed-slot routine (`lf_core::indexed_slot_assign`) against the four
//! verified 32-bit rewrites of its family.
//!
//! For each instance, `CASES` generated cases run. A case is a generated object and a generated table of
//! words, and an index that names a word of the table. The rewrite runs on the object in memory with the table
//! planted in its global, and the lifted routine runs on a copy of the same object with the same table. The
//! returned word and every byte of the object are compared.
//!
//! Deliberately wrong versions of the lifted routine run through the same contract, and each must be caught by
//! at least one case of every instance: the index not stored, the neighbouring table word read, the index and
//! the word written into each other's fields, the index returned, and the value written one word too far on.
//!
//! The index is drawn from the planted table's extent. The original reads whatever lies at the address the
//! index names; the rewrites are only ever run on indices inside the table the runtime plants, so the proof
//! covers those indices and no others.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from fixed seeds.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::INSTANCES;
use lf_26_assigndiff::rt::{TABLE_LEN, plant_table};
use lf_core::field_store::ObjectBytes;
use lf_core::indexed_slot_assign::IndexedSlotAssign;

/// Generated cases per instance.
const CASES: u32 = 1000;

/// Bytes in the generated object. Every field the instances write lies below this.
const OBJECT_BYTES: usize = 0x800;

/// One lock across the whole test program: the rewrites share the table global.
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

/// One generated case: the object before the call, the table, and the index.
struct Case {
    object: Object,
    table: [u32; TABLE_LEN],
    index: u32,
}

/// The case `case` of instance `instance`, from a seed that depends on both.
fn make_case(instance: usize, case: u32) -> Case {
    let seed =
        ((instance as u32) + 1).wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B);
    let mut rng = Rng::new(seed);
    let mut object = Object([0; OBJECT_BYTES]);
    for chunk in object.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    let mut table = [0u32; TABLE_LEN];
    for word in &mut table {
        *word = rng.next_u32();
    }
    let index = rng.next_u32() % TABLE_LEN as u32;
    Case {
        object,
        table,
        index,
    }
}

/// The verified rewrite on one case: its result, and the object afterwards.
fn run_rewrite(instance: usize, case: &Case) -> (u32, Object) {
    let mut object = case.object.clone();
    plant_table(&case.table);
    let this = object.0.as_mut_ptr() as usize as u32;
    let returned = (INSTANCES[instance].rewrite)(this, case.index);
    (returned, object)
}

/// The lifted routine on a copy of the case's object, with the same table.
fn run_lifted(instance: usize, case: &Case) -> (u32, Vec<u8>) {
    let named = &INSTANCES[instance];
    let routine = IndexedSlotAssign::new(named.index_offset, named.value_offset);
    let mut object = ObjectBytes::from_bytes(&case.object.0);
    let returned = routine.assign(&mut object, &case.table, case.index);
    (returned, object.bytes().to_vec())
}

/// A deliberately wrong version of the lifted routine.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The index is not stored in the index field.
    SkipIndex,
    /// The table word one position away from the index is read.
    ReadNeighbour,
    /// The index and the word are written into each other's fields.
    SwappedFields,
    /// The index is returned instead of the word.
    ReturnIndex,
    /// The word is written one word past the value field.
    ValueOneWordOn,
}

/// The wrong version on one case: its result, and the object afterwards.
fn run_wrong(wrong: Wrong, instance: usize, case: &Case) -> (u32, Vec<u8>) {
    let named = &INSTANCES[instance];
    let (index_offset, value_offset) = (named.index_offset, named.value_offset);
    let mut object = ObjectBytes::from_bytes(&case.object.0);
    let index = case.index;
    let word = case.table[index as usize];
    let neighbour = case.table[(index ^ 1) as usize];
    let returned = match wrong {
        Wrong::SkipIndex => {
            object.write(value_offset, &word.to_le_bytes());
            word
        }
        Wrong::ReadNeighbour => {
            object.write(index_offset, &index.to_le_bytes());
            object.write(value_offset, &neighbour.to_le_bytes());
            neighbour
        }
        Wrong::SwappedFields => {
            object.write(value_offset, &index.to_le_bytes());
            object.write(index_offset, &word.to_le_bytes());
            word
        }
        Wrong::ReturnIndex => {
            object.write(index_offset, &index.to_le_bytes());
            object.write(value_offset, &word.to_le_bytes());
            index
        }
        Wrong::ValueOneWordOn => {
            object.write(index_offset, &index.to_le_bytes());
            object.write(value_offset + 4, &word.to_le_bytes());
            word
        }
    };
    (returned, object.bytes().to_vec())
}

#[test]
fn lifted_routine_matches_every_verified_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    for instance in 0..INSTANCES.len() {
        for case in 0..CASES {
            let generated = make_case(instance, case);
            let (rewrite_result, rewrite_object) = run_rewrite(instance, &generated);
            let (lifted_result, lifted_object) = run_lifted(instance, &generated);
            assert_eq!(
                lifted_result, rewrite_result,
                "{} case {case}: returned word differs (index {})",
                INSTANCES[instance].name, generated.index
            );
            assert_eq!(
                lifted_object,
                rewrite_object.0.to_vec(),
                "{} case {case}: object bytes differ (index {})",
                INSTANCES[instance].name,
                generated.index
            );
        }
    }
}

#[test]
fn every_wrong_version_is_caught_by_every_instance() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    let wrongs = [
        Wrong::SkipIndex,
        Wrong::ReadNeighbour,
        Wrong::SwappedFields,
        Wrong::ReturnIndex,
        Wrong::ValueOneWordOn,
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
