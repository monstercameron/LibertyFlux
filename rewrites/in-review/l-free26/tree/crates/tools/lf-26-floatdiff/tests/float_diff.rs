//! Differential test of the lifted conditional float store (`lf_core::float_if_changed`) against the four verified
//! 32-bit rewrites of its family.
//!
//! For each instance, `CASES` generated cases run. A case is a generated object, whose float field holds a
//! generated value, and an incoming value. The incoming value is drawn from several pools so that every branch
//! is taken: a value equal to the stored one, the same magnitude with the sign flipped (so positive and negative
//! zero meet), arbitrary bit patterns, and a fixed set of special values that includes quiet and signaling
//! not-a-number patterns with their payloads. The rewrite runs on the object in memory and the lifted routine on
//! a copy of the same object. Every byte of the object is compared. The rewrite returns nothing, so no return
//! value is compared.
//!
//! Deliberately wrong versions of the lifted routine run through the same contract, and each must be caught by
//! at least one case of every instance: always writing, never writing, comparing the bits instead of the values
//! (which tells positive and negative zero apart), and writing four bytes further on.
//!
//! The test runs on the 32-bit target only, where the rewrites exist. Its inputs come from fixed seeds.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

#[path = "support/instances_gen.rs"]
mod instances;

use instances::INSTANCES;
use lf_core::field_store::ObjectBytes;
use lf_core::float_if_changed::FloatIfChanged;

/// Generated cases per instance.
const CASES: u32 = 1000;

/// Bytes in the generated object. Every float field the instances use lies below this.
const OBJECT_BYTES: usize = 0x200;

/// Bit patterns that matter to the comparison: zeros of both signs, one, infinities, quiet and signaling
/// not-a-number patterns with and without a sign and a payload, a subnormal and the largest finite value.
const SPECIAL: [u32; 14] = [
    0x0000_0000,
    0x8000_0000,
    0x3F80_0000,
    0xBF80_0000,
    0x7F80_0000,
    0xFF80_0000,
    0x7FC0_0000,
    0x7FC0_1234,
    0xFFC0_0000,
    0x7F80_0001,
    0xFF80_0001,
    0x0000_0001,
    0x7F7F_FFFF,
    0x8000_0001,
];

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

/// One generated case: the object before the call (its float field already holds the stored value) and the
/// incoming value's bits.
struct Case {
    object: Object,
    incoming: u32,
}

/// The incoming bits, drawn from the pools described in the module documentation.
fn pick_incoming(rng: &mut Rng, stored: u32) -> u32 {
    match rng.next_u32() % 4 {
        0 => stored,
        1 => stored ^ 0x8000_0000,
        2 => SPECIAL[(rng.next_u32() % SPECIAL.len() as u32) as usize],
        _ => rng.next_u32(),
    }
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
    let stored = if rng.next_u32() % 3 == 0 {
        SPECIAL[(rng.next_u32() % SPECIAL.len() as u32) as usize]
    } else {
        rng.next_u32()
    };
    let offset = INSTANCES[instance].offset;
    object.0[offset..offset + 4].copy_from_slice(&stored.to_le_bytes());
    let incoming = pick_incoming(&mut rng, stored);
    Case { object, incoming }
}

/// The verified rewrite on one case: the object afterwards.
fn run_rewrite(instance: usize, case: &Case) -> Object {
    let mut object = case.object.clone();
    let this = object.0.as_mut_ptr() as usize as u32;
    (INSTANCES[instance].rewrite)(this, f32::from_bits(case.incoming));
    object
}

/// The lifted routine on a copy of the case's object.
fn run_lifted(instance: usize, case: &Case) -> Vec<u8> {
    let routine = FloatIfChanged::new(INSTANCES[instance].offset);
    let mut object = ObjectBytes::from_bytes(&case.object.0);
    routine.set(&mut object, case.incoming);
    object.bytes().to_vec()
}

/// A deliberately wrong version of the lifted routine.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The incoming value is always written.
    AlwaysWrite,
    /// The incoming value is never written.
    NeverWrite,
    /// The two values are compared as bits, so positive and negative zero differ.
    BitwiseCompare,
    /// The value is written four bytes past the field.
    OffsetPlusFour,
}

/// The wrong version on one case: the object afterwards.
fn run_wrong(wrong: Wrong, instance: usize, case: &Case) -> Vec<u8> {
    let offset = INSTANCES[instance].offset;
    let mut object = ObjectBytes::from_bytes(&case.object.0);
    let stored = object.word(offset);
    let incoming = case.incoming.to_le_bytes();
    match wrong {
        Wrong::AlwaysWrite => object.write(offset, &incoming),
        Wrong::NeverWrite => {}
        Wrong::BitwiseCompare => {
            if stored != case.incoming {
                object.write(offset, &incoming);
            }
        }
        Wrong::OffsetPlusFour => {
            if f32::from_bits(stored) != f32::from_bits(case.incoming) {
                object.write(offset + 4, &incoming);
            }
        }
    }
    object.bytes().to_vec()
}

#[test]
fn lifted_routine_matches_every_verified_rewrite() {
    for instance in 0..INSTANCES.len() {
        for case in 0..CASES {
            let generated = make_case(instance, case);
            let rewrite_object = run_rewrite(instance, &generated);
            let lifted_object = run_lifted(instance, &generated);
            assert_eq!(
                lifted_object,
                rewrite_object.0.to_vec(),
                "{} case {case}: object bytes differ (incoming {:#010x})",
                INSTANCES[instance].name,
                generated.incoming
            );
        }
    }
}

#[test]
fn every_wrong_version_is_caught_by_every_instance() {
    let wrongs = [
        Wrong::AlwaysWrite,
        Wrong::NeverWrite,
        Wrong::BitwiseCompare,
        Wrong::OffsetPlusFour,
    ];
    for instance in 0..INSTANCES.len() {
        for wrong in wrongs {
            let mut caught = 0u32;
            for case in 0..CASES {
                let generated = make_case(instance, case);
                let rewrite_object = run_rewrite(instance, &generated);
                if run_wrong(wrong, instance, &generated) != rewrite_object.0.to_vec() {
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
