//! Differential test of the lifted row lookup against the verified rewrites of
//! its family.
//!
//! Each rewrite returns the row address of an index in an object when the
//! object's flag word is set, and zero otherwise. The lifted method is
//! `RowLookup::row_or_null` in `lf-core`, with the constants of the instance.
//! For each instance, CASES generated objects are built in memory; the rewrite
//! is called with the address of the object, and the lifted method is called
//! on a copy of the same bytes. The return values must match, and the rewrite
//! must leave the object's bytes unchanged, since it only reads.
//!
//! The rewrite reads the flag word and the base word from the object, and the
//! base word is a number that the rewrite never dereferences, so the test
//! needs no memory beyond the object itself.
//!
//! Wrong versions, each of which every instance must catch: the sense of the
//! flag test inverted, the index left out, the stride off by four, and the
//! flag read from the base word's offset.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use instances::{INSTANCES, Instance};
use lf_core::object_rows::{RowLookup, read_word};

/// Generated objects per instance.
const CASES: u32 = 1000;

/// Bytes in one generated object. Every offset the rewrites read is below this.
const OBJECT_BYTES: usize = 0x80;

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

    /// An index: mostly random, with the edge values the rewrite can see.
    fn index(&mut self) -> u32 {
        match self.next() % 8 {
            0 => 0,
            1 => 1,
            2 => u32::MAX,
            3 => 0x8000_0000,
            4 => (self.next() % 256) as u32,
            _ => self.word(),
        }
    }
}

/// A candidate: the same contract as the lifted method, for one instance.
type Candidate = fn(RowLookup, &[u8], u32) -> u32;

fn lifted(lookup: RowLookup, object: &[u8], index: u32) -> u32 {
    lookup.row_or_null(object, index)
}

fn wrong_inverted_flag(lookup: RowLookup, object: &[u8], index: u32) -> u32 {
    if read_word(object, lookup.flag_offset) != 0 {
        0
    } else {
        read_word(object, lookup.base_offset).wrapping_add(index.wrapping_mul(lookup.stride))
    }
}

fn wrong_index_left_out(lookup: RowLookup, object: &[u8], _index: u32) -> u32 {
    if read_word(object, lookup.flag_offset) == 0 {
        0
    } else {
        read_word(object, lookup.base_offset)
    }
}

fn wrong_stride_plus_four(lookup: RowLookup, object: &[u8], index: u32) -> u32 {
    if read_word(object, lookup.flag_offset) == 0 {
        0
    } else {
        read_word(object, lookup.base_offset).wrapping_add(index.wrapping_mul(lookup.stride + 4))
    }
}

fn wrong_flag_at_base_offset(lookup: RowLookup, object: &[u8], index: u32) -> u32 {
    if read_word(object, lookup.base_offset) == 0 {
        0
    } else {
        read_word(object, lookup.base_offset).wrapping_add(index.wrapping_mul(lookup.stride))
    }
}

/// The first mismatch of a candidate against the instance's rewrite, if any.
fn first_mismatch(
    instance: &Instance,
    lookup: RowLookup,
    candidate: Candidate,
    seed: u64,
) -> Option<String> {
    let mut rng = Rng(seed);
    for case in 0..CASES {
        let mut object = vec![0u8; OBJECT_BYTES];
        for byte in object.iter_mut() {
            *byte = rng.word() as u8;
        }
        // One object in four has a clear flag word, so the null path is tested.
        if rng.next() % 4 == 0 {
            object[lookup.flag_offset..lookup.flag_offset + 4].copy_from_slice(&0u32.to_le_bytes());
        }
        let index = rng.index();

        let before = object.clone();
        let this = object.as_ptr() as usize as u32;
        let returned = (instance.call)(this, index);
        if object != before {
            return Some(format!(
                "{} case {}: the rewrite changed the object, which it must only read",
                instance.name, case
            ));
        }

        let expected = candidate(lookup, &before, index);
        if returned != expected {
            return Some(format!(
                "{} case {} (index {:#x}): rewrite returned {:#x}, lift returned {:#x}",
                instance.name, case, index, returned, expected
            ));
        }
    }
    None
}

/// The lifted constants of instance `index`, as the lifted table has them.
fn lookup_of(index: usize) -> RowLookup {
    lf_core::object_rows::ROW_LOOKUPS[index]
}

macro_rules! instance_test {
    ($name:ident, $index:expr) => {
        #[test]
        fn $name() {
            let instance = &INSTANCES[$index];
            let lookup = lookup_of($index);
            assert_eq!(
                lookup.flag_offset, instance.flag_offset,
                "{}",
                instance.name
            );
            assert_eq!(
                lookup.base_offset, instance.base_offset,
                "{}",
                instance.name
            );
            assert_eq!(lookup.stride, instance.stride, "{}", instance.name);
            let seed = 0x5851_F42D_4C95_7F2D ^ ($index as u64 + 1);
            if let Some(message) = first_mismatch(instance, lookup, lifted, seed) {
                panic!("lifted method differs from the rewrite: {message}");
            }
            assert!(
                first_mismatch(instance, lookup, wrong_inverted_flag, seed).is_some(),
                "wrong version (inverted flag test) was not caught for {}",
                instance.name
            );
            assert!(
                first_mismatch(instance, lookup, wrong_index_left_out, seed).is_some(),
                "wrong version (index left out) was not caught for {}",
                instance.name
            );
            assert!(
                first_mismatch(instance, lookup, wrong_stride_plus_four, seed).is_some(),
                "wrong version (stride plus four) was not caught for {}",
                instance.name
            );
            assert!(
                first_mismatch(instance, lookup, wrong_flag_at_base_offset, seed).is_some(),
                "wrong version (flag read at the base offset) was not caught for {}",
                instance.name
            );
        }
    };
}

instance_test!(t_fn_00beb5c0, 0);
instance_test!(t_fn_00beb5e0, 1);
instance_test!(t_fn_00beb600, 2);
instance_test!(t_fn_00beb620, 3);
instance_test!(t_fn_00beb650, 4);
instance_test!(t_fn_00beb670, 5);
instance_test!(t_fn_00beb690, 6);
instance_test!(t_fn_00beb6b0, 7);
instance_test!(t_fn_00beb6d0, 8);
instance_test!(t_fn_00beb6f0, 9);
