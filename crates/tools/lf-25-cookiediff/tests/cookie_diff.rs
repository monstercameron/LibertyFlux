//! Differential test of the lifted cookie-seal routine against the verified
//! rewrites of its batch.
//!
//! Each instance stamps an object: it reads the seal counter and the object's
//! cookie word, writes a marker, a cookie, the counter and the vtable word, and
//! stores a value, then returns the object's address. For each instance, CASES
//! generated cases are run. Each case fills the object with random bytes, sets
//! the counter and the value to random words, and runs the rewrite on the object
//! at its own address. The lifted routine (`CookieSeal::seal`) runs on a copy of
//! the same bytes with the same counter and value. The returned address, every
//! byte of the object and the counter are compared.
//!
//! Wrong versions, each run against every instance and each required to fail:
//! the cookie store left out, the mask with one bit cleared, the counter not
//! advanced, and the value stored four bytes further on.
//!
//! The test runs on the 32-bit target only, where the rewrites exist.

#![cfg(target_arch = "x86")]
// The counter is read and written through the runtime's accessor, which checks
// the address; this test program is the only place that uses unsafe.
#![allow(unsafe_code)]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{CookieInstance, INSTANCES};
use lf_25_cookiediff::rt::COUNTER_VA;
use lf_core::field_store::ObjectBytes;
use lf_core::object_cookie::{COOKIE_OFFSET, CookieSeal, VALUE_OFFSET, ValueWidth, cookie};

/// Bytes in the generated object. Every field the routines write lies below this.
const OBJECT_BYTES: usize = 0x200;
/// Cases per instance for the differential comparison.
const CASES: u32 = 1000;
/// Cases per instance and wrong version; a wrong version must be caught by one of them.
const WRONG_CASES: u32 = 64;
/// The marker the batch writes: the same in every instance.
const MARKER: u32 = 0x00E7_E048;
/// The number of instances the generated table lists.
const INSTANCE_COUNT: usize = 4;

/// One lock across the whole test program: the rewrites share the counter.
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

/// One generated case: an object, a counter and a value.
#[derive(Clone)]
struct Case {
    object: Object,
    counter: u32,
    value: u32,
}

fn case_for(seed: u32) -> Case {
    let mut rng = Rng::new(seed);
    let mut object = Object([0; OBJECT_BYTES]);
    for chunk in object.0.chunks_mut(4) {
        let word = rng.next_u32().to_le_bytes();
        chunk.copy_from_slice(&word[..chunk.len()]);
    }
    let counter = rng.next_u32();
    let value = rng.next_u32();
    Case {
        object,
        counter,
        value,
    }
}

/// The counter word of the runtime.
fn set_counter(value: u32) {
    // SAFETY: the test holds the lock, and the accessor checks the address.
    unsafe { lf_25_cookiediff::global::<u32>(COUNTER_VA).write(value) }
}

fn counter() -> u32 {
    // SAFETY: the test holds the lock, and the accessor checks the address.
    unsafe { lf_25_cookiediff::global::<u32>(COUNTER_VA).read() }
}

/// A lifted candidate: given the object, the counter and the value, it seals the
/// object and returns the address the caller should report.

fn lifted(seal: &CookieSeal) -> impl Fn(&mut ObjectBytes, &mut u32, u32) + '_ {
    move |object, counter, value| seal.seal(object, counter, value)
}

