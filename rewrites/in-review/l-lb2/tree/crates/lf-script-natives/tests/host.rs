//! Host tests for the lifted native dispatch: descriptor sanity, the three
//! forwarding functions against a recording fake, and the panic domains.

use lf_script_natives::desc::{DISCARD_DESCS, FULL_DESCS, MASKED_DESCS, ZERO_DESCS};
use lf_script_natives::{
    NativeDesc, NativeEngine, forward, forward_full, forward_masked, forward_zero,
};

struct Fake {
    answer: u32,
    seen: Vec<(String, Vec<u32>)>,
}

impl NativeEngine for Fake {
    fn call(&mut self, native: &NativeDesc, args: &[u32]) -> u32 {
        self.seen
            .push((native.name.to_string(), args.to_vec()));
        self.answer
    }
}

#[test]
fn descriptors_are_sane() {
    for (table, want) in [
        (DISCARD_DESCS, "discard"),
        (MASKED_DESCS, "masked"),
        (FULL_DESCS, "full"),
        (ZERO_DESCS, "zero"),
    ] {
        assert!(!table.is_empty(), "{want} table is empty");
        for d in table {
            assert!(!d.name.is_empty());
            assert!(d.nargs as usize <= 24, "{} nargs {}", d.name, d.nargs);
            assert!(
                d.coerce >> d.nargs == 0,
                "{} coerces beyond nargs ({:#x} >> {})",
                d.name,
                d.coerce,
                d.nargs
            );
            if d.quirk >= 0 {
                assert!(
                    d.coerce >> d.quirk & 1 == 1,
                    "{} quirks uncoerced word {}",
                    d.name,
                    d.quirk
                );
            }
            assert!(!d.forwards_ctx, "{} forwards ctx", d.name);
        }
        let _ = want;
    }
    assert_eq!(DISCARD_DESCS.len(), 1371);
    assert_eq!(MASKED_DESCS.len(), 529);
    assert_eq!(FULL_DESCS.len(), 211);
    assert_eq!(ZERO_DESCS.len(), 51);
}

#[test]
fn forward_coerces_and_returns_the_answer() {
    let desc = NativeDesc {
        name: "t",
        hash: 1,
        nargs: 3,
        coerce: 0b101,
        quirk: -1,
        forwards_ctx: false,
    };
    let mut fake = Fake {
        answer: 0x1234_5678,
        seen: Vec::new(),
    };
    let out = forward(&mut fake, &desc, &[7, 8, 0]);
    assert_eq!(out, 0x1234_5678);
    assert_eq!(fake.seen.len(), 1);
    assert_eq!(fake.seen[0].1, vec![1, 8, 0]);
}

#[test]
fn masked_and_full_store_and_return() {
    let desc = NativeDesc {
        name: "t",
        hash: 1,
        nargs: 1,
        coerce: 0,
        quirk: -1,
        forwards_ctx: false,
    };
    let mut fake = Fake {
        answer: 0xABC,
        seen: Vec::new(),
    };
    let mut slot = 0xDEAD;
    assert_eq!(forward_masked(&mut fake, &desc, &[0], &mut slot), 0xBC);
    assert_eq!(slot, 0xBC);
    assert_eq!(forward_full(&mut fake, &desc, &[0], &mut slot), 0xABC);
    assert_eq!(slot, 0xABC);
}

#[test]
fn zero_forwards_and_returns_zero() {
    let desc = NativeDesc {
        name: "t",
        hash: 1,
        nargs: 2,
        coerce: 0b10,
        quirk: -1,
        forwards_ctx: false,
    };
    let mut fake = Fake {
        answer: 99,
        seen: Vec::new(),
    };
    assert_eq!(forward_zero(&mut fake, &desc, &[5, 6]), 0);
    assert_eq!(fake.seen[0].1, vec![5, 1]);
}

#[test]
fn extra_input_words_are_ignored() {
    let desc = NativeDesc {
        name: "t",
        hash: 1,
        nargs: 1,
        coerce: 0,
        quirk: -1,
        forwards_ctx: false,
    };
    let mut fake = Fake {
        answer: 0,
        seen: Vec::new(),
    };
    forward(&mut fake, &desc, &[5, 6, 7]);
    assert_eq!(fake.seen[0].1, vec![5]);
}

#[test]
#[should_panic(expected = "descriptor wants 2")]
fn short_input_panics() {
    let desc = NativeDesc {
        name: "t",
        hash: 1,
        nargs: 2,
        coerce: 0,
        quirk: -1,
        forwards_ctx: false,
    };
    let mut fake = Fake {
        answer: 0,
        seen: Vec::new(),
    };
    forward(&mut fake, &desc, &[5]);
}
