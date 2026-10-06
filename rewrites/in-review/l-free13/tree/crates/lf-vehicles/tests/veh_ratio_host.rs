//! Host tests for the vehicle ratio coefficients: edge cases, the bank,
//! and the registry counts. Portable: NaN results assert `is_nan` only,
//! never payload bits (hardware differs); the 32-bit differential crates
//! pin every bit.

use lf_vehicles::veh_ratio::registry::{self, State};
use lf_vehicles::veh_ratio::{RatioBank, RatioCell};

/// Refreshes a cell built from raw source bits and returns the value bits.
fn refreshed(numer: u32, denom: u32) -> u32 {
    let mut cell = RatioCell::new(
        f32::from_bits(numer),
        f32::from_bits(denom),
        f32::from_bits(0xDEAD_BEEF),
    );
    cell.refresh();
    cell.value().to_bits()
}

#[test]
fn ordinary_quotients_are_exact() {
    assert_eq!(refreshed(0x3F80_0000, 0x4000_0000), 0x3F00_0000); // 1/2 = 0.5
    assert_eq!(refreshed(0x4000_0000, 0x3F80_0000), 0x4000_0000); // 2/1 = 2
    assert_eq!(refreshed(0x4480_0000, 0x4400_0000), 0x3FAA_AAAB); // 1024/768
    assert_eq!(refreshed(0xC000_0000, 0x4000_0000), 0xBF80_0000); // -2/2 = -1
    assert_eq!(refreshed(0x0000_0001, 0x0000_0001), 0x3F80_0000); // min-sub/min-sub = 1
}

#[test]
fn zeros_follow_ieee_sign_rules() {
    assert_eq!(refreshed(0x0000_0000, 0x3F80_0000), 0x0000_0000); // +0/1 = +0
    assert_eq!(refreshed(0x8000_0000, 0x3F80_0000), 0x8000_0000); // -0/1 = -0
    assert_eq!(refreshed(0x3F80_0000, 0x0000_0000), 0x7F80_0000); // 1/+0 = +inf
    assert_eq!(refreshed(0x3F80_0000, 0x8000_0000), 0xFF80_0000); // 1/-0 = -inf
    assert_eq!(refreshed(0x8000_0000, 0x3F80_0000), 0x8000_0000); // -0/1 = -0
}

#[test]
fn zero_over_zero_is_nan() {
    assert!(f32::from_bits(refreshed(0x0000_0000, 0x0000_0000)).is_nan());
    assert!(f32::from_bits(refreshed(0x8000_0000, 0x0000_0000)).is_nan());
    assert!(f32::from_bits(refreshed(0x8000_0000, 0x8000_0000)).is_nan());
}

#[test]
fn infinities_propagate() {
    assert_eq!(refreshed(0x7F80_0000, 0x4000_0000), 0x7F80_0000); // inf/2
    assert_eq!(refreshed(0x4000_0000, 0x7F80_0000), 0x0000_0000); // 2/inf = +0
    assert_eq!(refreshed(0x4000_0000, 0xFF80_0000), 0x8000_0000); // 2/-inf = -0
    assert!(f32::from_bits(refreshed(0x7F80_0000, 0x7F80_0000)).is_nan()); // inf/inf
    assert!(f32::from_bits(refreshed(0xFF80_0000, 0x7F80_0000)).is_nan());
}

#[test]
fn nan_inputs_stay_nan() {
    assert!(f32::from_bits(refreshed(0x7FC0_0000, 0x3F80_0000)).is_nan());
    assert!(f32::from_bits(refreshed(0x3F80_0000, 0x7FC0_0000)).is_nan());
    assert!(f32::from_bits(refreshed(0xFFC0_0000, 0xFFC0_0000)).is_nan());
}

#[test]
fn subnormals_divide_without_traps() {
    assert_eq!(refreshed(0x0000_0001, 0x3F80_0000), 0x0000_0001); // min-sub/1
    assert_eq!(refreshed(0x007F_FFFF, 0x3F80_0000), 0x007F_FFFF); // max-sub/1
    assert_eq!(refreshed(0x3F80_0000, 0x7F7F_FFFF), 0x0020_0000); // 1/max = 2^-128 subnormal
}

#[test]
fn cell_accessors_round_trip() {
    let mut cell = RatioCell::new(1.0, 2.0, 0.0);
    assert_eq!((cell.numer(), cell.denom(), cell.value()), (1.0, 2.0, 0.0));
    cell.set_sources(3.0, 4.0);
    assert_eq!((cell.numer(), cell.denom(), cell.value()), (3.0, 4.0, 0.0));
    cell.refresh();
    assert_eq!(cell.value().to_bits(), 0x3F40_0000); // 3/4 = 0.75
}

#[test]
fn bank_refreshes_one_cell() {
    let mut bank = RatioBank::new(vec![
        RatioCell::new(1.0, 2.0, 9.0),
        RatioCell::new(1.0, 4.0, 9.0),
    ]);
    assert_eq!(bank.len(), 2);
    assert!(!bank.is_empty());
    bank.refresh(1);
    assert_eq!(bank.get(0).unwrap().value().to_bits(), 0x4110_0000); // untouched 9.0
    assert_eq!(bank.get(1).unwrap().value().to_bits(), 0x3E80_0000); // 1/4
    assert!(bank.get(2).is_none());
}

#[test]
fn bank_refresh_all_covers_every_cell() {
    let mut bank = RatioBank::new(vec![
        RatioCell::new(1.0, 2.0, 0.0),
        RatioCell::new(2.0, 1.0, 0.0),
        RatioCell::new(1.0, 0.0, 0.0),
    ]);
    bank.refresh_all();
    assert_eq!(bank.get(0).unwrap().value().to_bits(), 0x3F00_0000);
    assert_eq!(bank.get(1).unwrap().value().to_bits(), 0x4000_0000);
    assert_eq!(bank.get(2).unwrap().value().to_bits(), 0x7F80_0000);
}

#[test]
fn empty_bank_refresh_all_is_quiet() {
    let mut bank = RatioBank::new(Vec::new());
    assert!(bank.is_empty());
    assert_eq!(bank.len(), 0);
    bank.refresh_all();
}

#[test]
#[should_panic]
fn bank_refresh_past_the_end_panics() {
    let mut bank = RatioBank::new(vec![RatioCell::new(1.0, 1.0, 0.0)]);
    bank.refresh(1);
}

#[test]
fn registry_covers_the_whole_family() {
    assert_eq!(registry::ROWS.len(), 113);
    assert!(registry::ROWS.iter().all(|r| r.state == State::Proven));
    let mut indexes: Vec<usize> = registry::ROWS.iter().map(|r| r.index).collect();
    indexes.sort_unstable();
    assert_eq!(indexes, (0..113).collect::<Vec<_>>());
    let mut names: Vec<&str> = registry::ROWS.iter().map(|r| r.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), 113);
}