/// The first mismatch of a candidate against the instance's rewrite, if any.
fn first_mismatch(
    instance: &CookieInstance,
    candidate: &dyn Fn(&mut ObjectBytes, &mut u32, u32),
    cases: u32,
    seed: u32,
) -> Option<String> {
    for case in 0..cases {
        let mut run =
            case_for(seed.wrapping_mul(0x9E37_79B9) ^ (case + 1).wrapping_mul(0x85EB_CA6B));
        let original = run.clone();
        let this = run.object.0.as_mut_ptr() as usize as u32;
        set_counter(run.counter);
        let returned = (instance.call)(this, run.value);
        let rewrite_counter = counter();
        if returned != this {
            return Some(format!(
                "{} case {}: rewrite returned {:#x}, the object's address is {:#x}",
                instance.name, case, returned, this
            ));
        }

        let mut copy = ObjectBytes::from_bytes(&original.object.0);
        let mut lifted_counter = original.counter;
        candidate(&mut copy, &mut lifted_counter, original.value);
        if lifted_counter != rewrite_counter {
            return Some(format!(
                "{} case {}: rewrite left the counter {:#x}, the lift left {:#x}",
                instance.name, case, rewrite_counter, lifted_counter
            ));
        }
        if let Some(offset) = (0..OBJECT_BYTES).find(|&k| run.object.0[k] != copy.byte(k)) {
            return Some(format!(
                "{} case {}: byte {:#x} of the object is {:#x} in the rewrite and {:#x} in the lift",
                instance.name,
                case,
                offset,
                run.object.0[offset],
                copy.byte(offset)
            ));
        }
    }
    None
}

/// A deliberately wrong version of a routine.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The cookie store is left out.
    CookieLeftOut,
    /// The mask has one bit cleared.
    MaskBitCleared,
    /// The counter is not advanced.
    CounterNotAdvanced,
    /// The value is stored four bytes further on.
    ValueShifted,
}

const WRONG_KINDS: [Wrong; 4] = [
    Wrong::CookieLeftOut,
    Wrong::MaskBitCleared,
    Wrong::CounterNotAdvanced,
    Wrong::ValueShifted,
];

/// The lifted routine run as the wrong version describes.
fn run_wrong(
    seal: &CookieSeal,
    wrong: Wrong,
    object: &mut ObjectBytes,
    counter: &mut u32,
    value: u32,
) {
    let count = *counter;
    let previous = object.word(COOKIE_OFFSET);
    object.write(0, &seal.marker.to_le_bytes());
    match wrong {
        Wrong::CookieLeftOut => {}
        Wrong::MaskBitCleared => {
            object.write(
                COOKIE_OFFSET,
                &(previous ^ ((previous ^ count) & 0x3FFE)).to_le_bytes(),
            );
        }
        _ => object.write(COOKIE_OFFSET, &cookie(previous, count).to_le_bytes()),
    }
    if !matches!(wrong, Wrong::CounterNotAdvanced) {
        *counter = count.wrapping_add(1);
    }
    object.write(0, &seal.vtable.to_le_bytes());
    let offset = match wrong {
        Wrong::ValueShifted => VALUE_OFFSET + 4,
        _ => VALUE_OFFSET,
    };
    match seal.value_width {
        ValueWidth::Word => object.write(offset, &value.to_le_bytes()),
        ValueWidth::Byte => object.write(offset, &[value as u8]),
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
            let seal = CookieSeal::new(MARKER, instance.vtable, instance.width);
            let seed = 0x5851_F42D ^ ($index as u32 + 1).wrapping_mul(0x9E37_79B9);
            let right = lifted(&seal);
            if let Some(message) = first_mismatch(instance, &right, CASES, seed) {
                panic!("lifted routine differs from the rewrite: {message}");
            }
            for wrong in WRONG_KINDS {
                let wrong_seal = seal;
                let candidate = move |object: &mut ObjectBytes, counter: &mut u32, value: u32| {
                    run_wrong(&wrong_seal, wrong, object, counter, value)
                };
                assert!(
                    first_mismatch(instance, &candidate, WRONG_CASES, seed).is_some(),
                    "wrong version ({wrong:?}) was not caught for {}",
                    instance.name
                );
            }
        }
    };
}

instance_test!(test_fn_008dbba0, 0, "rw_008dbba0");
instance_test!(test_fn_008dbc20, 1, "rw_008dbc20");
instance_test!(test_fn_008dbc90, 2, "rw_008dbc90");
instance_test!(test_fn_008dbcc0, 3, "rw_008dbcc0");
