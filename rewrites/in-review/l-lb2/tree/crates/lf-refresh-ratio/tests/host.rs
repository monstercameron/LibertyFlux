//! Host tests: descriptor sanity and the refresh function.

use lf_refresh_ratio::{RatioBank, RatioDesc, RatioSlot, desc::DESCS, refresh_ratio};

fn eq_f32(a: f32, b: f32) -> bool {
    a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan())
}

#[test]
fn ratio_refreshes_each_slot() {
    assert_eq!(DESCS.len(), 69);
    for (i, d) in DESCS.iter().enumerate() {
        assert!(!d.name.is_empty());
        assert_eq!(d.index as usize, i);
    }
    let mut bank = RatioBank {
        slots: vec![
            RatioSlot {
                num: 7.0,
                den: 2.0,
                out: 0.0
            };
            DESCS.len()
        ],
    };
    refresh_ratio(&mut bank, &DESCS[0]);
    assert!(eq_f32(bank.slots[0].out, 3.5));
    assert!(eq_f32(bank.slots[1].out, 0.0));
    bank.slots[5] = RatioSlot {
        num: 1.0,
        den: 0.0,
        out: 0.0,
    };
    refresh_ratio(
        &mut bank,
        &RatioDesc {
            name: "t",
            index: 5,
        },
    );
    assert!(eq_f32(bank.slots[5].out, f32::INFINITY));
    bank.slots[6] = RatioSlot {
        num: 0.0,
        den: 0.0,
        out: 0.0,
    };
    refresh_ratio(
        &mut bank,
        &RatioDesc {
            name: "t",
            index: 6,
        },
    );
    assert!(bank.slots[6].out.is_nan());
}

#[test]
#[should_panic(expected = "outside 69 slots")]
fn ratio_index_outside_bank_panics() {
    let mut bank = RatioBank {
        slots: vec![
            RatioSlot {
                num: 1.0,
                den: 1.0,
                out: 0.0
            };
            DESCS.len()
        ],
    };
    refresh_ratio(
        &mut bank,
        &RatioDesc {
            name: "t",
            index: 10_000,
        },
    );
}
